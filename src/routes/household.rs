use tide::{Request, Result};
use tide::prelude::*;
use serde_json::Value;

pub async fn get_household(mut req: Request<()>) -> Result<Value> {
    Ok(json!({
        "meta": { "count": 2 },
        "animals": [
            { "type": "cat", "name": "chashu" },
            { "type": "cat", "name": "nori" }
        ]
    }))
}

pub async fn put_household(mut req: Request<()>) -> Result<Value> {
    Ok(json!({
        "meta": { "count": 2 },
        "animals": [
            { "type": "cat", "name": "chashu" },
            { "type": "cat", "name": "nori" }
        ]
    }))
}
