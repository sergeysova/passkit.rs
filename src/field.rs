use serde_derive::{Deserialize, Serialize};
use util::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Field {
    /// Attributed value of the field.
    /// The value may contain HTML markup for links.
    /// Only the `<a>` tag and its href attribute are supported.
    /// For example, the following is key-value pair specifies a link with the text “Edit my profile”:
    /// `"attributedValue": "<a href='http://example.com/customers/123'>Edit my profile</a>"`
    /// This key’s value overrides the text specified by the value key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributed_value: Option<String>,

    /// Format string for the alert text that is displayed when the pass is updated.
    /// The format string must contain the escape `%@`, which is replaced with the field’s new value.
    /// For example, `“Gate changed to %@.”`
    /// If you don’t specify a change message, the user isn’t notified when the field changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_message: Option<String>,

    /// Data detectors that are applied to the field’s value.
    /// The default value is all data detectors. Provide an empty array to use no data detectors.
    /// Data detectors are applied only to back fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_detector_types: Option<Vec<DataDetectorType>>,

    /// The key must be unique within the scope of the entire pass.
    /// For example, `“departure-gate.”`
    pub key: String,

    /// Label text for the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Alignment for the field’s contents.
    #[serde(skip_serializing_if = "TextAlignment::is_natural")]
    pub text_alignment: TextAlignment,

    /// Value of the field, for example, `42`.
    pub value: Value,

    #[serde(flatten)]
    pub date: Option<FieldDate>,

    #[serde(flatten)]
    pub number: Option<FieldNumber>,
}

impl<TKey, TLabel, TValue> From<(TKey, TLabel, TValue)> for Field
where
    TKey: Into<String>,
    TLabel: Into<String>,
    TValue: Into<Value>,
{
    fn from((key, label, value): (TKey, TLabel, TValue)) -> Field {
        Field {
            key: key.into(),
            label: Some(label.into()),
            value: value.into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Int(i32),
    Float(f64),
}

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::String(value)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &str) -> Value {
        Value::String(value.into())
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Value {
        Value::Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value::Float(value)
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::String("".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataDetectorType {
    #[serde(rename = "PKDataDetectorTypePhoneNumber")]
    PhoneNumber,
    #[serde(rename = "PKDataDetectorTypeLink")]
    Link,
    #[serde(rename = "PKDataDetectorTypeAddress")]
    Address,
    #[serde(rename = "PKDataDetectorTypeCalendarEvent")]
    CalendarEvent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TextAlignment {
    #[serde(rename = "PKTextAlignmentLeft")]
    Left,
    #[serde(rename = "PKTextAlignmentCenter")]
    Center,
    #[serde(rename = "PKTextAlignmentRight")]
    Right,
    #[serde(rename = "PKTextAlignmentNatural")]
    Natural,
}

impl Default for TextAlignment {
    fn default() -> TextAlignment {
        TextAlignment::Natural
    }
}

impl TextAlignment {
    pub fn is_natural(&self) -> bool {
        match *self {
            TextAlignment::Natural => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct FieldDate {
    /// Style of date to display
    pub date_style: DateTimeStyle,

    /// Always display the time and date in the given time zone, not in the user’s current time zone.
    /// The default value is false.
    /// The format for a date and time always requires a time zone, even if it will be ignored.
    /// For backward compatibility with iOS 6, provide an appropriate time zone,
    /// so that the information is displayed meaningfully even without ignoring time zones.
    /// This key does not affect how relevance is calculated.
    #[serde(skip_serializing_if = "is_false")]
    pub ignores_time_zone: bool,

    /// If true, the label’s value is displayed as a relative date;
    /// otherwise, it is displayed as an absolute date.
    /// The default value is false.
    /// This key does not affect how relevance is calculated.
    #[serde(skip_serializing_if = "is_false")]
    pub is_relative: bool,

    /// Style of time to display
    pub time_style: DateTimeStyle,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DateTimeStyle {
    /// PKDateStyleNone
    #[serde(rename = "PKDateStyleNone")]
    None,

    /// PKDateStyleShort
    #[serde(rename = "PKDateStyleShort")]
    Short,

    /// PKDateStyleMedium
    #[serde(rename = "PKDateStyleMedium")]
    Medium,

    /// PKDateStyleLong
    #[serde(rename = "PKDateStyleLong")]
    Long,

    /// PKDateStyleFull
    #[serde(rename = "PKDateStyleFull")]
    Full,
}

impl Default for DateTimeStyle {
    fn default() -> Self {
        DateTimeStyle::Medium
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct FieldNumber {
    /// ISO 4217 currency code for the field’s value.
    pub currency_code: String,

    /// Style of number to display.
    /// Number styles have the same meaning as the Cocoa number formatter styles with corresponding names.
    pub number_style: NumberStyle,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NumberStyle {
    /// PKNumberStyleDecimal
    #[serde(rename = "PKNumberStyleDecimal")]
    Decimal,

    /// PKNumberStylePercent
    #[serde(rename = "PKNumberStylePercent")]
    Percent,

    /// PKNumberStyleScientific
    #[serde(rename = "PKNumberStyleScientific")]
    Scientific,

    /// PKNumberStyleSpellOut
    #[serde(rename = "PKNumberStyleSpellOut")]
    SpellOut,
}

impl Default for NumberStyle {
    fn default() -> Self {
        NumberStyle::Decimal
    }
}

impl Field {
    pub fn new<Label, Key, Val, Change>(label: Label, key: Key, value: Val, change: Change) -> Self
    where
        Label: Into<String>,
        Key: Into<String>,
        Val: Into<Value>,
        Change: Into<String>,
    {
        Field {
            key: key.into(),
            label: Some(label.into()),
            value: value.into(),
            change_message: Some(change.into()),
            ..Default::default()
        }
    }
}
