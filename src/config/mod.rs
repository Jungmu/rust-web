use serde::Deserialize;
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub pg: deadpool_postgres::Config,
}
