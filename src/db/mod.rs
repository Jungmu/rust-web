pub mod models;

use deadpool_postgres::Client;
use sha2::{Digest, Sha256};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::errors::Error;
use models::Admin;

pub async fn login_admin(client: &Client, admin_info: Admin) -> Result<Admin, Error> {
    let _stmt = include_str!("../sql/login_admin.sql");
    let _stmt = _stmt.replace("$table_fields", &Admin::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let mut hasher = Sha256::new();

    hasher.update(admin_info.password);

    let password_hash: String = format!("{:X}", hasher.finalize());

    client
        .query(&stmt, &[&admin_info.id, &password_hash])
        .await?
        .iter()
        .map(|row| Admin::from_row_ref(row).unwrap())
        .collect::<Vec<Admin>>()
        .pop()
        .ok_or(Error::NotFound) // more applicable for SELECTs
}
