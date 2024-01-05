mod http;
mod routes;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]

async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let sqlx_connection = PgPoolOptions::new()
        .max_connections(250)
        .connect(&url)
        .await
        .unwrap();

    http::serve(sqlx_connection).await?;

    Ok(())
}
