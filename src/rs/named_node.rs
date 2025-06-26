use crate::rs::{term::Term, term_like::TermLike};

#[derive(Clone, Eq, Debug)]
pub struct NamedNode {
    value: String,
}

impl NamedNode {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl PartialEq for NamedNode {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for NamedNode {
    fn eq(&self, other: &Term) -> bool {
        match other {
            Term::NamedNode(nn) => self == nn,
            _ => false,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<NamedNode> for Term {
    fn eq(&self, other: &NamedNode) -> bool {
        match self {
            Term::NamedNode(nn) => other == nn,
            _ => false,
        }
    }

    fn ne(&self, other: &NamedNode) -> bool {
        !self.eq(other)
    }
}

impl TermLike for NamedNode {
    fn value(&self) -> &str {
        &self.value
    }

    fn as_term(self) -> Term {
        Term::NamedNode(self)
    }

    fn to_term(&self) -> Term {
        Term::NamedNode(self.to_owned())
    }
}
