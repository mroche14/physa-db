//! Pure synthesis: fetched GraphQL data + fixed `now` → [`State`].
//!
//! No I/O, no time source. `now_rfc3339` is injected so tests and CI
//! runs are reproducible byte-for-byte.

#[cfg(test)]
use super::fetch::ProjectItemNode;
use super::fetch::{Fetched, IssueNode, MilestoneNode, PullNode};
use super::state::{ActivityEvent, Milestone, State, Stats};

const IN_PROGRESS_LABEL: &str = "status:in-progress";
const IN_PROGRESS_STATUS: &str = "In Progress";
const ACTIVITY_CAP: usize = 50;

#[must_use]
pub fn build_state(fetched: &Fetched, now_rfc3339: &str, cutoff_30d_rfc3339: &str) -> State {
    State {
        generated_at: now_rfc3339.to_string(),
        stats: build_stats(fetched, cutoff_30d_rfc3339),
        milestones: build_milestones(&fetched.milestones),
        activity: build_activity(&fetched.issues, &fetched.pulls),
    }
}

#[expect(
    clippy::cast_possible_truncation,
    reason = "a single repo will never carry more than u32::MAX issues"
)]
fn count_u32<T>(iter: impl Iterator<Item = T>) -> u32 {
    iter.count() as u32
}

fn build_stats(f: &Fetched, cutoff: &str) -> Stats {
    let open_issues = count_u32(f.issues.iter().filter(|i| i.state == "OPEN"));

    // Label is the primary truth source per AGENTS.md §6.1 (agents flip
    // `status:in-progress` on claim; Projects v2 Status is a human-viewed
    // board field that is not always kept in sync by automation).
    let in_progress = count_u32(
        f.issues
            .iter()
            .filter(|i| i.state == "OPEN" && i.labels.iter().any(|l| l == IN_PROGRESS_LABEL)),
    );

    if let Some(items) = &f.project_items {
        let pv2_in_progress = items
            .iter()
            .filter(|p| p.status.as_deref() == Some(IN_PROGRESS_STATUS))
            .count();
        tracing::debug!(
            pv2_in_progress,
            label_in_progress = in_progress,
            "projects_v2 observed (not used for stats; labels are truth)"
        );
    }

    let done_last_30d = count_u32(
        f.issues
            .iter()
            .filter(|i| closed_since(i.closed_at.as_deref(), cutoff))
            .map(|_| ())
            .chain(
                f.pulls
                    .iter()
                    .filter(|p| closed_since(p.merged_at.as_deref(), cutoff))
                    .map(|_| ()),
            ),
    );

    let current_milestone = f
        .milestones
        .iter()
        .find(|m| m.state == "OPEN")
        .map(|m| split_milestone(&m.title).0)
        .unwrap_or_default();

    Stats {
        open_issues,
        in_progress,
        done_last_30d,
        current_milestone,
    }
}

fn build_milestones(raw: &[MilestoneNode]) -> Vec<Milestone> {
    let mut out: Vec<Milestone> = raw
        .iter()
        .map(|m| {
            let (name, title) = split_milestone(&m.title);
            Milestone {
                name,
                title,
                done: m.closed_issue_count,
                total: m.open_issue_count + m.closed_issue_count,
            }
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn build_activity(issues: &[IssueNode], pulls: &[PullNode]) -> Vec<ActivityEvent> {
    let mut events: Vec<ActivityEvent> = Vec::new();

    for i in issues {
        events.push(ActivityEvent {
            at: i.created_at.clone(),
            kind: "issue.opened".into(),
            title: i.title.clone(),
            url: i.url.clone(),
        });
        if let Some(at) = &i.closed_at {
            events.push(ActivityEvent {
                at: at.clone(),
                kind: "issue.closed".into(),
                title: i.title.clone(),
                url: i.url.clone(),
            });
        }
    }

    for p in pulls {
        events.push(ActivityEvent {
            at: p.created_at.clone(),
            kind: "pr.opened".into(),
            title: p.title.clone(),
            url: p.url.clone(),
        });
        if let Some(at) = &p.merged_at {
            events.push(ActivityEvent {
                at: at.clone(),
                kind: "pr.merged".into(),
                title: p.title.clone(),
                url: p.url.clone(),
            });
        } else if let Some(at) = &p.closed_at {
            events.push(ActivityEvent {
                at: at.clone(),
                kind: "pr.closed".into(),
                title: p.title.clone(),
                url: p.url.clone(),
            });
        }
    }

    events.sort_by(|a, b| {
        b.at.cmp(&a.at)
            .then_with(|| a.kind.cmp(&b.kind))
            .then_with(|| a.url.cmp(&b.url))
    });
    events.truncate(ACTIVITY_CAP);
    events
}

fn closed_since(when: Option<&str>, cutoff: &str) -> bool {
    when.is_some_and(|w| w.as_bytes() >= cutoff.as_bytes())
}

/// Split a milestone title like `"M0 — Foundation"` into `("M0", "Foundation")`.
/// Falls back to `(whole, "")` if no em-dash is present.
fn split_milestone(raw: &str) -> (String, String) {
    const EM_DASH: char = '\u{2014}';
    raw.find(EM_DASH).map_or_else(
        || (raw.trim().to_string(), String::new()),
        |idx| {
            let (name, rest) = raw.split_at(idx);
            let title = rest[EM_DASH.len_utf8()..].trim();
            (name.trim().to_string(), title.to_string())
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn issue(state: &str, created: &str, closed: Option<&str>, labels: &[&str]) -> IssueNode {
        IssueNode {
            title: "t".into(),
            url: format!("https://example/{created}"),
            state: state.into(),
            created_at: created.into(),
            closed_at: closed.map(str::to_string),
            labels: labels.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    fn pr(state: &str, created: &str, merged: Option<&str>, closed: Option<&str>) -> PullNode {
        PullNode {
            title: "p".into(),
            url: format!("https://example/pr/{created}"),
            state: state.into(),
            created_at: created.into(),
            closed_at: closed.map(str::to_string),
            merged_at: merged.map(str::to_string),
        }
    }

    #[test]
    fn split_milestone_em_dash() {
        assert_eq!(
            split_milestone("M0 — Foundation"),
            ("M0".into(), "Foundation".into())
        );
    }

    #[test]
    fn split_milestone_no_dash() {
        assert_eq!(
            split_milestone("backlog"),
            ("backlog".into(), String::new())
        );
    }

    #[test]
    fn stats_in_progress_ignores_project_items_prefers_label() {
        // Agents flip the label; the Projects v2 Status field is a human
        // board view that is not always in sync. Per AGENTS.md §6.1 the
        // label is authoritative. Even when project_items claim "In
        // Progress", we must fall back to label counts.
        let f = Fetched {
            issues: vec![
                issue("OPEN", "2026-04-01T00:00:00Z", None, &[]),
                issue("OPEN", "2026-04-02T00:00:00Z", None, &[]),
            ],
            pulls: vec![],
            milestones: vec![],
            project_items: Some(vec![
                ProjectItemNode {
                    status: Some("In Progress".into()),
                },
                ProjectItemNode {
                    status: Some("In Progress".into()),
                },
            ]),
        };
        let s = build_state(&f, "2026-04-20T00:00:00Z", "2026-03-21T00:00:00Z");
        assert_eq!(s.stats.in_progress, 0);
    }

    #[test]
    fn stats_in_progress_counts_labelled_open_issues() {
        let f = Fetched {
            issues: vec![
                issue(
                    "OPEN",
                    "2026-04-01T00:00:00Z",
                    None,
                    &["status:in-progress"],
                ),
                issue("OPEN", "2026-04-02T00:00:00Z", None, &[]),
                issue(
                    "CLOSED",
                    "2026-04-03T00:00:00Z",
                    Some("2026-04-10T00:00:00Z"),
                    &["status:in-progress"],
                ),
            ],
            pulls: vec![],
            milestones: vec![],
            project_items: None,
        };
        let s = build_state(&f, "2026-04-20T00:00:00Z", "2026-03-21T00:00:00Z");
        assert_eq!(s.stats.in_progress, 1);
    }

    #[test]
    fn done_last_30d_counts_issues_and_merged_prs_only() {
        let f = Fetched {
            issues: vec![
                issue(
                    "CLOSED",
                    "2026-03-01T00:00:00Z",
                    Some("2026-04-10T00:00:00Z"),
                    &[],
                ),
                issue(
                    "CLOSED",
                    "2026-01-01T00:00:00Z",
                    Some("2026-02-01T00:00:00Z"),
                    &[],
                ),
            ],
            pulls: vec![
                pr(
                    "MERGED",
                    "2026-04-01T00:00:00Z",
                    Some("2026-04-15T00:00:00Z"),
                    Some("2026-04-15T00:00:00Z"),
                ),
                pr(
                    "CLOSED",
                    "2026-04-01T00:00:00Z",
                    None,
                    Some("2026-04-15T00:00:00Z"),
                ),
            ],
            milestones: vec![],
            project_items: None,
        };
        let s = build_state(&f, "2026-04-20T00:00:00Z", "2026-03-21T00:00:00Z");
        assert_eq!(s.stats.done_last_30d, 2);
    }

    #[test]
    fn activity_sorted_desc_and_capped() {
        let f = Fetched {
            issues: (0..30)
                .map(|n| {
                    let d = format!("2026-04-{:02}T00:00:00Z", n + 1);
                    issue("OPEN", &d, None, &[])
                })
                .collect(),
            pulls: vec![],
            milestones: vec![],
            project_items: None,
        };
        let s = build_state(&f, "2026-04-20T00:00:00Z", "2026-03-21T00:00:00Z");
        assert!(s.activity.len() <= ACTIVITY_CAP);
        for pair in s.activity.windows(2) {
            assert!(pair[0].at >= pair[1].at, "activity must sort desc by at");
        }
    }

    #[test]
    fn current_milestone_is_first_open() {
        let f = Fetched {
            issues: vec![],
            pulls: vec![],
            milestones: vec![
                MilestoneNode {
                    title: "M0 — Foundation".into(),
                    state: "OPEN".into(),
                    due_on: Some("2026-06-01T00:00:00Z".into()),
                    open_issue_count: 3,
                    closed_issue_count: 5,
                },
                MilestoneNode {
                    title: "M1 — Launch".into(),
                    state: "OPEN".into(),
                    due_on: Some("2026-09-01T00:00:00Z".into()),
                    open_issue_count: 10,
                    closed_issue_count: 0,
                },
            ],
            project_items: None,
        };
        let s = build_state(&f, "2026-04-20T00:00:00Z", "2026-03-21T00:00:00Z");
        assert_eq!(s.stats.current_milestone, "M0");
        assert_eq!(s.milestones.len(), 2);
        assert_eq!(s.milestones[0].total, 8);
    }

    #[test]
    fn output_is_stable_across_runs() {
        let f = Fetched {
            issues: vec![
                issue("OPEN", "2026-04-10T00:00:00Z", None, &[]),
                issue(
                    "CLOSED",
                    "2026-04-05T00:00:00Z",
                    Some("2026-04-15T00:00:00Z"),
                    &[],
                ),
            ],
            pulls: vec![],
            milestones: vec![],
            project_items: None,
        };
        let a = build_state(&f, "2026-04-20T00:00:00Z", "2026-03-21T00:00:00Z");
        let b = build_state(&f, "2026-04-20T00:00:00Z", "2026-03-21T00:00:00Z");
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            serde_json::to_string(&b).unwrap()
        );
    }
}
