// Based on common.xsd

// targetNamespace="http://www.onvif.org/ver10/schema"

// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:tt="http://www.onvif.org/ver10/schema"

use crate::utils;
use macro_utils::*;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

//generated file
// Unique identifier for a physical or logical resource.
// Tokens should be assigned such that they are unique within a device. Tokens
// must be at least unique within its class.
// Length up to 64 characters.
#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct ReferenceToken(pub String);

// Range of values greater equal Min value and less equal Max value.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IntRange {
    #[yaserde(prefix = "tt", rename = "Min")]
    pub min: i32,

    #[yaserde(prefix = "tt", rename = "Max")]
    pub max: i32,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Vector2D {
    #[yaserde(attribute, rename = "x")]
    pub x: f64,

    #[yaserde(attribute, rename = "y")]
    pub y: f64,

    // Pan/tilt coordinate space selector. The following options are defined:
    #[yaserde(attribute, rename = "space")]
    pub space: Option<String>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Vector1D {
    #[yaserde(attribute, rename = "x")]
    pub x: f64,

    // Zoom coordinate space selector. The following options are defined:
    #[yaserde(attribute, rename = "space")]
    pub space: Option<String>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ptzvector {
    // Pan and tilt position. The x component corresponds to pan and the y component
    // to tilt.
    #[yaserde(prefix = "tt", rename = "PanTilt")]
    pub pan_tilt: Option<Vector2D>,

    // A zoom position.
    #[yaserde(prefix = "tt", rename = "Zoom")]
    pub zoom: Option<Vector1D>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ptzstatus {
    // Specifies the absolute position of the PTZ unit together with the Space
    // references. The default absolute spaces of the corresponding PTZ
    // configuration MUST be referenced within the Position element.
    #[yaserde(prefix = "tt", rename = "Position")]
    pub position: Option<Ptzvector>,

    // Indicates if the Pan/Tilt/Zoom device unit is currently moving, idle or in an
    // unknown state.
    #[yaserde(prefix = "tt", rename = "MoveStatus")]
    pub move_status: Option<PtzmoveStatus>,

    // States a current PTZ error.
    #[yaserde(prefix = "tt", rename = "Error")]
    pub error: Option<String>,

    // Specifies the UTC time when this status was generated.
    #[yaserde(prefix = "tt", rename = "UtcTime")]
    pub utc_time: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzmoveStatus {
    #[yaserde(prefix = "tt", rename = "PanTilt")]
    pub pan_tilt: Option<MoveStatus>,

    #[yaserde(prefix = "tt", rename = "Zoom")]
    pub zoom: Option<MoveStatus>,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum MoveStatus {
    Idle,
    Moving,
    Unknown,

    __Unknown__(String),
}

impl Default for MoveStatus {
    fn default() -> MoveStatus {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Vector {
    #[yaserde(attribute, rename = "x")]
    pub x: Option<f64>,

    #[yaserde(attribute, rename = "y")]
    pub y: Option<f64>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Rectangle {
    #[yaserde(attribute, rename = "bottom")]
    pub bottom: Option<f64>,

    #[yaserde(attribute, rename = "top")]
    pub top: Option<f64>,

    #[yaserde(attribute, rename = "right")]
    pub right: Option<f64>,

    #[yaserde(attribute, rename = "left")]
    pub left: Option<f64>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Polygon {
    #[yaserde(prefix = "tt", rename = "Point")]
    pub point: Vec<Vector>,
}

// type Polygon = Polygon;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Color {
    #[yaserde(attribute, rename = "X")]
    pub x: f64,

    #[yaserde(attribute, rename = "Y")]
    pub y: f64,

    #[yaserde(attribute, rename = "Z")]
    pub z: f64,

    // Acceptable values:
    #[yaserde(attribute, rename = "Colorspace")]
    pub colorspace: Option<String>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ColorCovariance {
    #[yaserde(attribute, rename = "XX")]
    pub xx: f64,

    #[yaserde(attribute, rename = "YY")]
    pub yy: f64,

    #[yaserde(attribute, rename = "ZZ")]
    pub zz: f64,

    #[yaserde(attribute, rename = "XY")]
    pub xy: Option<f64>,

    #[yaserde(attribute, rename = "XZ")]
    pub xz: Option<f64>,

    #[yaserde(attribute, rename = "YZ")]
    pub yz: Option<f64>,

    // Acceptable values are the same as in tt:Color.
    #[yaserde(attribute, rename = "Colorspace")]
    pub colorspace: Option<String>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Transformation {
    #[yaserde(prefix = "tt", rename = "Translate")]
    pub translate: Option<Vector>,

    #[yaserde(prefix = "tt", rename = "Scale")]
    pub scale: Option<Vector>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<TransformationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct TransformationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct GeoLocation {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // East west location as angle.
    #[yaserde(attribute, rename = "lon")]
    pub lon: Option<f64>,

    // North south location as angle.
    #[yaserde(attribute, rename = "lat")]
    pub lat: Option<f64>,

    // Hight in meters above sea level.
    #[yaserde(attribute, rename = "elevation")]
    pub elevation: Option<f64>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct GeoOrientation {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Rotation around the x axis.
    #[yaserde(attribute, rename = "roll")]
    pub roll: Option<f64>,

    // Rotation around the y axis.
    #[yaserde(attribute, rename = "pitch")]
    pub pitch: Option<f64>,

    // Rotation around the z axis.
    #[yaserde(attribute, rename = "yaw")]
    pub yaw: Option<f64>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct LocalLocation {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // East west location as angle.
    #[yaserde(attribute, rename = "x")]
    pub x: Option<f64>,

    // North south location as angle.
    #[yaserde(attribute, rename = "y")]
    pub y: Option<f64>,

    // Offset in meters from the sea level.
    #[yaserde(attribute, rename = "z")]
    pub z: Option<f64>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct LocalOrientation {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Rotation around the y axis.
    #[yaserde(attribute, rename = "pan")]
    pub pan: Option<f64>,

    // Rotation around the z axis.
    #[yaserde(attribute, rename = "tilt")]
    pub tilt: Option<f64>,

    // Rotation around the x axis.
    #[yaserde(attribute, rename = "roll")]
    pub roll: Option<f64>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Entity {
    Device,
    VideoSource,
    AudioSource,

    __Unknown__(String),
}

impl Default for Entity {
    fn default() -> Entity {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct LocationEntity {
    // Location on earth.
    #[yaserde(prefix = "tt", rename = "GeoLocation")]
    pub geo_location: Option<GeoLocation>,

    // Orientation relative to earth.
    #[yaserde(prefix = "tt", rename = "GeoOrientation")]
    pub geo_orientation: Option<GeoOrientation>,

    // Indoor location offset.
    #[yaserde(prefix = "tt", rename = "LocalLocation")]
    pub local_location: Option<LocalLocation>,

    // Indoor orientation offset.
    #[yaserde(prefix = "tt", rename = "LocalOrientation")]
    pub local_orientation: Option<LocalOrientation>,

    // Entity type the entry refers to, use a value from the tt:Entity enumeration.
    #[yaserde(attribute, rename = "Entity")]
    pub entity: Option<String>,

    // Optional entity token.
    #[yaserde(attribute, rename = "Token")]
    pub token: Option<ReferenceToken>,

    // If this value is true the entity cannot be deleted.
    #[yaserde(attribute, rename = "Fixed")]
    pub fixed: Option<bool>,

    // Optional reference to the XAddr of another devices DeviceManagement service.
    #[yaserde(attribute, rename = "GeoSource")]
    pub geo_source: Option<String>,

    // If set the geo location is obtained internally.
    #[yaserde(attribute, rename = "AutoGeo")]
    pub auto_geo: Option<bool>,
}