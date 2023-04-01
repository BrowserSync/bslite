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
      resolve: RouteResolver::FilePath(FilePath { file: file.into() }),
    }
  }
}

impl StaticRoute {
  pub fn dir(path: impl Into<String>, dir: impl Into<PathBuf>) -> Self {
    Self {
      path: path.into(),
      resolve: RouteResolver::DirPath(DirPath { dir: dir.into() }),
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

#[derive(Debug, serde::Deserialize, PartialEq, Clone)]
pub struct RawString {
  raw: String,
}

#[derive(Debug, serde::Deserialize, PartialEq, Clone)]
pub struct FilePath {
  file: PathBuf,
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
        file: PathBuf::from("index.js")
      })
    )
  }

  #[test]
  fn test_static_route_for_content() {
    let expected = StaticRoute {
      path: String::from("/"),
      resolve: RouteResolver::RawString(RawString {
        raw: String::from("haha"),
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
