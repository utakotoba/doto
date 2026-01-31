mod cli;
mod config;
mod list;
mod messages;
mod progress;
mod renderer;

use std::error::Error;

use clap::Parser;

use crate::cli::Cli;
use crate::config::{apply_args, load_config_with_context, resolve_sort_config};
use crate::list::run_list;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let Cli {
        ref config,
        config_format,
        ref dotenv,
        no_dotenv,
        ..
    } = cli;

    let config =
        load_config_with_context(no_dotenv, dotenv.as_ref(), config.as_ref(), config_format)?;

    let mut config = config;
    apply_args(&mut config, &cli);
    let (sort_config, warnings) = resolve_sort_config(config.sort.take(), &cli)?;
    if let Some(sort_config) = sort_config {
        config.sort = Some(sort_config);
    }
    run_list(config, warnings)?;

    Ok(())
}
