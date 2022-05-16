use assert_cmd::prelude::*;
use assert_fs::{prelude::*, TempDir};
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;

use gibcite::test_utils::*;

#[test]
fn zotero_doesnt_exist() -> Result<(), Box<dyn Error>> {
    let dir = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("gibcite")?;

    cmd.arg("--input").arg(dir.path());
    cmd.arg("someCitationKey");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Zotero").and(predicate::str::contains("not found")));

    Ok(())
}

#[test]
fn bibtex_doesnt_exist() -> Result<(), Box<dyn Error>> {
    let file = assert_fs::NamedTempFile::new("zotero.sqlite")?;
    file.write_str("some data")?; // Required to create the file

    let mut cmd = Command::cargo_bin("gibcite")?;

    cmd.arg("--input").arg(file.path().parent().unwrap());
    cmd.arg("someCitationKey");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("BibTeX").and(predicate::str::contains("not found")));

    Ok(())
}

fn setup_database_files() -> Result<TempDir, Box<dyn Error>> {
    let dir = assert_fs::TempDir::new()?;

    setup_database()?.backup(
        rusqlite::DatabaseName::Main,
        dir.join("zotero.sqlite"),
        None,
    )?;
    setup_bibtex_database()?.backup(
        rusqlite::DatabaseName::Main,
        dir.join("better-bibtex.sqlite"),
        None,
    )?;

    Ok(dir)
}

fn setup_command() -> Result<(Command, TempDir), Box<dyn Error>> {
    let dir = setup_database_files().unwrap();
    let mut cmd = Command::cargo_bin("gibcite")?;

    cmd.arg("--input").arg(dir.path());
    Ok((cmd, dir))
}

#[test]
fn mock_key() -> Result<(), Box<dyn Error>> {
    let (mut cmd, _dir) = setup_command().unwrap();
    cmd.arg("kowalskiSampleTitle2022");

    cmd.assert().success();

    Ok(())
}
