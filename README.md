# vola-bitcask

A tiny Rust implementation of a [Bitcask](https://riak.com/assets/bitcask-intro.pdf)-style
key-value store, built for learning purposes.

## What this is

Bitcask is a log-structured storage engine: writes are appended to a log file,
and an in-memory index maps each key to its latest position on disk. This
project is a minimal, educational implementation — not production-ready.

## Features

- `set <key> <value>` — append a write to the log and update the index
- `get <key>` — look up a key via the in-memory index and read its value from disk
- `scan` — iterate over all keys currently in the index
- In-memory index (key → file offset) rebuilt/maintained as writes happen

## Not implemented (yet)

- Compaction / merging of old log segments
- Deletes (tombstones)
- Concurrent access

## Usage

```bash
cargo run -- set foo bar
cargo run -- get foo
cargo run -- scan
```
