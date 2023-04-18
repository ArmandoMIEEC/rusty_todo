//Printing Info:
//- eprintln!("{error}"), for flushing errors immediately to stderr;
//- writeln!(stdout_buff, info), for placing info into the stdout buffer (auto-flushes when it reaches 8Kb);
//- stdout_buff.flush(), for flushing stdout buffer immediately.
//Why: println!() flushes every time, making writeln!() more performant when printing immediately is not needed.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use todo::commands;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "Rusty To-Do is a minimalistic CLI To-Do tool made with Rust")]
#[command(long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ///Creates a new list
    Create { list_name: String },
}

fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Soft debug is on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(Commands::Create { list_name }) => commands::create(list_name.clone()).unwrap(),
        None => {}
    }
}
