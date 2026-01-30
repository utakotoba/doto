use std::error::Error;
use std::fs;

use doto_core::{ScanConfig, scan};
use tempfile::TempDir;

#[test]
fn scan_multiple_roots() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let root_a = temp.path().join("a");
    let root_b = temp.path().join("b");
    fs::create_dir_all(&root_a)?;
    fs::create_dir_all(&root_b)?;

    let a_file = root_a.join("a.rs");
    let b_file = root_b.join("b.rs");
    fs::write(&a_file, "// TODO a\n")?;
    fs::write(&b_file, "// TODO b\n")?;

    let config = ScanConfig::builder().root(&root_a).root(&root_b).build();
    let result = scan(config)?;

    let mut paths = result
        .marks
        .iter()
        .map(|mark| mark.path.as_ref().clone())
        .collect::<Vec<_>>();
    paths.sort();

    assert_eq!(result.stats.matches, 2);
    assert_eq!(paths, vec![a_file, b_file]);
    Ok(())
}
