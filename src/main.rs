use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Result};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
struct Person {
    picture: String,
    name: String,
    answer: String,
    answer_state: String,
}

impl Person {
    fn new(name: String, picture: String) -> Self {
        Person {
            picture,
            name,
            answer: format!(""),
            answer_state: format!(""),
        }
    }
}

async fn index() -> Result<NamedFile> {
    let path: PathBuf = PathBuf::from("static/index.html");
    Ok(NamedFile::open(path)?)
}

async fn people() -> Result<HttpResponse> {
    let base = "http://localhost:8080";
    let mut people = vec![];
    let mut files = tokio::fs::read_dir("static/pics/partners/").await?;

    while let Some(f) = files.next_entry().await? {
        let file_name = f
            .file_name()
            .into_string()
            .expect("Not valid UTF-8 file name");
        let persons_name = file_name.replace(".jpg", "").replace("_", " ");
        let person = Person::new(persons_name, format!("{}/pics/{}", base, file_name));
        people.push(person);
    }

    let rng = &mut rand::thread_rng();
    people.shuffle(rng);

    Ok(HttpResponse::Ok().json(people))
}

async fn pics(req: HttpRequest) -> Result<NamedFile> {
    let tmp: String = req.match_info().query("filename").parse().unwrap();
    let path: PathBuf = PathBuf::from(format!("static/pics/{}", tmp));
    // let path: PathBuf = req.match_info().query("filename").parse().unwrap();
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
            .route("/pics/{filename:.*}", web::get().to(pics))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
