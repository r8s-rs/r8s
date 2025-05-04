use actix_web::{App, HttpServer, web, middleware};
use std::collections::{VecDeque, BTreeMap};
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, Mutex};
use application::State;
use std::env::var;
use actix::Actor;

mod infrastructure;
mod application;
//mod execucao2;
mod actors;
mod domain;
mod http;
//mod execucao;
mod executador;

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
        webhook_v1_pendings: Arc::new(Mutex::new(BTreeMap::new())),
        workflow_pendings: Arc::new(Mutex::new(VecDeque::new())),
    });

    actors::WorkflowToQueue {
        state: data.clone(),
    }.start();

    actors::WebhookV1ToWorkflow {
        state: data.clone(),
    }.start();

    //executador::enqueue_next_nodes(&pool, 1, 119, None).await;

    let port = std::env::var("R8S_HTTP_PORT").unwrap_or("5000".to_string()).parse::<u16>().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .service(
                web::resource("/wh/{path:.*}")
                    .route(
                        web::route()
                        .to(http::webhook_http::webhook_http)
                    )
            ).service(
                web::resource("/wf")
                    .post(http::WorkflowHttp::store)
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}