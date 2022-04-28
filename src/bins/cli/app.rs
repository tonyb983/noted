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

pub enum Subcommand {
    Add,
    Search,
    List,
    Update,
    Delete,
}

impl Subcommand {
    pub fn build_command(&self) -> clap::Command<'static> {
        match self {
            Subcommand::Add => clap::Command::new(self.as_str())
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
                ),
            Subcommand::Search => clap::Command::new(self.as_str())
                .about("Searches through existing notes for the given text.")
                .aliases(&["search", "fd"])
                .arg(
                    clap::Arg::new("text")
                        .help("The text to search for.")
                        .required(true)
                        .value_name("SEARCH_TEXT"),
                )
                .arg(
                    clap::Arg::new("search-in")
                        .help("What fields to search for the given text. Defaults to all aka full text search.")
                        .short('s')
                        .long("search")
                        .alias("fields")
                        .alias("search-in")
                        .multiple_occurrences(true)
                        .multiple_values(true)
                        .takes_value(true)
                        .ignore_case(true)
                        .require_equals(false)
                        .value_delimiter(',')
                        .possible_values(&["title", "content", "tags", "all"])
                        .default_value("all"),
                ),
            Subcommand::List => clap::Command::new(self.as_str())
                .alias("ls")
                .about("List all notes"),
            Subcommand::Update => clap::Command::new(self.as_str())
                .about("update the indicated note"),
            Subcommand::Delete => clap::Command::new(self.as_str())
                .alias("rm")
                .about("delete the indicated note"),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Subcommand::Add => "add",
            Subcommand::Search => "find",
            Subcommand::List => "list",
            Subcommand::Update => "update",
            Subcommand::Delete => "delete",
        }
    }
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
        .arg(
            clap::Arg::new("text")
                .help("The text to search for.")
                .required(true)
                .value_name("SEARCH_TEXT"),
        )
        .arg(
            clap::Arg::new("search-in")
                .help("What fields to search for the given text. Defaults to all aka full text search.")
                .short('s')
                .long("search")
                .alias("fields")
                .alias("search-in")
                .multiple_occurrences(true)
                .takes_value(true)
                .ignore_case(true)
                .require_equals(false)
                .possible_values(&["title", "content", "tags", "all"])
                .default_value("all"),
        )
}

fn list_all_cmd() -> clap::Command<'static> {
    clap::Command::new("list")
        .alias("ls")
        .about("List all notes")
        .arg(
            clap::Arg::new("sort-field")
                .help("The order by which to display the notes. Defaults to recently updated.")
                .short('s')
                .long("sort")
                .alias("order")
                .alias("field")
                .default_value("updated")
                .takes_value(true)
                .required(false)
                .possible_values(&["updated", "created", "title", "content", "tags"]),
        )
        .arg(
            clap::Arg::new("sort-type")
                .help("Whether the sort order is ascending or descending. Defaults to descending.")
                .short('t')
                .long("type")
                .alias("sort-type")
                .default_value("desc")
                .takes_value(true)
                .required(false)
                .possible_values(&["asc", "desc"]),
        )
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
