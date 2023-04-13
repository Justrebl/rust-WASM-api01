use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_echo(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    let builder = http::Response::builder();

    let resp = builder
        .body(req.into_body())
        .unwrap();

    return Ok(resp);
}
