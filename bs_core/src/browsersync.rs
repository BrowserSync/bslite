use crate::cli::Cli;
use crate::server::Server;
use crate::static_route::StaticRoute;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct Browsersync {
  pub servers: BTreeMap<String, Server>,
  #[serde(default)]
  pub routes: Vec<StaticRoute>,
}

impl<'a> Browsersync {
  pub fn base_server(&'a self) -> &'a Server {
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
      let server = Server {
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
  dbg!(b);
}
