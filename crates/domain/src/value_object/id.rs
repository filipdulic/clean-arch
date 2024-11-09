//! A generalised ID type for entities and aggregates.

use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use uuid::Uuid;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct Id<T> {
    id: Uuid,
    // The `fn() -> T` is a trick to tell the compiler that we don't own anything.
    marker: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self {
            id: id.into(),
            marker: PhantomData,
        }
    }
}

impl<T> From<Id<T>> for Uuid {
    fn from(id: Id<T>) -> Self {
        id.id
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(hasher)
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(from: Uuid) -> Self {
        Self::new(from)
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_id() {
        let id: Id<()> = Id::new(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
        assert_eq!(format!("{}", id), "550e8400-e29b-41d4-a716-446655440000");
    }
}
