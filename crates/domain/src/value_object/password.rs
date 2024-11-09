use std::{
    fmt::{self, Debug, Display},
    marker::PhantomData,
};
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Password<T>(String, PhantomData<T>);

impl<T> Password<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into(), PhantomData)
    }
}

impl<T> AsRef<str> for Password<T> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<T> From<Password<T>> for String {
    fn from(from: Password<T>) -> Self {
        from.0
    }
}

impl<T> Debug for Password<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Display for Password<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
