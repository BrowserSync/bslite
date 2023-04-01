use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
  #[arg(long)]
  pub dry_run: bool,
  #[arg(long)]
  pub from: Option<PathBuf>,
  pub paths: Vec<PathBuf>,
}

#[cfg(test)]
mod tests {
  use crate::cli::Cli;
  use clap::Parser;

  #[test]
  fn test_from_args() {
    let v = Cli::try_parse_from(["--from", ".", "a", "b", "c"]);
    let v2 = Cli::try_parse_from(["--from", "site.yaml", "a", "b", "c", "--dry-run"]);
    println!("{:?}", v);
    println!("{:?}", v2);
  }
}
