use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rand::rngs::StdRng;
use rand::SeedableRng;
use tempfile::TempDir;

use koda_core::{ScanConfig, scan};

use colored::Colorize;

use crate::generator::generate_tree;
use crate::options::RunOptions;

pub struct RunSummary {
    pub files_scanned: u64,
    pub matches: u64,
    pub files_skipped: u64,
    pub elapsed_secs: f64,
    pub bytes: u64,
}

pub fn run_once(options: RunOptions) -> Result<RunSummary, Box<dyn std::error::Error>> {
    let mut rng = StdRng::seed_from_u64(options.seed);

    let temp;
    let root_base = if let Some(out) = options.out_dir.as_ref() {
        fs::create_dir_all(out)?;
        out.clone()
    } else {
        temp = TempDir::new()?;
        temp.path().to_path_buf()
    };

    let mut roots = Vec::new();
    let mut total_bytes = 0u64;
    for idx in 0..options.roots {
        let root = root_base.join(format!("root_{idx}"));
        fs::create_dir_all(&root)?;
        total_bytes += generate_tree(&root, &options, &mut rng)?;
        roots.push(root);
    }

    if !options.scan {
        return Ok(RunSummary {
            files_scanned: 0,
            matches: 0,
            files_skipped: 0,
            elapsed_secs: 0.0,
            bytes: total_bytes,
        });
    }

    let scan_config = ScanConfig::builder().roots(roots).build();
    let start = Instant::now();
    let result = scan(scan_config)?;
    let elapsed = start.elapsed().as_secs_f64().max(1e-9);

    Ok(RunSummary {
        files_scanned: result.stats.files_scanned,
        matches: result.stats.matches,
        files_skipped: result.stats.files_skipped,
        elapsed_secs: elapsed,
        bytes: total_bytes,
    })
}

pub fn label_for_run(options: &RunOptions) -> String {
    format!(
        "roots={} depth={} dirs={} files={} ratio={}",
        options.roots,
        options.depth,
        options.dirs_per_level,
        options.files_per_dir,
        options.mark_ratio
    )
}

pub fn run_summary(label: &str, summary: &RunSummary) {
    let mb = summary.bytes as f64 / (1024.0 * 1024.0);
    let throughput = if summary.elapsed_secs > 0.0 {
        mb / summary.elapsed_secs
    } else {
        0.0
    };
    let header = format_label(label).bold();
    let files = format!("{}", summary.files_scanned).cyan();
    let matches = format!("{}", summary.matches).yellow();
    let skipped = format!("{}", summary.files_skipped).dimmed();
    let elapsed = format!("{:.3}s", summary.elapsed_secs).green();
    let mbps = format!("{:.2} MB/s", throughput).magenta();
    let bytes = format!("{}", summary.bytes).dimmed();

    println!("{header}");
    println!("  files_scanned: {files}  matches: {matches}  skipped: {skipped}");
    println!("  elapsed: {elapsed}  throughput: {mbps}  bytes: {bytes}");
}

fn format_label(label: &str) -> String {
    if label.starts_with("roots") && label.contains('_') {
        let mut text = label.to_string();
        for key in ["roots", "depth", "dirs", "files", "ratio"] {
            text = text.replace(key, &format!("{key}="));
        }
        return text.replace('_', " ");
    }
    label.to_string()
}

pub fn format_out_dir(base: &Path, label: &str) -> PathBuf {
    base.join(label)
}
