use serde::{de::Error, Deserializer, Serializer};

// --- SERIALIZE ---
pub fn serialize<S>(value: &i128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // For JSON, serialize as a string
    serializer.collect_str(value)
}

// --- DESERIALIZE ---
// The visitor needs to be able to handle both a string and a native i128.
struct I128Visitor;

impl<'de> serde::de::Visitor<'de> for I128Visitor {
    type Value = i128;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string containing an i128 or an i128")
    }

    // Handle the string case
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        v.parse::<i128>().map_err(E::custom)
    }

    // Handle the native i128 case
    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<i128, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(I128Visitor)
}
