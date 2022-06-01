# Noted

CLI & TUI application to take and track notes.

Generate Coverage (with `cargo-llvm-cov`):

LCOV: `cargo llvm-cov --all-features --workspace --lcov --output-path coverage/lcov.info`

HTML: `cargo llvm-cov --all-features --workspace --html`

## Crate Structure

- data - contains test dbs and some (as of yet unused) sql functions
- src
    - bin - The runners for each section of the application, thin runners that only call a function from the matching "bins" module
        - cli.rs - The runner for the command line interface
        - icli.rs - The runner for the "interactive" command line interface (think "$ gh repo create")
        - runner.rs - Generic bin used for testing and development
        - tui.rs - The runner for the full blown terminal interface
    - bins - Contains the actual code that is related to each different binary in "bin"
    - db - Database code. Current implemented as a simple binary file. Plan to expand to a more "Pluggable" Datasource.
    - macros - Simple / helpful macros used within the crate. Currently unused.
    - server - Currently unused.
    - services - Currently unused, contains the beginnings of a "Repository" class for data access abstraction.
    - term_ui - Currently unused, will eventually contain the widgets and whatnot that make up the "TUI" portion of the application.
    - types - Types used throughout the crate, including the "full" "Note" struct, the "Error" and "Result" types, note DTOs, and api parameters.
    - util - Common utilities used throughout the crate.
        - id.rs - Contains the "TinyId" type used as a shorter, more user-friendly, alternative to full blown UUIDs.
        - persist.rs - Contains utilities for saving and loading types from bytes and files, supporting multiple serialization methods.
        - variadic - Contains the `ZeroOrMore` and `OneOrMore` types which can be used to somewhat simulate variadic arguments for functions. Used as such: `fn takes_any_number_of_strings(values: impl Into<OneOrMore<String>>)`. This would be able to called as either `takes_any_number_of_strings("Hello".to_string())` or `takes_any_number_of_strings(&["Hello".to_string(), "World".to_string()])`
        - wrapping - A small experimentation with almost completely const wrapping numbers, such that `pub type FiveThroughTen = Wrapping<5, 10>` would make the `FiveThroughTen` type 100% sure to include a number that is >= 5 and <= 10, and compatible with all standard mathematical operations. *Mostly* complete but it is unclear how useful this will end up being.