<!--
 Copyright (c) 2022 Tony Barbitta
 
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

# General Todo-List

- [ ] Write app-wide types into their own `types` module.
    - [x] [error](./src/types/error.rs) - Crate wide `Error` and `Result` types. Mostly Complete.
    - [x] [note](./src/types/note.rs) - The full "Note" struct. Dtos are found in [note_dto](./src/types/note_dto.rs). Mostly Complete.
    - [ ] [api](./src/types/api/mod.rs) - These are the "standard api parameters", filtering, ordering, and sorting. Mostly Complete.
    - [ ] [api](./src/types/api/mod.rs) - These are the "standard api parameters", filtering, ordering, and sorting. Mostly Complete.
    - [ ] [taglist](./src/types/taglist.rs) - Just an idea, having the list of tags inside of `Note` be its own separate struct, which would be a thin wrapper around `Vec<String>` that adds some necessary functionality. Implemented but not yet used, TBD if it will ever be.
- [ ] Explore fuzzy text matching, library vs hand-rolled.
- [ ] Explore revision, change tracking, and rollbacks.
- [ ] Consider pulling `Persistence` and `TinyId` into their own separate crate / repo.
    - [ ] `Persistence`
    - [x] [`tinyid`](https://crates.io/crates/tinyid)
- [ ] Setup database access and testing.
    - [ ] Multiple databases?
    - [ ] "Code to the interface, not the implementation"
    - [ ] What access lib, `rusqlite`, `diesel`, `sqlx`, etc?
    - [ ] File database? Cloud database? All of the above?
    - [ ] RocksDB? Sqlite? Bonzai?
- [ ] Work with `clap` (probably?) for command line arg parsing for `cli` bin
- [x] Build components of the `icli` bin
    - All are now "fundamentally complete", but probably buggy as all hell
    - Added dependencies: `termimad` for markdown rendering of notes, `minime` for multiline terminal text editing, which could not be accomplished using either `dialoguer` or `inquire` (`dialoguer` *does* have an `Editor` component, but it does not seem to work with the "EDITOR" environmental variable which honestly makes it seem pretty pointless, I don't want to have to dig and probe around the users "PATH" variable to find a working text editor, especially if they have already created their own default using the accepted standard method) 
- [ ] Work with `tui` (probably?) for user interface for `tui` bin
- [ ] So far there are 3 planned "front-ends", the normal cli (one command, with args, like `git`), the full interactive terminal interface using `tui` crate, and the "interactive cli" which I envision to be similar to the experience of running `gh repo create`, the app will query the user for information and build a command / request out of their responses, somewhat between the other two. In outlining these it is clear that there needs to be some layer between the [`Database`](./src/db/file.rs) and the front-ends. The front-end parts should essentially be able to send in their request, and then await a response. That response should then be displayed to the user, but how this is done will depend on the front-end. This brings us to a few different architectural components:
    - [ ] "Parsing" (bad word for this) type "Service" - turns the front-end input into a request
    - [ ] "Repository" type "Service" - takes requests, applies them against the database, returns the results
    - [ ] "Printer" type "Service" - takes a database response and formats it in a nice way to be displayed by the front-end. This will very much be front-end specific, but there might be some overlap between how some of them do things (i.e. the `cli` and the `icli` will probably display results in the same way)
- [ ] Explore client-server architecture for "pluggable" front-ends
    - [ ] (g)RPC?