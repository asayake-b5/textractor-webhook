use std::net::TcpListener;

pub mod server;
pub mod websocket;

pub fn find_free_port() -> Option<u16> {
    (8000..55000).find(|port| TcpListener::bind(("127.0.0.1", *port)).is_ok())
}
