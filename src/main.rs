use std::{cmp::Ordering, io, num::ParseIntError};
use rand::Rng;

#[derive(PartialEq, Eq)]
enum TermType {
    NamedNode,
    BlankNode,
    Literal,
    Variable,
    DefaultGraph
}

trait BaseTerm {
    // const TERM_TYPE: &str;
    // fn term_type() -> &'static str;
    fn term_type() -> &'static TermType;
    fn value(&self) -> &str;
}

struct NamedNode {
    // term_type: &'static str,
    value: String
}

impl NamedNode {
    const TERM_TYPE: TermType = TermType::NamedNode;
}

struct BlankNode { 
    value: String
}

struct Literal {
    value: String,
    language: String,
    datatype: NamedNode
}

impl BaseTerm for Literal {
    fn term_type() -> &'static TermType {
        &TermType::Literal
    }

    fn value(&self) -> &str {
        self.value.as_str()
    }
}

struct Variable {
    term_type: &'static str, 
    value: String
}

impl BaseTerm for Variable {
    fn term_type() -> &'static TermType {
        &TermType::Variable
    }

    fn value(&self) -> &str {
        self.value.as_str()
    }
}

struct DefaultGraph {
    term_type: &'static str, 
    value: String
}

impl BaseTerm for DefaultGraph {
    fn term_type() -> &'static TermType {
        &TermType::DefaultGraph
    }

    fn value(&self) -> &str {
        self.value.as_str()
    }
}

enum Term {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
    Variable(Variable),
    DefaultGraph(DefaultGraph),
}

impl Term {
    fn term_type(&self) -> &TermType {
        match self {
            Term::BlankNode(_) => &TermType::BlankNode,
            Term::DefaultGraph(_) => &DefaultGraph::term_type(),
            Term::Literal(_) => &Literal::term_type(),
            Term::NamedNode(_) => &NamedNode::TERM_TYPE,
            Term::Variable(_) => &Variable::term_type()
        }
    }

    fn value(&self) -> &str {
        match self {
            Term::BlankNode(t) => &t.value,
            Term::DefaultGraph(t) => t.value(),
            Term::Literal(t) => t.value(),
            Term::NamedNode(t) => &t.value,
            Term::Variable(t) => t.value(),
        }
    }

    fn equals(&self, other: &Self) -> bool {
        match self {
            Term::Literal(t) => match other {
                Term::Literal(o) => {
                    t.value() == o.value() &&
                        t.datatype.value == o.datatype.value &&
                        t.language == t.language
                },
                _ => false
            },
            _ => self.term_type() == other.term_type() && 
                self.value() == other.value()
        }
    }
}

struct Quad {
    subject: Term,
    predicate: Term,
    object: Term,
    graph: Term
}

impl Quad {
    fn equals(&self, other: &Quad) -> bool {
        self.subject.equals(&other.subject) &&
        self.predicate.equals(&other.predicate) &&
        self.object.equals(&other.object) &&
        self.graph.equals(&other.graph)
    }
}

fn main() {
    
}