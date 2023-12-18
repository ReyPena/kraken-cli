mod modules;

extern crate clap;
extern crate term_size;

use clap::Command;
use term_size::dimensions;
use modules::interactive_menu::init_interactive_menu;

fn main() {
    let app = Command::new("Krampus")
        .version("1.0.0")
        .author("Rey")
        .about("Utility extendable cli");

    let matches = app.subcommand(
        Command::new("interactive")
            .visible_alias("i")
            .about("Run in interactive mode")
    )
        // .arg(Arg::new("interactive")
        // .long("interactive")
        // .short('i')
        // .help("Run in interactive mode"))
        .get_matches();

    let term_dimensions: (usize, usize) = match dimensions() {
        Some((w, h)) => (w, h),
        None => (80, 20),
    };

    match matches.subcommand() {
        Some(("interactive", _sub_matches)) => init_interactive_menu(term_dimensions),
        _ => {
            // No subcommand provided, display help or handle accordingly
            println!("Please provide a subcommand. Use '--help' for usage information.");
        }
    }
}