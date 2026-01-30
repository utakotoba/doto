mod cli;
mod generator;
mod matrix;
mod options;
mod runner;

use clap::Parser;

use crate::cli::{Cli, Command};
use crate::matrix::run_matrix;
use crate::options::{MatrixOptions, RunOptions};
use crate::runner::{label_for_run, run_once, run_summary};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Run(args) => {
            let options = RunOptions::from(args);
            let label = label_for_run(&options);
            let summary = run_once(options)?;
            run_summary(&label, &summary);
        }
        Command::Matrix(args) => {
            let options = MatrixOptions::from(args);
            run_matrix(options)?;
        }
    }
    Ok(())
}
