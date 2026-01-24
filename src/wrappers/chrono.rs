use chrono::NaiveDate;
use serde_lite::{Deserialize, Error, Intermediate, Serialize};
use std::borrow::Cow;
use std::str::FromStr;

// Wrapper around NaiveDate
#[derive(PartialEq, Eq, Hash, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NaiveDateWrapper(pub NaiveDate);

impl Serialize for NaiveDateWrapper {
    fn serialize(&self) -> Result<Intermediate, Error> {
        // Convert `NaiveDate` to ISO 8601 date string (YYYY-MM-DD)
        let date_str = self.0.format("%Y-%m-%d").to_string();
        Ok(Intermediate::String(Cow::Owned(date_str)))
    }
}

impl Deserialize for NaiveDateWrapper {
    fn deserialize(intermediate: &Intermediate) -> Result<Self, Error> {
        // Expect the intermediate to be a string and parse it to `NaiveDate`
        if let Intermediate::String(date_str) = intermediate {
            NaiveDate::from_str(&date_str)
                .map(NaiveDateWrapper)
                .map_err(|_| Error::custom("Invalid date format"))
        } else {
            Err(Error::custom("Expected a string for NaiveDate"))
        }
    }
}

// Conversion from NaiveDate to NaiveDateWrapper
impl From<NaiveDate> for NaiveDateWrapper {
    fn from(date: NaiveDate) -> Self {
        NaiveDateWrapper(date)
    }
}

// Conversion from NaiveDateWrapper to NaiveDate
impl From<NaiveDateWrapper> for NaiveDate {
    fn from(wrapper: NaiveDateWrapper) -> Self {
        wrapper.0
    }
}
