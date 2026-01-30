mod cli;
mod dotenv;
mod run;
mod settings;

use std::error::Error;

use clap::Parser;

use crate::cli::{Cli, Command};
use crate::dotenv::load_dotenv;
use crate::run::run_list;
use crate::settings::{apply_args, load_settings};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let Cli {
        config,
        config_format,
        dotenv,
        no_dotenv,
        command,
    } = cli;

    match command {
        Command::List(args) => {
            load_dotenv(no_dotenv, dotenv.as_ref())?;
            let mut settings = load_settings(config.as_ref(), config_format)?;
            apply_args(&mut settings, args);
            run_list(settings)?;
        }
    }

    Ok(())
}
