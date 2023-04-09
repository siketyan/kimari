use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::decision::Decision;
use crate::operator::{Error, Operate};
use crate::Operator;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Any {
    pub operators: Vec<Operator>,
}

impl Operate for Any {
    fn operate<C>(&self, context: &C) -> Result<Decision, Error>
    where
        C: Context,
    {
        Decision::try_from_iter_any(self.operators.iter().map(|o| o.operate(context)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operator;
    use crate::Value;

    #[test]
    fn deserialize() {
        // language=yaml
        let yaml = r#"
        operators:
          - type: equals
            where: foo.bar
            to: 123
          - type: notEquals
            where: foo.bar
            to: abc
        "#;

        let operator = serde_yaml::from_str::<Any>(yaml).unwrap();

        assert_eq!(
            Any {
                operators: vec![
                    Operator::Equals(operator::Equals {
                        r#where: "foo.bar".to_string(),
                        to: Value::from(123),
                    }),
                    Operator::NotEquals(operator::NotEquals {
                        r#where: "foo.bar".to_string(),
                        to: Value::from("abc"),
                    })
                ]
            },
            operator,
        );
    }
}
