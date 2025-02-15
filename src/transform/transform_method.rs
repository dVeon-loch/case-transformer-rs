use serde::{Deserialize, Serialize};

pub enum TransformMethod {
    UPPER,
    LOWER
}

impl<'de> Deserialize<'de> for TransformMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.to_lowercase().as_str() {
            "uppercase" => Ok(TransformMethod::UPPER),
            "lowercase" => Ok(TransformMethod::LOWER),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid TransformMethod: `{}`. Expected `uppercase` or `lowercase`",
                s
            ))),
        }
    }
}

impl Serialize for TransformMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Map the enum variants to their corresponding string values
        match self {
            TransformMethod::UPPER => serializer.serialize_str("uppercase"),
            TransformMethod::LOWER => serializer.serialize_str("lowercase"),
        }
    }
}