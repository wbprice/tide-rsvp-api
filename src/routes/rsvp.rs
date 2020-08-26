use tide::{Body, Request, Result};
use tide::prelude::*;
use serde_json::Value;

pub async fn get(mut req: Request<()>) -> Result<Value> {
    Ok(json!({
        "meta": { "count": 2 },
        "animals": [
            { "type": "cat", "name": "chashu" },
            { "type": "cat", "name": "nori" }
        ]
    }))
}

pub async fn put(mut req: Request<()>) -> Result<Value> {
    Ok(json!({
        "meta": { "count": 2 },
        "animals": [
            { "type": "cat", "name": "chashu" },
            { "type": "cat", "name": "nori" }
        ]
    }))
}
