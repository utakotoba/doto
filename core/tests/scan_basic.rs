use std::error::Error;
use std::fs;

use koda_core::{scan, ScanConfig};
use tempfile::TempDir;

#[test]
fn scan_default_regex_finds_marks() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let file_path = temp.path().join("main.rs");
    fs::write(&file_path, "fn main() {\n// TODO: one\n// FIXME: two\n}\n")?;

    let config = ScanConfig::builder().root(temp.path()).build();
    let result = scan(config)?;

    assert_eq!(result.stats.files_scanned, 1);
    assert_eq!(result.stats.matches, 2);

    let mut marks = result
        .marks
        .iter()
        .map(|mark| (mark.path.as_ref().clone(), mark.line, mark.mark.clone()))
        .collect::<Vec<_>>();
    marks.sort_by(|a, b| a.1.cmp(&b.1));

    assert_eq!(marks.len(), 2);
    assert_eq!(marks[0].0, file_path);
    assert_eq!(marks[0].1, 2);
    assert_eq!(marks[0].2, "TODO");
    assert_eq!(marks[1].0, file_path);
    assert_eq!(marks[1].1, 3);
    assert_eq!(marks[1].2, "FIXME");
    Ok(())
}

#[test]
fn scan_custom_regex() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let file_path = temp.path().join("notes.txt");
    fs::write(&file_path, "NOTE: keep\nTODO: keep\n")?;

    let config = ScanConfig::builder()
        .root(temp.path())
        .regex(r"\bNOTE\b")
        .build();
    let result = scan(config)?;

    assert_eq!(result.stats.files_scanned, 1);
    assert_eq!(result.stats.matches, 1);
    assert_eq!(result.marks.len(), 1);
    assert_eq!(result.marks[0].path.as_ref(), &file_path);
    assert_eq!(result.marks[0].line, 1);
    assert_eq!(result.marks[0].mark, "NOTE");
    Ok(())
}
