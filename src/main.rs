#![deny(rust_2018_idioms)]
use clap::{App, AppSettings};

mod precmd;
mod prompt;

fn main() {
    let matches = App::new("Purs")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(precmd::cli_arguments())
        .subcommand(prompt::cli_arguments())
        .get_matches();

    match matches.subcommand() {
        ("precmd", Some(sub_matches)) => precmd::display(sub_matches),
        ("prompt", Some(sub_matches)) => prompt::display(sub_matches),
        _ => (),
    }
}
