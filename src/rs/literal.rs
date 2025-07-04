use std::fmt::Display;

use crate::rs::named_node::NamedNode;
use crate::rs::{term::Term, term_like::TermLike};

#[derive(Clone, Eq, Debug)]
pub struct Literal {
    value: String,
    language: String,
    direction: Option<LanguageDirection>,
    datatype: NamedNode,
}

impl Literal {
    pub fn new(
        value: &str,
        language: Option<&str>,
        direction: Option<&LanguageDirection>,
        datatype: Option<&NamedNode>,
    ) -> Self {
        Self {
            value: value.to_owned(),
            language: match language {
                Some(l) => l.to_owned(),
                None => "".to_owned(),
            },
            direction: direction.cloned(),
            datatype: if let Some(_l) = language {
                if let Some(_d) = direction {
                    NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#dirLangString")
                } else {
                    NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString")
                }
            } else if let Some(d) = datatype {
                d.to_owned()
            } else {
                NamedNode::new("http://www.w3.org/2001/XMLSchema#string")
            },
        }
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn direction(&self) -> Option<&LanguageDirection> {
        self.direction.as_ref()
    }

    pub fn datatype(&self) -> &NamedNode {
        &self.datatype
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
            && self.language() == other.language()
            && self.direction() == other.direction()
            && self.datatype() == other.datatype()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for Literal {
    fn eq(&self, other: &Term) -> bool {
        match other {
            Term::Literal(l) => self == l,
            _ => false,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Literal> for Term {
    fn eq(&self, other: &Literal) -> bool {
        match self {
            Term::Literal(l) => other == l,
            _ => false,
        }
    }

    fn ne(&self, other: &Literal) -> bool {
        !self.eq(other)
    }
}

impl TermLike for Literal {
    fn value(&self) -> &str {
        &self.value
    }

    fn as_term(self) -> Term {
        Term::Literal(self)
    }

    fn to_term(&self) -> Term {
        Term::Literal(self.to_owned())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LanguageDirection {
    LeftToRight,
    RightToLeft,
}

impl Display for LanguageDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageDirection::LeftToRight => write!(f, "ltr"),
            LanguageDirection::RightToLeft => write!(f, "rtl"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rs::test_data::equality_setup;

    #[test]
    fn same_literals_equal() {
        let data = equality_setup();

        assert_eq!(data.literal_foo_1, data.literal_foo_1);
        assert_eq!(data.literal_foo_1, data.literal_foo_2);
        assert_eq!(data.literal_foo_1, data.term_literal_foo);
        assert_eq!(data.term_literal_foo, data.literal_foo_2);

        let node_1 = Literal::new("foo", Some("gb"), None, None);
        let node_2 = Literal::new("foo", Some("gb"), None, None);

        assert_eq!(node_1, node_2);

        let node_1 = Literal::new(
            "foo",
            Some("en"),
            Some(&LanguageDirection::RightToLeft),
            None,
        );
        let node_2 = Literal::new(
            "foo",
            Some("en"),
            Some(&LanguageDirection::RightToLeft),
            None,
        );

        assert_eq!(node_1, node_2);

        let node_1 = Literal::new(
            "foo",
            Some("en"),
            Some(&LanguageDirection::RightToLeft),
            Some(&NamedNode::new("foo")),
        );
        let node_2 = Literal::new(
            "foo",
            Some("en"),
            Some(&LanguageDirection::RightToLeft),
            Some(&NamedNode::new("foo")),
        );

        assert_eq!(node_1, node_2);
    }

    #[test]
    fn different_literals_not_equal() {
        let data = equality_setup();

        assert_ne!(data.literal_foo_1, data.literal_bar);
        assert_ne!(data.literal_foo_2, data.term_literal_bar);
        assert_ne!(data.term_literal_bar, data.literal_foo_2);
    }

    #[test]
    fn term_inequality_works() {
        let data = equality_setup();

        assert_ne!(data.literal_foo_1, data.term_node_foo);
        assert_ne!(data.literal_foo_1, data.term_blank_foo);
        assert_ne!(data.literal_foo_1, data.term_variable_foo);
        assert_ne!(data.literal_foo_1, data.term_default_graph_1);
        assert_ne!(data.literal_foo_1, data.term_quad_foo);
    }
}
