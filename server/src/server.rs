use actix::{Addr, Message};

use crate::websocket::MyWs;

#[derive(Default)]
pub struct SocketServer {
    pub sessions: Vec<Addr<MyWs>>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PushMessage(pub String);

impl SocketServer {
    pub fn push(&self, msg: &str) {
        for session in self.sessions.iter() {
            session.do_send(PushMessage(msg.to_string()));
        }
    }
}
