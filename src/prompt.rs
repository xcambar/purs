use std::env;
use ansi_term::Colour::{Red, Purple};

const SYMBOL:&str = "❯";

pub fn display() {
  let var = env::var("_LAST_RETURN_STATUS");
  let last_command = match var {
    Ok(val) => val.parse::<u8>().unwrap(),
    Err(_e) => 1
  };

  let printable_prompt = match last_command {
    0 => Purple.paint(SYMBOL),
    _ => Red.paint(SYMBOL),
  };

  print!("{}", printable_prompt);
}