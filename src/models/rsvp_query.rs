use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct RSVPQuery {
    pub household_id: Uuid,
}

impl Default for RSVPQuery {
    fn default() -> Self {
        RSVPQuery {
            household_id: Uuid::new_v4(),
        }
    }
}
