use crate::typed::CanonicalConstantIdentifier;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Hash, Eq, PartialOrd, Ord)]
pub enum CoreIdentifier<'ast> {
    Source(&'ast str),
    Call(usize),
    Constant(CanonicalConstantIdentifier<'ast>),
    Condition(usize),
}

impl<'ast> fmt::Display for CoreIdentifier<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoreIdentifier::Source(s) => write!(f, "{}", s),
            CoreIdentifier::Call(i) => write!(f, "#CALL_RETURN_AT_INDEX_{}", i),
            CoreIdentifier::Constant(c) => write!(f, "{}/{}", c.module.display(), c.id),
            CoreIdentifier::Condition(i) => write!(f, "#CONDITION_{}", i),
        }
    }
}

impl<'ast> From<&'ast str> for CoreIdentifier<'ast> {
    fn from(s: &str) -> CoreIdentifier {
        CoreIdentifier::Source(s)
    }
}

impl<'ast> From<CanonicalConstantIdentifier<'ast>> for CoreIdentifier<'ast> {
    fn from(s: CanonicalConstantIdentifier<'ast>) -> CoreIdentifier<'ast> {
        CoreIdentifier::Constant(s)
    }
}

/// A identifier for a variable
#[derive(Debug, PartialEq, Clone, Hash, Eq, PartialOrd, Ord)]
pub struct Identifier<'ast> {
    /// the id of the variable
    pub id: CoreIdentifier<'ast>,
    /// the version of the variable, used after SSA transformation
    pub version: usize,
}

impl<'ast> TryInto<&'ast str> for Identifier<'ast> {
    type Error = ();

    fn try_into(self) -> Result<&'ast str, Self::Error> {
        match self.id {
            CoreIdentifier::Source(i) => Ok(i),
            _ => Err(()),
        }
    }
}

impl<'ast> fmt::Display for Identifier<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.version == 0 {
            write!(f, "{}", self.id)
        } else {
            write!(f, "{}_{}", self.id, self.version)
        }
    }
}

impl<'ast> From<CanonicalConstantIdentifier<'ast>> for Identifier<'ast> {
    fn from(id: CanonicalConstantIdentifier<'ast>) -> Identifier<'ast> {
        Identifier::from(CoreIdentifier::Constant(id))
    }
}

impl<'ast> From<&'ast str> for Identifier<'ast> {
    fn from(id: &'ast str) -> Identifier<'ast> {
        Identifier::from(CoreIdentifier::Source(id))
    }
}

impl<'ast> From<CoreIdentifier<'ast>> for Identifier<'ast> {
    fn from(id: CoreIdentifier<'ast>) -> Identifier<'ast> {
        Identifier { id, version: 0 }
    }
}

impl<'ast> Identifier<'ast> {
    pub fn version(mut self, version: usize) -> Self {
        self.version = version;
        self
    }
}
