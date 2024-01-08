use std::net::SocketAddr;

use crate::http::ApiContext;
use axum::{
    extract::{ConnectInfo, Path},
    http::HeaderMap,
    response::Redirect,
    routing::get,
    Extension, Router,
};
use ipgeolocate::{Locator, Service};
use sqlx::{Error, PgPool, Row};
use tracing::info;
#[derive(Debug, sqlx::FromRow)]
struct LinkResponse {
    id: i32,
    original_url: String,
}
pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(Redirect::to("http://localhost:3000")))
        .route("/*path", get(get_another_page))
}
async fn get_another_page(
    header: HeaderMap,
    ctx: Extension<ApiContext>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(path): Path<String>,
) -> Redirect {
    let service = Service::IpApi;
    info!(header = ?header, "Header");
    info!(addr = ?addr, "Address");
    let geolocation = match Locator::get(addr.to_string().as_str(), service).await {
        Ok(ip) => format!("{}", ip.country),
        Err(error) => format!("Error"),
    };
    let link = match extract_link(&path, &ctx.db, addr.to_string(), header, geolocation).await {
        Ok(link) => link,
        Err(_) => return Redirect::to("http://localhost:3000/404"),
    };

    Redirect::temporary(&link.original_url.as_str())
}

async fn extract_link(
    path: &String,
    pool: &PgPool,
    addr: String,
    header: HeaderMap,
    geolocation: String,
) -> Result<LinkResponse, Error> {
    let q = "SELECT * FROM links WHERE short_url = $1";
    let link: LinkResponse = sqlx::query_as(q).bind(path).fetch_one(pool).await?;
    let q = "UPDATE linkclicks SET ClickCount = ClickCount + 1,ipaddress = $1, useragent = $2, referrer = $3,  geographiclocation = $4 WHERE linkid = $5";
    let insertion = sqlx::query(q)
        .bind(addr.to_string())
        .bind(header.get("user-agent").unwrap().to_str().unwrap())
        .bind(header.get("referer").unwrap().to_str().unwrap())
        .bind(geolocation)
        .bind(&link.id)
        .execute(pool)
        .await?;
    info!(insertion = ?insertion, "Insertion");
    info!(link = ?link, "Link");
    Ok(link)
}
