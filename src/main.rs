use actix_files::NamedFile;
use actix_web::error;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use rand::seq::SliceRandom;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tera::{Context, Tera};

#[derive(Serialize, Deserialize, Clone)]
struct Person {
    picture: String,
    name: String,
}

impl Person {
    fn new(name: String, picture: String) -> Self {
        Person { picture, name }
    }
}

async fn render_tmpl(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("protocol", &data.protocol);
    ctx.insert("domain", &data.domain);
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn index() -> Result<NamedFile> {
    let path: PathBuf = PathBuf::from("static/index.html");
    Ok(NamedFile::open(path)?)
}

async fn people() -> Result<HttpResponse> {
    let mut people = vec![];
    let mut files = tokio::fs::read_dir("static/pics/partners/").await?;

    while let Some(f) = files.next_entry().await? {
        let file_name = f
            .file_name()
            .into_string()
            .expect("Not valid UTF-8 file name");
        let persons_name = file_name.replace(".jpg", "").replace("_", " ");
        let person = Person::new(persons_name, format!("pics/{}", file_name));
        people.push(person);
    }

    let rng = &mut rand::thread_rng();
    people.shuffle(rng);

    Ok(HttpResponse::Ok().json(people))
}

async fn pics(req: HttpRequest) -> Result<NamedFile> {
    let file_name: String = req.match_info().query("filename").parse().unwrap();
    let re = Regex::new(r"\w+\.(jpg|jpeg|png)").expect("Invalid regex");
    if !re.is_match(&file_name) {
        println!("{}", file_name);
        return Err(error::ErrorNotFound(format!("Not Found")));
    }
    let path: PathBuf = PathBuf::from(format!("static/pics/{}", file_name));
    Ok(NamedFile::open(path)?)
}

struct AppData {
    tmpl: Tera,
    protocol: String,
    domain: String,
}

// #[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .data(AppData {
                tmpl: tera,
                protocol: format!("http"),
                domain: format!("localhost:8080"),
            })
            .route("/", web::get().to(render_tmpl))
            .route("/people", web::get().to(people))
            .route("/pics/{filename:.*}", web::get().to(pics))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
