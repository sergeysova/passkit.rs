use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Personalization {
    pub required_personalization_fields: Vec<PersonalizationField>,

    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_and_conditions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PersonalizationField {
    /// PKPassPersonalizationFieldName
    #[serde(rename = "PKPassPersonalizationFieldName")]
    Name,

    /// PKPassPersonalizationFieldPostalCode
    #[serde(rename = "PKPassPersonalizationFieldPostalCode")]
    PostalCode,

    /// PKPassPersonalizationFieldEmailAddress
    #[serde(rename = "PKPassPersonalizationFieldEmailAddress")]
    EmailAddress,

    /// PKPassPersonalizationFieldPhoneNumber
    #[serde(rename = "PKPassPersonalizationFieldPhoneNumber")]
    PhoneNumber,
}

mod test {

    #[test]
    fn example() {
        use super::*;
        let pers = Personalization {
            required_personalization_fields: vec![
                PersonalizationField::Name,
                PersonalizationField::PhoneNumber,
                PersonalizationField::PostalCode,
            ],
            description: "Enter your information to sign up and earn points.".to_string(),
            terms_and_conditions: Some("Terms".into()),
        };

        let json = serde_json::to_string_pretty(&pers).unwrap();

        println!("{}", json);
        println!("{:#?}", pers);
    }
}
