use serde_json::Value;
use tide::prelude::*;
use tide::{Body, Error, Request, Response, Result, StatusCode};

use crate::{
    models::{RSVPQuery, RSVP},
    services::RSVPService,
};

pub async fn put_rsvp(mut req: Request<()>) -> Result<Response> {
    let rsvp_id = req.param("rsvp_id")?;
    let rsvp: RSVP = req.body_json().await?;

    match RSVPService.put(rsvp_id, rsvp.clone()).await {
        Ok(_) => Ok(Body::from_json(&rsvp)?.into()),
        Err(err) => Ok(Response::new(StatusCode::InternalServerError)),
    }
}

pub async fn get_rsvp(mut req: Request<()>) -> Result<Value> {
    let query: RSVPQuery = req.query()?;

    let household: Vec<RSVP> = RSVPService.get_by_household(query.household_id).await;
    Ok(json!(household));
}
