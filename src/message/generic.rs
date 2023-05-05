use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ddd::entity::Entity;
use crate::ddd::metadata::MetaData;
use crate::message::base::Message;
use crate::message::payload::Payload;

/// `EventMessage` is a generic event message.
///
/// It can be used to represent event messages that are not domain specific. Domain specific event
/// messages should have their structs.
///
/// # Examples
///
/// ```
/// use axon::message::generic::EventMessage;
///
/// // Typed domain event message
/// struct MyDomainEventMessage;
///
/// // Not typed event message with empty payload
/// let message = EventMessage::new("SomethingHappened", Option::<String>::None);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventMessage {
    /// Unique identifier of `EventMessage`
    identifier: Entity,
    /// MetaData of the `EventMessage`
    metadata: MetaData,
    /// Payload of the `EventMessage`
    payload: Payload,
}

impl EventMessage {
    /// Constructs a new `EventMessage` from given event name and payload.
    ///
    /// `MataData` field of `EventMessage` will be constructed from the `Entity`
    /// struct which identifies the instance of the `EventMessage`.
    ///
    /// # Arguments
    ///
    /// * `event_name` - Name of the event
    /// * `payload` - `Payload` of the event
    ///
    /// # Examples
    ///
    /// Create an `EventMessage` with name and payload:
    ///
    /// ```
    /// use axon::message::base::Message;
    /// use axon::message::generic::EventMessage;
    ///
    /// let message = EventMessage::new("SomeMessage", "Message Contents");
    ///
    /// assert!(format!("{message}").contains("SomeMessage"));
    /// ```
    pub fn new(event_name: impl Into<String>, payload: impl Into<Value>) -> Self {
        let identifier = Entity::from_name(event_name);
        let metadata = identifier.as_metadata();

        EventMessage {
            identifier,
            metadata,
            payload: Payload::new(payload),
        }
    }

    pub fn set_identifier(mut self, identifier: impl Into<Entity>) -> Self {
        self.identifier = identifier.into();
        self
    }

    pub fn add_meta(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.metadata.add(key.into(), value);
        self
    }
}

impl Message for EventMessage {
    fn identifier(&self) -> Entity {
        self.identifier.clone()
    }

    fn metadata(&self) -> MetaData {
        self.metadata.clone()
    }

    fn payload(&self) -> Payload {
        self.payload.clone()
    }
}

impl Display for EventMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

#[cfg(test)]
mod message_test {
    use crate::ddd::entity::Entity;
    use crate::message::base::Message;
    use crate::message::generic::EventMessage;
    use crate::message::payload::Payload;

    #[test]
    fn build_generic_event_message() {
        let expected_id = Entity::new("id", "name");
        let expected_payload = Payload::new("my payload");

        let event_message =
            EventMessage::new("some_event", "my payload").set_identifier(expected_id.clone());

        assert_eq!(event_message.identifier(), expected_id);
        assert_eq!(event_message.payload(), expected_payload);
    }
}
