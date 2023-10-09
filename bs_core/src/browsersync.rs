use crate::cli::Cli;
use crate::server::BsServer;
use crate::static_route::StaticRoute;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct Browsersync {
  pub servers: BTreeMap<String, BsServer>,
  #[serde(default)]
  pub routes: Vec<StaticRoute>,
}

impl<'a> Browsersync {
  pub fn base_server(&'a self) -> &'a BsServer {
    self
      .servers
      .first_key_value()
      .map(|(_k, v)| v)
      .expect("must unwrap first")
  }
}

impl Browsersync {
  pub fn from_cli(value: Cli, cwd: PathBuf) -> Self {
    let mut bs: Self = if let Some(from) = value.from {
      let next = cwd.join(from);
      let str = fs::read_to_string(next).expect("tried to read");
      serde_yaml::from_str(str.as_str()).expect("format of yaml")
    } else {
      let mut bs = Browsersync::default();
      let server = BsServer {
        bind: Default::default(),
        routes: vec![],
      };
      bs.servers.insert("default".into(), server);
      bs
    };

    bs.routes = value
      .paths
      .iter()
      .map(|dir| StaticRoute::dir("/", dir))
      .collect::<Vec<_>>();

    bs
  }
}

#[test]
fn test_base() {
  let bytes = include_str!("../../examples/one-file/site.yaml");
  let b: Browsersync = serde_yaml::from_str(bytes).expect("test");
  use actix_router::ResourceDef;
  let resource = ResourceDef::new(["/profile", "/user/{id}"]);
  assert!(resource.is_match("/profile"));
  assert!(resource.is_match("/user/123"));
  assert!(!resource.is_match("user/123"));
  assert!(!resource.is_match("/foo"));
}

#[test]
fn test_server() {}
