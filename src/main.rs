use std::fs;

use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use actix_web::http::StatusCode;

const ASSET_JSON: &str = "/etc/asset-links/asset.json";

#[get("/assetlinks.json")]
async fn assetlinks() -> impl Responder {
    let content = fs::read_to_string(ASSET_JSON).unwrap_or("[]".to_owned());
    HttpResponse::with_body(StatusCode::OK, content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(assetlinks))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
