use clap::{App, Arg, ArgMatches, SubCommand};
use std::env;
use nix::unistd;

const INSERT_SYMBOL: &str = "❯";
const COMMAND_SYMBOL: &str = "⬢";
const COMMAND_KEYMAP: &str = "vicmd";
const NO_ERROR: &str = "0";
const SSH_SESSION_ENV: &str = "SSH_TTY";


fn get_username() -> String {
    match env::var("USER") {
        Ok(name) => name,
        Err(_) => "".to_string(),
    }
}

fn get_hostname() -> String {
    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed getting hostname");
    hostname_cstr.to_str().expect("Hostname wasn't valid UTF-8").to_string()
}

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

    let ssh_user_host = match env::var(SSH_SESSION_ENV) {
        Ok(_) => format!("{}@{} ", get_username(), get_hostname()),
        Err(_) => "".to_string(),
    };

    print!("{}{}%F{{{}}}{}%f ", ssh_user_host, venv, shell_color, symbol);
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
