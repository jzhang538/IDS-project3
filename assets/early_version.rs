use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Error};
use actix_web::{get, web::ServiceConfig, HttpResponse};
use shuttle_service::ShuttleActixWeb;
extern crate chinese_dictionary;
use chinese_dictionary::query;
use chinese_dictionary::{MeasureWord, WordEntry};
use chinese_dictionary::{tokenize};

#[get("/query/{sent}")]
async fn q(info: web::Path<String>) -> Result<String, Error> {
    // Querying the dictionary returns an `Option<Vec<&WordEntry>>`
    // Read more about the WordEntry struct below
    let text = info.to_string();
    let text_str = text.as_str();
    let results = query(text_str).unwrap();
    let cn_str = results[0].simplified.to_string();
    Ok(cn_str)
}

#[get("/play")]
async fn playgame() -> HttpResponse {
    let options = ["rock", "paper", "scissors"];
    let computer_choice = options[rand::random::<usize>() % options.len()];
    HttpResponse::Ok().body(format!(
        "The computer played: {}",
        computer_choice
    ))
}

#[shuttle_service::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(q);
    })
}


#[shuttle_service::main]
async fn main() -> std::io::Result<()> {
    println!("Running the service");
    HttpServer::new(|| App::new().service(hello).service(q).service(s))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
