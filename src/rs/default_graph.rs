use crate::rs::{term::Term, term_like::TermLike};

#[derive(Clone, Eq, Debug)]
pub struct DefaultGraph {}

impl DefaultGraph {
    pub fn new() -> Self {
        Self {}
    }
}

impl PartialEq for DefaultGraph {
    fn eq(&self, _other: &Self) -> bool {
        true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for DefaultGraph {
    fn eq(&self, other: &Term) -> bool {
        match other {
            Term::DefaultGraph(dg) => self == dg,
            _ => false,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<DefaultGraph> for Term {
    fn eq(&self, other: &DefaultGraph) -> bool {
        match self {
            Term::DefaultGraph(dg) => other == dg,
            _ => false,
        }
    }

    fn ne(&self, other: &DefaultGraph) -> bool {
        !self.eq(other)
    }
}

impl TermLike for DefaultGraph {
    fn value(&self) -> &str {
        ""
    }

    fn as_term(self) -> Term {
        Term::DefaultGraph(self)
    }

    fn to_term(&self) -> Term {
        Term::DefaultGraph(self.to_owned())
    }
}
