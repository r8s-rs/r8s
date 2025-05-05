use actix_web::{App, HttpServer, web, middleware};
use fjall::{Config, PartitionCreateOptions};
use sqlx::postgres::PgPoolOptions;
use std::collections::VecDeque;
use std::fs::create_dir_all;
use std::sync::{Arc, Mutex};
use application::State;
use log::{info, error};
use std::env::var;
use actix::Actor;

mod infrastructure;
mod application;
mod actors;
mod domain;
mod http;
mod executador;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let base_path = infrastructure::repositories::FileRepository::get_base_path();

    let fjall_path = infrastructure::repositories::FileRepository::get_fjall_path();

    info!("As configurações/filas do r8s estão sendo salvas em [{base_path}]");

    let _ = create_dir_all(base_path);

    let keyspace = Config::new(fjall_path).open().expect("Não foi possível criar o keyspace fjall");

    let keyspace = Arc::new(keyspace);

    let partitions = application::Partitions {
        webhook_v1_pendings: keyspace.open_partition(
            domain::entities::partitions::WEBHOOK_V1_PENDINGS,
            PartitionCreateOptions::default(),
        ).expect("Não foi possível abrir a partição")
    };

    let partitions = Arc::new(partitions);

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

    match sqlx::migrate!().run(&pool).await {
        Ok(()) => info!("As migrações foram aplicadas!"),
        Err(e) => error!("Falha ao aplicar migrações {e}")
    }

    let data = web::Data::new(State {
        db: pool.clone(),
        keyspace: keyspace.clone(),
        partitions: partitions,
        workflow_pendings: Arc::new(Mutex::new(VecDeque::new())),
    });

    actors::WorkflowToQueue {
        state: data.clone(),
    }.start();

    actors::WebhookV1ToExecution {
        state: data.clone(),
    }.start();

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