extern crate ansi_term;
extern crate git2;
extern crate regex;

use std::env;
use ansi_term::Colour::{Cyan, Blue, Purple, Red};
use ansi_term::ANSIStrings;
use git2::{Repository, StatusOptions};
use regex::Regex;

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

fn repo_status(r: Repository) -> String {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let head = r.head().unwrap();
    let shorthand = Cyan.paint(head.shorthand().unwrap().to_string());
    let statuses = r.statuses(Some(&mut opts)).unwrap();

    let mut is_dirty = false;

    for entry in statuses.iter().filter(|e| e.status() != git2::STATUS_CURRENT) {
        is_dirty = match entry.status() {
            s if s.contains(git2::STATUS_INDEX_NEW) => true,
            s if s.contains(git2::STATUS_INDEX_MODIFIED) => true,
            s if s.contains(git2::STATUS_INDEX_DELETED) => true,
            s if s.contains(git2::STATUS_INDEX_RENAMED) => true,
            s if s.contains(git2::STATUS_INDEX_TYPECHANGE) => true,
            s if s.contains(git2::STATUS_WT_NEW) => true,
            s if s.contains(git2::STATUS_WT_MODIFIED) => true,
            s if s.contains(git2::STATUS_WT_DELETED) => true,
            s if s.contains(git2::STATUS_WT_RENAMED) => true,
            s if s.contains(git2::STATUS_WT_TYPECHANGE) => true,
            _ => false,
        };
        match is_dirty {
            true => break,
            false => continue,
        }
    }
    if is_dirty == true {
        return ANSIStrings(&[shorthand, Red.bold().paint("*")]).to_string();
    } else {
        return ANSIStrings(&[shorthand]).to_string();
    }
}

fn main() {
    let my_path = env::current_dir().unwrap();
    let display_path = Blue.paint(fmt_current_path(my_path.to_str().unwrap()));

    let branch = match Repository::open(my_path) {
        Ok(repo) => repo_status(repo),
        Err(e) => String::from(""),
    };
    let display_branch = Cyan.paint(branch);

    println!("");
    println!("{} {}", display_path, display_branch);
    print!(" {} ", Purple.paint("❯"));
}
