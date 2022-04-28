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

mod app;

/// Runs the Command Line Interface for the notes application.
///
/// ## Errors
/// - If the underlying process errors.
pub fn run_cli(args: std::env::Args) -> crate::Result<()> {
    println!("Noted CLI. Args: {}", args.collect::<Vec<_>>().join(" "));
    // let mut app = app::create_app();
    let mut app = app::create_app();
    let input = app.get_matches();
    println!("ArgMatches: {:#?}", input);
    Ok(())
}
