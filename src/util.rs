use serde_derive::{Serialize, Deserialize};

pub fn is_false(value: &bool) -> bool {
    *value == false
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Empty {}
