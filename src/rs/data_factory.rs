use crate::rs::blank_node::BlankNode;
use crate::rs::default_graph::DefaultGraph;
use crate::rs::literal::Literal;
use crate::rs::named_node::NamedNode;
use crate::rs::quad::Quad;
use crate::rs::variable::Variable;

use crate::rs::quad_graph::QuadGraph;
use crate::rs::quad_object::QuadObject;
use crate::rs::quad_predicate::QuadPredicate;
use crate::rs::quad_subject::QuadSubject;

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

    pub fn quad(
        subject: &QuadSubject,
        predicate: &QuadPredicate,
        object: &QuadObject,
        graph: Option<&QuadGraph>,
    ) -> Quad {
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
