use std::env;
use std::collections::HashMap;
use uuid::Uuid;
use rusoto_dynamodb::{DynamoDbClient, DynamoDb, QueryInput, AttributeValue};
use rusoto_core::Region;
use dynomite::FromAttributes;
use tide::{Result, Error, StatusCode};

use crate::models::RSVP;

pub struct RSVPService;

impl RSVPService {
    pub async fn get_by_household(&self, household_id: Uuid) -> Result<Vec<RSVP>> {
        let client = DynamoDbClient::new(Region::UsEast1);

        // Query RSVP records to get records with a common household ID
        let mut query = HashMap::new(); 
        query.insert(
            ":household_id".to_string(),
            AttributeValue {
                s: Some(household_id.to_string()),
                ..AttributeValue::default()
            }
        );
        
        // Make the request
        let request = client.query(QueryInput {
            table_name: env::var("RSVP_TABLE_NAME").unwrap(),
            key_condition_expression: Some("household_id = :household_id".to_string()),
            expression_attribute_values: Some(query),
            ..QueryInput::default()
        }).await;

        // Check the request
        if let Ok(response) = request {
            // If the request was successful, convert the response into an array of RSVP items
            match response.items {
                Some(items) => {
                    let rsvps: Vec<RSVP> = items
                        .into_iter()
                        .map(|item| RSVP::from_attrs(item).unwrap())
                        .collect();
                    Ok(rsvps)
                },  
                None => {
                    Ok(vec![])
                }
            }
        } else {
            Err(Error::from_str(
                StatusCode::InternalServerError, 
                "Something bad happened".to_string())
            )
        }
    }

    pub async fn put(&self, id: Uuid, rsvp: RSVP) -> Result<RSVP> {
        let client = DynamoDbClient::new(Region::UsEast1);

    }
}
