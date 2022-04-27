# Noted

CLI & TUI application to take and track notes.

Generate Coverage (with `cargo-llvm-cov`):
LCOV: `cargo llvm-cov --all-features --workspace --lcov --output-path coverage/lcov.info`
HTML: `cargo llvm-cov --all-features --workspace --html`

Todolist:
- [ ] Write app-wide types into their own `types` module.
- [ ] Setup app-wide `Error` and `Result` types.
- [ ] Explore fuzzy text matching, library vs hand-rolled.
- [ ] Explore revision, change tracking, and rollbacks.
- [ ] Setup database access and testing.
    - [ ] Multiple databases?
    - [ ] "Code to the interface, not the implementation"
    - [ ] What access lib, `rusqlite`, `diesel`, `sqlx`, etc?
    - [ ] File database? Cloud database? All of the above?
    - [ ] RocksDB? Sqlite? Bonzai?
- [ ] Work with `clap` (probably?) for command line arg parsing for `cli` bin
- [ ] Work with `tui` (probably?) for user interface for `tui` bin
- [ ] Explore client-server architecture for "pluggable" front-ends
    - [ ] (g)RPC?
