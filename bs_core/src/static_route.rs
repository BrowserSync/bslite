use std::collections::BTreeMap;
use std::fmt::{write, Display, Formatter};
use std::path::PathBuf;

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub struct StaticRoute {
  pub path: String,
  #[serde(flatten)]
  pub resolve: RouteResolver,
}

impl StaticRoute {
  pub fn file(path: impl Into<String>, file: impl Into<PathBuf>) -> Self {
    Self {
      path: path.into(),
      resolve: RouteResolver::FilePath(FilePath {
        file: file.into(),
        headers: Default::default(),
      }),
    }
  }
  pub fn dir(path: impl Into<String>, dir: impl Into<PathBuf>) -> Self {
    Self {
      path: path.into(),
      resolve: RouteResolver::DirPath(DirPath { dir: dir.into() }),
    }
  }
  pub fn raw(path: impl Into<String>, raw: impl Into<String>) -> Self {
    Self {
      path: path.into(),
      resolve: RouteResolver::RawString(RawString {
        raw: raw.into(),
        headers: Default::default(),
      }),
    }
  }
}

#[derive(Debug, serde::Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum RouteResolver {
  RawString(RawString),
  FilePath(FilePath),
  DirPath(DirPath),
}

impl Display for RouteResolver {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let string = match self {
      RouteResolver::RawString(RawString { raw, .. }) => {
        format!("RouteResolver::RawString {}", raw.bytes().len())
      }
      RouteResolver::FilePath(FilePath { file, headers }) => {
        format!("RouteResolver::FilePath {} bytes", file.display())
      }
      RouteResolver::DirPath(DirPath { dir }) => {
        format!("RouteResolver::DirPath {} bytes", dir.display())
      }
    };
    write!(f, "{}", string)
  }
}

#[derive(Debug, serde::Deserialize, PartialEq, Clone)]
pub struct RawString {
  pub raw: String,
  #[serde(default)]
  pub headers: BTreeMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq, Clone)]
pub struct FilePath {
  pub file: PathBuf,
  pub headers: BTreeMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq, Clone)]
pub struct DirPath {
  pub dir: PathBuf,
}

#[cfg(test)]
mod tests {
  use crate::static_route::{FilePath, RawString, RouteResolver, StaticRoute};
  use std::path::PathBuf;

  #[test]
  fn test_static_route() {
    let r = StaticRoute::file("/", "index.js");
    assert_eq!(r.path, String::from("/"));
    assert_eq!(
      r.resolve,
      RouteResolver::FilePath(FilePath {
        file: PathBuf::from("index.js"),
        headers: Default::default(),
      })
    )
  }

  #[test]
  fn test_static_route_for_content() {
    let expected = StaticRoute {
      path: String::from("/"),
      resolve: RouteResolver::RawString(RawString {
        raw: String::from("haha"),
        headers: Default::default(),
      }),
    };
    let json = r#"
            {
               "path": "/",
               "raw": "haha"
            }
        "#;
    let r: StaticRoute = serde_json::from_str(json).expect("test");
    assert_eq!(r, expected)
  }
  #[test]
  fn test_static_route_for_file() {
    let expected = StaticRoute::file("/anything", "index.js");
    let json = r#"
        {
           "path": "/anything",
           "file": "index.js"
        }
        "#;
    let r: StaticRoute = serde_json::from_str(json).expect("test");
    assert_eq!(r, expected)
  }
  #[test]
  fn test_static_route_for_dir() {
    let expected = StaticRoute::file("/anything", "index.js");
    let json = r#"
        {
           "path": "/",
           "dir": "public"
        }
        "#;
    let r: StaticRoute = serde_json::from_str(json).expect("test");
    assert_eq!(r, expected)
  }
}
