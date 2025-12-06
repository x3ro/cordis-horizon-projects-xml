use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

static DATE_FORMAT: &'static str = "%Y-%m-%d";
static DATETIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug, PartialEq, Clone)]
pub struct CordisDate(NaiveDate);

impl<'de> Deserialize<'de> for CordisDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let dt = NaiveDate::parse_from_str(&s, DATE_FORMAT).map_err(serde::de::Error::custom)?;
        Ok(CordisDate(dt))
    }
}

impl Serialize for CordisDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}", self.0.format(DATE_FORMAT));
        serializer.serialize_str(&s)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CordisDateTime(DateTime<Utc>);

impl<'de> Deserialize<'de> for CordisDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt =
            NaiveDateTime::parse_from_str(&s, DATETIME_FORMAT).map_err(serde::de::Error::custom)?;
        Ok(CordisDateTime(DateTime::<Utc>::from_naive_utc_and_offset(
            dt, Utc,
        )))
    }
}

impl Serialize for CordisDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}", self.0.format(DATETIME_FORMAT));
        serializer.serialize_str(&s)
    }
}
