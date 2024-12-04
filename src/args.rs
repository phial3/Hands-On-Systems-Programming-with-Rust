use clap::Parser;
use std::path::PathBuf;


#[derive(Parser, Debug, Default)]
#[command(name = "pipeviewer")]
#[command(about = "A powerful pipe viewer tool")]
pub struct Args {
    /// Read from a file instead of stdin
    #[arg(short, long)]
    pub infile: Option<PathBuf>,

    /// Write output to a file instead of stdout
    #[arg(short = 'o', long = "outfile")]
    pub outfile: Option<PathBuf>,

    /// Enable silent mode
    #[arg(short, long)]
    pub silent: bool,
}