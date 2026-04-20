//! xtask — workspace dev-task runner for physa-db.
//!
//! Invoked through the `justfile` (`just snapshot-dashboard`, `just seed-issues`, …).
//! Each subcommand is a first-class workflow that any contributor can reproduce locally.

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use xtask::dashboard;

#[derive(Parser)]
#[command(name = "xtask", about = "physa-db workspace dev tasks")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Regenerate `dashboard/data/state.json` from GitHub Issues + Projects v2.
    SnapshotDashboard {
        /// Print the JSON to stdout instead of writing to disk.
        #[arg(long)]
        dry_run: bool,
        /// Override the output path (default: `dashboard/data/state.json`).
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Create GitHub issues from `docs/seed-issues.md`.
    SeedIssues {
        /// If true, print what would be created without calling the API.
        #[arg(long, default_value_t = true)]
        dry_run: bool,
    },
    /// Emit the agent prompt that walks through profiling a competitor codename.
    ResearchPrompt {
        /// Competitor codename (e.g. `ALPHA`). Never a real name.
        codename: String,
    },
    /// Aggregate criterion results and produce a markdown report.
    BenchReport,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    match cli.cmd {
        Cmd::SnapshotDashboard { dry_run, out } => dashboard::run(&dashboard::Args {
            dry_run,
            output: out,
        }),
        Cmd::SeedIssues { dry_run } => seed_issues(dry_run),
        Cmd::ResearchPrompt { codename } => research_prompt(&codename),
        Cmd::BenchReport => bench_report(),
    }
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "stub becomes fallible when the gh API call lands"
)]
fn seed_issues(dry_run: bool) -> Result<()> {
    tracing::warn!(
        dry_run,
        "seed-issues: not yet implemented — will parse docs/seed-issues.md and call `gh issue create`"
    );
    Ok(())
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "stub becomes fallible when prompt file I/O is added"
)]
fn research_prompt(codename: &str) -> Result<()> {
    let prompt = format!(
        "You are a senior OSS-intel analyst. Profile competitor {codename}.\n\
         Deliverable: fill in `private/research/competitors/{lower}.md` using the `_template.md`.\n\
         Rules:\n\
         - Refer to the competitor ONLY by codename {codename} in every artifact.\n\
         - Cite only PUBLIC sources (docs, repos, published benchmarks, forum posts).\n\
         - Do NOT commit. The file is gitignored.\n\
         - Produce a separate PR against `docs/requirements/feature-matrix.md` \
           with attribution-free feature/perf conclusions.\n\
         See AGENTS.md §7 and ADR-0006 for the full policy.\n",
        codename = codename,
        lower = codename.to_lowercase()
    );
    println!("{prompt}");
    Ok(())
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "stub becomes fallible when criterion report aggregation is added"
)]
fn bench_report() -> Result<()> {
    tracing::warn!("bench-report: not yet implemented");
    Ok(())
}
