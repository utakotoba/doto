use std::path::PathBuf;

use crate::options::{MatrixOptions, RunOptions};

pub enum Command {
    Run(RunOptions),
    Matrix(MatrixOptions),
}

pub fn parse_args(
    mut args: impl Iterator<Item = String>,
) -> Result<Command, Box<dyn std::error::Error>> {
    let mut peek = args.next();
    let is_matrix = matches!(peek.as_deref(), Some("matrix"));
    let is_run = matches!(peek.as_deref(), Some("run"));
    if is_matrix || is_run {
        peek = None;
    }
    let iter = peek.into_iter().chain(args);

    if is_matrix {
        parse_matrix(iter)
    } else {
        parse_run(iter)
    }
}

fn parse_run(
    mut args: impl Iterator<Item = String>,
) -> Result<Command, Box<dyn std::error::Error>> {
    let mut options = RunOptions::default();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--roots" => {
                options.roots = parse_usize(args.next(), "--roots")?;
            }
            "--depth" => {
                options.depth = parse_usize(args.next(), "--depth")?;
            }
            "--dirs" => {
                options.dirs_per_level = parse_usize(args.next(), "--dirs")?;
            }
            "--files" => {
                options.files_per_dir = parse_usize(args.next(), "--files")?;
            }
            "--min-lines" => {
                options.min_lines = parse_usize(args.next(), "--min-lines")?;
            }
            "--max-lines" => {
                options.max_lines = parse_usize(args.next(), "--max-lines")?;
            }
            "--mark-ratio" => {
                options.mark_ratio = parse_f64(args.next(), "--mark-ratio")?;
            }
            "--seed" => {
                options.seed = parse_u64(args.next(), "--seed")?;
            }
            "--out" => {
                let value = args.next().ok_or("--out expects a value")?;
                options.out_dir = Some(PathBuf::from(value));
            }
            "--no-scan" => {
                options.scan = false;
            }
            _ => {
                return Err(format!("unknown arg: {arg}").into());
            }
        }
    }
    Ok(Command::Run(options))
}

fn parse_matrix(
    mut args: impl Iterator<Item = String>,
) -> Result<Command, Box<dyn std::error::Error>> {
    let mut options = MatrixOptions::default();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--roots" => {
                options.roots = parse_list_usize(args.next(), "--roots")?;
            }
            "--depth" => {
                options.depth = parse_list_usize(args.next(), "--depth")?;
            }
            "--dirs" => {
                options.dirs_per_level = parse_list_usize(args.next(), "--dirs")?;
            }
            "--files" => {
                options.files_per_dir = parse_list_usize(args.next(), "--files")?;
            }
            "--min-lines" => {
                options.min_lines = parse_usize(args.next(), "--min-lines")?;
            }
            "--max-lines" => {
                options.max_lines = parse_usize(args.next(), "--max-lines")?;
            }
            "--mark-ratio" => {
                options.mark_ratio = parse_list_f64(args.next(), "--mark-ratio")?;
            }
            "--seed" => {
                options.seed = parse_u64(args.next(), "--seed")?;
            }
            "--out" => {
                let value = args.next().ok_or("--out expects a value")?;
                options.out_dir = Some(PathBuf::from(value));
            }
            _ => {
                return Err(format!("unknown arg: {arg}").into());
            }
        }
    }
    Ok(Command::Matrix(options))
}

fn parse_list_usize(
    value: Option<String>,
    name: &str,
) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let value = value.ok_or_else(|| format!("{name} expects a value"))?;
    value
        .split(',')
        .map(|item| {
            let item = item.trim();
            if item.is_empty() {
                Err(format!("{name} contains empty value").into())
            } else {
                Ok(item.parse()?)
            }
        })
        .collect()
}

fn parse_list_f64(
    value: Option<String>,
    name: &str,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let value = value.ok_or_else(|| format!("{name} expects a value"))?;
    value
        .split(',')
        .map(|item| {
            let item = item.trim();
            if item.is_empty() {
                Err(format!("{name} contains empty value").into())
            } else {
                Ok(item.parse()?)
            }
        })
        .collect()
}

fn parse_usize(value: Option<String>, name: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let value = value.ok_or_else(|| format!("{name} expects a value"))?;
    Ok(value.parse()?)
}

fn parse_u64(value: Option<String>, name: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let value = value.ok_or_else(|| format!("{name} expects a value"))?;
    Ok(value.parse()?)
}

fn parse_f64(value: Option<String>, name: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let value = value.ok_or_else(|| format!("{name} expects a value"))?;
    Ok(value.parse()?)
}
