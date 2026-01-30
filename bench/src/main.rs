mod args;
mod generator;
mod matrix;
mod options;
mod runner;

use crate::args::{Command, parse_args};
use crate::matrix::run_matrix;
use crate::runner::{run_once, run_summary};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match parse_args(std::env::args().skip(1))? {
        Command::Run(options) => {
            let summary = run_once(options)?;
            run_summary("run", &summary);
        }
        Command::Matrix(options) => {
            run_matrix(options)?;
        }
    }
    Ok(())
}
