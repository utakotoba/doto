use colored::Colorize;

use crate::options::{MatrixOptions, RunOptions};
use crate::runner::{format_out_dir, run_once, run_summary};

pub fn run_matrix(options: MatrixOptions) -> Result<(), Box<dyn std::error::Error>> {
    let mut count = 0usize;
    for roots in &options.roots {
        for depth in &options.depth {
            for dirs in &options.dirs_per_level {
                for files in &options.files_per_dir {
                    for ratio in &options.mark_ratio {
                        count += 1;
                        let label = format!(
                            "roots{}_depth{}_dirs{}_files{}_ratio{}",
                            roots,
                            depth,
                            dirs,
                            files,
                            ratio
                        );
                        let out_dir = options
                            .out_dir
                            .as_ref()
                            .map(|base| format_out_dir(base, &label));
                        let run = RunOptions {
                            roots: *roots,
                            depth: *depth,
                            dirs_per_level: *dirs,
                            files_per_dir: *files,
                            min_lines: options.min_lines,
                            max_lines: options.max_lines,
                            mark_ratio: *ratio,
                            seed: options.seed,
                            out_dir,
                            scan: true,
                        };
                        let summary = run_once(run)?;
                        run_summary(&label, &summary);
                    }
                }
            }
        }
    }
    println!("{}", format!("matrix runs: {count}").dimmed());
    Ok(())
}
