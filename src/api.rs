use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("RTAK is alive!")
}

pub async fn start_rest_server(bind_addr: &str) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health))
        .bind(bind_addr)?
        .run()
        .await
}
