#![feature(io_error_more)]

use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::iter::Iterator;
use tiny_http::{Header, Request, Response, Server};

static RESPONSE: Lazy<HashMap<&'static str, (&'static str, Option<Header>)>> = Lazy::new(|| {
    HashMap::<&'static str, (&'static str, Option<Header>)>::from([
        ("/", ("data/{}/index.html", None)),
        ("/Build/WebGL.data", ("data/{}/Build/WebGL.data", None)),
        (
            "/Build/WebGL.framework.js",
            ("data/{}/Build/WebGL.framework.js", None),
        ),
        (
            "/Build/WebGL.loader.js",
            ("data/{}/Build/WebGL.loader.js", None),
        ),
        (
            "/Build/WebGL.wasm",
            (
                "data/{}/Build/WebGL.wasm",
                Header::from_bytes(&b"Content-Type"[..], &b"application/wasm"[..])
                    .unwrap()
                    .into(),
            ),
        ),
        (
            "/TemplateData/favicon.ico",
            ("data/{}/TemplateData/favicon.ico", None),
        ),
        (
            "/TemplateData/fullscreen-button.png",
            ("data/{}/TemplateData/fullscreen-button.png", None),
        ),
        (
            "/TemplateData/progress-bar-empty-dark.png",
            ("data/{}/TemplateData/progress-bar-empty-dark.png", None),
        ),
        (
            "/TemplateData/progress-bar-empty-light.png",
            ("data/{}/TemplateData/progress-bar-empty-light.png", None),
        ),
        (
            "/TemplateData/progress-bar-full-dark.png",
            ("data/{}/TemplateData/progress-bar-full-dark.png", None),
        ),
        (
            "/TemplateData/progress-bar-full-light.png",
            ("data/{}/TemplateData/progress-bar-full-light.png", None),
        ),
        (
            "/TemplateData/unity-logo-dark.png",
            ("data/{}/TemplateData/unity-logo-dark.png", None),
        ),
        (
            "/TemplateData/unity-logo-light.png",
            ("data/{}/TemplateData/unity-logo-light.png", None),
        ),
        (
            "/TemplateData/webgl-logo.png",
            ("data/{}/TemplateData/webgl-logo.png", None),
        ),
        (
            "/TemplateData/style.css",
            ("data/{}/TemplateData/style.css", None),
        ),
        (
            "StreamingAssets",
            (
                "data/{}//StreamingAssets/UnityServicesProjectConfiguration.json",
                Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                    .unwrap()
                    .into(),
            ),
        ),
    ])
});

pub fn serve(name: &str) -> Result<()> {
    let port = std::fs::read_to_string("SERVE_PORT").unwrap();
    let port = port.trim();

    match Server::http(&format!("0.0.0.0:{port}")) {
        Ok(server) => {
            for request in server.incoming_requests() {
                if let Some(response) = match_response(name, &request) {
                    request.respond(response)?;
                }
            }
        }
        Err(err) => {
            return Err(Error::new(
                ErrorKind::NetworkUnreachable,
                format!("Cannot connect to port {port} due to {err}"),
            )
            .into());
        }
    }

    Ok(())
}

fn match_response(name: &str, request: &Request) -> Option<Response<File>> {
    let request_url = request.url();
    if let Some((reponse, ref header)) = RESPONSE.get(request_url) {
        return get_response_with_file(reponse.replace("{}", name).as_ref(), (*header).clone());
    }
    panic!("Cannot process the request: {request_url}");
}

fn get_response_with_file(path: &str, header: Option<Header>) -> Option<Response<File>> {
    match File::open(path) {
        Ok(file) => {
            let mut response = Response::from_file(file);
            if let Some(header) = header {
                response.add_header(header);
            }
            return Some(response);
        }
        Err(err) => {
            log::error!("Error while opening file: {}", err);
            return None;
        }
    }
}

fn main() -> Result<()> {
    let args = std::env::args();
    if args.len() != 2_usize {
        return Err(Error::new(
            ErrorKind::ConnectionRefused,
            "The server expects exactly one argument".to_string(),
        )
        .into());
    }

    Ok(serve(args.last().unwrap().as_str())?)
}
