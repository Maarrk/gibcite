use directories::UserDirs;
use rusqlite::Connection;
use std::{error::Error, path::PathBuf};

pub fn get_database_path(input: &Option<PathBuf>) -> Result<Box<PathBuf>, String> {
    let path_buf = match input {
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
        return Err(format!(
            "The path to database file does not exist: {:?}",
            path_buf
        ));
    }

    Ok(Box::new(path_buf))
}

pub fn count_items(conn: &Connection) -> Result<usize, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT itemID FROM items")?;
    let rows = stmt.query_map([], |_| Ok(()))?;

    Ok(rows.count())
}

// Make sure to follow cases in ../tests/mock.sql when writing tests
#[cfg(test)]
mod tests {
    use rusqlite::Batch;

    use super::*;

    fn setup_database() -> Result<Connection, Box<dyn Error>> {
        let conn = Connection::open_in_memory()?;
        static SQL_INIT: &'static str = include_str!("../tests/mock.sql");
        let mut batch = Batch::new(&conn, SQL_INIT);
        while let Some(mut stmt) = batch.next()? {
            stmt.execute([])?;
        }

        return Ok(conn);
    }

    #[test]
    fn item_count() {
        let conn = setup_database().unwrap();
        assert_eq!(1, count_items(&conn).unwrap());
    }
}
