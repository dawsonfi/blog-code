use env_logger::{Builder, Target};
use lambda_http::{service_fn, Body, Error, IntoResponse, Request, Response};
use log::{info, LevelFilter};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .init();
    lambda_http::run(service_fn(func)).await?;
    Ok(())
}

async fn func(event: Request) -> Result<impl IntoResponse, Error> {
    info!("Event(info): {:?}", event);

    let body: Value = serde_json::from_str(match event.body() {
        Body::Text(ref text) => text.as_str(),
        _ => "{}",
    })
    .unwrap();

    Ok(Response::builder()
        .status(200)
        .body(format!(
            "{{\"subject_name\": \"{}\"}}",
            body["name"].as_str().unwrap()
        ))
        .unwrap())
}
