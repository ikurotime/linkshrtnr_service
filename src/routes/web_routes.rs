use axum::{extract::Path, response::Redirect, routing::get, Extension, Router};
use sqlx::{Error, PgPool, Row};

use crate::http::ApiContext;
struct LinkResponse {
    long_url: String,
}
pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(Redirect::to("https://linkshrtnr.com")))
        .route("/*path", get(get_another_page))
}
async fn get_another_page(ctx: Extension<ApiContext>, Path(path): Path<String>) -> Redirect {
    let link = match extract_link(path, &ctx.db).await {
        Ok(link) => link,
        Err(_) => return Redirect::to("https://linkshrtnr.com/404"),
    };
    Redirect::temporary(link.long_url.as_str())
}

async fn extract_link(path: String, pool: &PgPool) -> Result<LinkResponse, Error> {
    let q = "SELECT * FROM links WHERE short_url = $1";
    let link = sqlx::query(q).bind(path).fetch_one(pool).await?;

    let link = LinkResponse {
        long_url: link.try_get("long_url")?,
    };
    Ok(link)
}
