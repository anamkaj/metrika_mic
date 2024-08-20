use crate::{
    server::handler::{
        goals::{handler_all_goals, handler_delete_counter, handler_goals, handler_update_goals},
        statisticas::handler_get_statistics,
    },
    utils::create_table::create_table,
};
use axum::{
    http::HeaderValue,
    routing::{delete, get, post, put},
    Router,
};
use dotenv::dotenv;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn server_router() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url_connect: String = std::env::var("CLIENT_TABLE").unwrap();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&url_connect)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app_state: Arc<AppState> = Arc::new(AppState { db: pool.clone() });

    // ? Create table
    match create_table(&app_state.db).await {
        Ok(result) => {
            println!("✅ {}", result);
            true
        }
        Err(err) => {
            println!("🔥 Failed to create table: {:?}", err);
            false
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app: Router = Router::new()
        .route("/api/statistics", post(handler_get_statistics))
        .route("/api/metrika/goals", get(handler_goals))
        .route("/api/blizko/all-goals", get(handler_all_goals))
        .route("/api/blizko/update-goals", put(handler_update_goals))
        .route("/api/delete-count", delete(handler_delete_counter))
        .with_state(app_state)
        .layer(cors);

    println!("Server started successfully at 0.0.0.0:8080");

    let listener: TcpListener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
