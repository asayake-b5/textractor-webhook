use std::sync::RwLock;

use actix_files::Files;
use actix_web::{
    http::header::ContentType,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use clap::Parser;
use serde::Deserialize;
use textractor_server::{find_free_port, server::SocketServer, websocket::websocket_entry};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(short, long)]
    bind: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TextractorContent {
    content: String,
}

async fn push_line(
    _: HttpRequest,
    req: web::Json<TextractorContent>,
    data: web::Data<RwLock<SocketServer>>,
) -> impl Responder {
    data.read().unwrap().push(&req.content);

    HttpResponse::Ok()
}

async fn homepage(_: HttpRequest) -> impl Responder {
    let body = std::fs::read_to_string("static/index.html").unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)
}

#[actix_web::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    let args = Args::parse();

    let url = if let Some(ip) = args.bind {
        ip
    } else {
        let port = find_free_port().expect("Couldn't find any port to bind to.");
        format!("127.0.0.1:{port}")
    };
    println!("Binding to {url}");

    let server = Data::new(RwLock::new(SocketServer::default()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&server))
            .route("/", web::get().to(homepage))
            .route("/api/push", web::post().to(push_line))
            .route("/ws", web::get().to(websocket_entry))
            .service(Files::new("/static", "./static/"))
    })
    .bind(&url)?
    .run()
    .await
}
