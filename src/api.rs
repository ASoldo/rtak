use actix::{Actor, ActorContext, StreamHandler};
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use actix_web::{Error, HttpRequest, web};
use actix_web_actors::ws;

pub struct WsSession;

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
}

// Handle incoming WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(format!("Echo: {}", text)),
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsSession {}, &req, stream)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("RTAK is alive!")
}

pub async fn start_rest_server(bind_addr: &str) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .route("/ws", web::get().to(ws_index))
    })
    .bind(bind_addr)?
    .run()
    .await
}
