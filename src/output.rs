use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{get_creators, get_date_parts, get_field};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CSLItem {
    pub id: String,
    #[serde(rename = "abstract")]
    pub abstract_note: String,
    pub title: String,
    pub author: Vec<CSLAuthor>,
    #[serde(rename = "citation-key")]
    pub citation_key: String,
    pub issued: CSLDate,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CSLAuthor {
    pub family: String,
    pub given: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CSLDate {
    #[serde(rename = "date-parts")]
    pub date_parts: Vec<Vec<i32>>,
}

impl CSLDate {
    fn new(date: (i32, Option<i32>, Option<i32>)) -> Self {
        let mut parts = vec![date.0];
        if let Some(m) = date.1 {
            parts.push(m);
            if let Some(d) = date.2 {
                parts.push(d);
            }
        }
        Self {
            date_parts: vec![parts],
        }
    }
}

fn make_csl(
    conn: &Connection,
    citation_key: &str,
    item_id: i64,
) -> Result<CSLItem, Box<dyn Error>> {
    Ok(CSLItem {
        id: citation_key.into(),
        abstract_note: get_field(&conn, item_id, "abstractNote")?,
        title: get_field(&conn, item_id, "title")?,
        author: get_creators(&conn, item_id)?
            .iter()
            .map(|(first, last)| CSLAuthor {
                family: last.into(),
                given: first.into(),
            })
            .collect(),
        citation_key: citation_key.into(),
        issued: CSLDate::new(get_date_parts(&conn, item_id)?),
    })
}

pub fn csl_json(
    conn: &Connection,
    citation_key: &str,
    item_id: i64,
) -> Result<String, Box<dyn Error>> {
    let serialized = serde_json::to_string(&vec![make_csl(&conn, citation_key, item_id)?])?;
    Ok(serialized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    fn mock_item() -> CSLItem {
        CSLItem {
            id: "kowalskiSampleTitle2022".into(),
            abstract_note: "Abstract for a sample article".into(),
            title: "A sample title".into(),
            author: vec![CSLAuthor {
                family: "Kowalski".into(),
                given: "Jan".into(),
            }],
            citation_key: "kowalskiSampleTitle2022".into(),
            issued: CSLDate::new((2022, Some(5), Some(15))),
        }
    }

    #[test]
    fn mock_struct() {
        let conn = setup_database().unwrap();
        let citation_key = "kowalskiSampleTitle2022";
        let item_id = 1;

        assert_eq!(mock_item(), make_csl(&conn, citation_key, item_id).unwrap());
    }

    #[test]
    fn example_json() {
        let conn = setup_database().unwrap();
        let citation_key = "kowalskiSampleTitle2022";
        let item_id = 1;

        let csl_example = r#"
        {
            "id": "kowalskiSampleTitle2022",
            "abstract": "Abstract for a sample article",
            "container-title": "",
            "DOI": "",
            "event": "",
            "page": "",
            "source": "",
            "title": "A sample title",
            "author": [
                {
                    "family": "Kowalski",
                    "given": "Jan"
                }
            ],
            "issued": {
                "date-parts": [
                    [
                        2022,
                        5,
                        15
                    ]
                ]
            },
            "citation-key": "kowalskiSampleTitle2022"
        }"#;
        assert_eq!(
            serde_json::from_str::<CSLItem>(csl_example).unwrap(),
            make_csl(&conn, citation_key, item_id).unwrap()
        )
    }
}
