use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::decision::Decision;
use crate::operator::{Error, Operate, Operator};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Rule {
    #[serde(flatten)]
    operator: Operator,
}

impl Rule {
    /// Determine whether the context satisfies the rule or not.
    pub fn is_satisfied_by<C>(&self, context: &C) -> Result<Decision, Error>
    where
        C: Context,
    {
        self.operator.operate(context)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Rules {
    #[serde(flatten)]
    map: BTreeMap<String, Rule>,
}

impl Rules {
    /// Find a rule that is satisfied by the context.
    /// If multiple rules matched to the context, returns the first one.
    pub fn find<'a, C>(&'a self, context: &'a C) -> Result<Option<(&str, &Rule)>, Error>
    where
        C: Context,
    {
        match self.find_all(context).next() {
            Some(Ok(r)) => Ok(Some(r)),
            Some(Err(e)) => Err(e),
            _ => Ok(None),
        }
    }

    /// Find all rules that are satisfied by the context.
    pub fn find_all<'a, C>(
        &'a self,
        context: &'a C,
    ) -> impl Iterator<Item = Result<(&str, &Rule), Error>> + 'a
    where
        C: Context,
    {
        self.map
            .iter()
            .filter_map(|(name, rule)| match rule.is_satisfied_by(context) {
                Ok(Decision::Accept) => Some(Ok((name.as_str(), rule))),
                Ok(Decision::Reject) => None,
                Err(e) => Some(Err(e)),
            })
    }
}

impl Deref for Rules {
    type Target = BTreeMap<String, Rule>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Rules {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
