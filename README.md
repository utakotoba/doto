![doto_header](https://github.com/user-attachments/assets/83006b00-bb4c-4ba4-93cf-867b9fe99d45)

### doto &middot; comment mark tracker

Track, filter, and navigate code comment anchors like TODO, NOTE and FIXME directly from your terminal.

### Features

- Fast workspace scanning with low memory usage
- Built-in mark detection (TODO, FIXME, NOTE, WARN, ERROR, INFO)
- Filters by mark, language, path, and folder
- Sort and group by mark, language, path, or folder in pipeline
- Respects .gitignore and common build/artifact directories
- Cancellation and progress support

### Install

Currently, we only support to install by cargo. Add Homebrew in the near future (via Tap).

```sh
cargo install doto --locked
```

### Basic usage

Scan the current directory:

```sh
# Scan the current directory.
doto
```

Scan specific paths:

```sh
# Scan only the specified paths.
doto crates/cli crates/core
```

Limit to specific marks or languages:

```sh
# Filter to specific marks and languages.
doto --filter-mark TODO --filter-mark FIXME --filter-language rs
```

### Sorting pipeline

Sort and group via a pipeline of stages. Stages are: `mark`, `language`, `path`, `folder`.

```sh
# Group by mark, then language, then folder.
doto --sort mark,language,folder
```

Stage overrides only apply when that stage exists in the pipeline:

```sh
# Override mark priority inside the mark stage.
doto --sort mark --sort-mark-priority "FIXME=0,TODO=1"

# Change language grouping order.
doto --sort language --sort-lang-order name

# Path grouping order.
doto --sort path --sort-path-order desc

# Folder grouping: depth relative to scan root, and order.
doto --sort folder --sort-folder-depth 2 --sort-folder-order desc
```

### Performance Notes

On a Chromium-sized repo (~7M SLOC), scanning completes in ~3s with ~55MB peak memory on a modern laptop.

> We are going to add detailed benchmark here...

### License

Copyright © 2026 Ly (Ling Yu). Licensed under the MIT.
