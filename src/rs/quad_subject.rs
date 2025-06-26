use crate::rs::quad::Quad;
use crate::rs::{term::Term, term_like::TermLike};

use crate::rs::blank_node::BlankNode;
use crate::rs::named_node::NamedNode;
use crate::rs::variable::Variable;

#[derive(Clone, Eq, Debug)]
pub enum QuadSubject {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Variable(Variable),
    Quad(Box<Quad>),
}

impl PartialEq for QuadSubject {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::NamedNode(self_nn) => match other {
                Self::NamedNode(other_nn) => self_nn == other_nn,
                _ => false,
            },
            Self::BlankNode(self_bn) => match other {
                Self::BlankNode(other_bn) => self_bn == other_bn,
                _ => false,
            },
            Self::Variable(self_v) => match other {
                Self::Variable(other_v) => self_v == other_v,
                _ => false,
            },
            Self::Quad(self_q) => match other {
                Self::Quad(other_q) => self_q == other_q,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for QuadSubject {
    fn eq(&self, other: &Term) -> bool {
        match self {
            QuadSubject::NamedNode(named_node) => named_node == other,
            QuadSubject::BlankNode(blank_node) => blank_node == other,
            QuadSubject::Variable(variable) => variable == other,
            QuadSubject::Quad(quad) => **quad == *other,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<QuadSubject> for Term {
    fn eq(&self, other: &QuadSubject) -> bool {
        match other {
            QuadSubject::NamedNode(named_node) => named_node == self,
            QuadSubject::BlankNode(blank_node) => blank_node == self,
            QuadSubject::Variable(variable) => variable == self,
            QuadSubject::Quad(quad) => **quad == *self,
        }
    }

    fn ne(&self, other: &QuadSubject) -> bool {
        !self.eq(other)
    }
}

impl TermLike for QuadSubject {
    fn value(&self) -> &str {
        match self {
            QuadSubject::NamedNode(named_node) => named_node.value(),
            QuadSubject::BlankNode(blank_node) => blank_node.value(),
            QuadSubject::Variable(variable) => variable.value(),
            QuadSubject::Quad(quad) => quad.value(),
        }
    }

    fn as_term(self) -> Term {
        match self {
            QuadSubject::NamedNode(named_node) => named_node.as_term(),
            QuadSubject::BlankNode(blank_node) => blank_node.as_term(),
            QuadSubject::Variable(variable) => variable.as_term(),
            QuadSubject::Quad(quad) => quad.as_term(),
        }
    }

    fn to_term(&self) -> Term {
        match self {
            QuadSubject::NamedNode(named_node) => named_node.to_term(),
            QuadSubject::BlankNode(blank_node) => blank_node.to_term(),
            QuadSubject::Variable(variable) => variable.to_term(),
            QuadSubject::Quad(quad) => quad.to_term(),
        }
    }
}
