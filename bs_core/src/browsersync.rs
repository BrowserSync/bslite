use crate::bind_address::BindAddressOptions;

pub struct Browsersync {
  pub servers: Vec<Server>,
}

#[derive(Default)]
pub struct Server {
  pub bind_address: BindAddressOptions,
}
