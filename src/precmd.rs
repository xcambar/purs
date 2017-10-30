use std::env;
use ansi_term::Colour::{Cyan, Blue, Red};
use ansi_term::ANSIStrings;
use git2::{self, Repository, StatusOptions};
use regex::Regex;
use clap::{ArgMatches, App, SubCommand};
use tico::tico;

fn shorten_path(cwd: &str) -> String {
  let friendly_path = match env::home_dir() {
    Some(path) => Regex::new(path.to_str().unwrap()).unwrap().replace(cwd, "~"),
    _ => return String::from("")
  };

  tico(&friendly_path)
}

fn repo_status(r: &Repository) -> Option<String> {
  let mut opts = StatusOptions::new();
  opts.include_untracked(true);
  let head = match r.head() {
    Ok(head) => head,
    Err(_) => return None
  };

  let shorthand = Cyan.paint(head.shorthand().unwrap().to_string());
  let statuses = match r.statuses(Some(&mut opts)) {
    Ok(statuses) => statuses,
    Err(_) => return None
  };

  let mut is_dirty = false;

  for entry in statuses.iter().filter(
    |e| e.status() != git2::STATUS_CURRENT,
  ) {
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

    if is_dirty { break }
  }
  let mut out = vec![shorthand];
  if is_dirty {
    out.push(Red.bold().paint("*"));
  }

  Some(ANSIStrings(&out).to_string())
}

pub fn display(_sub: &ArgMatches) {
  let my_path = env::current_dir().unwrap();
  let display_path = Blue.paint(shorten_path(my_path.to_str().unwrap()));

  let branch = match Repository::discover(my_path) {
    Ok(repo) => repo_status(&repo),
    Err(_e) => None,
  };
  let display_branch = Cyan.paint(branch.unwrap_or_default());

  println!("\n{} {}", display_path, display_branch);
}

pub fn cli_arguments<'a>() -> App<'a, 'a> {
  SubCommand::with_name("precmd")
}
