use crate::static_route::StaticRoute;
use crate::BindOptions;

#[derive(Default, Clone, Debug, serde::Deserialize)]
pub struct Server {
  #[serde(default)]
  pub bind: BindOptions,
  #[serde(default)]
  pub routes: Vec<StaticRoute>,
}
