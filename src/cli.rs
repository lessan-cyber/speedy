use clap::Parser;

/// CLI for testing internet speed!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Simple mode without the fancy UI animation
    #[arg(short, long)]
    pub simple: bool,

    /// Test download speed only
    #[arg(short, long)]
    pub download_only: bool,

    /// Test upload speed only
    #[arg(short, long)]
    pub upload_only: bool,

    /// Output the result in JSON format
    #[arg(short, long)]
    pub json: bool,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}
