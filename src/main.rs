use axum::{response::Html, routing::get, Router, Extension};
use std::net::SocketAddr;
use std::time::Duration;
use tracing_subscriber::{prelude::*, layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::postgres::{PgPoolOptions, PgPool};

#[tokio::main]
async fn main() {
    tracing::info!("tracing_subscriber initializing...");
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("D2Q_LOG").unwrap_or_else(|_| "warn,d2q=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("database connnection pool setting...");
    let db_connection_str = std::env::var("D2Q_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://tstuser:tstsecretpassword@localhost/tstdb".to_string());

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");

    tracing::info!("building application...");
    // build our application with some routes
    let app = Router::new()
        .route(
            "/",
            get(handler).post(handler2),
        )
        .layer(Extension(pool));

    tracing::info!("running application...");
    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    //
    // // build our application with a route
    // let app = Router::new().route("/", get(handler));
    //
    // // run it
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // // println!("listening on {}", addr);
    // tracing::debug!("listening on {}", addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
//
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
async fn handler2() -> Html<&'static str> {
    Html("<h1>Your Request's Method is POST!!!</h1>")
}