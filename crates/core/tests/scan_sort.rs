use std::error::Error;
use std::fs;

use doto_core::{DimensionStage, LanguageSortConfig, MarkSortConfig, ScanConfig, scan};
use tempfile::TempDir;

#[test]
fn scan_applies_sort_pipeline() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let rust_a = temp.path().join("a.rs");
    let rust_z = temp.path().join("z.rs");
    let ts_todo = temp.path().join("b.ts");
    let ts_fixme = temp.path().join("c.ts");

    fs::write(&rust_a, "// TODO: a\n")?;
    fs::write(&rust_z, "// TODO: z\n")?;
    fs::write(&ts_todo, "// TODO: ts\n")?;
    fs::write(&ts_fixme, "// FIXME: ts\n")?;

    let pipeline = vec![
        DimensionStage::Mark(MarkSortConfig::default()),
        DimensionStage::Language(LanguageSortConfig::default()),
    ];

    let config = ScanConfig::builder()
        .root(temp.path())
        .sort_pipeline(pipeline)
        .build();
    let result = scan(config)?;

    let ordered = result
        .marks
        .iter()
        .map(|mark| (mark.mark, mark.path.file_name().unwrap().to_string_lossy()))
        .collect::<Vec<_>>();

    assert_eq!(ordered.len(), 4);
    assert_eq!(ordered[0].0, "FIXME");
    assert_eq!(ordered[0].1, "c.ts");
    assert_eq!(ordered[1].0, "TODO");
    Ok(())
}
