use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

#[derive(Clone)]
pub struct UserName<T>(String, PhantomData<T>);

impl<T> UserName<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into(), PhantomData)
    }
}

impl<T> AsRef<str> for UserName<T> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<T> From<UserName<T>> for String {
    fn from(from: UserName<T>) -> Self {
        from.0
    }
}

impl<T> Debug for UserName<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Display for UserName<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
