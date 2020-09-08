mod models;
mod routes;
mod services;

use crate::routes::{get_rsvp, put_rsvp};

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();

    app.at("/rsvp").get(get_rsvp);
    app.at("/rsvp/:id").put(put_rsvp);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
