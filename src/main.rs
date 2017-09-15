extern crate ansi_term;
extern crate regex;

use regex::Regex;

use std::env;
use ansi_term::Colour::{Cyan, Blue, Purple};

fn first_char(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    return match chars.len() {
        0 => String::from(""),
        _ => chars[0].to_string()
    }
}

fn fmt_current_path(cwd: &str) -> String {
    let home: Regex = match env::home_dir() {
        Some(path) => Regex::new(path.to_str().unwrap()).unwrap(),
        None => Regex::new("").unwrap(),
    };
    let friendly_path = home.replace(cwd, "~").to_string();
    let mut friendly_path_split: Vec<&str> = friendly_path.split("/").collect();
    let current_dir = friendly_path_split.pop().unwrap().to_string();
    let mut short_path: Vec<String> = friendly_path_split.iter().map(|s| first_char(s)).collect();
    short_path.push(current_dir);
    short_path.join("/")
}

fn main() {
    let my_path = env::current_dir().unwrap();
    let display_path = Blue.paint(fmt_current_path(my_path.to_str().unwrap()));

    let branch ="master";
    let display_branch = Cyan.paint(branch);

    println!("{} {}", display_path, display_branch);
    print!(" {} ", Purple.paint("❯"));
}
