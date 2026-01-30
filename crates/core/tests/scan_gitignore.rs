use std::error::Error;
use std::fs;

use koda_core::{scan, ScanConfig};
use tempfile::TempDir;

#[test]
fn scan_respects_gitignore() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    fs::write(temp.path().join(".gitignore"), "ignored.txt\n")?;

    let ignored = temp.path().join("ignored.txt");
    let kept = temp.path().join("kept.txt");

    fs::write(&ignored, "TODO ignored\n")?;
    fs::write(&kept, "TODO kept\n")?;

    let config = ScanConfig::builder().root(temp.path()).build();
    let result = scan(config)?;

    assert_eq!(result.stats.matches, 1);
    assert_eq!(result.marks.len(), 1);
    assert_eq!(result.marks[0].path.as_ref(), &kept);
    Ok(())
}

#[test]
fn scan_can_disable_gitignore() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    fs::write(temp.path().join(".gitignore"), "ignored.txt\n")?;

    let ignored = temp.path().join("ignored.txt");
    fs::write(&ignored, "TODO ignored\n")?;

    let config = ScanConfig::builder()
        .root(temp.path())
        .follow_gitignore(false)
        .build();
    let result = scan(config)?;

    assert_eq!(result.stats.matches, 1);
    assert_eq!(result.marks.len(), 1);
    assert_eq!(result.marks[0].path.as_ref(), &ignored);
    Ok(())
}
