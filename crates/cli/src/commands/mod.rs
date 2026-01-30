mod list;

use std::error::Error;

use crate::cli::TuiArgs;
use crate::config::Config;

pub use list::run_list;
mod renderer;

pub fn run_tui(_config: Config, _args: TuiArgs) -> Result<(), Box<dyn Error>> {
    Err("TUI mode is not implemented yet".into())
}
