use std::fmt::{Display, Formatter};

use std::net::SocketAddr;

#[derive(Debug)]
pub struct BindAddress {
  pub port: u16,
  pub ip: String,
}

impl BindAddress {
  pub fn from_socket_addr(sa: impl Into<SocketAddr>) -> Self {
    let add = sa.into();
    Self {
      port: add.port(),
      ip: add.ip().to_string(),
    }
  }
}

impl Display for BindAddress {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "http://{}:{}", self.ip, self.port)
  }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum BindHostOptions {
  LocalHost,
  AllInterfaces,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct BindOptions {
  pub port: Option<u16>,
  pub host: Option<BindHostOptions>,
}

impl BindOptions {
  pub fn ip(&self) -> String {
    match self.host {
      None | Some(BindHostOptions::LocalHost) => "127.0.0.1",
      Some(BindHostOptions::AllInterfaces) => "0.0.0.0",
    }
    .to_string()
  }
}

impl Default for BindOptions {
  fn default() -> Self {
    Self {
      host: Some(BindHostOptions::LocalHost),
      port: Some(3210),
    }
  }
}
