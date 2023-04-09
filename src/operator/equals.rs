use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::decision::Decision;
use crate::operator::{Error, Operate};
use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Equals {
    pub r#where: String,
    pub to: Value,
}

impl Operate for Equals {
    fn operate<C>(&self, context: &C) -> Result<Decision, Error>
    where
        C: Context,
    {
        Ok((&context.get_from_context_at(&self.r#where)? == &self.to).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        // language=yaml
        let yaml = r#"
        where: foo.bar
        to:
          - 123
          - 'abc'
        "#;

        let operator = serde_yaml::from_str::<Equals>(yaml).unwrap();

        assert_eq!(
            Equals {
                r#where: "foo.bar".to_string(),
                to: Value::from(vec![Value::from(123), Value::from("abc")]),
            },
            operator,
        );
    }
}
