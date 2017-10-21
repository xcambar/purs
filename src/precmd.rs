use std::env;
use ansi_term::Colour::{Cyan, Blue, Red, Green, Purple};
use ansi_term::{ANSIStrings, ANSIGenericString};
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
  let mut out = vec![];

  if let Some(name) = get_head_shortname(r) {
    out.push(Cyan.paint(name));
  }

  if let Some((ahead, behind)) = get_ahead_behind(r) {
    if ahead > 0 {
      out.push(Cyan.paint(format!("↑{}", ahead)));
    }
    if behind > 0 {
      out.push(Cyan.paint(format!("↓{}", behind)));
    }
  }

  if let Some((index_change, wt_change, conflicted, untracked)) = count_files_statuses(r) {
    if index_change == 0 && wt_change == 0 && conflicted == 0 && untracked == 0 {
      out.push(Green.paint("✔"));
    } else {
      if index_change > 0 {
        out.push(Green.paint(format!("♦{}", index_change)));
      }
      if conflicted > 0 {
        out.push(Red.paint(format!("✖{}", conflicted)));
      }
      if wt_change > 0 {
        out.push(ANSIGenericString::from(format!("✚{}", wt_change)));
      }
      if untracked > 0 {
        out.push(ANSIGenericString::from("…"));
      }
    }
  }

  if let Some(action) = get_action(r) {
    out.push(Purple.paint(format!(" {}", action)));
  }

  Some(ANSIStrings(&out).to_string())
}

fn get_ahead_behind(r: &Repository) -> Option<(usize, usize)> {
  let head = try_opt!(r.head().ok());
  if !head.is_branch() {
    return None
  }

  let head_name = try_opt!(head.shorthand());
  let head_branch = try_opt!(r.find_branch(head_name, git2::BranchType::Local).ok());
  let upstream = try_opt!(head_branch.upstream().ok());
  let head_oid = try_opt!(head.target());
  let upstream_oid = try_opt!(upstream.get().target());

  r.graph_ahead_behind(head_oid, upstream_oid).ok()
}

fn get_head_shortname(r: &Repository) -> Option<String> {
  let head = try_opt!(r.head().ok());
  if let Some(shorthand) = head.shorthand() {
    if shorthand != "HEAD" {
      return Some(shorthand.to_string())
    }
  }

  let object = try_opt!(head.peel(git2::ObjectType::Commit).ok());
  let short_id = try_opt!(object.short_id().ok());

  Some(format!(":{}", short_id.iter().map(|ch| *ch as char).collect::<String>()))
}

fn count_files_statuses(r: &Repository) -> Option<(usize, usize, usize, usize)> {
  let mut opts = StatusOptions::new();
  opts.include_untracked(true);

  fn count_files(statuses: &git2::Statuses, status: git2::Status) -> usize {
    statuses.iter().filter(|entry| entry.status().intersects(status)).count()
  }

  let statuses = try_opt!(r.statuses(Some(&mut opts)).ok());

  Some((
    count_files(
      &statuses,
      git2::STATUS_INDEX_NEW |
      git2::STATUS_INDEX_MODIFIED |
      git2::STATUS_INDEX_DELETED |
      git2::STATUS_INDEX_RENAMED |
      git2::STATUS_INDEX_TYPECHANGE
    ),
    count_files(
      &statuses,
      git2::STATUS_WT_MODIFIED |
      git2::STATUS_WT_DELETED |
      git2::STATUS_WT_TYPECHANGE |
      git2::STATUS_WT_RENAMED
    ),
    count_files(&statuses, git2::STATUS_CONFLICTED),
    count_files(&statuses, git2::STATUS_WT_NEW),
  ))
}

// Based on https://github.com/zsh-users/zsh/blob/ed4e37e45c2f5761981cdc6027a5d6abc753176a/Functions/VCS_Info/Backends/VCS_INFO_get_data_git#L11
fn get_action(r: &Repository) -> Option<String> {
  let gitdir = r.path();

  for tmp in &[
    gitdir.join("rebase-apply"),
    gitdir.join("rebase"),
    gitdir.join("..").join(".dotest"),
  ] {
    if tmp.join("rebasing").exists() {
      return Some("rebase".to_string());
    }
    if tmp.join("applying").exists() {
      return Some("am".to_string());
    }
    if tmp.exists() {
      return Some("am/rebase".to_string());
    }
  }

  for tmp in &[
    gitdir.join("rebase-merge").join("interactive"),
    gitdir.join(".dotest-merge").join("interactive"),
  ] {
    if tmp.exists() {
      return Some("rebase-i".to_string());
    }
  }

  for tmp in &[
    gitdir.join("rebase-merge"),
    gitdir.join(".dotest-merge"),
  ] {
    if tmp.exists() {
      return Some("rebase-m".to_string());
    }
  }

  if gitdir.join("MERGE_HEAD").exists() {
    return Some("merge".to_string());
  }

  if gitdir.join("BISECT_LOG").exists() {
    return Some("bisect".to_string());
  }

  if gitdir.join("CHERRY_PICK_HEAD").exists() {
    if gitdir.join("sequencer").exists() {
      return Some("cherry-seq".to_string());
    } else {
      return Some("cherry".to_string());
    }
  }

  if gitdir.join("sequencer").exists() {
    return Some("cherry-or-revert".to_string());
  }

  None
}

pub fn display(_sub: &ArgMatches) {
  let my_path = env::current_dir().unwrap();
  let display_path = Blue.paint(shorten_path(my_path.to_str().unwrap()));

  let branch = match Repository::discover(my_path) {
    Ok(repo) => repo_status(&repo),
    Err(_e) => None,
  };
  let display_branch = Cyan.paint(branch.unwrap_or_default());

  println!("");
  println!("{} {}", display_path, display_branch);
}

pub fn cli_arguments<'a>() -> App<'a, 'a> {
  SubCommand::with_name("precmd")
}
