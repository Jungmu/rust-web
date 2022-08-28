mod api;
mod config;
mod db;
mod errors;
mod route;

#[cfg(test)]
mod tests;

use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::config::Config as MyConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: MyConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(route::hello)
            .service(api::login_admin)
    })
    .workers(4)
    .bind(config.server_addr.clone())?
    .run()
    .await
}
