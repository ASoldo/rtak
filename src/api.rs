use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use actix_web::{Error, HttpRequest, web};
use actix_web_actors::ws;

pub struct WsSession {
    broadcaster: Addr<crate::broadcaster::Broadcaster>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendText(pub String);

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.broadcaster
            .do_send(crate::broadcaster::RegisterSession(ctx.address()));
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.broadcaster
            .do_send(crate::broadcaster::UnregisterSession(ctx.address()));
    }
}

impl Handler<SendText> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: SendText, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
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

async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    broadcaster: web::Data<Addr<crate::broadcaster::Broadcaster>>,
) -> Result<HttpResponse, Error> {
    let session = WsSession {
        broadcaster: broadcaster.get_ref().clone(),
    };
    ws::start(session, &req, stream)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("RTAK is alive!")
}

pub async fn start_rest_server(
    bind_addr: &str,
    broadcaster: Addr<crate::broadcaster::Broadcaster>,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(broadcaster.clone())) // add this line
            .service(health)
            .route("/ws", web::get().to(ws_index))
    })
    .bind(bind_addr)?
    .run()
    .await
}
