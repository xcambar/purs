extern crate ansi_term;

use std::env;
use ansi_term::Colour::{Cyan, Blue, Purple};

fn main() {
    let my_path = env::current_dir().unwrap();
    let display_path = Blue.paint(my_path.to_string_lossy());

    let branch ="master";
    let display_branch = Cyan.paint(branch);

    println!("{} {}", display_path, display_branch);
    print!(" {}", Purple.paint("❯"));
}
