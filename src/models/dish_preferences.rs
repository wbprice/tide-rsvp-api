use dynomite_derive::Attribute;
use serde::{Deserialize, Serialize};

#[derive(Attribute, PartialEq, Serialize, Deserialize)]
pub enum DishPreferences {
    Chicken,
    Steak,
    Pancakes,
}
