use tide::{Body, Request, Result};
use tide::prelude::*;
use serde_json::Value;

use crate::services::RSVPService;

pub async fn put_rsvp(mut req: Request<()>) -> Result<Value> {
    let household_id = req.param("household_id")?;
    let rsvp_id = req.param("rsvp_id")?;
    let rsvp;

    let rsvp = RSVPService::put(household_id, rsvp_id, rsvp);
}
