use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Error};
extern crate chinese_dictionary;
use chinese_dictionary::query;
use chinese_dictionary::{MeasureWord, WordEntry};
use chinese_dictionary::{tokenize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hi! You can use our CNTranslator via following URL queries: (a) Translate an English phrase into Chinese via query/<xxx> (b) Segment a Chinese sentence via seg/<xxx>.")
}

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

#[get("/seg/{sent}")]
async fn s(info: web::Path<String>) -> Result<String, Error> {
	let text = info.to_string();
	let text_str = text.as_str();
	let results = tokenize(text_str);
	let mut result_str = "".to_string();
	for (pos, r) in results.iter().enumerate() {
        result_str.push_str(r);
        result_str.push_str(" ");
    }
    Ok(result_str)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running the service");
    HttpServer::new(|| App::new().service(hello).service(q).service(s))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
