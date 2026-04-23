//! Placeholder macro-benchmark entrypoint.
//!
//! This keeps `just bench-macro` truthful during M0/M1: the command exists,
//! parses the intended shape, and clearly reports that no real benchmark
//! workload has executed yet.

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && matches!(args[1].as_str(), "-h" | "--help") {
        print_usage();
        return ExitCode::SUCCESS;
    }

    let mut sf = String::from("1");
    let mut iter = args.iter().skip(1);
    let Some(cmd) = iter.next() else {
        print_usage();
        return ExitCode::FAILURE;
    };

    if cmd != "run" {
        eprintln!("physa-bench: unsupported command `{cmd}`");
        print_usage();
        return ExitCode::FAILURE;
    }

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--sf" => {
                let Some(value) = iter.next() else {
                    eprintln!("physa-bench: `--sf` requires a value");
                    return ExitCode::FAILURE;
                };
                sf.clone_from(value);
            }
            "-h" | "--help" => {
                print_usage();
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("physa-bench: unsupported argument `{other}`");
                print_usage();
                return ExitCode::FAILURE;
            }
        }
    }

    println!("== physa-bench placeholder ==");
    println!("command: run");
    println!("scale factor: {sf}");
    println!("status: placeholder");
    println!("note: no LDBC/SNAP dataset was loaded and no benchmark was executed.");
    println!("next step: replace this scaffold with the real harness tracked in issue #47.");

    ExitCode::SUCCESS
}

fn print_usage() {
    println!("Usage: physa-bench run [--sf <scale-factor>]");
}
