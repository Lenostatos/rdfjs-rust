use crate::rs::term_like::TermLike;

use crate::rs::blank_node::BlankNode;
use crate::rs::default_graph::DefaultGraph;
use crate::rs::literal::Literal;
use crate::rs::named_node::NamedNode;
use crate::rs::quad::Quad;
use crate::rs::variable::Variable;

#[derive(Clone, Eq, Debug)]
pub enum Term {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
    Variable(Variable),
    DefaultGraph(DefaultGraph),
    Quad(Box<Quad>),
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Term::NamedNode(self_nn) => match other {
                Term::NamedNode(other_nn) => self_nn == other_nn,
                _ => false,
            },
            Term::BlankNode(self_bn) => match other {
                Term::BlankNode(other_bn) => self_bn == other_bn,
                _ => false,
            },
            Term::Literal(self_l) => match other {
                Term::Literal(other_l) => self_l == other_l,
                _ => false,
            },
            Term::Variable(self_v) => match other {
                Term::Variable(other_v) => self_v == other_v,
                _ => false,
            },
            Term::DefaultGraph(self_dg) => match other {
                Term::DefaultGraph(other_dg) => self_dg == other_dg,
                _ => false,
            },
            Term::Quad(self_q) => match other {
                Term::Quad(other_q) => self_q == other_q,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl TermLike for Term {
    fn value(&self) -> &str {
        match self {
            Term::NamedNode(nn) => nn.value(),
            Term::BlankNode(bn) => bn.value(),
            Term::Literal(l) => l.value(),
            Term::Variable(v) => v.value(),
            Term::DefaultGraph(dg) => dg.value(),
            Term::Quad(q) => q.value(),
        }
    }

    fn as_term(self) -> Term {
        self
    }

    fn to_term(&self) -> Term {
        self.clone()
    }
}
