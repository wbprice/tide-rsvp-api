use serde::{Deserialize, Serialize};
use tide::prelude::*;
use tide::{Body, Request, Result};

#[derive(Deserialize, Serialize)]
struct Message {
    text: String
}

async fn echo(mut request: Request<()>) -> Result {
    let payload : Message = request.body_json().await?;
    println!("message: {}", payload.text);
    Ok(payload.into())
}

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::start();

    let mut app = tide::new();

    app.at("/").get(|_| async { Ok("Hello, world!") });

    app.at("/echo").post(echo);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
