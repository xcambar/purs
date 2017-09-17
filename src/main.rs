extern crate ansi_term;
extern crate clap;
extern crate git2;
extern crate regex;

use clap::{App, AppSettings, Arg, SubCommand};

mod prompt;
mod precmd;

fn main() {
    let matches = App::new("Pure prompt")
      .setting(AppSettings::SubcommandRequired)
      .subcommand(SubCommand::with_name("precmd"))
      .subcommand(
        SubCommand::with_name("prompt")
          .arg(
            Arg::with_name("last_return_code")
              .short("r")
              .takes_value(true)
          )
          .arg(
            Arg::with_name("keymap")
              .short("k")
              .takes_value(true)
          ),
      )
      .get_matches();

    match matches.subcommand() {
      ("precmd", _) => precmd::display(),
      ("prompt", Some(sub_matches)) => {
        let last_return_code = sub_matches.value_of("last_return_code").unwrap();
        let keymap = sub_matches.value_of("keymap").unwrap();
        prompt::display(last_return_code, keymap);
      }
      _ => (),
    }
}