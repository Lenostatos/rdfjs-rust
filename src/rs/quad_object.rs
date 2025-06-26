use crate::rs::{term::Term, term_like::TermLike};

use crate::rs::blank_node::BlankNode;
use crate::rs::literal::Literal;
use crate::rs::named_node::NamedNode;
use crate::rs::variable::Variable;

#[derive(Clone, Eq, Debug)]
pub enum QuadObject {
    NamedNode(NamedNode),
    Literal(Literal),
    BlankNode(BlankNode),
    Variable(Variable),
}

impl PartialEq for QuadObject {
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
            Self::Literal(self_q) => match other {
                Self::Literal(other_q) => self_q == other_q,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for QuadObject {
    fn eq(&self, other: &Term) -> bool {
        match self {
            QuadObject::NamedNode(named_node) => named_node == other,
            QuadObject::Literal(literal) => literal == other,
            QuadObject::BlankNode(blank_node) => blank_node == other,
            QuadObject::Variable(variable) => variable == other,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<QuadObject> for Term {
    fn eq(&self, other: &QuadObject) -> bool {
        match other {
            QuadObject::NamedNode(named_node) => named_node == self,
            QuadObject::Literal(literal) => literal == self,
            QuadObject::BlankNode(blank_node) => blank_node == self,
            QuadObject::Variable(variable) => variable == self,
        }
    }

    fn ne(&self, other: &QuadObject) -> bool {
        !self.eq(other)
    }
}

impl TermLike for QuadObject {
    fn value(&self) -> &str {
        match self {
            QuadObject::NamedNode(named_node) => named_node.value(),
            QuadObject::Literal(literal) => literal.value(),
            QuadObject::BlankNode(blank_node) => blank_node.value(),
            QuadObject::Variable(variable) => variable.value(),
        }
    }

    fn as_term(self) -> Term {
        match self {
            QuadObject::NamedNode(named_node) => named_node.as_term(),
            QuadObject::Literal(literal) => literal.as_term(),
            QuadObject::BlankNode(blank_node) => blank_node.as_term(),
            QuadObject::Variable(variable) => variable.as_term(),
        }
    }

    fn to_term(&self) -> Term {
        match self {
            QuadObject::NamedNode(named_node) => named_node.to_term(),
            QuadObject::Literal(literal) => literal.to_term(),
            QuadObject::BlankNode(blank_node) => blank_node.to_term(),
            QuadObject::Variable(variable) => variable.to_term(),
        }
    }
}
