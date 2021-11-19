use actix_web::{App, HttpServer, HttpRequest, Result, web, HttpResponse, error};
use askama::Template;
use std::env;


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    h1: String
}

async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let html = IndexTemplate {
        h1: "Yo".to_string()
    }.render().map_err(|e| {
        error::ErrorInternalServerError(e.to_string())
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "4200".to_owned());
    println!("Server starting on port {} ...", port);
    HttpServer::new(||
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(actix_files::Files::new("/js", "./assets/js").show_files_listing())
            .service(actix_files::Files::new("/stylesheets", "./assets/stylesheets").show_files_listing())
            .service(actix_files::Files::new("/images", "./assets/images").show_files_listing())
    )
        .bind("0.0.0.0:".to_owned() + &*port)?
        .run()
        .await
}