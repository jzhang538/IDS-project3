use actix_web::{get, web, web::ServiceConfig, HttpResponse, Error};
use shuttle_actix_web::ShuttleActixWeb;
extern crate chinese_dictionary;
use chinese_dictionary::query;

#[get("/query/{sent}")]
async fn hello_world(info: web::Path<String>) -> Result<String, Error> {
    let text = info.to_string();
	let text_str = text.as_str();
	let results = query(text_str).unwrap();
	let cn_str = results[0].simplified.to_string();
	println!("{}", cn_str);
    Ok(cn_str)
}

#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };
    Ok(config.into())
}
