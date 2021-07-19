use std::fmt;
use std::str::FromStr;
use std::str::ParseBoolError;
use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;

#[derive(Debug)]
pub enum Boolean {
    True,
    False
}
impl Serialize for Boolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            Boolean::True => serializer.serialize_bool(true),
            Boolean::False => serializer.serialize_bool(false)
        }
    }
}
impl<'de> Deserialize<'de> for Boolean {
    fn deserialize<D>(deserializer: D) -> Result<Boolean, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct BooleanVisitor;
        impl<'de> Visitor<'de> for BooleanVisitor {
            type Value = Boolean;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a boolean value is required")
            }
            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
                where
                    E: de::Error,
            {
                match value {
                    true => Ok(Boolean::True),
                    false => Ok(Boolean::False),
                }
            }
        }
        deserializer.deserialize_identifier(BooleanVisitor)
    }
}
impl FromStr for Boolean {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let boolean_string = s.trim().to_lowercase();
        bool::from_str(boolean_string.as_str())
            .map(|value| match value {
                true => Boolean::True,
                false => Boolean::False,
            })
    }
}