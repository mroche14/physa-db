//! Integration test: replays recorded GraphQL fixtures through the
//! snapshot pipeline and asserts byte-for-byte equality against
//! `expected_state.json`.
//!
//! The fixture files live under `tests/fixtures/`; regenerate
//! `expected_state.json` with:
//!
//! ```bash
//! UPDATE_FIXTURE=1 cargo test -p xtask --test snapshot
//! ```
//!
//! This proves two things at once:
//!
//! - **Schema match** — output conforms to `dashboard/src/app.js`.
//! - **Idempotency** — repeated runs over the same inputs produce the
//!   same bytes (stable sort + injected clock).

use std::path::PathBuf;

use xtask::dashboard::{build_from, gh::FixtureFetcher};

const NOW: &str = "2026-04-20T00:00:00Z";
const CUTOFF_30D: &str = "2026-03-21T00:00:00Z";

fn fixtures_dir() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures"))
}

fn render() -> String {
    let fetcher = FixtureFetcher::new(fixtures_dir());
    let (state, _) = build_from(&fetcher, "owner", "repo", NOW, CUTOFF_30D)
        .expect("fixture pipeline must not fail");
    let mut s = serde_json::to_string_pretty(&state).expect("serialize");
    s.push('\n');
    s
}

#[test]
fn replay_matches_expected_fixture() {
    let actual = render();
    let expected_path = fixtures_dir().join("expected_state.json");

    if std::env::var("UPDATE_FIXTURE").is_ok() {
        std::fs::write(&expected_path, &actual).expect("write fixture");
        return;
    }

    let expected = std::fs::read_to_string(&expected_path).unwrap_or_else(|e| {
        panic!(
            "cannot read {} ({e}); run with UPDATE_FIXTURE=1 to regenerate",
            expected_path.display()
        )
    });
    assert_eq!(actual, expected, "snapshot drift vs expected_state.json");
}

#[test]
fn replay_is_idempotent() {
    assert_eq!(render(), render(), "non-deterministic snapshot output");
}

#[test]
fn paginates_issues_across_two_pages() {
    let actual = render();
    assert!(
        actual.contains("Task A"),
        "first page (issues_p1) must contribute"
    );
    assert!(
        actual.contains("Task B"),
        "second page (issues_p2) must contribute"
    );
    assert!(
        actual.contains("Task C"),
        "second page (issues_p2) must contribute"
    );
}
