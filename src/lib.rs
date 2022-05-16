use directories::UserDirs;
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::Value;
use std::{error::Error, path::PathBuf};

pub fn get_database_path(input: &Option<PathBuf>) -> Result<Box<PathBuf>, String> {
    let path_buf = match input {
        Some(path) => path.to_owned(),
        None => {
            if let Some(user_dirs) = UserDirs::new() {
                user_dirs.home_dir().join("Zotero")
            } else {
                return Err("could not get user directory location".into());
            }
        }
    };

    if !path_buf.join("zotero.sqlite").exists() {
        return Err(format!(
            "the Zotero database file not found at: {:?}",
            path_buf
        ));
    }
    if !path_buf.join("better-bibtex.sqlite").exists() {
        return Err(format!(
            "the Better BibTeX database file not found at: {:?}",
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

#[derive(Deserialize)]
struct BibtexItem {
    #[serde(rename = "citekey")]
    pub citation_key: String,
    #[serde(rename = "pinned")]
    pub _pinned: bool,
    #[serde(rename = "itemID")]
    pub item_id: i64,
    #[serde(rename = "libraryID")]
    pub _library_id: i64,
    #[serde(rename = "itemKey")]
    pub _item_key: String,
}

pub fn get_item_id(bibtex_conn: &Connection, citation_key: &str) -> Result<i64, Box<dyn Error>> {
    let mut stmt = bibtex_conn.prepare(
        "
        SELECT 'better-bibtex'.data
        FROM 'better-bibtex'
        WHERE 'better-bibtex'.name = 'better-bibtex.citekey';",
    )?;
    let row: String = stmt.query_row([], |r| r.get(0))?;
    let root: Value = serde_json::from_str(&row)?;
    match &root["data"] {
        Value::Array(data) => {
            for datum in data {
                let item: BibtexItem = serde_json::from_value(datum.to_owned())?;
                if item.citation_key == citation_key {
                    return Ok(item.item_id);
                }
            }
        }
        _ => Err("didn't find an array in the 'data' key")?,
    }
    Err(format!(
        "didn't find any item with citation key '{}'",
        citation_key
    ))?
}

pub fn get_creators(
    conn: &Connection,
    item_id: i64,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
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

pub fn get_field(
    conn: &Connection,
    item_id: i64,
    field_name: &str,
) -> Result<String, Box<dyn Error>> {
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

pub mod test_utils {
    use rusqlite::{Batch, Connection};
    use std::error::Error;

    pub fn setup_database() -> Result<Connection, Box<dyn Error>> {
        let conn = Connection::open_in_memory()?;
        static SQL_INIT: &'static str = include_str!("../tests/mock/zotero.sql");
        let mut batch = Batch::new(&conn, SQL_INIT);
        while let Some(mut stmt) = batch.next()? {
            stmt.execute([])?;
        }

        return Ok(conn);
    }

    pub fn setup_bibtex_database() -> Result<Connection, Box<dyn Error>> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "
            CREATE TABLE 'better-bibtex' (
                name TEXT PRIMARY KEY NOT NULL,
                data TEXT NOT NULL
            )",
            [],
        )?;
        static CITEKEY: &'static str = include_str!("../tests/mock/better-bibtex_citekey.json");
        conn.execute(
            "INSERT INTO 'better-bibtex' VALUES ('better-bibtex.citekey', ?)",
            [CITEKEY],
        )?;

        Ok(conn)
    }
}

// Make sure to follow cases in ../tests/mock when writing tests
#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn item_count() {
        let conn = setup_database().unwrap();
        assert_eq!(1, count_items(&conn).unwrap());
    }

    #[test]
    fn mock_item_id() {
        let bibtex_conn = setup_bibtex_database().unwrap();
        assert_eq!(
            1,
            get_item_id(&bibtex_conn, "kowalskiSampleTitle2022").unwrap()
        );
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
