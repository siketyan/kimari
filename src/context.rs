use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use crate::value::Value;

#[derive(Debug, PartialEq, Eq)]
pub struct ErroredPath(String);

impl Display for ErroredPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a, I> From<I> for ErroredPath
where
    I: IntoIterator<Item = &'a str>,
{
    fn from(value: I) -> Self {
        Self(
            value
                .into_iter()
                .map(|s| s.replace('.', "\\."))
                .collect::<Vec<_>>()
                .join("."),
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected path: {0}")]
    UnexpectedPath(ErroredPath),

    #[error("Unexpected index in an array: {0}")]
    UnexpectedIndex(#[from] ParseIntError),

    #[error("{0}")]
    Other(Box<dyn std::error::Error>),
}

pub trait Context {
    fn get_from_context<'a, I>(&self, path: I) -> Result<Value, Error>
    where
        I: IntoIterator<Item = &'a str>;

    fn get_from_context_at(&self, path: &str) -> Result<Value, Error> {
        self.get_from_context(path.split('.'))
    }
}

macro_rules! impl_context_primitive {
    ($t: ty) => {
        impl $crate::context::Context for $t {
            fn get_from_context<'a, I>(
                &self,
                path: I,
            ) -> Result<$crate::Value, $crate::context::Error>
            where
                I: IntoIterator<Item = &'a str>,
            {
                let path = path.into_iter().collect::<Vec<_>>();
                if path.is_empty() {
                    Ok($crate::Value::from(self))
                } else {
                    Err($crate::context::Error::UnexpectedPath(path.into()))
                }
            }
        }
    };
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
mod bits32 {
    impl_context_primitive!(i32);
    impl_context_primitive!(u32);
}

#[cfg(target_pointer_width = "64")]
mod bits64 {
    impl_context_primitive!(i64);
    impl_context_primitive!(u64);
}

impl_context_primitive!(isize);
impl_context_primitive!(usize);
impl_context_primitive!(String);
impl_context_primitive!(&str);

impl Context for Value {
    fn get_from_context<'a, I>(&self, path: I) -> Result<Value, Error>
    where
        I: IntoIterator<Item = &'a str>,
    {
        match self {
            Self::Integer(i) => i.get_from_context(path),
            Self::String(s) => s.get_from_context(path),
            Self::Array(a) => a.get_from_context(path),
            Self::Optional(o) => o.as_deref().get_from_context(path),
        }
    }
}

impl Context for &Value {
    fn get_from_context<'a, I>(&self, path: I) -> Result<Value, Error>
    where
        I: IntoIterator<Item = &'a str>,
    {
        (*self).get_from_context(path)
    }
}

impl<T> Context for Option<T>
where
    T: Context,
{
    fn get_from_context<'a, I>(&self, path: I) -> Result<Value, Error>
    where
        I: IntoIterator<Item = &'a str>,
    {
        match self {
            Some(ctx) => ctx.get_from_context(path),
            _ => Ok(Value::Optional(None)),
        }
    }
}

impl<T> Context for Vec<T>
where
    T: Context,
    for<'b> Value: From<&'b Vec<T>> + From<T>,
{
    fn get_from_context<'a, I>(&self, path: I) -> Result<Value, Error>
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut path = path.into_iter();
        let index = match path.next() {
            Some(i) => usize::from_str(i)?,
            _ => return Ok(Value::from(self)),
        };

        match self.get(index) {
            Some(ctx) => ctx.get_from_context(path),
            _ => Ok(Value::Optional(None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_primitives() {
        assert_eq!(Value::from(123_i32), 123_i32.get_from_context([]).unwrap());
        assert_eq!(Value::from(123_u32), 123_u32.get_from_context([]).unwrap());
        assert_eq!(Value::from(123_i64), 123_i64.get_from_context([]).unwrap());
        assert_eq!(Value::from(123_u64), 123_u64.get_from_context([]).unwrap());
        assert_eq!(
            Value::from(123_isize),
            123_isize.get_from_context([]).unwrap(),
        );
        assert_eq!(
            Value::from(123_usize),
            123_usize.get_from_context([]).unwrap(),
        );
        assert_eq!(Value::from("abc"), "abc".get_from_context([]).unwrap());
        assert_eq!(
            Value::from("abc"),
            "abc".to_string().get_from_context([]).unwrap(),
        );
    }

    #[test]
    fn context_optional() {
        assert_eq!(
            Value::from(Some("abc")),
            Some("abc").get_from_context([]).unwrap(),
        );
    }

    #[test]
    fn context_array() {
        assert_eq!(
            Value::from(vec![Value::from("abc"), Value::from("def")]),
            vec!["abc", "def"].get_from_context([]).unwrap(),
        );

        assert_eq!(
            Value::from("abc"),
            vec!["abc", "def"].get_from_context(["0"]).unwrap(),
        );
        assert_eq!(
            Value::from("def"),
            vec!["abc", "def"].get_from_context(["1"]).unwrap(),
        );
        assert_eq!(
            Value::Optional(None),
            vec!["abc", "def"].get_from_context(["2"]).unwrap(),
        );

        assert!(matches!(
            vec!["abc", "def"].get_from_context(["abc"]).unwrap_err(),
            Error::UnexpectedIndex(_),
        ));
    }
}
