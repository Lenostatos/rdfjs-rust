use std::{error::Error, fmt};

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
        if let TermType::NamedNode(t) = &other.term {
            self == t
        } else {
            false
        }
    }

    fn to_term_type(&self) -> TermType {
        TermType::NamedNode(self.clone())
    }

    fn to_term(&self) -> Term {
        Term { term_type: self.term_type, value: self.value.clone(), term: self.to_term_type() }
    }
}

#[derive(Clone, PartialEq)]
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
        if let TermType::BlankNode(t) = &other.term {
            self == t
        } else {
            false
        }
    }

    fn to_term_type(&self) -> TermType {
        TermType::BlankNode(self.clone())
    }

    fn to_term(&self) -> Term {
        Term { term_type: self.term_type, value: self.value.clone(), term: self.to_term_type() }
    }
}

#[derive(Clone, PartialEq)]
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
        if let TermType::Literal(t) = &other.term {
            self == t
        } else {
            false
        }
    }

    fn to_term_type(&self) -> TermType {
        TermType::Literal(self.clone())
    }

    fn to_term(&self) -> Term {
        Term { term_type: self.term_type, value: self.value.clone(), term: self.to_term_type() }
    }
}

#[derive(Clone, PartialEq)]
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
        if let TermType::Variable(t) = &other.term {
            self == t
        } else {
            false
        }
    }

    fn to_term_type(&self) -> TermType {
        TermType::Variable(self.clone())
    }

    fn to_term(&self) -> Term {
        Term { term_type: self.term_type, value: self.value.clone(), term: self.to_term_type() }
    }
}

#[derive(Clone, PartialEq)]
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
        if let TermType::DefaultGraph(t) = &other.term {
            self == t
        } else {
            false
        }
    }

    fn to_term_type(&self) -> TermType {
        TermType::DefaultGraph(self.clone())
    }

    fn to_term(&self) -> Term {
        Term { term_type: self.term_type, value: self.value.clone(), term: self.to_term_type() }
    }
}

#[derive(Clone, PartialEq)]
enum TermType {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
    Variable(Variable),
    DefaultGraph(DefaultGraph),
}

impl TermType {
    pub fn term_type(&self) -> &'static str {
        match self {
            TermType::BlankNode(t) => t.term_type,
            TermType::DefaultGraph(t) => t.term_type,
            TermType::Literal(t) => t.term_type,
            TermType::NamedNode(t) => t.term_type,
            TermType::Variable(t) => t.term_type,
        }
    }

    pub fn value(&self) -> &str {
        match self {
            TermType::BlankNode(t) => &t.value,
            TermType::DefaultGraph(t) => &t.value,
            TermType::Literal(t) => &t.value,
            TermType::NamedNode(t) => &t.value,
            TermType::Variable(t) => &t.value,
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        match self {
            TermType::Literal(t) => match other {
                TermType::Literal(o) => {
                    t == o
                },
                _ => false
            },
            _ => self.term_type() == other.term_type() && 
                self.value() == other.value()
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Term {
    pub term_type: &'static str,
    pub value: String,

    term: TermType
}

impl Term {
    pub fn new(
        term_type: &'static str, 
        value: &str, 
        language: Option<&str>, 
        datatype: Option<&NamedNode>
    ) -> Result<Term, InvalidTermTypeError> {
        match term_type {
            "NamedNode" => Ok(NamedNode::new(value).to_term()),
            "BlankNode" => Ok(BlankNode::new(value).to_term()),
            "Literal" => Ok(Literal::new(value, language, datatype).to_term()),
            "Variable" => Ok(Variable::new(value).to_term()),
            "DefaultGraph" => Ok(DefaultGraph::new(value).to_term()),
            _ => Err(InvalidTermTypeError { invalid_term_type: term_type.to_string() })
        }
    }

    pub fn equals(&self, other: &Term) -> bool {
        self.term.equals(&other.term)
    }
}

#[derive(Debug)]
pub struct InvalidTermTypeError {
    invalid_term_type: String
}

impl fmt::Display for InvalidTermTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid term type: {}", self.invalid_term_type)
    }
}

impl Error for InvalidTermTypeError {}

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