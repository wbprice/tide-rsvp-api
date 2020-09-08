use crate::models::{ContactStatus, DietaryRestrictions, DishPreferences};
use dynomite::Item;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Item, Clone)]
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
    pub dish_preferences: Option<DishPreferences>,
}
