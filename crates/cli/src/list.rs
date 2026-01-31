use std::error::Error;
use std::sync::{Arc, Mutex};

use doto_core::{ScanConfig, scan_grouped};

use crate::config::Config;
use crate::messages::{MessageLevel, MessageSink, render_messages};
use crate::progress::DeferredProgress;
use crate::renderer::render_list;

pub fn run_list(config: Config, warnings: Vec<String>) -> Result<(), Box<dyn Error>> {
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

    if let Some(regex) = config.regex {
        builder = builder.regex(regex);
    }
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

    if !warnings.is_empty()
        || result.stats.issues.walk_errors > 0
        || result.stats.issues.metadata_errors > 0
        || result.stats.issues.io_errors > 0
        || result.stats.files_skipped > 0
    {
        if let Ok(mut sink) = messages.lock() {
            for warning in warnings {
                sink.push(MessageLevel::Warning, warning);
            }
            push_issue_summary(&mut sink, &result.stats);
            push_skip_summary(&mut sink, &result.stats);
        }
    }

    render_messages(&messages.lock().unwrap().drain())?;
    Ok(())
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
        parts.push(format!("max size {}", stats.skips.max_file_size));
    }
    if stats.skips.metadata > 0 {
        parts.push(format!("metadata {}", stats.skips.metadata));
    }
    if stats.skips.io > 0 {
        parts.push(format!("I/O {}", stats.skips.io));
    }
    if stats.skips.unsupported_syntax > 0 {
        parts.push(format!("unsupported {}", stats.skips.unsupported_syntax));
    }
    if stats.skips.binary > 0 {
        parts.push(format!("binary {}", stats.skips.binary));
    }
    if !parts.is_empty() {
        sink.push(
            MessageLevel::Warning,
            format!("skipped files by reason: {}", parts.join(", ")),
        );
    }
}
