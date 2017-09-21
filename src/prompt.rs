use clap::{ArgMatches, App, SubCommand, Arg};

const INSERT_SYMBOL:&str = "❯";
const COMMAND_SYMBOL:&str = "⬢";

pub fn display(sub_matches: &ArgMatches) {
  let last_return_code = sub_matches.value_of("last_return_code").unwrap();
  let keymap = sub_matches.value_of("keymap").unwrap();
      
  
  let symbol = match keymap {
    "vicmd" => COMMAND_SYMBOL,
    _ => INSERT_SYMBOL,
  };

  let shell_color = match (symbol, last_return_code) {
    (COMMAND_SYMBOL, _) => 3,
    (_, "0") => 5,
    _ => 9,
  };

  print!("%F{{{}}}{}%f ", shell_color, symbol);
}

pub fn cli_arguments<'a>() -> App<'a, 'a> {
  return SubCommand::with_name("prompt")
    .arg(
      Arg::with_name("last_return_code")
        .short("r")
        .takes_value(true)
    )
    .arg(
      Arg::with_name("keymap")
        .short("k")
        .takes_value(true)
    );
}