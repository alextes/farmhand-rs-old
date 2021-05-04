use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum Base {
    Usd,
    Btc,
    Eth,
}

impl Display for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Base::Usd => write!(f, "usd"),
            Base::Btc => write!(f, "btc"),
            Base::Eth => write!(f, "eth"),
        }
    }
}

impl Serialize for Base {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            Base::Usd => "usd",
            Base::Btc => "btc",
            Base::Eth => "eth",
        })
    }
}

impl<'de> Deserialize<'de> for Base {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "usd" => Ok(Base::Usd),
            "btc" => Ok(Base::Btc),
            "eth" => Ok(Base::Eth),
            _ => Err(Error::custom("base currency should be usd, btc, or eth")),
        }
    }
}
