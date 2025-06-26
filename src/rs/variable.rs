use crate::rs::{term::Term, term_like::TermLike};

#[derive(Clone, Eq, Debug)]
pub struct Variable {
    value: String,
}

impl Variable {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for Variable {
    fn eq(&self, other: &Term) -> bool {
        match other {
            Term::Variable(v) => self == v,
            _ => false,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Variable> for Term {
    fn eq(&self, other: &Variable) -> bool {
        match self {
            Term::Variable(v) => other == v,
            _ => false,
        }
    }

    fn ne(&self, other: &Variable) -> bool {
        !self.eq(other)
    }
}

impl TermLike for Variable {
    fn value(&self) -> &str {
        &self.value
    }

    fn as_term(self) -> Term {
        Term::Variable(self)
    }

    fn to_term(&self) -> Term {
        Term::Variable(self.to_owned())
    }
}
