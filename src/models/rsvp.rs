use crate::{ContactStatus, DietaryRestrictions, DishPreferences};
use uuid::Uuid;

pub struct RSVP {
    pub household_id: Uuid,
    pub id: Uuid,
    pub name: String,
    pub email_address: String,
    pub attending: bool,
    pub contact_status: ContactStatus,
    pub dietary_restrictions: Option<DietaryRestrictions>,
    pub dietary_restrictions_other: Option<String>,
    pub dish_preferences: Option<DishPreferences>,
}
