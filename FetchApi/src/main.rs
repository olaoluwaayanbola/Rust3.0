use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ChannelSnippet {
    title: String,
    description: String,
    published_at: Option<String>, // make this an Option to handle missing fields
    thumbnails: Thumbnails,
}

#[derive(Debug, Serialize, Deserialize)]
struct Thumbnails {
    default: Thumbnail,
    medium: Thumbnail,
    high: Thumbnail,
}

#[derive(Debug, Serialize, Deserialize)]
struct Thumbnail {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChannelItem {
    id: String,
    snippet: ChannelSnippet,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChannelListResponse {
    items: Vec<ChannelItem>,
}

#[get("/api/channels")]
async fn get_channels() -> impl Responder {
    let api_key = "AIzaSyDmF_yW-GHYdyTc9kuYMeBp-Ptm0UqE-5g";
    let username = "mrbeast";
    // let url = format!("https://www.googleapis.com/youtube/v3/channels?part=snippet&forUsername={}&key={}", username, api_key);

    match reqwest::get(&url).await {
        Ok(response) => match response.json::<ChannelListResponse>().await {
            Ok(json) => HttpResponse::Ok().json(json),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["content-type"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(get_channels)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
