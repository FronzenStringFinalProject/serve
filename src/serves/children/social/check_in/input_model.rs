use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct SpecMonth {
    pub month: Option<Month>,
}

#[derive(Debug)]
pub struct Month(pub i32);

impl<'de> Deserialize<'de> for Month {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let month = i32::deserialize(deserializer)?;
        if (1..=12).contains(&month) {
            Ok(Self(month))
        } else {
            Err(serde::de::Error::custom("out of Month[1-12] range"))
        }
    }
}
