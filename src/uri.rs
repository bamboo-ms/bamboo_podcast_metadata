use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

pub struct Uri(http::Uri);

impl ToString for Uri {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for Uri {
    type Err = <http::Uri as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Uri(http::Uri::from_str(s)?))
    }
}

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;

        Ok(Uri::from_str(&buf).map_err(serde::de::Error::custom)?)
    }
}
