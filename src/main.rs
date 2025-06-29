#[derive(Clone, PartialEq)]
pub struct NamedNode {
    pub term_type: &'static str,
    pub value: String,
}

impl NamedNode {
    pub fn new(value: &str) -> Self {
        Self {
            term_type: "NamedNode",
            value: value.to_string(),
        }
    }

    pub fn equals(&self, other: &Term) -> bool {
        if let TermType::NamedNode(t) = &other.term_type_enum {
            self == *t
        } else {
            false
        }
    }

    fn as_term_type(&self) -> TermType {
        TermType::NamedNode(&self)
    }

    pub fn as_term(&self) -> Term {
        Term { 
            term_type: self.term_type, 
            value: self.value.clone(), 
            term_type_enum: self.as_term_type() 
        }
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
        if let TermType::BlankNode(t) = &other.term_type_enum {
            self == *t
        } else {
            false
        }
    }

    fn as_term_type(&self) -> TermType {
        TermType::BlankNode(&self)
    }

    pub fn as_term(&self) -> Term {
        Term { 
            term_type: self.term_type, 
            value: self.value.clone(), 
            term_type_enum: self.as_term_type() 
        }
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
        if let TermType::Literal(t) = &other.term_type_enum {
            self == *t
        } else {
            false
        }
    }

    fn as_term_type(&self) -> TermType {
        TermType::Literal(&self)
    }

    pub fn as_term(&self) -> Term {
        Term { 
            term_type: self.term_type, 
            value: self.value.clone(), 
            term_type_enum: self.as_term_type() 
        }
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
        if let TermType::Variable(t) = &other.term_type_enum {
            self == *t
        } else {
            false
        }
    }

    fn as_term_type(&self) -> TermType {
        TermType::Variable(&self)
    }

    pub fn as_term(&self) -> Term {
        Term { 
            term_type: self.term_type, 
            value: self.value.clone(), 
            term_type_enum: self.as_term_type() 
        }
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
        if let TermType::DefaultGraph(t) = &other.term_type_enum {
            self == *t
        } else {
            false
        }
    }

    fn as_term_type(&self) -> TermType {
        TermType::DefaultGraph(&self)
    }

    pub fn as_term(&self) -> Term {
        Term { 
            term_type: self.term_type, 
            value: self.value.clone(), 
            term_type_enum: self.as_term_type() 
        }
    }
}

#[derive(Clone, PartialEq)]
enum TermType<'a> {
    NamedNode(&'a NamedNode),
    BlankNode(&'a BlankNode),
    Literal(&'a Literal),
    Variable(&'a Variable),
    DefaultGraph(&'a DefaultGraph),
}

impl TermType<'_> {
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
pub struct Term<'a> {
    pub term_type: &'static str,
    pub value: String,

    term_type_enum: TermType<'a>
}

impl Term<'_> {
    pub fn equals(&self, other: &Term) -> bool {
        self.term_type_enum.equals(&other.term_type_enum)
    }
}

pub struct Quad<'a> {
    pub subject: Term<'a>,
    pub predicate: Term<'a>,
    pub object: Term<'a>,
    pub graph: Term<'a>
}

impl Quad<'_> {
    pub fn equals(&self, other: &Quad) -> bool {
        self.subject.equals(&other.subject) &&
        self.predicate.equals(&other.predicate) &&
        self.object.equals(&other.object) &&
        self.graph.equals(&other.graph)
    }
}

fn main() {
    
}