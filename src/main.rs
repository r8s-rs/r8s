use actix_web::{App, HttpServer, web, middleware};
use sqlx::postgres::PgPoolOptions;
use std::env::var;
mod domain;
mod application;
mod http;
mod infrastructure;

use application::State;


use application::executor::run_workflow;
use infrastructure::reader::read_workflow;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(
            var("R8S_POSTGRES_CONNECTIONS").unwrap_or(
                "5".to_string()
            ).parse::<u32>().unwrap()
        )
        .test_before_acquire(false)
        .connect(
            var("R8S_POSTGRES_URL").expect("R8S_POSTGRES_URL expected").as_str()
        )
        .await
        .expect("Failed to connect to database");

    let data = web::Data::new(State {
        db: pool.clone(),
    });

    let port = std::env::var("R8S_HTTP_PORT").unwrap_or("5000".to_string()).parse::<u16>().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .service(
                web::resource("/wh/{path:.*}")
                    .route(web::route()
                    .to(http::webhook::webhook))
        )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}