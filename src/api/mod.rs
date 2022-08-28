pub mod response;

use actix_web::{post, web, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::db::{self, models::Admin};
use crate::errors::Error;

#[post("/login")]
async fn login_admin(
    user: web::Json<Admin>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: Admin = user.into_inner();

    let client: Client = db_pool.get().await.map_err(Error::PoolError)?;

    let result = db::login_admin(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(response::parse(result)))
}
