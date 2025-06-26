use crate::rs::named_node::NamedNode;
use crate::rs::{term::Term, term_like::TermLike};

#[derive(Clone, Eq, Debug)]
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
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
            && self.language == other.language
            && self.direction == other.direction
            && self.datatype == other.datatype
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
