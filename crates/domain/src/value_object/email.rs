use std::{
    fmt::{self, Debug, Display},
    marker::PhantomData,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

#[derive(Clone)]
pub struct Email<T>(String, PhantomData<T>);

impl<T> Email<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into(), PhantomData)
    }
}

impl<T> AsRef<str> for Email<T> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<T> From<Email<T>> for String {
    fn from(from: Email<T>) -> Self {
        from.0
    }
}

impl<T> Debug for Email<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Display for Email<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
