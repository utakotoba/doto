use std::error::Error;
use std::fs;

use koda_core::{ScanConfig, scan};
use tempfile::TempDir;

#[test]
fn scan_only_matches_in_comments() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let file_path = temp.path().join("lib.rs");
    let contents = r#"
fn main() {
    let s = "TODO: not a comment";
    let t = "// TODO: not a comment either";
    // TODO: in line comment
    /*
        FIXME: in block comment
    */
}
"#;
    fs::write(&file_path, contents)?;

    let config = ScanConfig::builder().root(temp.path()).build();
    let result = scan(config)?;

    let mut marks = result
        .marks
        .iter()
        .map(|mark| (mark.line, mark.mark.clone()))
        .collect::<Vec<_>>();
    marks.sort();

    assert_eq!(marks.len(), 2);
    assert_eq!(marks[0].1, "TODO");
    assert_eq!(marks[1].1, "FIXME");
    Ok(())
}

#[test]
fn scan_ignores_backtick_strings_in_js() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let file_path = temp.path().join("main.ts");
    let contents = r#"
const note = `// TODO: not a comment`;
// TODO: comment
"#;
    fs::write(&file_path, contents)?;

    let config = ScanConfig::builder().root(temp.path()).build();
    let result = scan(config)?;

    let mut marks = result
        .marks
        .iter()
        .map(|mark| (mark.line, mark.mark.clone()))
        .collect::<Vec<_>>();
    marks.sort();

    assert_eq!(marks.len(), 1);
    assert_eq!(marks[0].1, "TODO");
    Ok(())
}
