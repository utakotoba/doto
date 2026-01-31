![doto_header](https://github.com/user-attachments/assets/83006b00-bb4c-4ba4-93cf-867b9fe99d45)

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

### License

Copyright © 2026 Ly (Ling Yu). Licensed under the MIT.
