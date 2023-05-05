use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ddd::constant::ENTITY_ANONYMOUS;

/// Axon Resource Name.
///
/// `Arn` is similar in structure to Amazon Resource Names (ARN).
/// https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html
/// Implement this trait on any resource like a `Message` to encode metadata as parts.
///
/// # Examples
///
/// ```
/// use axon::ddd::entity::Arn;
///
/// struct EmailReceived {
///     from: String,
///     to: String,
///     body: String,
///     id: usize
/// }
///
/// impl Arn for EmailReceived {
///     fn arn_string(&self) -> String {
///         format!("axon:email:from-{}:to-{}/{}", self.from, self.to, self.id)
///     }
///
///     fn arn_name(&self) -> String {
///         "email".to_owned()
///     }
/// }
/// ```
pub trait Arn {
    /// Returns `String` representation of `ARN`
    fn arn_string(&self) -> String;

    /// Return name of resource
    fn arn_name(&self) -> String;
}

/// `Entity` struct defines properties of a unique element.
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Entity {
    /// Unique identifier of the entity
    id: String,
    /// Name of the entity
    name: String,
}

impl Entity {
    /// Constructs a new `Entity` from given id and name.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier.
    /// * `name` - Name of the `Entity`. Does not need to be unique.
    ///
    /// # Examples
    ///
    /// Create an `Entity` for an event called "SomeEvent" with id "123":
    /// ```
    /// use axon::ddd::entity::Entity;
    ///
    /// let entity = Entity::new("123", "SomeEvent");
    ///
    /// assert_eq!("SomeEvent:123", format!("{entity}"));
    /// assert_eq!("123", entity.id());
    /// assert_eq!("SomeEvent", entity.name());
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Entity {
            id: id.into(),
            name: name.into(),
        }
    }

    /// Constructs a new `Entity` from given  name. Id will be a randomly
    /// generated `Uuid`.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the `Entity`. Does not need to be unique.
    ///
    /// # Examples
    ///
    /// Create an `Entity` for an event called "SomeEvent":
    /// ```
    /// use axon::ddd::entity::Entity;
    ///
    /// let entity = Entity::from_name("SomeEvent");
    ///
    /// assert_eq!("SomeEvent", entity.name());
    pub fn from_name(name: impl Into<String>) -> Self {
        Entity {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
        }
    }

    /// Unique id of `Entity`
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// Name of `Entity`
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            id: Uuid::new_v4().to_string(),
            name: ENTITY_ANONYMOUS.to_owned(),
        }
    }
}

impl From<String> for Entity {
    fn from(value: String) -> Self {
        Entity::from_name(value)
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.id)
    }
}

impl Arn for Entity {
    fn arn_string(&self) -> String {
        format!("arn:{}/{}", self.name, self.id)
    }

    fn arn_name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod test_entity {
    use uuid::Uuid;

    use crate::ddd::constant::ENTITY_ANONYMOUS;
    use crate::ddd::entity::{Arn, Entity};

    #[test]
    fn test_arn() {
        let id = Uuid::new_v4().to_string();
        let entity = Entity::new(id, "hello-event");

        assert!(!entity.arn_string().is_empty());
        assert_eq!("hello-event", entity.arn_name());
    }

    #[test]
    fn default_entity() {
        let e = Entity::default();

        assert_eq!(e.name, ENTITY_ANONYMOUS);
        assert!(!e.id.is_empty());
    }

    #[test]
    fn new_entity() {
        let id = Uuid::new_v4().to_string();
        let e = Entity::new(id.clone(), "hello");

        assert_eq!(e.name, "hello");
        assert_eq!(e.id, id);
    }
}
