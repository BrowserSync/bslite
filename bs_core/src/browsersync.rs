
use crate::cli::Cli;
use crate::server::Server;
use crate::static_route::StaticRoute;
use std::collections::BTreeMap;


#[derive(Debug, Clone, Default)]
pub struct Browsersync {
  pub servers: BTreeMap<String, Server>,
}

impl<'a> Browsersync {
  pub fn base_server(&'a self) -> &'a Server {
    self.servers.get("default").expect("unreachable")
  }
}

impl From<Cli> for Browsersync {
  fn from(value: Cli) -> Self {
    let mut server = Server {
      bind: Default::default(),
      routes: vec![],
    };

    for path in &value.paths {
      let as_route = StaticRoute::dir("/", path);
      server.routes.push(as_route);
    }

    let mut bs = Browsersync::default();
    bs.servers.insert("default".into(), server);
    bs
  }
}
