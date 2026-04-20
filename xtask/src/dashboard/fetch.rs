//! GraphQL queries + pagination. Parses the raw `serde_json::Value`
//! responses into internal node types consumed by `snapshot.rs`.
//!
//! The queries are intentionally narrow — we ask for exactly the fields
//! the `state.json` schema needs, nothing more. Adding a field here is
//! cheap; adding one we don't use inflates rate-limit cost.

use anyhow::{Context, Result};
use serde_json::{Value, json};

use super::gh::GraphQLFetcher;

#[derive(Debug)]
pub struct Fetched {
    pub issues: Vec<IssueNode>,
    pub pulls: Vec<PullNode>,
    pub milestones: Vec<MilestoneNode>,
    pub project_items: Option<Vec<ProjectItemNode>>,
}

#[derive(Debug)]
pub struct IssueNode {
    pub title: String,
    pub url: String,
    pub state: String,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub labels: Vec<String>,
}

#[derive(Debug)]
pub struct PullNode {
    pub title: String,
    pub url: String,
    pub state: String,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
}

#[derive(Debug)]
pub struct MilestoneNode {
    pub title: String,
    pub state: String,
    pub due_on: Option<String>,
    pub open_issue_count: u32,
    pub closed_issue_count: u32,
}

#[derive(Debug)]
pub struct ProjectItemNode {
    pub status: Option<String>,
}

const ISSUES_QUERY: &str = r"
query($owner:String!, $repo:String!, $cursor:String) {
  repository(owner:$owner, name:$repo) {
    issues(first:100, after:$cursor, states:[OPEN, CLOSED], orderBy:{field:UPDATED_AT, direction:DESC}) {
      pageInfo { hasNextPage endCursor }
      nodes {
        title url state createdAt closedAt
        labels(first:20) { nodes { name } }
      }
    }
  }
}";

const PULLS_QUERY: &str = r"
query($owner:String!, $repo:String!, $cursor:String) {
  repository(owner:$owner, name:$repo) {
    pullRequests(first:100, after:$cursor, states:[OPEN, CLOSED, MERGED], orderBy:{field:UPDATED_AT, direction:DESC}) {
      pageInfo { hasNextPage endCursor }
      nodes { title url state createdAt closedAt mergedAt }
    }
  }
}";

const MILESTONES_QUERY: &str = r"
query($owner:String!, $repo:String!) {
  repository(owner:$owner, name:$repo) {
    milestones(first:50, states:[OPEN, CLOSED], orderBy:{field:DUE_DATE, direction:ASC}) {
      nodes { title state dueOn openIssueCount closedIssueCount }
    }
  }
}";

const PROJECTS_QUERY: &str = r"
query($owner:String!, $repo:String!, $cursor:String) {
  repository(owner:$owner, name:$repo) {
    projectsV2(first:1) {
      nodes {
        number
        items(first:100, after:$cursor) {
          pageInfo { hasNextPage endCursor }
          nodes {
            fieldValues(first:20) {
              nodes {
                __typename
                ... on ProjectV2ItemFieldSingleSelectValue {
                  name
                  field { ... on ProjectV2SingleSelectField { name } }
                }
              }
            }
          }
        }
      }
    }
  }
}";

pub fn fetch_all(f: &dyn GraphQLFetcher, owner: &str, repo: &str) -> Result<Fetched> {
    Ok(Fetched {
        issues: fetch_issues(f, owner, repo)?,
        pulls: fetch_pulls(f, owner, repo)?,
        milestones: fetch_milestones(f, owner, repo)?,
        project_items: fetch_project_items(f, owner, repo)?,
    })
}

fn fetch_issues(f: &dyn GraphQLFetcher, owner: &str, repo: &str) -> Result<Vec<IssueNode>> {
    let mut out = Vec::new();
    let mut cursor: Option<String> = None;
    let mut page = 1;
    loop {
        let vars = json!({ "owner": owner, "repo": repo, "cursor": cursor });
        let label = format!("issues_p{page}");
        let resp = f.run(&label, ISSUES_QUERY, &vars)?;
        let root = resp
            .pointer("/data/repository/issues")
            .context("issues: missing repository.issues")?;
        for node in root["nodes"].as_array().unwrap_or(&vec![]) {
            out.push(parse_issue(node)?);
        }
        if !page_has_next(root) {
            break;
        }
        cursor = page_end_cursor(root).map(str::to_string);
        page += 1;
    }
    Ok(out)
}

fn fetch_pulls(f: &dyn GraphQLFetcher, owner: &str, repo: &str) -> Result<Vec<PullNode>> {
    let mut out = Vec::new();
    let mut cursor: Option<String> = None;
    let mut page = 1;
    loop {
        let vars = json!({ "owner": owner, "repo": repo, "cursor": cursor });
        let label = format!("pulls_p{page}");
        let resp = f.run(&label, PULLS_QUERY, &vars)?;
        let root = resp
            .pointer("/data/repository/pullRequests")
            .context("pulls: missing repository.pullRequests")?;
        for node in root["nodes"].as_array().unwrap_or(&vec![]) {
            out.push(parse_pull(node)?);
        }
        if !page_has_next(root) {
            break;
        }
        cursor = page_end_cursor(root).map(str::to_string);
        page += 1;
    }
    Ok(out)
}

fn fetch_milestones(f: &dyn GraphQLFetcher, owner: &str, repo: &str) -> Result<Vec<MilestoneNode>> {
    let vars = json!({ "owner": owner, "repo": repo });
    let resp = f.run("milestones", MILESTONES_QUERY, &vars)?;
    let mut out = Vec::new();
    for node in resp
        .pointer("/data/repository/milestones/nodes")
        .and_then(Value::as_array)
        .context("milestones: missing repository.milestones.nodes")?
    {
        out.push(parse_milestone(node)?);
    }
    Ok(out)
}

fn fetch_project_items(
    f: &dyn GraphQLFetcher,
    owner: &str,
    repo: &str,
) -> Result<Option<Vec<ProjectItemNode>>> {
    let mut out = Vec::new();
    let mut cursor: Option<String> = None;
    let mut page = 1;
    loop {
        let vars = json!({ "owner": owner, "repo": repo, "cursor": cursor });
        let label = format!("projects_p{page}");
        let resp = f.run(&label, PROJECTS_QUERY, &vars)?;
        let projects = resp
            .pointer("/data/repository/projectsV2/nodes")
            .and_then(Value::as_array);
        let Some(project) = projects.and_then(|v| v.first()) else {
            return Ok(None);
        };
        let items = project
            .pointer("/items")
            .context("projects_v2: missing items")?;
        for node in items["nodes"].as_array().unwrap_or(&vec![]) {
            out.push(parse_project_item(node));
        }
        if !page_has_next(items) {
            break;
        }
        cursor = page_end_cursor(items).map(str::to_string);
        page += 1;
    }
    Ok(Some(out))
}

fn page_has_next(root: &Value) -> bool {
    root.pointer("/pageInfo/hasNextPage")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn page_end_cursor(root: &Value) -> Option<&str> {
    root.pointer("/pageInfo/endCursor").and_then(Value::as_str)
}

fn parse_issue(node: &Value) -> Result<IssueNode> {
    Ok(IssueNode {
        title: str_field(node, "title")?.to_string(),
        url: str_field(node, "url")?.to_string(),
        state: str_field(node, "state")?.to_string(),
        created_at: str_field(node, "createdAt")?.to_string(),
        closed_at: node
            .get("closedAt")
            .and_then(Value::as_str)
            .map(str::to_string),
        labels: node
            .pointer("/labels/nodes")
            .and_then(Value::as_array)
            .map(|a| {
                a.iter()
                    .filter_map(|l| l.get("name").and_then(Value::as_str).map(str::to_string))
                    .collect()
            })
            .unwrap_or_default(),
    })
}

fn parse_pull(node: &Value) -> Result<PullNode> {
    Ok(PullNode {
        title: str_field(node, "title")?.to_string(),
        url: str_field(node, "url")?.to_string(),
        state: str_field(node, "state")?.to_string(),
        created_at: str_field(node, "createdAt")?.to_string(),
        closed_at: node
            .get("closedAt")
            .and_then(Value::as_str)
            .map(str::to_string),
        merged_at: node
            .get("mergedAt")
            .and_then(Value::as_str)
            .map(str::to_string),
    })
}

fn parse_milestone(node: &Value) -> Result<MilestoneNode> {
    Ok(MilestoneNode {
        title: str_field(node, "title")?.to_string(),
        state: str_field(node, "state")?.to_string(),
        due_on: node
            .get("dueOn")
            .and_then(Value::as_str)
            .map(str::to_string),
        open_issue_count: u32_field(node, "openIssueCount"),
        closed_issue_count: u32_field(node, "closedIssueCount"),
    })
}

fn parse_project_item(node: &Value) -> ProjectItemNode {
    let status = node
        .pointer("/fieldValues/nodes")
        .and_then(Value::as_array)
        .and_then(|fvs| {
            fvs.iter().find_map(|fv| {
                let field_name = fv.pointer("/field/name").and_then(Value::as_str);
                if field_name == Some("Status") {
                    fv.get("name").and_then(Value::as_str).map(str::to_string)
                } else {
                    None
                }
            })
        });
    ProjectItemNode { status }
}

fn str_field<'a>(node: &'a Value, key: &str) -> Result<&'a str> {
    node.get(key)
        .and_then(Value::as_str)
        .with_context(|| format!("missing field `{key}`"))
}

fn u32_field(node: &Value, key: &str) -> u32 {
    node.get(key)
        .and_then(Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
        .unwrap_or(0)
}
