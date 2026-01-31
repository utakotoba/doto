### doto &middot; comment mark tracker

Track, filter, and navigate code comment anchors like TODO, NOTE and FIXME directly from your terminal.

### Install

```sh
cargo install doto
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

### Common options

```text
--include <GLOB>        Include glob patterns (repeatable)
--exclude <GLOB>        Exclude glob patterns (repeatable)
--gitignore <true|false> Follow .gitignore
--hidden <true|false>   Include hidden files
--read-buffer-size <N>  Read buffer size in bytes
--sort <STAGES>         Sort stages: mark,language,path,folder
```

### License

Copyright © 2026 Ly (Ling Yu). Licensed under the MIT.
