//! `snapshot-dashboard` xtask subcommand.
//!
//! Glues the GraphQL fetcher (`gh.rs`), the response parsing
//! (`fetch.rs`), and the pure synthesis (`snapshot.rs`) into a single
//! entry point invoked from `main.rs`.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

use self::fetch::Fetched;
use self::gh::{GhCli, GraphQLFetcher, resolve_repo};

pub mod fetch;
pub mod gh;
pub mod snapshot;
pub mod state;

const DEFAULT_OUTPUT: &str = "dashboard/data/state.json";
const DONE_WINDOW_DAYS: i64 = 30;

#[derive(Debug, Default)]
pub struct Args {
    pub dry_run: bool,
    pub output: Option<PathBuf>,
}

pub fn run(args: &Args) -> Result<()> {
    let (owner, repo) = resolve_repo()?;
    tracing::info!(owner, repo, "snapshot-dashboard starting");

    let now = resolve_now()?;
    let cutoff = now - Duration::days(DONE_WINDOW_DAYS);
    let now_rfc = now.format(&Rfc3339)?;
    let cutoff_rfc = cutoff.format(&Rfc3339)?;

    let client = GhCli;
    let data = fetch::fetch_all(&client as &dyn GraphQLFetcher, &owner, &repo)?;
    let state = snapshot::build_state(&data, &now_rfc, &cutoff_rfc);

    let mut json = serde_json::to_string_pretty(&state)?;
    json.push('\n');

    if args.dry_run {
        print!("{json}");
    } else {
        let path = args
            .output
            .clone()
            .unwrap_or_else(|| PathBuf::from(DEFAULT_OUTPUT));
        write_atomic(&path, json.as_bytes())?;
        tracing::info!(path = %path.display(), "wrote state.json");
    }
    Ok(())
}

/// Run the pipeline against an arbitrary fetcher and clock — used by
/// integration tests to replay recorded fixtures.
pub fn build_from(
    client: &dyn GraphQLFetcher,
    owner: &str,
    repo: &str,
    now_rfc: &str,
    cutoff_rfc: &str,
) -> Result<(state::State, Fetched)> {
    let data = fetch::fetch_all(client, owner, repo)?;
    let state = snapshot::build_state(&data, now_rfc, cutoff_rfc);
    Ok((state, data))
}

fn resolve_now() -> Result<OffsetDateTime> {
    if let Ok(raw) = std::env::var("PHYSA_SNAPSHOT_NOW") {
        return OffsetDateTime::parse(&raw, &Rfc3339)
            .with_context(|| format!("PHYSA_SNAPSHOT_NOW is not RFC3339: {raw}"));
    }
    Ok(OffsetDateTime::now_utc())
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("mkdir {}", parent.display()))?;
    }
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, bytes).with_context(|| format!("write {}", tmp.display()))?;
    std::fs::rename(&tmp, path)
        .with_context(|| format!("rename {} -> {}", tmp.display(), path.display()))?;
    Ok(())
}
