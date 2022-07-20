
use serde::{Deserialize, Serialize};
use actix_web::middleware::Logger;
use actix_web::{
    web, App,
    HttpRequest, HttpResponse, HttpServer,
    Error,
};
use futures::StreamExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HTTPStatusMsg {
    pub status: String,
    pub status_message: String,
}

impl HTTPStatusMsg {
    pub fn success(msg: &str) -> HTTPStatusMsg {
        HTTPStatusMsg {
            status: String::from("success"),
            status_message: String::from(msg),
        }
    }
}

pub async fn upload(
    _req: HttpRequest,
    mut body: web::Payload, // the actual file body
) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }
    println!("Got data {} bytes", bytes.len());

    // Do what you will with the bytes here...
    
    let msg = format!("Uploaded {} bytes", bytes.len());
    Ok(HttpResponse::Ok().json(HTTPStatusMsg::success(&msg)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "0.0.0.0";
    let port = 3030;
    println!("Server running on {}:{} 🚀", host, port);

    HttpServer::new(move || {
        App::new()
            .route(
                "/upload",
                web::post().to(upload),
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind((host, port))?
    .run()
    .await
}
