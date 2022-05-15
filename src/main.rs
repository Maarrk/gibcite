use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Citation key to find in the database
    citation_key: String,
    /// Output debugging information
    #[clap(short, long)]
    debug: bool,
    /// Zotero SQLite database location
    #[clap(long, parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        println!("{:?}", cli);
    }
}
