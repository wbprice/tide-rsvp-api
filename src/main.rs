mod models;
mod routes;
mod services;

use crate::routes::{get_rsvp, put_rsvp};
use async_std;

#[tokio::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();

    app.at("/rsvp").get(get_rsvp);
    app.at("/rsvp/:id").put(put_rsvp);

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
