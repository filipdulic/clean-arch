use std::{
    fmt::{self, Debug, Display},
    marker::PhantomData,
};

#[derive(Clone)]
pub struct Email<T>(String, PhantomData<T>);

impl<T> Email<T> {
    pub const fn new(name: String) -> Self {
        Self(name, PhantomData)
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
