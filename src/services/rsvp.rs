use dynomite::{attr_map, FromAttributes, Item};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, QueryInput, GetItemInput};
use std::env;
use tide::{Error, Result, StatusCode};
use uuid::Uuid;

use crate::models::RSVP;

pub struct RSVPService;

impl RSVPService {
    pub async fn get_by_household(&self, household_id: Uuid) -> Result<Vec<RSVP>> {
        let client = DynamoDbClient::new(Region::UsEast1);
        let table_name = env::var("RSVP_TABLE_NAME")?;

        // Make the request
        let response = client
            .query(QueryInput {
                table_name,
                key_condition_expression: Some("household_id = :household_id".to_string()),
                expression_attribute_values: Some(attr_map! {
                    ":household_id" => household_id.to_string()
                }),
                ..QueryInput::default()
            })
            .await?;

        // Map each item into an RSVP and return the list
        match response.items {
            Some(items) => {
                let rsvps: Vec<RSVP> = items
                    .into_iter()
                    .map(|item| RSVP::from_attrs(item).unwrap())
                    .collect();
                Ok(rsvps)
            }
            None => Ok(vec![]),
        }
    }

    pub async fn put(&self, id: Uuid, rsvp: RSVP) -> Result<()> {
        let client = DynamoDbClient::new(Region::UsEast1);
        let table_name = env::var("RSVP_TABLE_NAME")?;

        client
            .put_item(PutItemInput {
                table_name: table_name.clone(),
                item: rsvp.into(),
                ..PutItemInput::default()
            })
            .await?;

        Ok(())
    }
}
