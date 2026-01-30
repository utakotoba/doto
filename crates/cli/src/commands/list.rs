use std::error::Error;
use std::io::{self, Write};

use doto_core::{ScanConfig, scan};
use colored::Colorize;

use crate::commands::renderer::render_list;
use crate::config::Config;
use crate::progress::DeferredProgress;

pub fn run_list(config: Config, warnings: Vec<String>) -> Result<(), Box<dyn Error>> {
    let roots = if config.roots.is_empty() {
        vec![std::env::current_dir()?]
    } else {
        config.roots
    };

    let mut builder = ScanConfig::builder().roots(roots.clone());
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
    if let Some(max_file_size) = config.max_file_size {
        builder = builder.max_file_size(Some(max_file_size));
    }
    if let Some(threads) = config.threads {
        builder = builder.threads(Some(threads));
    }
    if let Some(read_buffer_size) = config.read_buffer_size {
        builder = builder.read_buffer_size(read_buffer_size);
    }
    if let Some(sort_config) = &config.sort {
        builder = builder.sort_config(sort_config.clone());
    }

    progress.clone().start_if_slow(std::time::Duration::from_millis(1500));
    let result = scan(builder.build())?;
    progress.finish();
    render_list(&result, &roots, config.sort.as_ref(), config.file_header)?;

    if result.marks.is_empty() {
        let mut stdout = io::BufWriter::new(io::stdout());
        writeln!(stdout, "{}", "no marks found".green())?;
        stdout.flush()?;
    }

    if !warnings.is_empty() || !result.warnings.is_empty() {
        let mut stderr = io::BufWriter::new(io::stderr());
        writeln!(stderr)?;
        for warning in warnings {
            writeln!(stderr, "{}", format!("warning: {warning}").yellow())?;
        }
        for warning in result.warnings {
            if let Some(path) = warning.path {
                let path_display = path.display();
                writeln!(
                    stderr,
                    "{}",
                    format!("warning: {}: {}", path_display, warning.message).yellow()
                )?;
            } else {
                writeln!(
                    stderr,
                    "{}",
                    format!("warning: {}", warning.message).yellow()
                )?;
            }
        }
        stderr.flush()?;
    }

    Ok(())
}
