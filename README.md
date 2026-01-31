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
doto
```

Scan specific paths:

```sh
doto crates/cli crates/core
```

Limit to specific marks or languages:

```sh
doto --filter-mark TODO --filter-mark FIXME --filter-language rs
```

### Performance Notes

On a Chromium-sized repo (~7M SLOC), scanning completes in ~3s with ~55MB peak memory on a modern laptop.

### License

Copyright © 2026 Ly (Ling Yu). Licensed under the MIT.
