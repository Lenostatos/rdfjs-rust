#[derive(Clone, PartialEq)]
pub struct NamedNode {
    pub term_type: &'static str,
    pub value: String,
}

impl NamedNode {
    pub fn new(value: &str) -> Self {
        Self {
            term_type: "NamedNode",
            value: value.to_string()
        }
    }

    pub fn equals(&self, other: &Term) -> bool {
        if let Term::NamedNode(t) = other {
            self == t
        } else {
            false
        }
    }
}

#[derive(PartialEq)]
pub struct BlankNode {
    pub term_type: &'static str,
    pub value: String
}

impl BlankNode {
    pub fn new(value: &str) -> Self {
        Self {
            term_type: "BlankNode",
            value: value.to_string()
        }
    }

    pub fn equals(&self, other: &Term) -> bool {
        if let Term::BlankNode(t) = other {
            self == t
        } else {
            false
        }
    }
}

#[derive(PartialEq)]
pub struct Literal {
    pub term_type: &'static str,
    pub value: String,
    pub language: String,
    pub datatype: NamedNode
}

impl Literal {
    pub fn new(value: &str, language: Option<&str>, datatype: Option<&NamedNode>) -> Self {
        Self {
            term_type: "Literal",
            value: value.to_string(),
            language: if let Some(l) = language {
                l.to_string()
            } else {
                "".to_string()
            },
            datatype: if let Some(d) = datatype {
                d.clone()
            } else {
                NamedNode::new("http://www.w3.org/2001/XMLSchema#string")
            }
        }
    }

    pub fn equals(&self, other: &Term) -> bool {
        if let Term::Literal(t) = other {
            self == t
        } else {
            false
        }
    }
}

#[derive(PartialEq)]
pub struct Variable {
    pub term_type: &'static str,
    pub value: String
}

impl Variable {
    pub fn new(value: &str) -> Self {
        Self {
            term_type: "Variable",
            value: value.to_string()
        }
    }

    pub fn equals(&self, other: &Term) -> bool {
        if let Term::Variable(t) = other {
            self == t
        } else {
            false
        }
    }
}

#[derive(PartialEq)]
pub struct DefaultGraph {
    pub term_type: &'static str,
    pub value: String
}

impl DefaultGraph {
    pub fn new(value: &str) -> Self {
        Self {
            term_type: "DefaultGraph",
            value: value.to_string()
        }
    }

    pub fn equals(&self, other: &Term) -> bool {
        if let Term::DefaultGraph(t) = other {
            self == t
        } else {
            false
        }
    }
}

pub enum Term {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
    Variable(Variable),
    DefaultGraph(DefaultGraph),
}

impl Term {
    pub fn term_type(&self) -> &'static str {
        match self {
            Term::BlankNode(t) => t.term_type,
            Term::DefaultGraph(t) => t.term_type,
            Term::Literal(t) => t.term_type,
            Term::NamedNode(t) => t.term_type,
            Term::Variable(t) => t.term_type,
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Term::BlankNode(t) => &t.value,
            Term::DefaultGraph(t) => &t.value,
            Term::Literal(t) => &t.value,
            Term::NamedNode(t) => &t.value,
            Term::Variable(t) => &t.value,
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        match self {
            Term::Literal(t) => match other {
                Term::Literal(o) => {
                    t.value == o.value &&
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

pub struct Quad {
    pub subject: Term,
    pub predicate: Term,
    pub object: Term,
    pub graph: Term
}

impl Quad {
    pub fn equals(&self, other: &Quad) -> bool {
        self.subject.equals(&other.subject) &&
        self.predicate.equals(&other.predicate) &&
        self.object.equals(&other.object) &&
        self.graph.equals(&other.graph)
    }
}

fn main() {
    
}