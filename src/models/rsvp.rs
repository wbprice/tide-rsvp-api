use serde::{Deserialize, Serialize};
use crate::models::{ContactStatus, DietaryRestrictions, DishPreferences};
use uuid::Uuid;
use dynomite::Item;

#[derive(Serialize, Deserialize, Item)]
pub struct RSVP {
    #[dynomite(partition_key)]
    pub household_id: Uuid,
    pub id: Uuid,
    #[dynomite(sort_key)]
    pub name: String,
    pub email_address: String,
    pub attending: bool,
    pub contact_status: ContactStatus,
    pub dietary_restrictions: Option<DietaryRestrictions>,
    pub dietary_restrictions_other: Option<String>,
    pub dish_preferences: Option<DishPreferences>
}

pub struct RSVPQuery {
    household_id: Uuid,
}
