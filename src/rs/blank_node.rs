use crate::rs::{term::Term, term_like::TermLike};

#[derive(Clone, Eq, Debug)]
pub struct BlankNode {
    value: String,
}

impl BlankNode {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl PartialEq for BlankNode {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for BlankNode {
    fn eq(&self, other: &Term) -> bool {
        match other {
            Term::BlankNode(bn) => self == bn,
            _ => false,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<BlankNode> for Term {
    fn eq(&self, other: &BlankNode) -> bool {
        match self {
            Term::BlankNode(bn) => other == bn,
            _ => false,
        }
    }

    fn ne(&self, other: &BlankNode) -> bool {
        !self.eq(other)
    }
}

impl TermLike for BlankNode {
    fn value(&self) -> &str {
        &self.value
    }

    fn as_term(self) -> Term {
        Term::BlankNode(self)
    }

    fn to_term(&self) -> Term {
        Term::BlankNode(self.to_owned())
    }
}
