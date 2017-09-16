extern crate ansi_term;
extern crate clap;
extern crate git2;
extern crate regex;

use clap::{App, Arg};

mod prompt;
mod precmd;

fn main() {
    let matches = App::new("Pure prompt")
        .arg(Arg::with_name("part")
            .index(1)
            .possible_values(&["precmd", "prompt"])
            .required(true))
        .get_matches();
    
    match matches.value_of("part").unwrap() {
        "precmd" => precmd::display(),
        "prompt" => prompt::display(),
        _ => (),
    }
}
