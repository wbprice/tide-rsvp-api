use tide::{Body, Request, Response, Result, StatusCode};
use uuid::Uuid;

use crate::{
    models::{RSVPQuery, RSVP},
    services::RSVPService,
};

pub async fn put_rsvp(mut req: Request<()>) -> Result<Response> {
    let _rsvp_id : Uuid = req.param("rsvp_id")?;
    let rsvp: RSVP = req.body_json().await?;

    match RSVPService.put(rsvp.clone()).await {
        Ok(_) => Ok(Body::from_json(&rsvp)?.into()),
        Err(_) => Ok(Response::new(StatusCode::InternalServerError)),
    }
}

pub async fn get_rsvp(req: Request<()>) -> Result<Response> {
    let query: RSVPQuery = req.query()?;

    match RSVPService.get_by_household(query.household_id).await {
        Ok(rsvps) => Ok(Body::from_json(&rsvps)?.into()),
        Err(error) => {
            dbg!(error);
            Ok(Response::new(StatusCode::InternalServerError))
        }
    }
}
