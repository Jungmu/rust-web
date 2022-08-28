use super::*;

use actix_web::{test, web, App};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

// use api::response;

fn get_pg_pool() -> Pool {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: MyConfig = config_.try_deserialize().unwrap();

    config.pg.create_pool(None, NoTls).unwrap()
}

#[actix_web::test]
async fn test_index_get() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(get_pg_pool().clone()))
            .service(route::hello),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[derive(Deserialize, Serialize)]
struct ReqLogin<'a> {
    id: &'a str,
    password: &'a str,
}

// #[actix_web::test]
// async fn test_login_admin() {
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(get_pg_pool().clone()))
//             .service(api::login_admin),
//     )
//     .await;

//     let req: ReqLogin = ReqLogin {
//         id: "id",
//         password: "password",
//     };

//     let req = test::TestRequest::post()
//         .uri("/login")
//         .set_json(req)
//         .to_request();
//     let resp: response::ResLoginAdmin = test::call_and_read_body_json(&app, req).await;
//     assert_eq!(resp.id, "id");
// }
