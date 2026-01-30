mod pipeline;
mod snippet;
mod style;

use std::io::{self, Write};
use std::path::{Path, PathBuf};

use colored::Colorize;
use doto_core::{Mark, ScanResult, SortConfig};

use crate::commands::renderer::pipeline::{GroupKey, GroupNode, build_group_tree};
use crate::commands::renderer::snippet::SnippetCache;
use crate::commands::renderer::style::{group_style_for, mark_header, mark_styled};

pub fn render_list(
    result: &ScanResult,
    roots: &[PathBuf],
    sort_config: Option<&SortConfig>,
    file_header: bool,
) -> io::Result<()> {
    let mut stdout = io::BufWriter::new(io::stdout());
    let line_width = line_number_width(result.marks.as_slice());
    let mut snippet_cache = SnippetCache::default();

    if result.marks.is_empty() {
        return Ok(());
    }

    let default_sort;
    let pipeline = if let Some(config) = sort_config {
        config.pipeline.as_slice()
    } else {
        default_sort = SortConfig::default();
        default_sort.pipeline.as_slice()
    };

    let groups = if pipeline.is_empty() {
        Vec::new()
    } else {
        build_group_tree(result.marks.as_slice(), pipeline, roots)
    };

    if groups.is_empty() {
        render_file_groups(
            &mut stdout,
            result.marks.as_slice(),
            roots,
            &mut snippet_cache,
            line_width,
            0,
            file_header,
        )?;
    } else {
        render_groups(
            &mut stdout,
            &groups,
            roots,
            &mut snippet_cache,
            line_width,
            0,
            file_header,
        )?;
    }
    stdout.flush()?;
    Ok(())
}

fn render_groups(
    out: &mut dyn Write,
    groups: &[GroupNode],
    roots: &[PathBuf],
    snippets: &mut SnippetCache,
    line_width: usize,
    depth: usize,
    file_header: bool,
) -> io::Result<()> {
    for group in groups {
        let label = group.label(roots);
        let header = format!("{label} ({})", group.count);
        let styled_header = match &group.key {
            GroupKey::Mark(mark) => mark_header(mark, &header),
            _ => group_style_for(&group.key).apply(header),
        };
        writeln!(out, "{}{}", indent(depth), styled_header)?;
        if !group.children.is_empty() {
            render_groups(
                out,
                &group.children,
                roots,
                snippets,
                line_width,
                depth + 1,
                file_header,
            )?;
        } else {
            render_file_groups(
                out,
                &group.items,
                roots,
                snippets,
                line_width,
                depth + 1,
                file_header,
            )?;
        }
    }
    Ok(())
}

fn render_file_groups(
    out: &mut dyn Write,
    items: &[Mark],
    roots: &[PathBuf],
    snippets: &mut SnippetCache,
    line_width: usize,
    depth: usize,
    file_header: bool,
) -> io::Result<()> {
    let mut buckets = group_by_file(items);
    for bucket in buckets.drain(..) {
        let mark_depth = if file_header { depth + 1 } else { depth };
        if file_header {
            let header = format!(
                "file: {} ({})",
                relativize_path(&bucket.path, roots).display(),
                bucket.items.len()
            );
            writeln!(out, "{}{}", indent(depth), header.bright_black().bold())?;
        }
        for mark in bucket.items {
            render_mark(out, &mark, roots, snippets, line_width, mark_depth)?;
        }
    }
    Ok(())
}

fn render_mark(
    out: &mut dyn Write,
    mark: &Mark,
    roots: &[PathBuf],
    snippets: &mut SnippetCache,
    line_width: usize,
    depth: usize,
) -> io::Result<()> {
    let relative = relativize_path(mark.path.as_ref(), roots);
    let styled_mark = mark_styled(mark.mark);
    writeln!(
        out,
        "{}{} {}",
        indent(depth),
        format!(
            "{}:{}:{}",
            relative.display(),
            mark.line,
            mark.column
        )
        .dimmed(),
        styled_mark
    )?;

    let line_text = snippets.line_for(mark.path.as_ref(), mark.line);
    let line_prefix = format!("{:>width$}", mark.line, width = line_width).dimmed();
    let content = line_text.unwrap_or("");
    writeln!(out, "{}{} | {}", indent(depth), line_prefix, content)?;
    Ok(())
}

fn indent(depth: usize) -> String {
    const INDENT: &str = "  ";
    INDENT.repeat(depth)
}

fn line_number_width(marks: &[Mark]) -> usize {
    let mut max_line = 1u32;
    for mark in marks {
        max_line = max_line.max(mark.line);
    }
    let digits = max_line.to_string().len();
    digits.max(3)
}

fn relativize_path(path: &Path, roots: &[PathBuf]) -> PathBuf {
    let mut best: Option<&PathBuf> = None;
    for root in roots {
        if path.starts_with(root) {
            match best {
                Some(current) if current.components().count() >= root.components().count() => {}
                _ => best = Some(root),
            }
        }
    }
    if let Some(root) = best {
        if let Ok(rel) = path.strip_prefix(root) {
            if rel.as_os_str().is_empty() {
                return PathBuf::from(".");
            }
            return rel.to_path_buf();
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        if let Ok(rel) = path.strip_prefix(cwd) {
            return rel.to_path_buf();
        }
    }
    path.file_name()
        .map(PathBuf::from)
        .unwrap_or_else(|| path.to_path_buf())
}

struct FileBucket {
    path: PathBuf,
    items: Vec<Mark>,
}

fn group_by_file(items: &[Mark]) -> Vec<FileBucket> {
    let mut buckets: Vec<FileBucket> = Vec::new();
    let mut index: std::collections::HashMap<PathBuf, usize> = std::collections::HashMap::new();

    for mark in items {
        let key = (*mark.path).clone();
        if let Some(&idx) = index.get(&key) {
            buckets[idx].items.push(mark.clone());
        } else {
            let idx = buckets.len();
            buckets.push(FileBucket {
                path: key.clone(),
                items: vec![mark.clone()],
            });
            index.insert(key, idx);
        }
    }

    buckets
}
