// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use super::NoteShape;

use tinyid::TinyId;

fn validate_add_note_json(s: &str) -> Result<(), String> {
    crate::flame_guard!("bins", "cli", "app", "validate_add_note_json");
    let converted: NoteShape = serde_json::from_str(s).map_err(|e| e.to_string())?;
    if converted.title.is_empty() && converted.content.is_empty() {
        return Err(
            "A note must have either a title or content, all other fields are optional."
                .to_string(),
        );
    }
    Ok(())
}

/// Arg-Name: `count`
fn create_count_arg() -> clap::Arg<'static> {
    crate::flame_guard!("bins", "cli", "app", "create_count_arg");
    clap::Arg::new("count")
        .help("How many results should be returned (default is all)")
        .long("count")
        .short('c')
        .alias("limit")
        .short_alias('l')
        .takes_value(true)
        .required(false)
}

/// Arg-Name: `order`
fn create_order_arg() -> clap::Arg<'static> {
    crate::flame_guard!("bins", "cli", "app", "create_order_arg");
    clap::Arg::new("order")
        .help("How to order, or sort, the results.")
        .long("order")
        .short('o')
        .takes_value(true)
        .required(false)
        .possible_values(&["title", "content", "tags", "created", "modified", "updated"])
        .default_value("modified")
}

/// Arg-Name: `reverse`
fn create_reverse_arg() -> clap::Arg<'static> {
    crate::flame_guard!("bins", "cli", "app", "create_reverse_arg");
    clap::Arg::new("reverse")
        .help("Reverse the order of the results (descending instead of ascending).")
        .long("reverse")
        .visible_alias("descending")
        .visible_alias("desc")
        .short('r')
        .takes_value(false)
        .required(false)
}

fn add_note_cmd() -> clap::Command<'static> {
    crate::flame_guard!("bins", "cli", "app", "add_note_cmd");
    clap::Command::new("add")
        .about("Add a new note")
        .arg(
            clap::Arg::new("title")
                .long_help("The main title of the note to be created. This is what will most often be displayed when reviewing multiple notes.")
                .help("The title, or heading, of the note.")
                .short('t')
                .long("title")
                .alias("heading")
                .alias("header")
                .takes_value(true)
                .required_unless_present_any(&["content", "json"]),
        )
        .arg(
            clap::Arg::new("content")
                .long_help("The main content, or body, of the note. Markdown is supported for note content.")
                .help("The content of the note to be created.")
                .long("content")
                .short('c')
                .alias("body")
                .short_alias('b')
                .takes_value(true)
                .required(false),
        )
        .arg(
            clap::Arg::new("tags")
                .long_help("A comma-separated list of strings which will be applied to the new note as tags. Tags can be used to better organize notes through cataloging and grouping, and allows for easier searching.")
                .help("Tags to add to the note. Separate multiple tags with commas.")
                .short('T')
                .long("tags")
                .takes_value(true)
                .required(false)
                .use_value_delimiter(true)
                .multiple_values(true)
                .require_value_delimiter(true)
                .require_equals(false),
        )
        .arg(
            clap::Arg::new("json")
                .help("Submit the note as a Json object (in string form).")
                .long_help("Submit the note as a JSON object. Expected format is { \"title\": \"...\", \"content\": \"...\", \"tags\": [\"...\", \"...\"] }. All fields are technically optional, but at least one of title or content is required.")
                .short('j')
                .long("json")
                .takes_value(true)
                .required(false)
                .exclusive(true)
                .validator(validate_add_note_json),
        )
}

fn update_note_cmd() -> clap::Command<'static> {
    crate::flame_guard!("bins", "cli", "app", "update_note_cmd");
    todo!("TODO: This")
}

fn search_notes_cmd() -> clap::Command<'static> {
    crate::flame_guard!("bins", "cli", "app", "search_notes_cmd");
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
                .help("What fields to search for the given text.")
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
        .arg(create_count_arg())
        .arg(create_order_arg())
        .arg(create_reverse_arg())
}

fn list_all_cmd() -> clap::Command<'static> {
    crate::flame_guard!("bins", "cli", "app", "list_all_cmd");
    clap::Command::new("list")
        .alias("ls")
        .about("List all notes")
        .arg(create_count_arg())
        .arg(create_order_arg())
        .arg(create_reverse_arg())
}

fn delete_note_cmd() -> clap::Command<'static> {
    crate::flame_guard!("bins", "cli", "app", "delete_note_cmd");
    clap::Command::new("delete")
        .alias("del")
        .alias("remove")
        .alias("rm")
        .about("delete the indicated note")
        .arg(
            clap::Arg::new("id")
                .help("The id of the note to delete.")
                .forbid_empty_values(true)
                .required(true)
                .value_name("NOTE_ID")
                .validator(|input| TinyId::from_str(input).map_err(|err| err.to_string())),
        )
}

/// Arg-Name: `interactive`
fn create_interactive_arg() -> clap::Arg<'static> {
    crate::flame_guard!("bins", "cli", "app", "create_interactive_arg");
    clap::Arg::new("interactive")
        .help("Run the command in interactive mode.")
        .long_help("Attempts to perform the given subcommand interactively (if possible). Interactive commands include: add, update, delete.")
        .long("interactive")
        .short('i')
        .takes_value(false)
        .required(false)
        .multiple_occurrences(false)
        .global(true)
}

/// Arg-Name: `verbose`
fn create_verbosity_arg() -> clap::Arg<'static> {
    crate::flame_guard!("bins", "cli", "app", "create_verbosity_arg");
    clap::Arg::new("verbose")
        .long_help("How verbose the output should be. Can be used multiple times to increase verbosit, i.e. '-v -v' or '-vvv'.")
        .help("Run the command in verbose mode.")
        .long("verbose")
        .short('v')
        .takes_value(false)
        .required(false)
        .multiple_occurrences(true)
        .global(true)
}

pub fn create_app() -> clap::Command<'static> {
    crate::flame_guard!("bins", "cli", "app", "create_app");
    clap::Command::new(clap::crate_name!())
        .bin_name(clap::crate_name!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .propagate_version(true)
        // TODO: This will probably have to change once the TUI is implemented (call to just the binary starts the TUI, call with args runs as CLI, call with '-i' runs as iCLI)
        .subcommand_required(true)
        .subcommand_help_heading("OPERATIONS")
        .arg_required_else_help(true)
        .help_expected(true)
        .infer_subcommands(true)
        .infer_long_args(true)
        .arg(create_verbosity_arg())
        .arg(create_interactive_arg())
        .subcommand(add_note_cmd().display_order(1))
        .subcommand(list_all_cmd().display_order(2))
        .subcommand(search_notes_cmd().display_order(3))
        .subcommand(delete_note_cmd().display_order(4))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    #[no_coverage]
    fn clap_cmd_debug_assert() {
        add_note_cmd().debug_assert();
        list_all_cmd().debug_assert();
        search_notes_cmd().debug_assert();
        delete_note_cmd().debug_assert();
        create_app().debug_assert();
    }
}
