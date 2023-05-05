use std::fmt::{Display, Formatter};
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a `Payload` structure from anything that can be a `Value`,
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    inner: Value,
}

impl Payload {
    /// Constructs a payload from `Value`
    ///
    /// # Arguments
    ///
    /// * `payload` - Anything that can be a `Value`.
    ///
    /// # Examples
    ///
    /// Create a `Payload` from a given `Value`:
    /// ```
    /// use std::ops::Deref;
    /// use axon::message::payload::Payload;
    ///
    /// let payload = Payload::new("Some value");
    ///
    /// assert_eq!("Some value",  payload.deref().as_str().unwrap());
    /// assert_eq!("\"Some value\"", format!("{payload}"));
    pub fn new(payload: impl Into<Value>) -> Self {
        Payload {
            inner: payload.into(),
        }
    }
}

impl Deref for Payload {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Display for Payload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod test_payload {
    use crate::message::payload::Payload;

    #[test]
    fn simple_payload() {
        let payload = Payload::new(123);

        assert!(payload.is_number());
        assert_eq!(payload.as_i64().unwrap(), 123);
    }
}
