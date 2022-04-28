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
- [ ] So far there are 3 planned "front-ends", the normal cli (one command, with args, like `git`), the full interactive terminal interface using `tui` crate, and the "interactive cli" which I envision to be similar to the experience of running `gh repo create`, the app will query the user for information and build a command / request out of their responses, somewhat between the other two. In outlining these it is clear that there needs to be some layer between the [`Database`](./src/db/file.rs) and the front-ends. The front-end parts should essentially be able to send in their request, and then await a response. That response should then be displayed to the user, but how this is done will depend on the front-end. This brings us to a few different architectural components:
    - [ ] "Parsing" (bad word for this) type "Service" - turns the front-end input into a request
    - [ ] "Repository" type "Service" - takes requests, applies them against the database, returns the results
    - [ ] "Printer" type "Service" - takes a database response and formats it in a nice way to be displayed by the front-end. This will very much be front-end specific, but there might be some overlap between how some of them do things (i.e. the `cli` and the `icli` will probably display results in the same way)
- [ ] Explore client-server architecture for "pluggable" front-ends
    - [ ] (g)RPC?
