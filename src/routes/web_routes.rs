use std::net::SocketAddr;

use crate::http::ApiContext;
use axum::{
    extract::{ConnectInfo, Path},
    http::HeaderMap,
    response::Redirect,
    routing::get,
    Extension, Router,
};
use sqlx::{Error, PgPool};
use tracing::info;
#[derive(Debug, sqlx::FromRow)]
struct LinkResponse {
    id: i32,
    original_url: String,
}
pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(Redirect::to("https://linkshrtnr.com/")))
        .route("/*path", get(get_another_page))
}
async fn get_another_page(
    header: HeaderMap,
    ctx: Extension<ApiContext>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(path): Path<String>,
) -> Redirect {
    info!(header = ?header, "Header");
    info!(addr = ?addr, "Address");

    let link = match extract_link(&path, &ctx.db, addr.to_string(), header).await {
        Ok(link) => link,
        Err(err) => {
            info!("ERROR: {:?}", err);
            return Redirect::to("https:linkshrtnr.com/404");
        }
    };

    Redirect::temporary(&link.original_url.as_str())
}

async fn extract_link(
    path: &String,
    pool: &PgPool,
    addr: String,
    header: HeaderMap,
) -> Result<LinkResponse, Error> {
    let q = "SELECT * FROM links WHERE short_url = $1";
    let link: LinkResponse = sqlx::query_as(q).bind(path).fetch_one(pool).await?;
    info!(link = ?link, "Link");
    let q = "UPDATE linkclicks SET ClickCount = ClickCount + 1,ipaddress = $1, useragent = $2, referrer = $3 WHERE linkid = $4";
    //extract referer from header or set it to empty string
    let mut user_agent = String::new();
    if let Some(ua) = header.get("user-agent") {
        user_agent = ua.to_str().unwrap().to_string();
    }
    let mut referer = String::new();
    if let Some(refer) = header.get("referer") {
        referer = refer.to_str().unwrap().to_string();
    }
    let insertion = sqlx::query(q)
        .bind(addr.to_string())
        .bind(user_agent)
        .bind(referer)
        .bind(&link.id)
        .execute(pool)
        .await?;
    info!(insertion = ?insertion, "Insertion");
    Ok(link)
}
