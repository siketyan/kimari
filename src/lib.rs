pub mod context;
mod decision;
pub mod operator;
mod rule;
mod value;

pub use decision::Decision;
pub use operator::{Operate, Operator};
pub use rule::{Rule, Rules};
pub use value::Value;

pub use kimari_derive::Context;

#[cfg(test)]
mod tests {
    use crate as kimari;
    use crate::context::Context;
    use kimari::*;

    #[test]
    fn integration_test() {
        #[derive(Context)]
        struct NestedContext {
            baz: String,
            array: Vec<usize>,
        }

        #[derive(Context)]
        struct MyContext {
            foo: usize,
            bar: NestedContext,
        }

        let ctx = MyContext {
            foo: 123,
            bar: NestedContext {
                baz: "abc".to_string(),
                array: vec![456, 789],
            },
        };

        assert_eq!(Value::from(123), ctx.get_from_context_at("foo").unwrap());
        assert_eq!(
            Value::from("abc"),
            ctx.get_from_context_at("bar.baz").unwrap(),
        );

        // language=yaml
        let yaml = r#"
        foo_is_123:
          type: equals
          where: foo
          to: 123
        bar_array_is_456_and_789:
          type: all
          operators:
            - type: equals
              where: bar.array.0
              to: 456
            - type: equals
              where: bar.array.1
              to: 789
        bar_baz_is_def:
          type: equals
          where: bar.baz
          to: def
        "#;

        let rules = serde_yaml::from_str::<Rules>(yaml).unwrap();

        assert_eq!(
            vec!["bar_array_is_456_and_789", "foo_is_123"],
            rules
                .find_all(&ctx)
                .map(|r| r.unwrap().0)
                .collect::<Vec<_>>(),
        )
    }
}
