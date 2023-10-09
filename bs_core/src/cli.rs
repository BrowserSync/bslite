use clap::{Error, Parser};
use std::path::PathBuf;
use std::vec::IntoIter;

#[derive(clap::Parser, Debug)]
pub struct Cli {
  #[arg(long)]
  pub dry_run: bool,
  #[arg(long)]
  pub from: Option<PathBuf>,
  pub paths: Vec<PathBuf>,
}

impl Cli {
  pub fn from_args(args: &[String]) -> Result<Cli, Error> {
    Cli::try_parse_from(args)
  }
}

#[cfg(test)]
mod tests {
  use crate::cli::Cli;
  use clap::Parser;

  #[test]
  fn test_from_args() {
    let v = Cli::try_parse_from(["bslite", "a", "b", "c"]);
    let v2 = Cli::try_parse_from(["bslite", "--from", "site.yaml", "a", "b", "c", "--dry-run"]);
    println!("{:?}", v);
    println!("{:?}", v2);
  }
}
