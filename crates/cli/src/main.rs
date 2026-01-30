mod cli;
mod commands;
mod config;

use std::error::Error;

use clap::Parser;

use crate::cli::{Cli, Command};
use crate::commands::{run_list, run_tui};
use crate::config::{apply_args, load_config_with_context, resolve_sort_config};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let Cli {
        config,
        config_format,
        dotenv,
        no_dotenv,
        command,
    } = cli;

    let config =
        load_config_with_context(no_dotenv, dotenv.as_ref(), config.as_ref(), config_format)?;

    match command {
        Command::List(args) => {
            let mut config = config;
            apply_args(&mut config, &args);
            let (sort_config, warnings) = resolve_sort_config(config.sort.take(), &args)?;
            if let Some(sort_config) = sort_config {
                config.sort = Some(sort_config);
            }
            run_list(config, warnings)?;
        }
        Command::Tui(args) => {
            run_tui(config, args)?;
        }
    }

    Ok(())
}
