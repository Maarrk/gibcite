use clap::Parser;
use rusqlite::{Connection, OpenFlags};
use std::path::PathBuf;
use std::process::exit;

use gibcite::{count_items, get_database_path};

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

    let database_path = get_database_path(&cli.input).unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(exitcode::NOINPUT)
    });
    if cli.debug {
        println!("{:?}", database_path);
    }

    let conn =
        Connection::open_with_flags(database_path.as_ref(), OpenFlags::SQLITE_OPEN_READ_ONLY)
            .unwrap_or_else(|err| {
                eprintln!("Could not open Zotero database: {}", err);
                exit(exitcode::UNAVAILABLE);
            });

    let item_count = count_items(&conn).unwrap_or_else(|err| {
        eprintln!("SQL error: {}", err);
        exit(exitcode::IOERR);
    });
    println!("Loaded Zotero database with {} items", item_count);

    exit(exitcode::OK);
}
