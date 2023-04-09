mod all;
mod any;
mod equals;
mod not_equals;

pub use all::*;
pub use any::*;
pub use equals::*;
pub use not_equals::*;

use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::decision::Decision;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected context: {0}")]
    UnexpectedContext(#[from] crate::context::Error),
}

pub trait Operate {
    fn operate<C>(&self, context: &C) -> Result<Decision, Error>
    where
        C: Context;
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Operator {
    All(All),
    Any(Any),
    Equals(Equals),
    NotEquals(NotEquals),
}

impl Operate for Operator {
    fn operate<C>(&self, context: &C) -> Result<Decision, Error>
    where
        C: Context,
    {
        match self {
            Self::All(o) => o.operate(context),
            Self::Any(o) => o.operate(context),
            Self::Equals(o) => o.operate(context),
            Self::NotEquals(o) => o.operate(context),
        }
    }
}
