use tide::{Request, Result};
use tide::prelude::*;
use serde_json::Value;

use crate::services::RSVPService;

pub async fn get_household(mut req: Request<()>) -> Result<Value> {
    let household_id = req.param("household_id")?;
    const household = RSVPService::get_by_household().await?;
    Ok(household);
}
