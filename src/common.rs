use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use chrono::{DateTime, Utc};
use core::fmt::Write;

#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;

pub type HeaderId = u32;
pub type Timestamp = DateTime<Utc>;

/// Current position of the AGV on the map. Optional: Can only be omitted for AGVs without the capability to localize themselves, e.g. line guided AGVs.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
pub struct AgvPosition {
    /// X-position on the map in reference to the map coordinate system. Precision is up to the specific implementation.
    pub x: f64,
    /// Y-position on the map in reference to the map coordinate system. Precision is up to the specific implementation.
    pub y: f64,
    /// Range: \[-pi..pi\] Orientation of the AGV.
    pub theta: f64,
    /// Unique identification of the map in which the position is referenced. Each map has the same origin of coordinates. When an AGV uses an elevator, e.g. leading from a departure floor to a target floor, it will disappear off the map of the departure floor and spawn in the related lift node on the map of the target floor.
    pub map_id: String,
    /// Additional information on the map.
    pub map_description: Option<String>,
    /// True if the AGVs position is initialized, false, if position is not initialized.
    pub position_initialized: bool,
    /// Describes the quality of the localization and therefore, can be used e.g. by SLAM-AGVs to describe how accurate the current position information is. 0.0: position unknown 1.0: position known Optional for vehicles that cannot estimate their localization score. Only for logging and visualization purposes
    pub localization_score: Option<f64>,
    /// Value for the deviation range of the position in meters. Optional for vehicles that cannot estimate their deviation e.g. grid-based localization. Only for logging and visualization purposes.
    pub deviation_range: Option<f64>,
}

/// This point describes the loads position on the AGV in the vehicle coordinates. The bounding_box_reference point is in the middle of the footprint of the load, so length/2 and width/2.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
pub struct BoundingBoxReference {
    /// x-coordinate of the point of reference.
    pub x: f64,
    /// y-coordinate of the point of reference.
    pub y: f64,
    /// z-coordinate of the point of reference.
    pub z: f64,
    /// Orientation of the loads bounding box. Important for tugger trains etc.
    pub theta: Option<f64>,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
pub struct ControlPoint {
    /// X coordinate described in the world coordinate system.
    pub x: f64,
    /// Y coordinate described in the world coordinate system.
    pub y: f64,
    /// Range: (0..Infinity). The weight with which this control point pulls on the curve. When not defined, the default will be 1.0.
    pub weight: Option<f64>,
    /// Range: \[-pi..pi\]. Orientation of the AGV on this position of the curve. The orientation is in world coordinates. When not defined the orientation of the AGV will be tangential to the curve.
    pub orientation: Option<f64>,
}

/// Dimensions of the load's bounding box in meters.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
pub struct LoadDimensions {
    /// Absolute length of the loads bounding box in meter.
    pub length: f64,
    /// Absolute width of the loads bounding box in meter.
    pub width: f64,
    /// Absolute height of the loads bounding box in meter. Optional: Set value only if known.
    pub height: Option<f64>,
}

/// Node position. The object is defined in chapter 6.6. Optional: master control has this information. Can be sent additionally, e.g. for debugging purposes.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
pub struct NodePosition {
    /// X coordinate described in the world coordinate system.
    pub x: f64,
    /// Y coordinate described in the world coordinate system.
    pub y: f64,
    /// Range: \[-pi..pi\]. Orientation of the AGV on the node. Optional: vehicle can plan the path by itself. If defined, the AGV has to assume the theta angle on this node. If previous edge disallows rotation, the AGV is to rotate on the node. If following edge has a differing orientation defined but disallows rotation, the AGV is to rotate on the node to the edges desired rotation before entering the edge.
    pub theta: Option<f64>,
    /// Indicates how exact an AGV has to drive over a node in order for it to count as traversed. If = 0: no deviation is allowed (no deviation means within the normal tolerance of the AGV manufacturer). If > 0: allowed deviation-radius in meters. If the AGV passes a node within the deviation-radius, the node is considered to have been traversed.
    pub allowed_deviation_xy: Option<f64>,
    /// Indicates how big the deviation of theta angle can be. The lowest acceptable angle is theta - allowed_deviation_theta and the highest acceptable angle is theta + allowed_deviation_theta. If = 0: no deviation is allowed (no deviation means within the normal tolerance of the AGV manufacturer).
    pub allowed_deviation_theta: Option<f64>,
    /// Unique identification of the map in which the position is referenced.
    /// Each map has the same origin of coordinates. When an AGV uses an elevator,
    /// e.g. leading from a departure floor to a target floor, it will disappear
    /// off the map of the departure floor and spawn in the related lift node on
    /// the map of the target floor.
    pub map_id: String,
    /// Verbose description of the Map.
    pub map_description: Option<String>,
}

/// The trajectory is to be communicated as a NURBS and is defined in chapter 6.4. Trajectory segments are from the point where the AGV starts to enter the edge until the point where it reports that the next node was traversed.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
pub struct Trajectory {
    /// Defines the number of control points that influence any given point on the curve. Increasing the degree increases continuity. If not defined, the default value is 1.
    pub degree: f64,
    /// Sequence of parameter values that determine where and how the control points affect the NURBS curve. knot_vector has size of number of control points + degree + 1
    pub knot_vector: Vec<f64>,
    /// List of JSON controlPoint objects defining the control points of the NURBS. This includes the start and end point.
    pub control_points: Vec<ControlPoint>,
}

/// The AGVs velocity in vehicle coordinates.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
pub struct Velocity {
    /// The AGVs velocity in its x direction.
    pub vx: Option<f64>,
    /// The AGVs velocity in its y direction.
    pub vy: Option<f64>,
    /// The AGVs turning speed around its z axis.
    pub omega: Option<f64>,
}

/// ActionParameter Object
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
pub struct ActionParameter {
    /// key-String for Parameter
    pub key: String,
    /// data type of Value, possible data types are: BOOL, NUMBER, INTEGER, FLOAT, STRING, OBJECT, ARRAY
    pub value_data_type: Option<ValueDataType>,
    /// value of the parameter, type determined by value_data_type
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_parameter_value")
    )]
    pub value: ParameterValue,
    /// free text: description of the parameter
    pub description: Option<String>,
    /// True: optional parameter
    pub is_optional: Option<bool>,
}

impl Default for ActionParameter {
    fn default() -> Self {
        Self {
            key: String::new(),
            value_data_type: None,
            value: ParameterValue::Null,
            description: None,
            is_optional: None,
        }
    }
}

/// Data type of Value.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum ValueDataType {
    Bool,
    Number,
    Integer,
    Float,
    String,
    Object,
    Array,
}

/// Parameter value that can hold any type as determined by ValueDataType.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(untagged)
)]
pub enum ParameterValue {
    Null,
    Bool(bool),
    Number(f64),
    Integer(i64),
    Float(f64),
    String(String),
    #[cfg(feature = "serde")]
    Object(serde_json::Value),
    #[cfg(feature = "serde")]
    Array(Vec<serde_json::Value>),
    #[cfg(not(feature = "serde"))]
    Object(String), // JSON string representation when serde is not available
    #[cfg(not(feature = "serde"))]
    Array(Vec<String>), // JSON string array representation when serde is not available
}

impl ParameterValue {
    /// Get the internal value as a string representation.
    /// This method provides a unified way to access the value regardless of the variant.
    pub fn get_value(&self) -> String {
        match self {
            ParameterValue::Null => "null".to_string(),
            ParameterValue::Bool(b) => {
                let mut s = String::new();
                write!(s, "{}", b).unwrap();
                s
            }
            ParameterValue::Number(n) => {
                let mut s = String::new();
                write!(s, "{}", n).unwrap();
                s
            }
            ParameterValue::Integer(i) => {
                let mut s = String::new();
                write!(s, "{}", i).unwrap();
                s
            }
            ParameterValue::Float(f) => {
                let mut s = String::new();
                write!(s, "{}", f).unwrap();
                s
            }
            ParameterValue::String(s) => s.clone(),
            #[cfg(feature = "serde")]
            ParameterValue::Object(obj) => {
                let mut s = String::new();
                write!(s, "{}", obj).unwrap();
                s
            }
            #[cfg(feature = "serde")]
            ParameterValue::Array(arr) => {
                let mut s = String::new();
                write!(s, "[").unwrap();
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(s, ", ").unwrap();
                    }
                    write!(s, "{}", v).unwrap();
                }
                write!(s, "]").unwrap();
                s
            }
            #[cfg(not(feature = "serde"))]
            ParameterValue::Object(s) => s.clone(),
            #[cfg(not(feature = "serde"))]
            ParameterValue::Array(arr) => {
                let mut s = String::new();
                write!(s, "[").unwrap();
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(s, ", ").unwrap();
                    }
                    write!(s, "{}", item).unwrap();
                }
                write!(s, "]").unwrap();
                s
            }
        }
    }

    /// Get the internal value as a string representation with proper formatting.
    /// This method provides a unified way to access the value regardless of the variant.
    pub fn get_internal_value(&self) -> String {
        self.get_value()
    }

    /// Get the boolean value if this is a Bool variant.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ParameterValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Get the number value if this is a Number variant.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            ParameterValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Get the integer value if this is an Integer variant.
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            ParameterValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Get the float value if this is a Float variant.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            ParameterValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Get the string value if this is a String variant.
    pub fn as_string(&self) -> Option<&String> {
        match self {
            ParameterValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get the object value if this is an Object variant.
    #[cfg(feature = "serde")]
    pub fn as_object(&self) -> Option<&serde_json::Value> {
        match self {
            ParameterValue::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Get the object value as a string if this is an Object variant (when serde is not available).
    #[cfg(not(feature = "serde"))]
    pub fn as_object_string(&self) -> Option<&String> {
        match self {
            ParameterValue::Object(s) => Some(s),
            _ => None,
        }
    }

    /// Get the array value if this is an Array variant.
    #[cfg(feature = "serde")]
    pub fn as_array(&self) -> Option<&Vec<serde_json::Value>> {
        match self {
            ParameterValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Get the array value as a string vector if this is an Array variant (when serde is not available).
    #[cfg(not(feature = "serde"))]
    pub fn as_array_strings(&self) -> Option<&Vec<String>> {
        match self {
            ParameterValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Check if this is a null value.
    pub fn is_null(&self) -> bool {
        matches!(self, ParameterValue::Null)
    }
}

#[cfg(feature = "serde")]
fn deserialize_parameter_value<'de, D>(deserializer: D) -> Result<ParameterValue, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Value;

    impl<'de> serde::de::Visitor<'de> for Value {
        type Value = ParameterValue;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            formatter.write_str("null, boolean, integer, float, string, object, or array")
        }

        fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
            Ok(ParameterValue::Bool(value))
        }

        fn visit_i8<E: serde::de::Error>(self, value: i8) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_i16<E: serde::de::Error>(self, value: i16) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_i32<E: serde::de::Error>(self, value: i32) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
            Ok(ParameterValue::Integer(value))
        }

        fn visit_u8<E: serde::de::Error>(self, value: u8) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_u16<E: serde::de::Error>(self, value: u16) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_u32<E: serde::de::Error>(self, value: u32) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_f32<E: serde::de::Error>(self, value: f32) -> Result<Self::Value, E> {
            self.visit_f64(value as f64)
        }

        fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
            Ok(ParameterValue::Float(value))
        }

        fn visit_char<E: serde::de::Error>(self, value: char) -> Result<Self::Value, E> {
            Ok(ParameterValue::String(String::from(value)))
        }

        fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
            self.visit_string(value.to_owned())
        }

        fn visit_borrowed_str<E: serde::de::Error>(
            self,
            value: &'de str,
        ) -> Result<Self::Value, E> {
            self.visit_string(value.to_owned())
        }

        fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
            Ok(ParameterValue::String(value))
        }

        fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(ParameterValue::Null)
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(ParameterValue::Object(serde::de::Deserialize::deserialize(
                serde::de::value::MapAccessDeserializer::new(map),
            )?))
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            Ok(ParameterValue::Array(serde::de::Deserialize::deserialize(
                serde::de::value::SeqAccessDeserializer::new(seq),
            )?))
        }
    }

    deserializer.deserialize_any(Value)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::{ActionParameter, ParameterValue, ValueDataType};
    use alloc::string::String;
    use googletest::prelude::*;
    use rstest::rstest;

    #[cfg(feature = "serde")]
    #[rstest]
    fn test_serde_ActionParameter_with_null_value() {
        let parameter = ActionParameter {
            key: String::from("my-null"),
            value_data_type: None,
            value: ParameterValue::Null,
            description: None,
            is_optional: None,
        };

        let json = r#"{"key":"my-null","valueDataType":null,"value":null,"description":null,"isOptional":null}"#;

        let to = serde_json::to_string(&parameter);
        let from = serde_json::from_str::<ActionParameter>(&json);

        assert_that!(to, ok(eq(json)));

        assert_that!(
            from,
            ok(matches_pattern!(ActionParameter {
                key: eq("my-null"),
                value_data_type: eq(&None),
                value: eq(&ParameterValue::Null),
                description: eq(&None),
                is_optional: eq(&None)
            }))
        );
    }

    #[cfg(feature = "serde")]
    #[rstest]
    fn test_serde_ActionParameter_with_bool_value() {
        let parameter = ActionParameter {
            key: String::from("my-bool"),
            value_data_type: Some(ValueDataType::Bool),
            value: ParameterValue::Bool(true),
            description: None,
            is_optional: None,
        };

        let json = r#"{"key":"my-bool","valueDataType":"BOOL","value":true,"description":null,"isOptional":null}"#;

        let to = serde_json::to_string(&parameter);
        let from = serde_json::from_str::<ActionParameter>(&json);

        assert_that!(to, ok(eq(json)));

        assert_that!(
            from,
            ok(matches_pattern!(ActionParameter {
                key: eq("my-bool"),
                value_data_type: eq(&Some(ValueDataType::Bool)),
                value: eq(&ParameterValue::Bool(true)),
                description: eq(&None),
                is_optional: eq(&None)
            }))
        );
    }

    #[cfg(feature = "serde")]
    #[rstest]
    fn test_deserialize_ActionParameter_with_integer_value() {
        let parameter = ActionParameter {
            key: String::from("my-integer"),
            value_data_type: Some(ValueDataType::Integer),
            value: ParameterValue::Integer(42),
            description: None,
            is_optional: None,
        };

        let json = r#"{"key":"my-integer","valueDataType":"INTEGER","value":42,"description":null,"isOptional":null}"#;

        let to = serde_json::to_string(&parameter);
        let from = serde_json::from_str::<ActionParameter>(&json);

        assert_that!(to, ok(eq(json)));

        assert_that!(
            from,
            ok(matches_pattern!(ActionParameter {
                key: eq("my-integer"),
                value_data_type: eq(&Some(ValueDataType::Integer)),
                value: eq(&ParameterValue::Integer(42)),
                description: eq(&None),
                is_optional: eq(&None)
            }))
        );
    }

    #[cfg(feature = "serde")]
    #[rstest]
    fn test_deserialize_ActionParameter_with_float_value() {
        let parameter = ActionParameter {
            key: String::from("my-float"),
            value_data_type: Some(ValueDataType::Float),
            value: ParameterValue::Float(42.73),
            description: None,
            is_optional: None,
        };

        let json = r#"{"key":"my-float","valueDataType":"FLOAT","value":42.73,"description":null,"isOptional":null}"#;

        let to = serde_json::to_string(&parameter);
        let from = serde_json::from_str::<ActionParameter>(&json);

        assert_that!(to, ok(eq(json)));

        assert_that!(
            from,
            ok(matches_pattern!(ActionParameter {
                key: eq("my-float"),
                value_data_type: eq(&Some(ValueDataType::Float)),
                value: eq(&ParameterValue::Float(42.73)),
                description: eq(&None),
                is_optional: eq(&None)
            }))
        );
    }

    #[cfg(feature = "serde")]
    #[rstest]
    fn test_deserialize_ActionParameter_with_string_value() {
        let parameter = ActionParameter {
            key: String::from("my-string"),
            value_data_type: Some(ValueDataType::String),
            value: ParameterValue::String(String::from("Hello World")),
            description: None,
            is_optional: None,
        };

        let json = r#"{"key":"my-string","valueDataType":"STRING","value":"Hello World","description":null,"isOptional":null}"#;

        let to = serde_json::to_string(&parameter);
        let from = serde_json::from_str::<ActionParameter>(&json);

        assert_that!(to, ok(eq(json)));

        assert_that!(
            from,
            ok(matches_pattern!(ActionParameter {
                key: eq("my-string"),
                value_data_type: eq(&Some(ValueDataType::String)),
                value: eq(&ParameterValue::String(String::from("Hello World"))),
                description: eq(&None),
                is_optional: eq(&None)
            }))
        );
    }

    #[cfg(feature = "serde")]
    #[rstest]
    fn test_clone_functionality() {
        let parameter = ActionParameter {
            key: String::from("test-key"),
            value_data_type: Some(ValueDataType::String),
            value: ParameterValue::String(String::from("test-value")),
            description: None,
            is_optional: None,
        };

        // Test Clone
        let cloned_parameter = parameter.clone();
        assert_eq!(parameter.key, cloned_parameter.key);
    }

    #[rstest]
    fn test_parameter_value_get_value() {
        // Test Null
        let null_value = ParameterValue::Null;
        assert_eq!(null_value.get_value(), "null");
        assert!(null_value.is_null());

        // Test Bool
        let bool_value = ParameterValue::Bool(true);
        assert_eq!(bool_value.get_value(), "true");
        assert_eq!(bool_value.as_bool(), Some(true));

        // Test Integer
        let int_value = ParameterValue::Integer(42);
        assert_eq!(int_value.get_value(), "42");
        assert_eq!(int_value.as_integer(), Some(42));

        // Test Float
        let float_value = ParameterValue::Float(3.14159);
        assert_eq!(float_value.get_value(), "3.14159");
        assert_eq!(float_value.as_float(), Some(3.14159));

        // Test String
        let string_value = ParameterValue::String(String::from("hello"));
        assert_eq!(string_value.get_value(), "hello");
        assert_eq!(string_value.as_string(), Some(&String::from("hello")));
    }

    #[rstest]
    fn test_parameter_value_type_checking() {
        let bool_value = ParameterValue::Bool(false);
        let int_value = ParameterValue::Integer(100);
        let string_value = ParameterValue::String(String::from("test"));

        // Test type checking
        assert_eq!(bool_value.as_bool(), Some(false));
        assert_eq!(bool_value.as_integer(), None);
        assert_eq!(bool_value.as_string(), None);

        assert_eq!(int_value.as_integer(), Some(100));
        assert_eq!(int_value.as_bool(), None);
        assert_eq!(int_value.as_string(), None);

        assert_eq!(string_value.as_string(), Some(&String::from("test")));
        assert_eq!(string_value.as_bool(), None);
        assert_eq!(string_value.as_integer(), None);
    }
}
