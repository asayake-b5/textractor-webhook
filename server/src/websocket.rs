use std::sync::RwLock;

use actix::{Actor, Handler, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::server::{PushMessage, SocketServer};

pub struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("got {text}",);
                ctx.text(text)
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<PushMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: PushMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

pub async fn websocket_entry(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<RwLock<SocketServer>>,
) -> Result<HttpResponse, Error> {
    let result = ws::WsResponseBuilder::new(MyWs {}, &req, stream).start_with_addr();
    if let Ok((ref addr, _)) = result {
        data.write().unwrap().sessions.push(addr.clone());
    }
    result.map(|(_, r)| r)
}
