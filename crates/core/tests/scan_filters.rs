use std::error::Error;
use std::fs;

use koda_core::{ScanConfig, scan};
use tempfile::TempDir;

#[test]
fn scan_respects_hidden_files_default() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let hidden_path = temp.path().join(".hidden.rs");
    fs::write(&hidden_path, "// TODO: hidden\n")?;

    let config = ScanConfig::builder()
        .root(temp.path())
        .follow_gitignore(false)
        .build();
    let result = scan(config)?;

    assert_eq!(result.stats.files_scanned, 0);
    assert_eq!(result.marks.len(), 0);
    Ok(())
}

#[test]
fn scan_includes_hidden_files_when_enabled() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let hidden_path = temp.path().join(".hidden.rs");
    fs::write(&hidden_path, "// TODO: hidden\n")?;

    let config = ScanConfig::builder()
        .root(temp.path())
        .include_hidden(true)
        .build();
    let result = scan(config)?;

    assert_eq!(result.stats.files_scanned, 1);
    assert_eq!(result.stats.matches, 1);
    assert_eq!(result.marks[0].path.as_ref(), &hidden_path);
    Ok(())
}

#[test]
fn scan_applies_include_exclude_overrides() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let keep_rs = temp.path().join("keep.rs");
    let skip_rs = temp.path().join("skip.rs");
    let other_txt = temp.path().join("other.txt");

    fs::write(&keep_rs, "// TODO: keep\n")?;
    fs::write(&skip_rs, "// TODO: skip\n")?;
    fs::write(&other_txt, "// TODO: other\n")?;

    let config = ScanConfig::builder()
        .root(temp.path())
        .include("*.rs")
        .exclude("skip.rs")
        .build();
    let result = scan(config)?;

    let mut paths = result
        .marks
        .iter()
        .map(|mark| mark.path.as_ref().clone())
        .collect::<Vec<_>>();
    paths.sort();

    assert_eq!(result.stats.matches, 1);
    assert_eq!(paths, vec![keep_rs]);
    Ok(())
}

#[test]
fn scan_respects_max_file_size() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let large_path = temp.path().join("large.rs");
    let content = "TODO\n".repeat(128);
    fs::write(&large_path, &content)?;

    let config = ScanConfig::builder()
        .root(temp.path())
        .max_file_size(Some(8))
        .build();
    let result = scan(config)?;

    assert_eq!(result.stats.files_scanned, 0);
    assert_eq!(result.stats.files_skipped, 1);
    assert_eq!(result.stats.matches, 0);
    Ok(())
}

#[test]
fn scan_builtin_excludes_can_be_disabled() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let node_modules = temp.path().join("node_modules");
    fs::create_dir_all(&node_modules)?;
    let file_path = node_modules.join("dep.rs");
    fs::write(&file_path, "// TODO: dep\n")?;

    let config = ScanConfig::builder().root(temp.path()).build();
    let result = scan(config)?;

    assert_eq!(result.stats.matches, 0);

    let config = ScanConfig::builder()
        .root(temp.path())
        .follow_gitignore(false)
        .builtin_excludes(false)
        .build();
    let result = scan(config)?;

    assert_eq!(result.stats.matches, 1);
    Ok(())
}
