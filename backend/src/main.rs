#![deny(clippy::all)]
#![warn(clippy::nursery)]

use clap::Parser;
use once_cell::sync::Lazy;
use poem::{
    http::Method,
    listener::TcpListener,
    middleware::{CookieJarManager, Cors, Tracing},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use routing::{franchise, person};
use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::OnceCell;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routing::{media, user};

pub mod error;
mod jwt_middleware;
mod routing;
pub mod types;
pub mod util;

pub mod queries;

pub async fn pool<'a>() -> &'a PgPool {
    static POOL: OnceCell<PgPool> = OnceCell::const_new();

    POOL.get_or_init(|| async {
        let db_connection_str = &CONFIG.database_url;

        PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(db_connection_str)
            .await
            .expect("can't connect to database")
    })
    .await
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, env)]
    jwt_secret: String,

    #[arg(
        long,
        env,
        default_value = "postgres://test:test@localhost/test?schema=public"
    )]
    database_url: String,

    #[arg(long, env, default_value = "http://localhost")]
    domain: String,
    #[arg(long, env, default_value = "3000")]
    port: u16,
    #[arg(long, env, default_value = "0.0.0.0")]
    host: String,
}

pub static CONFIG: Lazy<Args> = Lazy::new(Args::parse);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Lazy::force(&CONFIG);

    dbg!(&config);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,entertainment_logger_backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST)
        .allow_method(Method::PATCH)
        .allow_method(Method::PUT)
        .allow_method(Method::HEAD)
        .allow_method(Method::DELETE)
        .allow_credentials(true);

    let api_service = OpenApiService::new(
        (user::UserRoutes, media::MediaRoutes, franchise::FranchiseRoutes, person::PersonRoutes),
        "Entertainment logger API",
        "0.1",
    )
    .server(format!("{}/api/v1", &config.domain));

    let explorer = api_service.openapi_explorer();
    let spec = api_service.spec_endpoint();
    let spec_yml = api_service.spec_endpoint_yaml();

    tracing::info!("{}", &config.domain);
    tracing::info!("http://{}:{}", &config.host, &config.port);

    sqlx::migrate!("./migrations").run(pool().await).await.unwrap();

    let route = Route::new()
        .nest("/api/", explorer)
        .nest("/api/v1", api_service.with(cors))
        .nest("/api/spec.json", spec)
        .nest("/api/spec.yml", spec_yml)
        .with(Tracing)
        .with(CookieJarManager::new());

    Server::new(TcpListener::bind(format!(
        "{}:{}",
        &config.host, &config.port
    )))
    .run(route)
    .await?;

    Ok(())
}
