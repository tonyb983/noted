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
    println!("ArgMatches: {:?}", input);
    Ok(())
}
