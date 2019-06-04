use clap::{App, Arg, ArgMatches, SubCommand};

const INSERT_SYMBOL: &str = "❯";
const COMMAND_SYMBOL: &str = "⬢";
const COMMAND_KEYMAP: &str = "vicmd";
const NO_ERROR: &str = "0";

pub fn display(sub_matches: &ArgMatches<'_>) {
    let last_return_code = sub_matches.value_of("last_return_code").unwrap_or("0");
    let keymap = sub_matches.value_of("keymap").unwrap_or("US");
    let venv_name = sub_matches.value_of("venv").unwrap_or("");

    let symbol = match keymap {
        COMMAND_KEYMAP => COMMAND_SYMBOL,
        _ => INSERT_SYMBOL,
    };

    let shell_color = match (symbol, last_return_code) {
        (COMMAND_SYMBOL, _) => 3,
        (_, NO_ERROR) => 5,
        _ => 9,
    };

    let venv = match venv_name.len() {
        0 => String::from(""),
        _ => format!("%F{{11}}|{}|%f ", venv_name),
    };

    print!("{}%F{{{}}}{}%f ", venv, shell_color, symbol);
}

pub fn cli_arguments<'a>() -> App<'a, 'a> {
    SubCommand::with_name("prompt")
        .arg(
            Arg::with_name("last_return_code")
                .short("r")
                .takes_value(true),
        )
        .arg(Arg::with_name("keymap").short("k").takes_value(true))
        .arg(Arg::with_name("venv").long("venv").takes_value(true))
}
