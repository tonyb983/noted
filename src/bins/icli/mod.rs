// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//! `icli` Module
//!
//! This is going to be an **interactive** cli application, along the lines of `gh repo create`. It will not be a full blown TUI,
//! with a full interface and whatnot, but it will query the user in a pretty way, prompting for information until a request or
//! command has been "built", at which point it will execute and display results, very much like the normal `cli`.
