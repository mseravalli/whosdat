use actix_files::NamedFile;
use actix_web::Result;
use std::path::PathBuf;

async fn index() -> Result<NamedFile> {
    let path: PathBuf = PathBuf::from("static/index.html");
    Ok(NamedFile::open(path)?)
}

async fn people() -> Result<NamedFile> {
    let path: PathBuf = PathBuf::from("static/people.json");
    Ok(NamedFile::open(path)?)
}

// #[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/people", web::get().to(people))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
