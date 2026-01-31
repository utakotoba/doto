mod snippet;
mod style;

use std::io::{self, Write};
use std::path::{Path, PathBuf};

use colored::Colorize;
use doto_core::{GroupKey, GroupNode, GroupTree, Mark};
use crate::renderer::snippet::SnippetCache;
use crate::renderer::style::{group_style_for, mark_header, mark_styled};

pub fn render_list(tree: &GroupTree, roots: &[PathBuf], file_header: bool) -> io::Result<()> {
    let mut stdout = io::BufWriter::new(io::stdout());
    let line_width = line_number_width(tree);
    let mut snippet_cache = SnippetCache::default();

    if tree.total() == 0 {
        return Ok(());
    }

    if tree.groups.is_empty() {
        render_file_groups(
            &mut stdout,
            tree.items.as_slice(),
            roots,
            &mut snippet_cache,
            line_width,
            0,
            file_header,
        )?;
    } else {
        render_groups(
            &mut stdout,
            &tree.groups,
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
        let label = group_label(&group.key, roots);
        let header = format!("{label} ({})", group.count);
        let styled_header = match &group.key {
            GroupKey::Mark(mark) => mark_header(mark, &header),
            _ => group_style_for(&group.key).apply(header),
        };
        writeln!(out, "{}{}", indent(depth), styled_header)?;
        if !group.groups.is_empty() {
            render_groups(
                out,
                &group.groups,
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

fn line_number_width(tree: &GroupTree) -> usize {
    let mut max_line = 1u32;
    if !tree.items.is_empty() {
        for mark in &tree.items {
            max_line = max_line.max(mark.line);
        }
    }
    if !tree.groups.is_empty() {
        max_line = max_line.max(max_line_in_groups(&tree.groups));
    }
    let digits = max_line.to_string().len();
    digits.max(3)
}

fn max_line_in_groups(groups: &[GroupNode]) -> u32 {
    let mut max_line = 1u32;
    for group in groups {
        for mark in &group.items {
            max_line = max_line.max(mark.line);
        }
        if !group.groups.is_empty() {
            max_line = max_line.max(max_line_in_groups(&group.groups));
        }
    }
    max_line
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

fn group_label(key: &GroupKey, roots: &[PathBuf]) -> String {
    match key {
        GroupKey::Mark(value) => format!("mark: {value}"),
        GroupKey::Language(value) => format!("language: {value}"),
        GroupKey::Path(value) => format!("path: {}", display_group_path(value, roots)),
        GroupKey::Folder(value) => format!("folder: {}", display_group_path(value, roots)),
    }
}

fn display_group_path(path: &Path, roots: &[PathBuf]) -> String {
    if !path.is_absolute() {
        if path.as_os_str().is_empty() {
            return ".".to_string();
        }
        return path.display().to_string();
    }
    relativize_path(path, roots).display().to_string()
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
