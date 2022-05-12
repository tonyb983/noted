//! `cli` Module
//!
//! This will be a typical cli application that communicates through the use of command-line arguments.
//! Typical process will be:
//! - Use `clap` to parse args
//! - Build a DTO from the args
//! - Open a connection to the database
//!   - This is not exactly correct, there should definitely be something between the database and the cli. Repository pattern? Service pattern? Something.
//! - Process the command and return the results
//! - Format and pretty-print the results

use tinyid::TinyId;

use crate::types::{
    api::{Count, Filter, NoteFilter, Ordering},
    CreateNote, DeleteNote, NoteDto,
};

mod app;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct NoteShape {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CliVerbosity {
    Quiet,
    Normal,
    Verbose,
    VeryVerbose,
}

impl std::fmt::Display for CliVerbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::flame_guard!("bins", "cli", "CliVerbosity", "fmt");
        match self {
            CliVerbosity::Quiet => write!(f, "quiet"),
            CliVerbosity::Normal => write!(f, "normal"),
            CliVerbosity::Verbose => write!(f, "verbose"),
            CliVerbosity::VeryVerbose => write!(f, "very verbose"),
        }
    }
}

impl From<u64> for CliVerbosity {
    fn from(value: u64) -> Self {
        crate::flame_guard!("bins", "cli", "CliVerbosity", "CliVerbosity::from(usize)");
        match value {
            0 => CliVerbosity::Quiet,
            1 => CliVerbosity::Normal,
            2 => CliVerbosity::Verbose,
            _ => CliVerbosity::VeryVerbose,
        }
    }
}

/// Runs the Command Line Interface for the notes application.
///
/// ## Errors
/// - If the underlying process errors.
#[allow(clippy::too_many_lines, reason = "WIP")]
pub fn run_cli(args: std::env::Args) -> crate::Result {
    use crate::types::api::StringSearch;
    crate::flame_guard!("bins", "cli", "run_cli");

    println!("Noted CLI. Args: {}", args.collect::<Vec<_>>().join(" "));
    // let mut app = app::create_app();
    let mut app = app::create_app();
    let input = app.get_matches();
    println!("ArgMatches: {:#?}", input);

    let interactive = input.is_present("interactive");
    let verbosity: CliVerbosity = input.occurrences_of("verbose").into();
    match input.subcommand() {
        Some(("add", add_args)) => {
            let dto: CreateNote = if add_args.is_present("json") {
                let json_string = add_args
                    .value_of("json")
                    .expect("json arg has already been checked to exist");
                let NoteShape {
                    title,
                    content,
                    tags,
                } = serde_json::from_str(json_string)
                    .expect("json arg has already been validated to be valid");

                (title, content, tags).into()
            } else {
                let title = add_args
                    .value_of("title")
                    .map(ToString::to_string)
                    .unwrap_or_default();
                let content = add_args
                    .value_of("content")
                    .map(ToString::to_string)
                    .unwrap_or_default();
                let tags = add_args
                    .values_of("tags")
                    .map(|vals| vals.map(ToString::to_string).collect::<Vec<_>>())
                    .unwrap_or_default();

                (title, content, tags).into()
            };

            println!(
                "Running `add` command ({}interactively) with verbosity level of `{}`",
                if interactive { "" } else { "not " },
                verbosity
            );
            println!("CreateNote DTO: {:#?}", dto);
            // TODO: apply CreateNote dto to database
        }
        Some(("list", list_args)) => {
            println!(
                "Running `list` command ({}interactively) with verbosity level of `{}`",
                if interactive { "" } else { "not " },
                verbosity
            );
            let (order, count) = parse_order_count(list_args);
            let filter = Filter::empty();

            // TODO: Get notes from database and apply filter, order, and count
        }
        Some(("find", find_args)) => {
            println!(
                "Running `find` command ({}interactively) with verbosity level of `{}`",
                if interactive { "" } else { "not " },
                verbosity
            );
            let needle = find_args
                .value_of("text")
                .expect("text is required but could not be obtained");
            let (order, count) = parse_order_count(find_args);
            let fields: Vec<&str> = find_args
                .values_of("search-in")
                .map(Iterator::collect)
                .unwrap_or_default();

            let is_fts = fields.is_empty() || fields.contains(&"all");
            let filter = if is_fts {
                Filter::empty()
            } else {
                let mut f = Filter::empty();
                for field in fields {
                    match field {
                        "title" => f.add_filter(NoteFilter::title(StringSearch::contains(
                            needle.to_string(),
                            false,
                        ))),
                        "content" => f.add_filter(NoteFilter::content(StringSearch::contains(
                            needle.to_string(),
                            false,
                        ))),
                        "tags" => f.add_filter(NoteFilter::tag(StringSearch::contains(
                            needle.to_string(),
                            false,
                        ))),
                        _ => {}
                    }
                }
                f
            };

            // TODO: Query database using filter, order, and count
        }
        Some(("delete", delete_args)) => {
            println!(
                "Running `delete` command ({}interactively) with verbosity level of `{}`",
                if interactive { "" } else { "not " },
                verbosity
            );
            let id = delete_args.value_of_t_or_exit::<TinyId>("id");
            let dto: DeleteNote = id.into();
            println!("DeleteNote DTO: {0:?}\nID: {1} ({1:?}", dto, id);
            // TODO: apply DeleteNote dto to database
        }
        _ => unreachable!(),
    }

    // TODO: Convert ArgMatches to DTO

    // TODO: Spawn note / repository service to handle DTO request

    // TODO: Pretty print the results

    crate::flame_dump!(html);

    Ok(())
}

fn parse_order_count(args: &clap::ArgMatches) -> (Ordering, Count) {
    use crate::types::api::OrderBy;
    crate::flame_guard!("bins", "cli", "parse_order_count");
    let order_str = args.value_of("order").expect("order is always present");
    let mut ordering = Ordering::ascending(match order_str {
        "title" => OrderBy::Title,
        "content" => OrderBy::Content,
        "tags" => OrderBy::Tags,
        "created" | "create" => OrderBy::Created,
        _ => OrderBy::Updated,
    });
    if args.is_present("reverse") {
        ordering.reverse();
    }

    let count_num: usize = args.value_of_t("count").unwrap_or_default();

    (ordering, count_num.into())
}
