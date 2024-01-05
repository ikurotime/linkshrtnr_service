mod http;
mod routes;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let sqlx_connection = PgPoolOptions::new()
        .max_connections(250)
        .connect(&url)
        .await
        .unwrap();

    http::serve(sqlx_connection).await?;

    Ok(())
}
