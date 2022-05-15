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

fn get_creators(conn: &Connection, item_id: i64) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "
        SELECT creators.firstName, creators.lastName
        FROM creators
        JOIN itemCreators ON creators.creatorID = itemCreators.creatorID
        WHERE itemCreators.itemID = ?
        ORDER BY itemCreators.orderIndex ASC;",
    )?;
    let rows = stmt.query_map([item_id], |row| Ok((row.get(0)?, row.get(1)?)))?;

    let mut creators = Vec::new();
    for creator_result in rows {
        creators.push(creator_result?);
    }

    Ok(creators)
}

fn get_field(conn: &Connection, item_id: i64, field_name: &str) -> Result<String, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "
        SELECT itemDataValues.value
        FROM itemData
        JOIN fieldsCombined ON itemData.fieldID = fieldsCombined.fieldID
        JOIN itemDataValues ON itemData.valueID = itemDataValues.valueID
        WHERE itemData.itemID = ? AND fieldsCombined.fieldName = ?;",
    )?;
    let row: String = stmt.query_row(rusqlite::params![item_id, field_name], |r| r.get(0))?;
    Ok(row)
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

    #[test]
    fn mock_creators() {
        let conn = setup_database().unwrap();
        assert_eq!(
            vec![("Jan".to_string(), "Kowalski".to_string())],
            get_creators(&conn, 1).unwrap()
        );
    }

    #[test]
    fn mock_title() {
        let conn = setup_database().unwrap();
        assert_eq!(
            "A sample title".to_string(),
            get_field(&conn, 1, "title").unwrap()
        );
    }

    #[test]
    fn mock_abstract() {
        let conn = setup_database().unwrap();
        assert_eq!(
            "Abstract for a sample article".to_string(),
            get_field(&conn, 1, "abstractNote").unwrap()
        );
    }
}
