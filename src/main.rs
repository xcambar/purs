extern crate ansi_term;
extern crate clap;
extern crate git2;
extern crate regex;

use clap::{App, AppSettings};

mod prompt;
mod precmd;

fn main() {
    let matches = App::new("Pure prompt")
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