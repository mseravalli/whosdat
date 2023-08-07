use actix_files::NamedFile;
use actix_web::error;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use clap::Parser;
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

async fn render_tmpl(data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("protocol", &data.protocol);
    ctx.insert("domain", &data.domain);
    ctx.insert("port", &data.port);
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
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
    port: u16,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    using_https: bool,

    #[arg(short, long)]
    domain: String,

    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

// #[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    let args = Args::parse();
    let protocol = if args.using_https {
        format!("https")
    } else {
        format!("http")
    };
    let domain = args.domain.clone();
    let port = args.port;

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .data(AppData {
                tmpl: tera,
                protocol: protocol.clone(),
                domain: domain.clone(),
                port,
            })
            .route("/", web::get().to(render_tmpl))
            .route("/people", web::get().to(people))
            .route("/pics/{filename:.*}", web::get().to(pics))
    })
    .bind((args.domain, args.port))?
    .run()
    .await
}
