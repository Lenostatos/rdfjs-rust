pub trait TermLike {
    fn value(&self) -> &str;
    fn equals(&self, other: &Term) -> bool;

    fn as_term(self) -> Term;
    fn to_term(&self) -> Term;
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NamedNode {
    value: String,
}

impl NamedNode {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }

    fn equals_named_node(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl TermLike for NamedNode {
    fn value(&self) -> &str {
        &self.value
    }

    fn equals(&self, other: &Term) -> bool {
        match other {
            Term::NamedNode(nn) => self.equals_named_node(nn),
            _ => false,
        }
    }

    fn as_term(self) -> Term {
        Term::NamedNode(self)
    }

    fn to_term(&self) -> Term {
        Term::NamedNode(self.to_owned())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlankNode {
    value: String,
}

impl BlankNode {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl TermLike for BlankNode {
    fn value(&self) -> &str {
        &self.value
    }

    fn equals(&self, other: &Term) -> bool {
        match other {
            Term::BlankNode(bn) => self.value() == bn.value(),
            _ => false,
        }
    }

    fn as_term(self) -> Term {
        Term::BlankNode(self)
    }

    fn to_term(&self) -> Term {
        Term::BlankNode(self.to_owned())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Literal {
    value: String,
    pub language: String,
    pub direction: Option<String>,
    pub datatype: NamedNode,
}

impl Literal {
    pub fn new(
        value: &str,
        language: Option<&str>,
        direction: Option<&str>,
        datatype: Option<&NamedNode>,
    ) -> Self {
        Self {
            value: value.to_owned(),
            language: if let Some(l) = language {
                l.to_owned()
            } else {
                "".to_owned()
            },
            direction: if let Some(d) = direction {
                if let Some(_l) = language {
                    if d != "ltr" && d != "rtl" {
                        panic!(
                            "Literal language string direction should always be either 'ltr' or 'rtl'."
                        );
                    }
                }
                Some(d.to_owned())
            } else {
                Some("".to_owned())
            },
            datatype: if let Some(_l) = language {
                if let Some(_d) = direction {
                    NamedNode {
                        value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#dirLangString"
                            .to_owned(),
                    }
                } else {
                    NamedNode {
                        value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString".to_owned(),
                    }
                }
            } else if let Some(d) = datatype {
                d.to_owned()
            } else {
                NamedNode {
                    value: "http://www.w3.org/2001/XMLSchema#string".to_owned(),
                }
            },
        }
    }
}

impl TermLike for Literal {
    fn value(&self) -> &str {
        &self.value
    }

    fn equals(&self, other: &Term) -> bool {
        match other {
            Term::Literal(l) => {
                self.value() == l.value()
                    && self.language == l.language
                    && self.direction == l.direction
                    && self.datatype.equals_named_node(&l.datatype)
            }
            _ => false,
        }
    }

    fn as_term(self) -> Term {
        Term::Literal(self)
    }

    fn to_term(&self) -> Term {
        Term::Literal(self.to_owned())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Variable {
    value: String,
}

impl Variable {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl TermLike for Variable {
    fn value(&self) -> &str {
        &self.value
    }

    fn equals(&self, other: &Term) -> bool {
        match other {
            Term::Variable(v) => self.value() == v.value(),
            _ => false,
        }
    }

    fn as_term(self) -> Term {
        Term::Variable(self)
    }

    fn to_term(&self) -> Term {
        Term::Variable(self.to_owned())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DefaultGraph {}

impl DefaultGraph {
    pub fn new() -> Self {
        Self {}
    }
}

impl TermLike for DefaultGraph {
    fn value(&self) -> &str {
        ""
    }

    fn equals(&self, other: &Term) -> bool {
        match other {
            Term::DefaultGraph(_) => true,
            _ => false,
        }
    }

    fn as_term(self) -> Term {
        Term::DefaultGraph(self)
    }

    fn to_term(&self) -> Term {
        Term::DefaultGraph(self.to_owned())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Quad {
    pub subject: Term,
    pub predicate: Term,
    pub object: Term,
    pub graph: Term,
}

fn validate_quad_subject_type(term: &Term) {
    match term {
        Term::NamedNode(_) | Term::BlankNode(_) | Term::Variable(_) | Term::Quad(_) => (),
        _ => {
            panic!(
                "A quad subject should be either a NamedNode, a BlankNode, a Variable, or a Quad."
            );
        }
    };
}

fn validate_quad_predicate_type(term: &Term) {
    match term {
        Term::NamedNode(_) | Term::Variable(_) => (),
        _ => {
            panic!("A quad predicate should be either a NamedNode or a Variable.");
        }
    };
}

fn validate_quad_object_type(term: &Term) {
    match term {
        Term::NamedNode(_) | Term::Literal(_) | Term::BlankNode(_) | Term::Variable(_) => (),
        _ => {
            panic!(
                "A quad object should be either a NamedNode, a Literal, a BlankNode, or a Variable."
            );
        }
    };
}

fn validate_quad_graph_type(term: &Term) {
    match term {
        Term::DefaultGraph(_) | Term::NamedNode(_) | Term::BlankNode(_) | Term::Variable(_) => (),
        _ => {
            panic!(
                "A quad graph should be either the DefaultGraph, a NamedNode, a BlankNode, or a Variable."
            );
        }
    };
}

impl Quad {
    pub fn new(subject: &Term, predicate: &Term, object: &Term, graph: Option<&Term>) -> Self {
        validate_quad_subject_type(subject);
        validate_quad_predicate_type(predicate);
        validate_quad_object_type(object);
        if let Some(graph) = graph {
            validate_quad_graph_type(graph);
        };

        Self {
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
}

impl TermLike for Quad {
    fn value(&self) -> &str {
        ""
    }

    fn equals(&self, other: &Term) -> bool {
        match other {
            Term::Quad(q) => {
                self.subject.equals(&q.subject)
                    && self.predicate.equals(&q.predicate)
                    && self.object.equals(&q.object)
                    && self.graph.equals(&q.graph)
            }
            _ => false,
        }
    }

    fn as_term(self) -> Term {
        Term::Quad(Box::new(self))
    }

    fn to_term(&self) -> Term {
        Term::Quad(Box::new(self.to_owned()))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Term {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
    Variable(Variable),
    DefaultGraph(DefaultGraph),
    Quad(Box<Quad>),
}

impl Term {
    pub fn value(&self) -> &str {
        match self {
            Term::NamedNode(nn) => nn.value(),
            Term::BlankNode(bn) => bn.value(),
            Term::Literal(l) => l.value(),
            Term::Variable(v) => v.value(),
            Term::DefaultGraph(dg) => dg.value(),
            Term::Quad(q) => q.value(),
        }
    }

    pub fn equals(&self, other: &Term) -> bool {
        match self {
            Term::NamedNode(nn) => nn.equals(other),
            Term::BlankNode(bn) => bn.equals(other),
            Term::Literal(l) => l.equals(other),
            Term::Variable(v) => v.equals(other),
            Term::DefaultGraph(dg) => dg.equals(other),
            Term::Quad(q) => q.equals(other),
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
