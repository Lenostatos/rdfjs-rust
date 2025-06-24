#[derive(Clone, PartialEq, Eq)]
pub struct NamedNode {
    pub term_type: &'static str,
    pub value: String,
}

impl NamedNode {
    fn new(value: &str) -> Self {
        Self {
            term_type: "NamedNode",
            value: value.to_string(),
        }
    }

    pub fn equals(&self, other: Option<&Term>) -> bool {
        let Some(other) = other else { return false };

        if let TermType::NamedNode(nn) = &other.term_type_enum {
            self.value == nn.value
        } else {
            false
        }
    }

    fn as_term_type(self) -> TermType {
        TermType::NamedNode(self)
    }

    pub fn as_term(self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.clone(),
            term_type_enum: self.as_term_type(),
        }
    }

    fn to_term_type(&self) -> TermType {
        TermType::NamedNode(self.clone())
    }

    pub fn to_term(&self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.clone(),
            term_type_enum: self.to_term_type(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct BlankNode {
    pub term_type: &'static str,
    pub value: String,
}

impl BlankNode {
    fn new(value: &str) -> Self {
        Self {
            term_type: "BlankNode",
            value: value.to_string(),
        }
    }

    pub fn equals(&self, other: Option<&Term>) -> bool {
        let Some(other) = other else { return false };

        if let TermType::BlankNode(bn) = &other.term_type_enum {
            self.value == bn.value
        } else {
            false
        }
    }

    fn as_term_type(self) -> TermType {
        TermType::BlankNode(self)
    }

    pub fn as_term(self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.clone(),
            term_type_enum: self.as_term_type(),
        }
    }

    fn to_term_type(&self) -> TermType {
        TermType::BlankNode(self.clone())
    }

    pub fn to_term(&self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.clone(),
            term_type_enum: self.to_term_type(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Literal {
    pub term_type: &'static str,
    pub value: String,
    pub language: String,
    pub direction: Option<String>,
    pub datatype: NamedNode,
}

impl Literal {
    fn new(
        value: &str,
        language: Option<&str>,
        direction: Option<&str>,
        datatype: Option<&NamedNode>,
    ) -> Self {
        Self {
            term_type: "Literal",
            value: value.to_string(),
            language: if let Some(l) = language {
                l.to_string()
            } else {
                "".to_string()
            },
            direction: if let Some(d) = direction {
                if let Some(_l) = language {
                    if d != "ltr" && d != "rtl" {
                        panic!(
                            "Literal language string direction should always be either 'ltr' or 'rtl'."
                        );
                    }
                }
                Some(d.to_string())
            } else {
                Some("".to_string())
            },
            datatype: if let Some(_l) = language {
                if let Some(_d) = direction {
                    NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#dirLangString")
                } else {
                    NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString")
                }
            } else if let Some(d) = datatype {
                d.clone()
            } else {
                NamedNode::new("http://www.w3.org/2001/XMLSchema#string")
            },
        }
    }

    pub fn equals(&self, other: Option<&Term>) -> bool {
        let Some(other) = other else { return false };

        if let TermType::Literal(l) = &other.term_type_enum {
            self.value == l.value
                && self.language == l.language
                && self.direction == l.direction
                && self.datatype.equals(Some(&l.datatype.to_term()))
        } else {
            false
        }
    }

    fn as_term_type(self) -> TermType {
        TermType::Literal(self)
    }

    pub fn as_term(self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.clone(),
            term_type_enum: self.as_term_type(),
        }
    }

    fn to_term_type(&self) -> TermType {
        TermType::Literal(self.clone())
    }

    pub fn to_term(&self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.clone(),
            term_type_enum: self.to_term_type(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Variable {
    pub term_type: &'static str,
    pub value: String,
}

impl Variable {
    fn new(value: &str) -> Self {
        Self {
            term_type: "Variable",
            value: value.to_string(),
        }
    }

    pub fn equals(&self, other: Option<&Term>) -> bool {
        let Some(other) = other else { return false };

        if let TermType::Variable(v) = &other.term_type_enum {
            self.value == v.value
        } else {
            false
        }
    }

    fn as_term_type(self) -> TermType {
        TermType::Variable(self)
    }

    pub fn as_term(self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.clone(),
            term_type_enum: self.as_term_type(),
        }
    }

    fn to_term_type(&self) -> TermType {
        TermType::Variable(self.clone())
    }

    pub fn to_term(&self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.clone(),
            term_type_enum: self.to_term_type(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct DefaultGraph {
    pub term_type: &'static str,
    pub value: &'static str,
}

impl DefaultGraph {
    fn new() -> Self {
        Self {
            term_type: "DefaultGraph",
            value: "",
        }
    }

    pub fn equals(&self, other: Option<&Term>) -> bool {
        let Some(other) = other else { return false };

        if let TermType::DefaultGraph(_dg) = &other.term_type_enum {
            true
        } else {
            false
        }
    }

    fn as_term_type(self) -> TermType {
        TermType::DefaultGraph(self)
    }

    pub fn as_term(self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.to_string(),
            term_type_enum: self.as_term_type(),
        }
    }

    fn to_term_type(&self) -> TermType {
        TermType::DefaultGraph(self.clone())
    }

    pub fn to_term(&self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.to_string(),
            term_type_enum: self.to_term_type(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum TermType {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
    Variable(Variable),
    DefaultGraph(DefaultGraph),
    Quad(Box<Quad>),
}

impl TermType {
    pub fn term_type(&self) -> &'static str {
        match self {
            TermType::NamedNode(nn) => nn.term_type,
            TermType::BlankNode(bn) => bn.term_type,
            TermType::Literal(l) => l.term_type,
            TermType::Variable(v) => v.term_type,
            TermType::DefaultGraph(dg) => dg.term_type,
            TermType::Quad(q) => q.term_type,
        }
    }

    pub fn value(&self) -> &str {
        match self {
            TermType::NamedNode(nn) => &nn.value,
            TermType::BlankNode(bn) => &bn.value,
            TermType::Literal(l) => &l.value,
            TermType::Variable(v) => &v.value,
            TermType::DefaultGraph(dg) => &dg.value,
            TermType::Quad(q) => q.value,
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        match self {
            TermType::Literal(t) => match other {
                TermType::Literal(o) => t == o,
                _ => false,
            },
            _ => self.term_type() == other.term_type() && self.value() == other.value(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Term {
    pub term_type: &'static str,
    pub value: String,

    term_type_enum: TermType,
}

impl Term {
    pub fn equals(&self, other: Option<&Term>) -> bool {
        let Some(other) = other else { return false };

        self.term_type_enum.equals(&other.term_type_enum)
    }

    pub fn as_specific_term(self) -> TermType {
        self.term_type_enum
    }

    pub fn to_specific_term(&self) -> TermType {
        self.term_type_enum.clone()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Quad {
    pub term_type: &'static str,
    pub value: &'static str,
    pub subject: Term,
    pub predicate: Term,
    pub object: Term,
    pub graph: Term,
}

fn validate_quad_subject_type(term: &Term) {
    let valid_type = match term.term_type_enum {
        TermType::NamedNode(_)
        | TermType::BlankNode(_)
        | TermType::Variable(_)
        | TermType::Quad(_) => true,
        _ => false,
    };

    if !valid_type {
        panic!("A quad subject should be either a NamedNode, a BlankNode, a Variable, or a Quad.");
    };
}

fn validate_quad_predicate_type(term: &Term) {
    let valid_type = match term.term_type_enum {
        TermType::NamedNode(_) | TermType::Variable(_) => true,
        _ => false,
    };

    if !valid_type {
        panic!("A quad predicate should be either a NamedNode or a Variable.");
    };
}

fn validate_quad_object_type(term: &Term) {
    let valid_type = match term.term_type_enum {
        TermType::NamedNode(_)
        | TermType::Literal(_)
        | TermType::BlankNode(_)
        | TermType::Variable(_) => true,
        _ => false,
    };

    if !valid_type {
        panic!(
            "A quad object should be either a NamedNode, a Literal, a BlankNode, or a Variable."
        );
    };
}

fn validate_quad_graph_type(term: &Term) {
    let valid_type = match term.term_type_enum {
        TermType::DefaultGraph(_)
        | TermType::NamedNode(_)
        | TermType::BlankNode(_)
        | TermType::Variable(_) => true,
        _ => false,
    };

    if !valid_type {
        panic!(
            "A quad graph should be either the DefaultGraph, a NamedNode, a BlankNode, or a Variable."
        );
    };
}

impl Quad {
    fn new(subject: &Term, predicate: &Term, object: &Term, graph: Option<&Term>) -> Self {
        validate_quad_subject_type(subject);
        validate_quad_predicate_type(predicate);
        validate_quad_object_type(object);
        if let Some(graph) = graph {
            validate_quad_graph_type(graph);
        };

        Self {
            term_type: "Quad",
            value: "",
            subject: subject.to_owned(),
            predicate: predicate.to_owned(),
            object: object.to_owned(),
            graph: if let Some(g) = graph {
                g.to_owned()
            } else {
                DefaultGraph::new().as_term()
            },
        }
    }

    pub fn equals(&self, other: Option<&Quad>) -> bool {
        let Some(other) = other else { return false };

        self.subject.equals(Some(&other.subject))
            && self.predicate.equals(Some(&other.predicate))
            && self.object.equals(Some(&other.object))
            && self.graph.equals(Some(&other.graph))
    }

    fn as_term_type(self) -> TermType {
        TermType::Quad(Box::new(self))
    }

    pub fn as_term(self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.to_string(),
            term_type_enum: self.as_term_type(),
        }
    }

    fn to_term_type(&self) -> TermType {
        TermType::Quad(Box::new(self.clone()))
    }

    pub fn to_term(&self) -> Term {
        Term {
            term_type: self.term_type,
            value: self.value.to_string(),
            term_type_enum: self.to_term_type(),
        }
    }
}

pub struct DataFactory {
    blank_node_value_counter: usize,
}

impl DataFactory {
    pub fn new() -> Self {
        Self {
            blank_node_value_counter: 0,
        }
    }

    pub fn named_node(value: &str) -> NamedNode {
        NamedNode::new(value)
    }

    pub fn blank_node(&mut self, value: Option<&str>) -> BlankNode {
        if let Some(value) = value {
            BlankNode::new(value)
        } else {
            self.blank_node_value_counter += 1;
            BlankNode::new(self.blank_node_value_counter.to_string().as_str())
        }
    }

    pub fn literal(value: &str, language_or_datatype: Option<&LanguageOrDatatype>) -> Literal {
        match language_or_datatype {
            Some(language_or_datatype) => match language_or_datatype {
                LanguageOrDatatype::Language(language) => {
                    Literal::new(value, Some(language), None, None)
                }
                LanguageOrDatatype::Datatype(datatype) => {
                    Literal::new(value, None, None, Some(datatype))
                }
                LanguageOrDatatype::DirectionalLanguage(directional_language) => Literal::new(
                    value,
                    Some(&directional_language.language),
                    directional_language.direction.as_deref(),
                    None,
                ),
            },
            None => Literal::new(value, None, None, None),
        }
    }

    pub fn variable(value: &str) -> Variable {
        Variable::new(value)
    }

    pub fn default_graph() -> DefaultGraph {
        DefaultGraph::new()
    }

    pub fn quad(subject: &Term, predicate: &Term, object: &Term, graph: Option<&Term>) -> Quad {
        Quad::new(subject, predicate, object, graph)
    }
}

pub enum LanguageOrDatatype {
    Language(String),
    Datatype(NamedNode),
    DirectionalLanguage(DirectionalLanguage),
}

pub struct DirectionalLanguage {
    language: String,
    direction: Option<String>,
}

fn main() {}
