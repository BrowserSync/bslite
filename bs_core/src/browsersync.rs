use crate::bind_address::BindAddressOptions;
use std::path::PathBuf;

pub struct Browsersync {
  pub servers: Vec<Server>,
}

#[derive(Default, Debug)]
pub struct Server {
  pub bind_address: BindAddressOptions,
  pub dirs: Vec<PathBuf>,
}
