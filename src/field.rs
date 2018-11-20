use serde_derive::{Deserialize, Serialize};
use util::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Field {
    pub label: String,
    pub key: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_message: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub is_relative: bool,
}

impl Field {
    pub fn new<L, K, V>(label: L, key: K, value: V) -> Self
    where
        L: Into<String>,
        K: Into<String>,
        V: Into<String>,
    {
        Field {
            label: label.into(),
            key: key.into(),
            value: value.into(),
            change_message: None,
            is_relative: false,
        }
    }

    pub fn new_with_change<L, K, V, C>(label: L, key: K, value: V, change: C) -> Self
    where
        L: Into<String>,
        K: Into<String>,
        V: Into<String>,
        C: Into<String>,
    {
        Field {
            change_message: Some(change.into()),
            is_relative: false,
            ..Field::new(label, key, value)
        }
    }
}
