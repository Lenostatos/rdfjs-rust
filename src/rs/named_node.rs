use crate::rs::{term::Term, term_like::TermLike};

#[derive(Clone, Eq, Debug)]
pub struct NamedNode {
    value: String,
}

impl NamedNode {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl PartialEq for NamedNode {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for NamedNode {
    fn eq(&self, other: &Term) -> bool {
        match other {
            Term::NamedNode(nn) => self == nn,
            _ => false,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<NamedNode> for Term {
    fn eq(&self, other: &NamedNode) -> bool {
        match self {
            Term::NamedNode(nn) => other == nn,
            _ => false,
        }
    }

    fn ne(&self, other: &NamedNode) -> bool {
        !self.eq(other)
    }
}

impl TermLike for NamedNode {
    fn value(&self) -> &str {
        &self.value
    }

    fn as_term(self) -> Term {
        Term::NamedNode(self)
    }

    fn to_term(&self) -> Term {
        Term::NamedNode(self.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::rs::test_data::equality_setup;

    #[test]
    fn same_value_nodes_equal() {
        let data = equality_setup();

        assert_eq!(data.node_foo_1, data.node_foo_1);
        assert_eq!(data.node_foo_1, data.node_foo_2);
        assert_eq!(data.node_foo_1, data.term_node_foo);
        assert_eq!(data.term_node_foo, data.node_foo_2);
    }

    #[test]
    fn different_value_nodes_not_equal() {
        let data = equality_setup();

        assert_ne!(data.node_foo_1, data.node_bar);
        assert_ne!(data.node_foo_2, data.term_node_bar);
        assert_ne!(data.term_node_bar, data.node_foo_2);
    }

    #[test]
    fn term_inequality_works() {
        let data = equality_setup();

        assert_ne!(data.node_foo_1, data.term_blank_foo);
        assert_ne!(data.node_foo_1, data.term_literal_foo);
        assert_ne!(data.node_foo_1, data.term_variable_foo);
        assert_ne!(data.node_foo_1, data.term_default_graph_1);
        assert_ne!(data.node_foo_1, data.term_quad_foo);
    }
}
