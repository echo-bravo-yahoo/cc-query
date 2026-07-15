//! CLI entry point for ccq.

use std::io::IsTerminal;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

/// SQL REPL for querying Claude Code session data
#[derive(Debug, Parser)]
#[command(name = "ccq", version, about)]
struct Cli {
    /// Path to project (omit for all projects)
    project_path: Option<PathBuf>,

    /// Filter to sessions matching ID prefix
    #[arg(short, long)]
    session: Option<String>,

    /// Use directory directly as JSONL data source
    #[arg(short, long = "data-dir")]
    data_dir: Option<PathBuf>,

    /// Disable the default `DuckDB` sandbox (`allowed_directories` scoped to
    /// the directory being queried, no network/extension access, locked
    /// config). Off by default; pass --unsafe for COPY TO/httpfs/broader
    /// file access.
    #[arg(long = "unsafe")]
    unsafe_: bool,
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run() -> ccq::Result<()> {
    let cli = Cli::parse();

    let session = ccq::QuerySession::create(
        cli.project_path.as_deref(),
        cli.session.as_deref(),
        cli.data_dir.as_deref(),
        !cli.unsafe_,
    )?;

    if std::io::stdin().is_terminal() {
        ccq::repl::start_interactive(&session)
    } else {
        ccq::repl::run_piped(&session)
    }
}
