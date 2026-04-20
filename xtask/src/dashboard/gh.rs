//! GraphQL fetcher abstraction with two implementations:
//!
//! - [`GhCli`] shells out to `gh api graphql --input -` (production).
//! - [`FixtureFetcher`] reads pre-recorded JSON responses (tests).
//!
//! The trait keeps the snapshot pipeline hermetic — unit and integration
//! tests never touch the network.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use serde_json::Value;

pub trait GraphQLFetcher {
    /// Run one GraphQL query. `label` disambiguates fixture lookups and
    /// identifies the call in logs; production impls ignore it.
    fn run(&self, label: &str, query: &str, variables: &Value) -> Result<Value>;
}

pub struct GhCli;

impl GraphQLFetcher for GhCli {
    fn run(&self, label: &str, query: &str, variables: &Value) -> Result<Value> {
        tracing::debug!(label, "gh api graphql");
        let payload = serde_json::json!({ "query": query, "variables": variables });
        let mut child = Command::new("gh")
            .args(["api", "graphql", "--input", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("failed to spawn `gh`; install it or check $PATH")?;
        {
            let mut stdin = child.stdin.take().context("child stdin already closed")?;
            stdin.write_all(&serde_json::to_vec(&payload)?)?;
        }
        let out = child.wait_with_output()?;
        if !out.status.success() {
            bail!(
                "gh api graphql (label={label}) exited {:?}: {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        let json: Value =
            serde_json::from_slice(&out.stdout).context("gh api graphql did not return JSON")?;
        if let Some(errs) = json.get("errors") {
            bail!("GraphQL errors (label={label}): {errs}");
        }
        Ok(json)
    }
}

pub struct FixtureFetcher {
    base: PathBuf,
}

impl FixtureFetcher {
    pub fn new(base: impl Into<PathBuf>) -> Self {
        Self { base: base.into() }
    }
}

impl GraphQLFetcher for FixtureFetcher {
    fn run(&self, label: &str, _query: &str, _variables: &Value) -> Result<Value> {
        let path = self.base.join(format!("{label}.json"));
        let bytes =
            std::fs::read(&path).with_context(|| format!("fixture missing: {}", path.display()))?;
        serde_json::from_slice(&bytes)
            .with_context(|| format!("fixture not valid JSON: {}", path.display()))
    }
}

/// Resolve `(owner, repo)` from `gh repo view --json owner,name`.
///
/// This is the same source of truth `gh` uses everywhere, so a contributor
/// working on a fork sees their fork's data. Falls back to env overrides
/// `PHYSA_SNAPSHOT_OWNER` / `PHYSA_SNAPSHOT_REPO` to help CI pin a target.
pub fn resolve_repo() -> Result<(String, String)> {
    if let (Ok(owner), Ok(repo)) = (
        std::env::var("PHYSA_SNAPSHOT_OWNER"),
        std::env::var("PHYSA_SNAPSHOT_REPO"),
    ) {
        return Ok((owner, repo));
    }
    let out = Command::new("gh")
        .args(["repo", "view", "--json", "owner,name"])
        .output()
        .context("failed to run `gh repo view`")?;
    if !out.status.success() {
        bail!(
            "`gh repo view` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let v: Value = serde_json::from_slice(&out.stdout)?;
    let owner = v
        .pointer("/owner/login")
        .and_then(Value::as_str)
        .context("gh repo view: missing owner.login")?
        .to_string();
    let repo = v
        .pointer("/name")
        .and_then(Value::as_str)
        .context("gh repo view: missing name")?
        .to_string();
    Ok((owner, repo))
}
