use clap::Parser;
use directories::UserDirs;
use rusqlite::{Connection, OpenFlags};
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;

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

    let database_path = get_database_path(&cli).unwrap_or_else(|err| {
        println!("{}", err);
        exit(exitcode::NOINPUT)
    });
    if cli.debug {
        println!("{:?}", database_path);
    }

    let conn =
        Connection::open_with_flags(database_path.as_ref(), OpenFlags::SQLITE_OPEN_READ_ONLY)
            .unwrap_or_else(|err| {
                println!("Could not open Zotero database: {}", err);
                exit(exitcode::UNAVAILABLE);
            });

    let item_count = count_items(&conn).unwrap_or_else(|err| {
        println!("SQL error: {}", err);
        exit(exitcode::IOERR);
    });
    println!("Loaded Zotero database with {} items", item_count);

    exit(exitcode::OK);
}

fn get_database_path(cli: &Cli) -> Result<Box<PathBuf>, String> {
    let path_buf = match &cli.input {
        Some(path) => path.to_owned(),
        None => {
            if let Some(user_dirs) = UserDirs::new() {
                user_dirs.home_dir().join("Zotero").join("zotero.sqlite")
            } else {
                return Err("Could not get user directory location".into());
            }
        }
    };

    if !path_buf.exists() {
        return Err(format!("The path to database file does not exist"));
    }

    Ok(Box::new(path_buf))
}

fn count_items(conn: &Connection) -> Result<usize, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT itemID FROM items")?;
    let rows = stmt.query_map([], |_| Ok(()))?;

    Ok(rows.count())
}
