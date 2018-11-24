use field::Field;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use util::*;

/// The top level of the pass.json file is a dictionary.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pass {
    /// Brief description of the pass, used by the iOS accessibility technologies.
    pub description: String,

    /// Version of the file format. The value must be 1.
    pub format_version: i32,

    /// Display name of the organization that originated and signed the pass.
    pub organization_name: String,

    /// Pass type identifier, as issued by Apple.
    /// The value must correspond with your signing certificate.
    pub pass_type_identifier: String,

    /// Serial number that uniquely identifies the pass.
    /// No two passes with the same pass type identifier may have the same serial number.
    pub serial_number: String,

    /// Team identifier of the organization that originated and signed the pass, as issued by Apple.
    pub team_identifier: String,

    /// A URL to be passed to the associated app when launching it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "appLaunchURL")]
    #[serde(default)]
    pub app_launch_url: Option<String>,

    /// A list of iTunes Store item identifiers for the associated apps.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub associated_store_identifiers: Vec<i32>,

    #[serde(default)]
    pub user_info: HashMap<String, String>,

    /// Date and time when the pass expires.
    /// W3C date
    /// The value must be a complete date with hours and minutes, and may optionally include seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub expiration_date: Option<String>,

    /// Indicates that the pass is void—for example, a one time use coupon that has been redeemed.
    /// The default value is false.
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub voided: bool,

    /// Beacons marking locations where the pass is relevant.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub beacons: Vec<Beacon>,

    /// Locations where the pass is relevant. For example, the location of your store.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub locations: Vec<Location>,

    /// Maximum distance in meters from a relevant latitude and longitude that the pass is relevant.
    /// This number is compared to the pass’s default distance and the smaller value is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_distance: Option<u32>,

    /// Recommended for event tickets and boarding passes; otherwise optional.
    /// Date and time when the pass becomes relevant. For example, the start time of a movie.
    /// The value must be a complete date with hours and minutes, and may optionally include seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub relevant_date: Option<String>,

    #[serde(flatten)]
    pub style: Style,

    #[serde(default)]
    #[serde(flatten)]
    pub visual: Option<VisualAppearance>,

    /// Information used to update passes using the web service.
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[serde(default)]
    pub web_service: Option<WebService>,

    /// Information used for Value Added Service Protocol transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub nfc: Option<NFC>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct VisualAppearance {
    /// Information specific to the pass’s barcode.
    /// The system uses the first valid barcode dictionary in the array.
    /// Additional dictionaries can be added as fallbacks
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub barcodes: Vec<Barcode>,

    /// Background color of the pass, specified as an CSS-style RGB triple.
    /// For example, rgb(23, 187, 82).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub background_color: Option<String>,

    /// Foreground color of the pass, specified as a CSS-style RGB triple.
    /// For example, rgb(100, 10, 110).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub foreground_color: Option<String>,

    /// Optional for event tickets and boarding passes; otherwise not allowed.
    /// Identifier used to group related passes. If a grouping identifier is specified,
    /// passes with the same style, pass type identifier, and grouping identifier are displayed as a group.
    /// Otherwise, passes are grouped automatically. Use this to group passes that are tightly related,
    /// such as the boarding passes for different connections of the same trip.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub grouping_identifier: Option<String>,

    /// Color of the label text, specified as a CSS-style RGB triple.
    /// For example, rgb(255, 255, 255).
    /// If omitted, the label color is determined automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub label_color: Option<String>,

    /// Text displayed next to the logo on the pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub logo_text: Option<String>,

    /// If true, the strip image is displayed without a shine effect.
    /// The default value prior to iOS 7.0 is false.
    /// In iOS 7.0, a shine effect is never applied, and this key is deprecated.
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub suppress_strip_shine: bool,
}

/// Information about a location beacon.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Beacon {
    /// Unique identifier of a Bluetooth Low Energy location beacon.
    #[serde(rename = "proximityUUID")]
    pub proximity_uuid: String,

    /// Major identifier of a Bluetooth Low Energy location beacon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<u16>,

    /// Minor identifier of a Bluetooth Low Energy location beacon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<u16>,

    /// Text displayed on the lock screen when the pass is currently relevant.
    /// For example, a description of the nearby location such as “Store nearby on 1st and Main.”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_text: Option<String>,
}

/// Information about a location.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Location {
    /// Altitude, in meters, of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,

    /// Latitude, in meters, of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,

    /// Longitude, in meters, of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,

    /// Text displayed on the lock screen when the pass is currently relevant.
    /// For example, a description of the nearby location such as “Store nearby on 1st and Main.”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_text: Option<String>,
}

/// From<(altitude, latitude)>
impl From<(f64, f64)> for Location {
    fn from((alt, lat): (f64, f64)) -> Location {
        Location {
            altitude: Some(alt),
            latitude: Some(lat),
            ..Default::default()
        }
    }
}

/// From<(altitude, latitude, longitude)>
impl From<(f64, f64, f64)> for Location {
    fn from((alt, lat, lon): (f64, f64, f64)) -> Location {
        Location {
            altitude: Some(alt),
            latitude: Some(lat),
            longitude: Some(lon),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Style {
    BoardingPass(Structure),
    Coupon(Structure),
    EventTicket(Structure),
    Generic(Structure),
    StoreCard(Structure),
}

impl Default for Style {
    fn default() -> Style {
        Style::Generic(Default::default())
    }
}

/// Keys that define the structure of the pass.
/// These keys are used for all pass styles and partition the fields into the various parts of the pass.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Structure {
    /// Additional fields to be displayed on the front of the pass.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    auxiliary_fields: Vec<Field>,

    /// Fields to be on the back of the pass.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    back_fields: Vec<Field>,

    /// Fields to be displayed in the header on the front of the pass.
    /// Use header fields sparingly; unlike all other fields, they remain visible when a stack of passes are displayed.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    header_fields: Vec<Field>,

    /// Fields to be displayed prominently on the front of the pass.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    primary_fields: Vec<Field>,

    /// Fields to be displayed on the front of the pass.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    secondary_fields: Vec<Field>,

    /// Required for boarding passes; otherwise not allowed. Type of transit.
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_type: Option<TransitType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransitType {
    /// PKTransitTypeAir
    #[serde(rename = "PKTransitTypeAir")]
    Air,

    /// PKTransitTypeBoat
    #[serde(rename = "PKTransitTypeBoat")]
    Boat,

    /// PKTransitTypeBus
    #[serde(rename = "PKTransitTypeBus")]
    Bus,

    /// PKTransitTypeGeneric
    #[serde(rename = "PKTransitTypeGeneric")]
    Generic,

    /// PKTransitTypeTrain
    #[serde(rename = "PKTransitTypeTrain")]
    Train,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Barcode {
    /// Message or payload to be displayed as a barcode.
    pub message: String,

    /// Barcode format.
    pub format: BarcodeFormat,

    /// Text encoding that is used to convert the message from the string representation
    /// to a data representation to render the barcode.
    /// The value is typically iso-8859-1, but you may use another encoding that
    /// is supported by your barcode scanning infrastructure.
    pub message_encoding: String,

    /// Text displayed near the barcode.
    /// For example, a human-readable version of the barcode data in case the barcode doesn’t scan.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub alt_text: Option<String>,
}

/// From<(BardcodeFormat::Code128, "MESSAGE")>
impl<M: Into<String>> From<(BarcodeFormat, M)> for Barcode {
    fn from((format, message): (BarcodeFormat, M)) -> Barcode {
        Barcode {
            message: message.into(),
            format,
            ..Default::default()
        }
    }
}

impl Default for Barcode {
    fn default() -> Self {
        Barcode {
            message: String::new(),
            format: BarcodeFormat::Code128,
            message_encoding: String::from("iso-8859-1"),
            alt_text: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BarcodeFormat {
    /// PKBarcodeFormatQR
    #[serde(rename = "PKBarcodeFormatQR")]
    QR,

    /// PKBarcodeFormatPDF417
    #[serde(rename = "PKBarcodeFormatPDF417")]
    PDF417,

    /// PKBarcodeFormatAztec
    #[serde(rename = "PKBarcodeFormatAztec")]
    Aztec,

    /// PKBarcodeFormatCode128
    #[serde(rename = "PKBarcodeFormatCode128")]
    Code128,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebService {
    /// The authentication token to use with the web service. The token must be 16 characters or longer.
    pub authentication_token: String,

    /// The URL of a web service that conforms to the API described in PassKit Web Service Reference.
    /// https://developer.apple.com/library/archive/documentation/PassKit/Reference/PassKit_WebService/WebService.html#//apple_ref/doc/uid/TP40011988
    /// The web service must use the HTTPS protocol; the leading https:// is included in the value of this key.
    /// On devices configured for development, there is UI in Settings to allow HTTP web services.
    #[serde(rename = "webServiceURL")]
    pub web_service_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NFC {
    /// The payload to be transmitted to the Apple Pay terminal.
    /// Must be 64 bytes or less. Messages longer than 64 bytes are truncated by the system.
    pub message: String,

    /// The public encryption key used by the Value Added Services protocol.
    /// Use a Base64 encoded X.509 SubjectPublicKeyInfo structure containing a ECDH public key for group P256.
    #[serde(default)]
    pub encryption_public_key: Option<String>,
}

#[derive(Default, Clone)]
pub struct PassBuilder {
    serial_number: String,
    pass_type_identifier: String,
    team_identifier: String,
    organization_name: Option<String>,
    description: Option<String>,
    structure: Structure,
    app_launch_url: Option<String>,
    associated_store_identifiers: Vec<i32>,
    user_info: HashMap<String, String>,
    expiration_date: Option<String>,
    voided: bool,
    beacons: Vec<Beacon>,
    locations: Vec<Location>,
    max_distance: Option<u32>,
    relevant_date: Option<String>,
    visual: VisualAppearance,
    web_service: Option<WebService>,
    nfc: Option<NFC>,
}

impl PassBuilder {
    pub fn new<S, I, T>(
        serial_number: S,
        pass_type_identifier: I,
        team_identifier: T,
    ) -> PassBuilder
    where
        S: Into<String>,
        I: Into<String>,
        T: Into<String>,
    {
        PassBuilder {
            serial_number: serial_number.into(),
            pass_type_identifier: pass_type_identifier.into(),
            team_identifier: team_identifier.into(),
            ..Default::default()
        }
    }

    pub fn organization_name<O: Into<String>>(mut self, organization_name: O) -> PassBuilder {
        self.organization_name = Some(organization_name.into());
        self
    }

    pub fn description<D: Into<String>>(mut self, description: D) -> PassBuilder {
        self.description = Some(description.into());
        self
    }

    pub fn app_launch_url<U: Into<String>>(mut self, url: U) -> PassBuilder {
        self.app_launch_url = Some(url.into());
        self
    }

    pub fn add_associated_store_identifier(mut self, id: i32) -> PassBuilder {
        self.associated_store_identifiers.push(id);
        self
    }

    pub fn add_user_info<K, V>(mut self, key: K, value: V) -> PassBuilder
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.user_info.insert(key.into(), value.into());
        self
    }

    pub fn expiration_date<D: Into<String>>(mut self, date: D) -> PassBuilder {
        self.expiration_date = Some(date.into());
        self
    }

    pub fn voided(mut self) -> PassBuilder {
        self.voided = true;
        self
    }

    pub fn add_beacon(mut self, beacon: Beacon) -> PassBuilder {
        self.beacons.push(beacon);
        self
    }

    pub fn add_location<T: Into<Location>>(mut self, location: T) -> PassBuilder {
        self.locations.push(location.into());
        self
    }

    pub fn max_distance(mut self, distance: u32) -> PassBuilder {
        self.max_distance = Some(distance);
        self
    }

    pub fn relevant_date(mut self, date: String) -> PassBuilder {
        self.relevant_date = Some(date);
        self
    }

    pub fn add_auxiliary_field<T: Into<Field>>(mut self, field: T) -> PassBuilder {
        self.structure.auxiliary_fields.push(field.into());
        self
    }

    pub fn add_back_field<T: Into<Field>>(mut self, field: T) -> PassBuilder {
        self.structure.back_fields.push(field.into());
        self
    }

    pub fn add_header_field<T: Into<Field>>(mut self, field: T) -> PassBuilder {
        self.structure.header_fields.push(field.into());
        self
    }

    pub fn add_primary_field<T: Into<Field>>(mut self, field: T) -> PassBuilder {
        self.structure.primary_fields.push(field.into());
        self
    }

    pub fn add_secondary_field<T: Into<Field>>(mut self, field: T) -> PassBuilder {
        self.structure.auxiliary_fields.push(field.into());
        self
    }

    pub fn add_barcode<T: Into<Barcode>>(mut self, barcode: T) -> PassBuilder {
        self.visual.barcodes.push(barcode.into());
        self
    }

    pub fn background_color<C: Into<String>>(mut self, color: C) -> PassBuilder {
        self.visual.background_color = Some(color.into());
        self
    }

    pub fn foreground_color<C: Into<String>>(mut self, color: C) -> PassBuilder {
        self.visual.foreground_color = Some(color.into());
        self
    }

    pub fn grouping_identifier(mut self, identifier: String) -> PassBuilder {
        self.visual.grouping_identifier = Some(identifier);
        self
    }

    pub fn label_color(mut self, color: String) -> PassBuilder {
        self.visual.label_color = Some(color);
        self
    }

    pub fn logo_text<T: Into<String>>(mut self, text: T) -> PassBuilder {
        self.visual.logo_text = Some(text.into());
        self
    }

    pub fn suppress_strip_shine(mut self) -> PassBuilder {
        self.visual.suppress_strip_shine = true;
        self
    }

    pub fn web_service<T, U>(mut self, token: T, url: U) -> PassBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.web_service = Some(WebService {
            authentication_token: token.into(),
            web_service_url: url.into(),
        });
        self
    }

    pub fn nfc<M: Into<String>>(mut self, message: M, key: Option<String>) -> PassBuilder {
        self.nfc = Some(NFC {
            message: message.into(),
            encryption_public_key: key,
        });
        self
    }

    fn build(self, style: Style) -> Pass {
        Pass {
            format_version: 1,
            serial_number: self.serial_number,
            pass_type_identifier: self.pass_type_identifier,
            team_identifier: self.team_identifier,
            organization_name: self.organization_name.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            app_launch_url: self.app_launch_url,
            associated_store_identifiers: self.associated_store_identifiers,
            user_info: self.user_info,
            expiration_date: self.expiration_date,
            voided: self.voided,
            beacons: self.beacons,
            locations: self.locations,
            max_distance: self.max_distance,
            relevant_date: self.relevant_date,
            style,
            visual: Some(self.visual),
            web_service: self.web_service,
            nfc: self.nfc,
        }
    }

    pub fn finish_boarding_pass(self, transit_type: TransitType) -> Pass {
        let mut structure = self.structure.clone();
        structure.transit_type = Some(transit_type);
        self.build(Style::BoardingPass(structure))
    }

    pub fn finish_coupon(self) -> Pass {
        let structure = self.structure.clone();
        self.build(Style::Coupon(structure))
    }

    pub fn finish_event_ticket(self) -> Pass {
        let structure = self.structure.clone();
        self.build(Style::EventTicket(structure))
    }

    pub fn finish_generic(self) -> Pass {
        let structure = self.structure.clone();
        self.build(Style::Generic(structure))
    }

    pub fn finish_store_card(self) -> Pass {
        let structure = self.structure.clone();
        self.build(Style::StoreCard(structure))
    }
}

pub fn rgb(r: u8, g: u8, b: u8) -> String {
    format!("rgba({}, {}, {})", r, g, b)
}

mod test {
    #[test]
    fn ser_pass_example() {
        use super::*;

        let pass = PassBuilder::new("001", "pass.com.example", "CDHE9L6U22")
            .web_service(
                "vxwxd7J8AlNNFPS8k0a0FfUFtq0ewzFdc",
                "https://example.com/passes/",
            ).relevant_date("2012-07-22T14:25-08:00".into())
            .add_location((-122.3748889, 37.6189722))
            .add_barcode((
                BarcodeFormat::PDF417,
                "SFOJFK JOHN APPLESEED LH451 2012-07-22T14:25-08:00",
            )).organization_name("Skyport Airways")
            .description("Skyport Boarding Pass")
            .logo_text("Skyport Airways")
            .foreground_color(rgb(22, 55, 110))
            .background_color(rgb(22, 55, 110))
            .add_header_field(Field::new("GATE", "gate", "23", "Gate changed to %@."))
            .add_primary_field(("SAN FRANCISCO", "depart", "SFO"))
            .add_primary_field(("NEW YORK", "arrive", "JFK"))
            .add_secondary_field(("PASSENGER", "passenger", "John Appleseed"))
            .add_auxiliary_field(Field::new(
                "DEPART",
                "boardingTime",
                "2:25 PM",
                "Boarding time changed to %@.",
            )).add_auxiliary_field(Field::new(
                "FLIGHT",
                "flightNewName",
                "815",
                "Flight number changed to %@",
            )).add_auxiliary_field(("DESIG.", "class", "Coach"))
            .add_auxiliary_field(("DATE", "date", "7/22"))
            .add_back_field(("PASSPORT", "passport", "Canadian/Canadien"))
            .add_back_field((
                "RESIDENCE",
                "residence",
                "999 Infinite Loop, Apartment 42, Cupertino CA",
            )).finish_boarding_pass(TransitType::Air);

        println!("{}", serde_json::to_string_pretty(&pass).unwrap());
    }

    #[test]
    fn de_pass_example() {
        use super::*;

        let src = r#"
            {
  "formatVersion" : 1,
  "passTypeIdentifier": "pass.com.sergeysova.home",
  "serialNumber" : "0001",
  "teamIdentifier" : "CDHE9L6U22",
  "webServiceURL" : "https://example.com/passes/",
  "authenticationToken" : "vxwxd7J8AlNNFPS8k0a0FfUFtq0ewzFdc",
  "relevantDate" : "2012-07-22T14:25-08:00",
  "locations" : [
    {
      "longitude" : -122.3748889,
      "latitude" : 37.6189722
    }
  ],
  "barcode" : {
    "message" : "SFOJFK JOHN APPLESEED LH451 2012-07-22T14:25-08:00",
    "format" : "PKBarcodeFormatPDF417",
    "messageEncoding" : "iso-8859-1"
  },
  "organizationName" : "Skyport Airways",
  "description" : "Skyport Boarding Pass",
  "logoText" : "Skyport Airways",
  "foregroundColor" : "rgb(22, 55, 110)",
  "backgroundColor" : "rgb(50, 91, 185)",
  "boardingPass" : {
    "transitType" : "PKTransitTypeAir",
    "headerFields" : [
      {
        "label" : "GATE",
        "key" : "gate",
        "value" : "23",
        "changeMessage" : "Gate changed to %@."
      }
    ],
    "primaryFields" : [
      {
        "key" : "depart",
        "label" : "SAN FRANCISCO",
        "value" : "SFO"
      },
      {
        "key" : "arrive",
        "label" : "NEW YORK",
        "value" : "JFK"
      }
    ],
    "secondaryFields" : [
      {
        "key" : "passenger",
        "label" : "PASSENGER",
        "value" : "John Appleseed"
      }
    ],
    "auxiliaryFields" : [
      {
        "label" : "DEPART",
        "key" : "boardingTime",
        "value" :  "2:25 PM",
        "changeMessage" : "Boarding time changed to %@."
      },
      {
        "label" : "FLIGHT",
        "key" : "flightNewName",
        "value" : "815",
        "changeMessage" : "Flight number changed to %@"
      },
      {
        "key" : "class",
        "label" : "DESIG.",
        "value" : "Coach"
      },
      {
        "key" : "date",
        "label" : "DATE",
        "value" :  "7/22"
      }
    ],
    "backFields" : [
      {
        "key" : "passport",
        "label" : "PASSPORT",
        "value" : "Canadian/Canadien"
      },
      {
        "key" : "residence",
        "label" : "RESIDENCE",
        "value" : "999 Infinite Loop, Apartment 42, Cupertino CA"
      }
    ]
  }
}
        "#;

        let pass: Pass = serde_json::from_str(&src).unwrap();

        println!("{:#?}", pass);
    }
}
