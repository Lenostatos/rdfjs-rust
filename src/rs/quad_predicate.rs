use crate::rs::{term::Term, term_like::TermLike};

use crate::rs::named_node::NamedNode;
use crate::rs::variable::Variable;

#[derive(Clone, Eq, Debug)]
pub enum QuadPredicate {
    NamedNode(NamedNode),
    Variable(Variable),
}

impl PartialEq for QuadPredicate {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::NamedNode(self_nn) => match other {
                Self::NamedNode(other_nn) => self_nn == other_nn,
                _ => false,
            },
            Self::Variable(self_v) => match other {
                Self::Variable(other_v) => self_v == other_v,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for QuadPredicate {
    fn eq(&self, other: &Term) -> bool {
        match self {
            QuadPredicate::NamedNode(named_node) => named_node == other,
            QuadPredicate::Variable(variable) => variable == other,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<QuadPredicate> for Term {
    fn eq(&self, other: &QuadPredicate) -> bool {
        match other {
            QuadPredicate::NamedNode(named_node) => named_node == self,
            QuadPredicate::Variable(variable) => variable == self,
        }
    }

    fn ne(&self, other: &QuadPredicate) -> bool {
        !self.eq(other)
    }
}

impl TermLike for QuadPredicate {
    fn value(&self) -> &str {
        match self {
            QuadPredicate::NamedNode(named_node) => named_node.value(),
            QuadPredicate::Variable(variable) => variable.value(),
        }
    }

    fn as_term(self) -> Term {
        match self {
            QuadPredicate::NamedNode(named_node) => named_node.as_term(),
            QuadPredicate::Variable(variable) => variable.as_term(),
        }
    }

    fn to_term(&self) -> Term {
        match self {
            QuadPredicate::NamedNode(named_node) => named_node.to_term(),
            QuadPredicate::Variable(variable) => variable.to_term(),
        }
    }
}
