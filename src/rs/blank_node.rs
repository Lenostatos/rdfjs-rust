use crate::rs::{term::Term, term_like::TermLike};

#[derive(Clone, Eq, Debug)]
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

impl PartialEq for BlankNode {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for BlankNode {
    fn eq(&self, other: &Term) -> bool {
        match other {
            Term::BlankNode(bn) => self == bn,
            _ => false,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<BlankNode> for Term {
    fn eq(&self, other: &BlankNode) -> bool {
        match self {
            Term::BlankNode(bn) => other == bn,
            _ => false,
        }
    }

    fn ne(&self, other: &BlankNode) -> bool {
        !self.eq(other)
    }
}

impl TermLike for BlankNode {
    fn value(&self) -> &str {
        &self.value
    }

    fn as_term(self) -> Term {
        Term::BlankNode(self)
    }

    fn to_term(&self) -> Term {
        Term::BlankNode(self.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::rs::test_data::equality_setup;

    #[test]
    fn same_value_nodes_equal() {
        let data = equality_setup();

        assert_eq!(data.blank_foo_1, data.blank_foo_1);
        assert_eq!(data.blank_foo_1, data.blank_foo_2);
        assert_eq!(data.blank_foo_1, data.term_blank_foo);
        assert_eq!(data.term_blank_foo, data.blank_foo_2);
    }

    #[test]
    fn different_value_nodes_not_equal() {
        let data = equality_setup();

        assert_ne!(data.blank_foo_1, data.blank_bar);
        assert_ne!(data.blank_foo_2, data.term_blank_bar);
        assert_ne!(data.term_blank_bar, data.blank_foo_2);
    }

    #[test]
    fn term_inequality_works() {
        let data = equality_setup();

        assert_ne!(data.blank_foo_1, data.term_node_foo);
        assert_ne!(data.blank_foo_1, data.term_literal_foo);
        assert_ne!(data.blank_foo_1, data.term_variable_foo);
        assert_ne!(data.blank_foo_1, data.term_default_graph_1);
        assert_ne!(data.blank_foo_1, data.term_quad_foo);
    }
}
