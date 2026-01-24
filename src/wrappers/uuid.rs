use std::{borrow::Cow, str::FromStr};

use serde_lite::{Deserialize, Error, Intermediate, Serialize};
use uuid::Uuid;

// Wrapper around Uuid
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct UuidWrapper(pub Uuid);

impl Serialize for UuidWrapper {
    fn serialize(&self) -> Result<Intermediate, Error> {
        // Convert `Uuid` to a hyphenated string format
        let uuid_str = self.0.hyphenated().to_string();
        Ok(Intermediate::String(Cow::Owned(uuid_str)))
    }
}

impl Deserialize for UuidWrapper {
    fn deserialize(intermediate: &Intermediate) -> Result<Self, Error> {
        // Expect the intermediate to be a string and parse it to `Uuid`
        if let Intermediate::String(uuid_str) = intermediate {
            Uuid::from_str(uuid_str)
                .map(UuidWrapper)
                .map_err(|_| Error::custom("Invalid UUID format"))
        } else {
            Err(Error::custom("Expected a string for Uuid"))
        }
    }
}

// Conversion from Uuid to UuidWrapper
impl From<Uuid> for UuidWrapper {
    fn from(uuid: Uuid) -> Self {
        UuidWrapper(uuid)
    }
}

// Conversion from UuidWrapper to Uuid
impl From<UuidWrapper> for Uuid {
    fn from(wrapper: UuidWrapper) -> Self {
        wrapper.0
    }
}
