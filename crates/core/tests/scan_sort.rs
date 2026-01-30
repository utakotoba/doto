use std::error::Error;
use std::fs;

use koda_core::{
    FilenameSortConfig, LanguageSortConfig, MarkSortConfig, ScanConfig, SortStage, scan,
};
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
        SortStage::Mark(MarkSortConfig::default()),
        SortStage::Language(LanguageSortConfig::default()),
        SortStage::Filename(FilenameSortConfig::default()),
    ];

    let config = ScanConfig::builder()
        .root(temp.path())
        .sort_pipeline(pipeline)
        .build();
    let result = scan(config)?;

    let ordered = result
        .marks
        .iter()
        .map(|mark| (mark.mark.as_str(), mark.path.file_name().unwrap().to_string_lossy()))
        .collect::<Vec<_>>();

    assert_eq!(ordered.len(), 4);
    assert_eq!(ordered[0].0, "FIXME");
    assert_eq!(ordered[0].1, "c.ts");
    assert_eq!(ordered[1].0, "TODO");
    assert_eq!(ordered[1].1, "a.rs");
    assert_eq!(ordered[2].0, "TODO");
    assert_eq!(ordered[2].1, "z.rs");
    assert_eq!(ordered[3].0, "TODO");
    assert_eq!(ordered[3].1, "b.ts");
    Ok(())
}
