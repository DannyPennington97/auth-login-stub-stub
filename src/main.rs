pub mod lib;

use actix_web::{App, HttpServer, HttpRequest, Result, web, HttpResponse, error};
use askama::Template;
use std::env;
use std::time::Duration;
use serde_derive::Deserialize;
use std::fmt::Error;
use actix_web::error::ErrorBadRequest;
use config::{ConfigError, Config};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    h1: String,
    services: Vec<String>
}

#[derive(Deserialize)]
struct FormData {
    service: String,
}

async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let html = IndexTemplate {
        h1: "Select your service".to_string(),
        services: vec!["ers-returns".to_string(), "ers-checking".to_string()]
    }.render().map_err(|e| {
        error::ErrorInternalServerError(e.to_string())
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn handle_quick(_req: HttpRequest, form: web::Form<FormData>) -> Result<HttpResponse>  {
    let config: Config = lib::get_config(&form.service).map_err(|e: ConfigError| error::ErrorInternalServerError(e.to_string()))?;

    let redirect: String = config.get("redirect_url").unwrap();

    Ok(HttpResponse::Ok().content_type("text/plain").body(format!("Quick handler used {}! Redirect_url: {}", form.service, redirect)))
}

async fn handle_custom(_req: HttpRequest, form: web::Form<FormData>) -> Result<HttpResponse>  {
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!("Custom handler used {}!", form.service)))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "4200".to_owned());
    println!("Server starting on port {} ...", port);
    HttpServer::new(||
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/quick-submit").route(web::post().to(handle_quick)))
            .service(web::resource("/custom-submit").route(web::post().to(handle_custom)))
            .service(actix_files::Files::new("/js", "./assets/js").show_files_listing())
            .service(actix_files::Files::new("/stylesheets", "./assets/stylesheets").show_files_listing())
            .service(actix_files::Files::new("/images", "./assets/images").show_files_listing())
    )
        .bind("0.0.0.0:".to_owned() + &*port)?
        .run()
        .await
}