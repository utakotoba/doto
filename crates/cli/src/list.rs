use std::error::Error;
use std::sync::{Arc, Mutex};

use doto_core::{ScanConfig, scan_grouped};

use crate::config::Config;
use crate::messages::{MessageLevel, MessageSink, render_messages};
use crate::progress::DeferredProgress;
use crate::renderer::render_list;

pub fn run_list(config: Config, warnings: Vec<String>, verbose: bool) -> Result<(), Box<dyn Error>> {
    let roots = if config.roots.is_empty() {
        vec![std::env::current_dir()?]
    } else {
        config.roots
    };

    let mut builder = ScanConfig::builder().roots(roots.clone());
    let messages = Arc::new(Mutex::new(MessageSink::default()));
    let progress = DeferredProgress::new();
    let reporter = progress.clone();
    builder = builder.progress_reporter_arc(reporter);

    for include in config.include {
        builder = builder.include(include);
    }
    for exclude in config.exclude {
        builder = builder.exclude(exclude);
    }
    if let Some(gitignore) = config.gitignore {
        builder = builder.follow_gitignore(gitignore);
    }
    if let Some(hidden) = config.hidden {
        builder = builder.include_hidden(hidden);
    }
    if let Some(read_buffer_size) = config.read_buffer_size {
        builder = builder.read_buffer_size(read_buffer_size);
    }
    if let Some(sort_config) = &config.sort {
        builder = builder.sort_config(sort_config.clone());
    }
    if let Some(filter_config) = &config.filter {
        builder = builder.filter_config(filter_config.clone());
    }

    progress
        .clone()
        .start_if_slow(std::time::Duration::from_millis(1500));
    let result = scan_grouped(builder.build())?;
    progress.finish();
    if result.tree.total() > 76 {
        if let Ok(mut sink) = messages.lock() {
            sink.push(
                MessageLevel::Warning,
                format!(
                    "too many results ({}) to display well in the terminal; please narrow the scan directory or filters",
                    result.tree.total()
                ),
            );
        }
        render_messages(&messages.lock().unwrap().drain())?;
        return Ok(());
    }

    render_list(&result.tree, &roots, config.file_header)?;

    if result.tree.total() == 0 {
        if let Ok(mut sink) = messages.lock() {
            sink.push(MessageLevel::Success, "no marks found");
        }
    }

    if let Ok(mut sink) = messages.lock() {
        for warning in warnings {
            sink.push(MessageLevel::Warning, warning);
        }
        if has_issue_warnings(&result.stats) {
            push_issue_summary(&mut sink, &result.stats);
        }
        if verbose {
            push_skip_summary(&mut sink, &result.stats);
        }
        push_scan_summary(&mut sink, &result.stats);
    }

    render_messages(&messages.lock().unwrap().drain())?;
    Ok(())
}

fn has_issue_warnings(stats: &doto_core::ScanStats) -> bool {
    stats.issues.walk_errors > 0
        || stats.issues.metadata_errors > 0
        || stats.issues.io_errors > 0
}

fn push_scan_summary(sink: &mut MessageSink, stats: &doto_core::ScanStats) {
    let mut summary = format!(
        "scanned {} files ({} skipped)",
        stats.files_scanned, stats.files_skipped
    );
    if stats.skipped_issues > 0 {
        summary.push_str(&format!(", {} issues", stats.skipped_issues));
    }
    summary.push_str(&format!(", found {} marks", stats.matches));
    if stats.cancelled {
        summary.push_str(", cancelled");
    }
    sink.push(MessageLevel::Info, summary);
}

fn push_issue_summary(sink: &mut MessageSink, stats: &doto_core::ScanStats) {
    let mut parts = Vec::new();
    if stats.issues.walk_errors > 0 {
        parts.push(format!("{} traversal errors", stats.issues.walk_errors));
    }
    if stats.issues.metadata_errors > 0 {
        parts.push(format!("{} metadata errors", stats.issues.metadata_errors));
    }
    if stats.issues.io_errors > 0 {
        parts.push(format!("{} I/O errors", stats.issues.io_errors));
    }
    if !parts.is_empty() {
        sink.push(
            MessageLevel::Warning,
            format!("scan encountered {}", parts.join(", ")),
        );
    }
}

fn push_skip_summary(sink: &mut MessageSink, stats: &doto_core::ScanStats) {
    if stats.files_skipped == 0 {
        return;
    }
    let mut parts = Vec::new();
    if stats.skips.max_file_size > 0 {
        parts.push((stats.skips.max_file_size, "max size"));
    }
    if stats.skips.metadata > 0 {
        parts.push((stats.skips.metadata, "metadata"));
    }
    if stats.skips.io > 0 {
        parts.push((stats.skips.io, "I/O"));
    }
    if stats.skips.unsupported_syntax > 0 {
        parts.push((stats.skips.unsupported_syntax, "unsupported"));
    }
    if stats.skips.binary > 0 {
        parts.push((stats.skips.binary, "binary"));
    }
    if parts.is_empty() {
        return;
    }
    parts.sort_by(|a, b| b.0.cmp(&a.0));
    let top = parts
        .into_iter()
        .take(3)
        .map(|(count, label)| format!("{label} {count}"))
        .collect::<Vec<_>>();
    sink.push(
        MessageLevel::Info,
        format!("top skipped reasons: {}", top.join(", ")),
    );
}
