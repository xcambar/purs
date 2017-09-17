use std::env;

const SYMBOL:&str = "❯";

pub fn display() {
  let var = env::var("_LAST_RETURN_STATUS");
  let last_command = match var {
    Ok(val) => val.parse::<u8>().unwrap(),
    Err(_e) => 1
  };

  let shell_color = match last_command {
    0 => 5,
    _ => 9,
  };

  print!("%F{{{}}}{}%f", shell_color, SYMBOL);
}