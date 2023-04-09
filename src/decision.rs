use std::convert::Infallible;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Decision {
    Accept,
    Reject,
}

impl Decision {
    pub fn try_from_iter_all<I, E>(iter: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = Result<Self, E>>,
    {
        for decision in iter {
            if decision? == Self::Reject {
                return Ok(Self::Reject);
            }
        }

        Ok(Self::Accept)
    }

    pub fn try_from_iter_any<I, E>(iter: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = Result<Self, E>>,
    {
        for decision in iter {
            if decision? == Self::Accept {
                return Ok(Self::Accept);
            }
        }

        Ok(Self::Reject)
    }

    pub fn from_iter_all<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self::try_from_iter_all::<_, Infallible>(iter.into_iter().map(Ok)).unwrap()
    }

    pub fn from_iter_any<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self::try_from_iter_any::<_, Infallible>(iter.into_iter().map(Ok)).unwrap()
    }
}

impl From<bool> for Decision {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Accept,
            _ => Self::Reject,
        }
    }
}

impl From<Decision> for bool {
    fn from(value: Decision) -> Self {
        match value {
            Decision::Accept => true,
            _ => false,
        }
    }
}
