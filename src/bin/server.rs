#![feature(io_error_more)]

use tiny_http::{Server, Request, Response};
use std::io::{Error, ErrorKind};
use std::fs::File;
use anyhow::Result;

pub fn serve() -> Result<()> {
    let port = std::fs::read_to_string("SERVE_PORT").unwrap();
    let port = port.trim();

    let Ok(server) = Server::http(&format!("0.0.0.0:{port}")) else {
        return Err(
            Error::new(ErrorKind::NetworkUnreachable,
            format!("Cannot connect to port {port}")).into(),
        );
    };

    for request in server.incoming_requests() {
        if let Some(response) = match_response(&request) {
            request.respond(response)?;
        }
    }

    Ok(())
}

fn match_response(request: &Request) -> Option<Response<File>> {
    match dbg!(request.url()) {
        request => panic!("Cannot process the request: {request}"),
    }

    None
}

fn main() -> Result<()> {
    Ok(serve()?)
}
