//! Placeholder stress-harness entrypoint.
//!
//! This keeps `just stress` truthful during M0/M1: the command exists,
//! accepts the documented scenario argument, and clearly reports that no
//! adversarial workload has run yet.

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && matches!(args[1].as_str(), "-h" | "--help") {
        print_usage();
        return ExitCode::SUCCESS;
    }

    let scenario = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| String::from("smoke"));

    if scenario.starts_with('-') {
        eprintln!("physa-stress: unsupported option `{scenario}`");
        print_usage();
        return ExitCode::FAILURE;
    }

    println!("== physa-stress placeholder ==");
    println!("scenario: {scenario}");
    println!("status: placeholder");
    println!("verdict: placeholder (no workload executed, no invariants checked)");
    println!("note: replace this scaffold with the real harness tracked in issue #48.");

    ExitCode::SUCCESS
}

fn print_usage() {
    println!("Usage: physa-stress [scenario]");
}
