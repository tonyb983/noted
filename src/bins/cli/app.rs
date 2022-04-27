// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields)]
struct NoteShape {
    title: String,
    content: String,
    tags: Vec<String>,
}

fn validate_add_note_json(s: &str) -> Result<(), String> {
    let _note: NoteShape = serde_json::from_str(s).map_err(|err| err.to_string())?;
    Ok(())
}

fn add_note_cmd() -> clap::Command<'static> {
    clap::Command::new("add")
        .about("Add a new note")
        .arg(
            clap::Arg::new("title")
                .help("The title, or heading, of the note.")
                .short('t')
                .long("title")
                .alias("heading")
                .alias("header")
                // .default_value("")
                .takes_value(true)
                .required(false),
        )
        .arg(
            clap::Arg::new("content")
                .help("The main content, or body, of the note.")
                .short('c')
                .short_alias('b')
                .long("content")
                .alias("body")
                // .default_value("")
                .takes_value(true)
                .required(false),
        )
        .arg(
            clap::Arg::new("tags")
                .help("Tags to add to the note. Separate multiple tags with commas.")
                .short('T')
                .long("tags")
                // .default_value("")
                .takes_value(true)
                .required(false)
                .use_value_delimiter(true)
                .multiple_values(true)
                .require_value_delimiter(true),
        )
        .arg(
            clap::Arg::new("json")
                .short('j')
                .long("json")
                .takes_value(true)
                .required(false)
                .exclusive(true)
                .validator(validate_add_note_json),
        )
}

fn search_notes_cmd() -> clap::Command<'static> {
    clap::Command::new("find")
        .about("Searches through existing notes for the given text.")
        .aliases(&["search", "fd"])
}

fn list_all_cmd() -> clap::Command<'static> {
    clap::Command::new("list")
        .alias("ls")
        .about("List all notes")
}

pub fn create_app() -> clap::Command<'static> {
    clap::Command::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .subcommand(list_all_cmd())
        .subcommand(add_note_cmd())
        .subcommand(search_notes_cmd())
}

pub fn create_test_app() -> clap::Command<'static> {
    clap::Command::new("test").subcommand(
        clap::Command::new("sub1").arg(
            clap::Arg::new("arg1")
                .short('a')
                .long("arg")
                .takes_value(true)
                .exclusive(true),
        ),
    )
}
