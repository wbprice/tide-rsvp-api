use dynomite_derive::Attribute;
use serde::{Deserialize, Serialize};
#[derive(Attribute, PartialEq, Serialize, Deserialize)]
pub enum DietaryRestrictions {
    Vegetarian,
    Vegan,
    Pescatarian,
    GlutenFree,
    DairyFree,
}
