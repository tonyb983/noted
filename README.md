# Noted

CLI & TUI application to take and track notes.

Generate Coverage (with `cargo-llvm-cov`):

LCOV: `cargo llvm-cov --all-features --workspace --lcov --output-path coverage/lcov.info`

HTML: `cargo llvm-cov --all-features --workspace --html`

## Crate Structure

```
├── data - contains test dbs and some (as of yet unused) sql functions
└── src
    ├── bin - The runners for each section of the application, thin runners that only call a function from the matching "bins" module
    │   ├── cli.rs - The runner for the command line interface
    │   ├── icli.rs - The runner for the "interactive" command line interface (think "$ gh repo create")
    │   ├── runner.rs - Generic bin used for testing and development
    │   └── tui.rs - The runner for the full blown terminal interface
    ├── bins - Contains the actual code that is run from each binary in "bin"
    ├── db - Database code. Current implemented as a simple binary file. Plan to expand to a more "Pluggable" Datasource.
    ├── macros - Simple / helpful macros used within the crate. Currently unused.
    ├── server - Currently unused.
    ├── services - Currently unused, contains the beginnings of a "Repository" class for data access abstraction.
    ├── term_ui - Currently unused, will eventually contain the widgets and whatnot that make up the "TUI" portion of the application.
    ├── types - Types used throughout the crate, including the "full" "Note" struct, the "Error" and "Result" types, note DTOs, and api parameters.
    └── util - Common utilities used throughout the crate.
        ├── id.rs - Contains the "TinyId" type used as a shorter, more user-friendly, alternative to full blown UUIDs.
        └── persist.rs - Contains utilities for saving and loading types from bytes and files, supporting multiple serialization methods.
```

