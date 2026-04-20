//! physa — command-line interface for physa-db.
//!
//! M0: placeholder binary. Real subcommands land in M2+.

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    println!("physa-db CLI — v0.0.0 (pre-alpha, no subcommands implemented yet)");
    println!("See ROADMAP.md for milestone progress.");
    Ok(())
}
