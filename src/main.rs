use serde::{Deserialize, Serialize};
use tide::prelude::*; // Pulls in the json! macro.
use tide::{Body, Request};

mod models;
mod routes;
mod services;

use crate::models::RSVP;
use crate::routes::{get_household, put_rsvp};

#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();

    app.at("/household/:household_id").get(get_household);
    app.at("/household/:household_id/rsvp/:rsvp_id").put(put_rsvp);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
