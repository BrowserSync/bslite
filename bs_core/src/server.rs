use crate::static_route::StaticRoute;
use crate::BindOptions;


#[derive(Default, Clone, Debug)]
pub struct Server {
  pub bind: BindOptions,
  pub routes: Vec<StaticRoute>,
}
