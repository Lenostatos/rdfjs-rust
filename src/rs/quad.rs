use crate::rs::{term::Term, term_like::TermLike};

use crate::rs::default_graph::DefaultGraph;

use crate::rs::quad_graph::QuadGraph;
use crate::rs::quad_object::QuadObject;
use crate::rs::quad_predicate::QuadPredicate;
use crate::rs::quad_subject::QuadSubject;

#[derive(Clone, Eq, Debug)]
pub struct Quad {
    pub subject: QuadSubject,
    pub predicate: QuadPredicate,
    pub object: QuadObject,
    pub graph: QuadGraph,
}

impl Quad {
    pub fn new(
        subject: &QuadSubject,
        predicate: &QuadPredicate,
        object: &QuadObject,
        graph: Option<&QuadGraph>,
    ) -> Self {
        Self {
            subject: subject.to_owned(),
            predicate: predicate.to_owned(),
            object: object.to_owned(),
            graph: if let Some(g) = graph {
                g.to_owned()
            } else {
                QuadGraph::DefaultGraph(DefaultGraph::new())
            },
        }
    }
}

impl PartialEq for Quad {
    fn eq(&self, other: &Self) -> bool {
        self.subject == other.subject
            && self.predicate == other.predicate
            && self.object == other.object
            && self.graph == other.graph
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for Quad {
    fn eq(&self, other: &Term) -> bool {
        match other {
            Term::Quad(other_q) => *self == **other_q,
            _ => false,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Quad> for Term {
    fn eq(&self, other: &Quad) -> bool {
        match self {
            Term::Quad(self_q) => *other == **self_q,
            _ => false,
        }
    }

    fn ne(&self, other: &Quad) -> bool {
        !self.eq(other)
    }
}

impl TermLike for Quad {
    fn value(&self) -> &str {
        ""
    }

    fn as_term(self) -> Term {
        Term::Quad(Box::new(self))
    }

    fn to_term(&self) -> Term {
        Term::Quad(Box::new(self.to_owned()))
    }
}
