use std::fs;
use std::path::Path;

use rand::rngs::StdRng;
use rand::Rng;

use crate::options::RunOptions;

pub fn generate_tree(
    root: &Path,
    options: &RunOptions,
    rng: &mut StdRng,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut bytes = 0u64;
    generate_dirs(root, options, rng, 0, &mut bytes)?;
    Ok(bytes)
}

fn generate_dirs(
    dir: &Path,
    options: &RunOptions,
    rng: &mut StdRng,
    depth: usize,
    bytes: &mut u64,
) -> Result<(), Box<dyn std::error::Error>> {
    for file_idx in 0..options.files_per_dir {
        let lang = pick_language(rng);
        let file_name = format!("file_{file_idx}.{}", lang.ext);
        let path = dir.join(file_name);
        *bytes += write_file(&path, &lang, options, rng)?;
    }

    if depth >= options.depth {
        return Ok(());
    }

    for dir_idx in 0..options.dirs_per_level {
        let child = dir.join(format!("dir_{depth}_{dir_idx}"));
        fs::create_dir_all(&child)?;
        generate_dirs(&child, options, rng, depth + 1, bytes)?;
    }

    Ok(())
}

struct Lang {
    ext: &'static str,
    line_comment: &'static str,
    block_start: Option<&'static str>,
    block_end: Option<&'static str>,
    raw_string: bool,
}

fn pick_language(rng: &mut StdRng) -> Lang {
    match rng.gen_range(0..6) {
        0 => Lang {
            ext: "rs",
            line_comment: "//",
            block_start: Some("/*"),
            block_end: Some("*/"),
            raw_string: true,
        },
        1 => Lang {
            ext: "ts",
            line_comment: "//",
            block_start: Some("/*"),
            block_end: Some("*/"),
            raw_string: false,
        },
        2 => Lang {
            ext: "py",
            line_comment: "#",
            block_start: None,
            block_end: None,
            raw_string: false,
        },
        3 => Lang {
            ext: "go",
            line_comment: "//",
            block_start: Some("/*"),
            block_end: Some("*/"),
            raw_string: false,
        },
        4 => Lang {
            ext: "toml",
            line_comment: "#",
            block_start: None,
            block_end: None,
            raw_string: false,
        },
        _ => Lang {
            ext: "sh",
            line_comment: "#",
            block_start: None,
            block_end: None,
            raw_string: false,
        },
    }
}

fn write_file(
    path: &Path,
    lang: &Lang,
    options: &RunOptions,
    rng: &mut StdRng,
) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = rng.gen_range(options.min_lines..=options.max_lines);
    let mut content = String::new();
    for idx in 0..lines {
        let line = generate_line(lang, options, rng, idx);
        content.push_str(&line);
        content.push('\n');
    }
    fs::write(path, content.as_bytes())?;
    Ok(content.len() as u64)
}

fn generate_line(lang: &Lang, options: &RunOptions, rng: &mut StdRng, idx: usize) -> String {
    let roll: f64 = rng.r#gen();
    if roll < options.mark_ratio {
        let mark = pick_mark(rng);
        return format!("{} {}: generated", lang.line_comment, mark);
    }

    if roll < options.mark_ratio + 0.05 {
        let mark = pick_mark(rng);
        if lang.raw_string {
            return format!("let s = r#\"{} {} inside string\"#;", lang.line_comment, mark);
        }
        if lang.ext == "go" {
            return format!("const s = `{} {} raw`;", lang.line_comment, mark);
        }
        return format!("let s = \"{} {} inside string\";", lang.line_comment, mark);
    }

    if roll < options.mark_ratio + 0.08 {
        if let (Some(start), Some(end)) = (lang.block_start, lang.block_end) {
            return format!("{start} block {end}");
        }
    }

    format!("let v{} = {};", idx, rng.gen_range(0..1024))
}

fn pick_mark(rng: &mut StdRng) -> &'static str {
    match rng.gen_range(0..6) {
        0 => "TODO",
        1 => "FIXME",
        2 => "NOTE",
        3 => "WARN",
        4 => "ERROR",
        _ => "INFO",
    }
}
