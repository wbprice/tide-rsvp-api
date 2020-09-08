use tide::prelude::*;
use tide::{Body, Request, Response, Result, StatusCode};

use crate::{
    models::{RSVPQuery, RSVP},
    services::RSVPService,
};

pub async fn put_rsvp(mut req: Request<()>) -> Result<Response> {
    let rsvp_id = req.param("rsvp_id")?;
    let rsvp: RSVP = req.body_json().await?;

    match RSVPService.put(rsvp_id, rsvp.clone()).await {
        Ok(_) => Ok(Body::from_json(&rsvp)?.into()),
        Err(_) => Ok(Response::new(StatusCode::InternalServerError)),
    }
}

pub async fn get_rsvp(req: Request<()>) -> Result<Response> {
    let query: RSVPQuery = req.query()?;

    match RSVPService.get_by_household(query.household_id).await {
        Ok(rsvps) => Ok(Body::from_json(&rsvps)?.into()),
        Err(_) => Ok(Response::new(StatusCode::InternalServerError)),
    }
}
