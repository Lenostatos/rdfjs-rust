use crate::rs::{
    blank_node::BlankNode, default_graph::DefaultGraph, literal::Literal, named_node::NamedNode,
    quad::Quad, quad_object::QuadObject, quad_predicate::QuadPredicate, quad_subject::QuadSubject,
    term::Term, variable::Variable,
};

pub struct EqualityData {
    pub node_foo_1: NamedNode,
    pub node_foo_2: NamedNode,
    pub node_bar: NamedNode,

    pub blank_foo_1: BlankNode,
    pub blank_foo_2: BlankNode,
    pub blank_bar: BlankNode,

    pub literal_foo_1: Literal,
    pub literal_foo_2: Literal,
    pub literal_bar: Literal,

    pub variable_foo_1: Variable,
    pub variable_foo_2: Variable,
    pub variable_bar: Variable,

    pub default_graph_1: DefaultGraph,
    pub default_graph_2: DefaultGraph,

    pub quad_1_1: Quad,
    pub quad_1_2: Quad,
    pub quad_2: Quad,

    pub term_node_foo: Term,
    pub term_node_bar: Term,

    pub term_blank_foo: Term,
    pub term_blank_bar: Term,

    pub term_literal_foo: Term,
    pub term_literal_bar: Term,

    pub term_variable_foo: Term,
    pub term_variable_bar: Term,

    pub term_default_graph_1: Term,
    pub term_default_graph_2: Term,

    pub term_quad_foo: Term,
    pub term_quad_bar: Term,
}

pub fn equality_setup() -> EqualityData {
    EqualityData {
        node_foo_1: NamedNode::new("foo"),
        node_foo_2: NamedNode::new("foo"),
        node_bar: NamedNode::new("bar"),

        blank_foo_1: BlankNode::new("foo"),
        blank_foo_2: BlankNode::new("foo"),
        blank_bar: BlankNode::new("bar"),

        literal_foo_1: Literal::new("foo", None, None, None),
        literal_foo_2: Literal::new("foo", None, None, None),
        literal_bar: Literal::new("bar", None, None, None),

        variable_foo_1: Variable::new("foo"),
        variable_foo_2: Variable::new("foo"),
        variable_bar: Variable::new("bar"),

        default_graph_1: DefaultGraph::new(),
        default_graph_2: DefaultGraph::new(),

        quad_1_1: Quad::new(
            &QuadSubject::NamedNode(NamedNode::new("foo")),
            &QuadPredicate::NamedNode(NamedNode::new("foo")),
            &QuadObject::NamedNode(NamedNode::new("foo")),
            None,
        ),
        quad_1_2: Quad::new(
            &QuadSubject::NamedNode(NamedNode::new("foo")),
            &QuadPredicate::NamedNode(NamedNode::new("foo")),
            &QuadObject::NamedNode(NamedNode::new("foo")),
            None,
        ),
        quad_2: Quad::new(
            &QuadSubject::NamedNode(NamedNode::new("foo")),
            &QuadPredicate::NamedNode(NamedNode::new("foo")),
            &QuadObject::NamedNode(NamedNode::new("bar")),
            None,
        ),

        term_node_foo: Term::NamedNode(NamedNode::new("foo")),
        term_node_bar: Term::NamedNode(NamedNode::new("bar")),

        term_blank_foo: Term::BlankNode(BlankNode::new("foo")),
        term_blank_bar: Term::BlankNode(BlankNode::new("bar")),

        term_literal_foo: Term::Literal(Literal::new("foo", None, None, None)),
        term_literal_bar: Term::Literal(Literal::new("bar", None, None, None)),

        term_variable_foo: Term::Variable(Variable::new("foo")),
        term_variable_bar: Term::Variable(Variable::new("bar")),

        term_default_graph_1: Term::DefaultGraph(DefaultGraph::new()),
        term_default_graph_2: Term::DefaultGraph(DefaultGraph::new()),

        term_quad_foo: Term::Quad(Box::new(Quad::new(
            &QuadSubject::NamedNode(NamedNode::new("foo")),
            &QuadPredicate::NamedNode(NamedNode::new("foo")),
            &QuadObject::NamedNode(NamedNode::new("foo")),
            None,
        ))),
        term_quad_bar: Term::Quad(Box::new(Quad::new(
            &QuadSubject::NamedNode(NamedNode::new("bar")),
            &QuadPredicate::NamedNode(NamedNode::new("bar")),
            &QuadObject::NamedNode(NamedNode::new("bar")),
            None,
        ))),
    }
}
