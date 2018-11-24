use serde_derive::{Deserialize, Serialize};

pub fn is_false(value: &bool) -> bool {
    *value == false
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Empty {}
