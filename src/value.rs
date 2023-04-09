use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
    Integer(isize),
    String(String),
    Array(Vec<Value>),
    Optional(Option<Box<Value>>),
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
mod bits32 {
    use super::Value;

    impl From<i32> for Value {
        fn from(value: i32) -> Self {
            Self::from(value as isize)
        }
    }

    impl From<u32> for Value {
        fn from(value: u32) -> Self {
            Self::from(value as isize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
mod bits64 {
    use super::Value;

    impl From<i64> for Value {
        fn from(value: i64) -> Self {
            Self::from(value as isize)
        }
    }

    impl From<u64> for Value {
        fn from(value: u64) -> Self {
            Self::from(value as isize)
        }
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self::from(value as isize)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Self::Integer(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl<T> From<&T> for Value
where
    T: ToOwned<Owned = T>,
    Self: From<T>,
{
    fn from(value: &T) -> Self {
        Self::from(value.to_owned())
    }
}

impl<T> From<Vec<T>> for Value
where
    Self: From<T>,
{
    fn from(value: Vec<T>) -> Self {
        value.into_iter().collect()
    }
}

impl<T> From<Option<T>> for Value
where
    Self: From<T>,
{
    fn from(value: Option<T>) -> Self {
        Self::Optional(value.map(|v| Box::new(Self::from(v))))
    }
}

impl<V> FromIterator<V> for Value
where
    Self: From<V>,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self::Array(iter.into_iter().map(|v| v.into()).collect())
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Integer(a) => match other {
                Self::Integer(b) => a == b,
                Self::Optional(b) => self == b,
                _ => false,
            },
            Self::String(a) => match other {
                Self::String(b) => a == b,
                Self::Optional(b) => self == b,
                _ => false,
            },
            Self::Array(a) => match other {
                Self::Array(b) => a == b,
                _ => false,
            },
            Self::Optional(a) => match other {
                Self::Optional(b) => a == b,
                _ => a.as_deref().map(|a| a == other).unwrap_or(false),
            },
        }
    }
}

impl PartialEq<Option<Box<Self>>> for Value {
    fn eq(&self, other: &Option<Box<Self>>) -> bool {
        other.as_deref().map(|b| self == b).unwrap_or(false)
    }
}

impl Eq for Value {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        // language=yaml
        let yaml = r#"
        - 123
        - '123'
        - abc
        - - def
          - ghi
        - ~
        "#;
        let value = serde_yaml::from_str::<Value>(yaml).unwrap();

        assert_eq!(
            Value::from(vec![
                Value::from(123),
                Value::from("123"),
                Value::from("abc"),
                Value::from(vec![Value::from("def"), Value::from("ghi")]),
                Value::Optional(None),
            ]),
            value,
        )
    }

    #[test]
    fn eq_string_string() {
        assert_eq!(Value::from("abc"), Value::from("abc"));
        assert_eq!(Value::from("abc"), Value::from(Some("abc")));
        assert_eq!(Value::from(Some("abc")), Value::from("abc"));
        assert_eq!(Value::from(Some("abc")), Value::from(Some("abc")));

        assert_ne!(Value::from("abc"), Value::from("def"));
        assert_ne!(Value::from("abc"), Value::from(Some("def")));
        assert_ne!(Value::from("abc"), Value::from(None::<String>));
    }

    #[test]
    fn eq_array() {
        assert_eq!(
            Value::from(vec![Value::from(123), Value::from("abc")]),
            Value::from(vec![Value::from(123), Value::from("abc")]),
        );
        assert_eq!(
            Value::from(vec![Value::from(123), Value::from(None::<String>)]),
            Value::from(vec![Value::from(Some(123)), Value::from(None::<String>)]),
        );

        assert_ne!(
            Value::from(vec![Value::from(123), Value::from("abc")]),
            Value::from(vec![Value::from("abc"), Value::from(123)]),
        );
    }

    #[test]
    fn eq_optional() {
        assert_eq!(Value::from(None::<String>), Value::from(None::<String>));
    }
}
