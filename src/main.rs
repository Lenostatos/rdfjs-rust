#[derive(PartialEq, Eq)]
enum TermType {
    NamedNode,
    BlankNode,
    Literal,
    Variable,
    DefaultGraph
}

struct NamedNode {
    value: String
}

struct BlankNode {
    value: String
}

struct Literal {
    value: String,
    language: String,
    datatype: NamedNode
}

struct Variable {
    value: String
}

struct DefaultGraph {
    value: String
}

enum Term {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
    Variable(Variable),
    DefaultGraph(DefaultGraph),
}

impl Term {
    fn term_type(&self) -> &TermType {
        match self {
            Term::BlankNode(_) => &TermType::BlankNode,
            Term::DefaultGraph(_) => &TermType::DefaultGraph,
            Term::Literal(_) => &TermType::Literal,
            Term::NamedNode(_) => &TermType::NamedNode,
            Term::Variable(_) => &TermType::Variable,
        }
    }

    fn value(&self) -> &str {
        match self {
            Term::BlankNode(t) => &t.value,
            Term::DefaultGraph(t) => &t.value,
            Term::Literal(t) => &t.value,
            Term::NamedNode(t) => &t.value,
            Term::Variable(t) => &t.value,
        }
    }

    fn equals(&self, other: &Self) -> bool {
        match self {
            Term::Literal(t) => match other {
                Term::Literal(o) => {
                    t.value == o.value &&
                        t.datatype.value == o.datatype.value &&
                        t.language == t.language
                },
                _ => false
            },
            _ => self.term_type() == other.term_type() && 
                self.value() == other.value()
        }
    }
}

struct Quad {
    subject: Term,
    predicate: Term,
    object: Term,
    graph: Term
}

impl Quad {
    fn equals(&self, other: &Quad) -> bool {
        self.subject.equals(&other.subject) &&
        self.predicate.equals(&other.predicate) &&
        self.object.equals(&other.object) &&
        self.graph.equals(&other.graph)
    }
}

fn main() {
    
}