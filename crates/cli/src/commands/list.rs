use std::error::Error;
use std::io::{self, Write};

use koda_core::{ScanConfig, scan};

use crate::config::Config;

pub fn run_list(config: Config) -> Result<(), Box<dyn Error>> {
    let roots = if config.roots.is_empty() {
        vec![std::env::current_dir()?]
    } else {
        config.roots
    };

    let mut builder = ScanConfig::builder().roots(roots);

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

    let result = scan(builder.build())?;

    let mut stdout = io::BufWriter::new(io::stdout());
    for mark in result.marks {
        writeln!(
            stdout,
            "{}:{}:{} {}",
            mark.path.display(),
            mark.line,
            mark.column,
            mark.mark
        )?;
    }
    stdout.flush()?;

    if !result.warnings.is_empty() {
        let mut stderr = io::BufWriter::new(io::stderr());
        for warning in result.warnings {
            if let Some(path) = warning.path {
                writeln!(stderr, "warning: {}: {}", path.display(), warning.message)?;
            } else {
                writeln!(stderr, "warning: {}", warning.message)?;
            }
        }
        stderr.flush()?;
    }

    Ok(())
}
