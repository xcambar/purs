const INSERT_SYMBOL:&str = "❯";
const COMMAND_SYMBOL:&str = "⬢";

pub fn display(return_code: &str, keymap: &str) {
  let symbol = match keymap {
    "vicmd" => COMMAND_SYMBOL,
    _ => INSERT_SYMBOL,
  };

  let shell_color = match (symbol, return_code) {
    (COMMAND_SYMBOL, _) => 3,
    (_, "0") => 5,
    _ => 9,
  };

  print!("%F{{{}}}{}%f ", shell_color, symbol);
}