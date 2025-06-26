use crate::rs::{term::Term, term_like::TermLike};

use crate::rs::blank_node::BlankNode;
use crate::rs::default_graph::DefaultGraph;
use crate::rs::named_node::NamedNode;
use crate::rs::variable::Variable;

#[derive(Clone, Eq, Debug)]
pub enum QuadGraph {
    DefaultGraph(DefaultGraph),
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Variable(Variable),
}

impl PartialEq for QuadGraph {
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
            Self::DefaultGraph(self_q) => match other {
                Self::DefaultGraph(other_q) => self_q == other_q,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for QuadGraph {
    fn eq(&self, other: &Term) -> bool {
        match self {
            QuadGraph::NamedNode(named_node) => named_node == other,
            QuadGraph::BlankNode(blank_node) => blank_node == other,
            QuadGraph::Variable(variable) => variable == other,
            QuadGraph::DefaultGraph(default_graph) => default_graph == other,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<QuadGraph> for Term {
    fn eq(&self, other: &QuadGraph) -> bool {
        match other {
            QuadGraph::NamedNode(named_node) => named_node == self,
            QuadGraph::BlankNode(blank_node) => blank_node == self,
            QuadGraph::Variable(variable) => variable == self,
            QuadGraph::DefaultGraph(default_graph) => default_graph == self,
        }
    }

    fn ne(&self, other: &QuadGraph) -> bool {
        !self.eq(other)
    }
}

impl TermLike for QuadGraph {
    fn value(&self) -> &str {
        match self {
            QuadGraph::NamedNode(named_node) => named_node.value(),
            QuadGraph::BlankNode(blank_node) => blank_node.value(),
            QuadGraph::Variable(variable) => variable.value(),
            QuadGraph::DefaultGraph(default_graph) => default_graph.value(),
        }
    }

    fn as_term(self) -> Term {
        match self {
            QuadGraph::NamedNode(named_node) => named_node.as_term(),
            QuadGraph::BlankNode(blank_node) => blank_node.as_term(),
            QuadGraph::Variable(variable) => variable.as_term(),
            QuadGraph::DefaultGraph(default_graph) => default_graph.as_term(),
        }
    }

    fn to_term(&self) -> Term {
        match self {
            QuadGraph::NamedNode(named_node) => named_node.to_term(),
            QuadGraph::BlankNode(blank_node) => blank_node.to_term(),
            QuadGraph::Variable(variable) => variable.to_term(),
            QuadGraph::DefaultGraph(default_graph) => default_graph.to_term(),
        }
    }
}
