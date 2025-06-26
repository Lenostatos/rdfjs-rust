pub trait TermLike {
    fn value(&self) -> &str;

    fn as_term(self) -> Term;
    fn to_term(&self) -> Term;
}

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

#[derive(Clone, Eq, Debug)]
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

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for Variable {
    fn eq(&self, other: &Term) -> bool {
        match other {
            Term::Variable(v) => self == v,
            _ => false,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Variable> for Term {
    fn eq(&self, other: &Variable) -> bool {
        match self {
            Term::Variable(v) => other == v,
            _ => false,
        }
    }

    fn ne(&self, other: &Variable) -> bool {
        !self.eq(other)
    }
}

impl TermLike for Variable {
    fn value(&self) -> &str {
        &self.value
    }

    fn as_term(self) -> Term {
        Term::Variable(self)
    }

    fn to_term(&self) -> Term {
        Term::Variable(self.to_owned())
    }
}

#[derive(Clone, Eq, Debug)]
pub struct DefaultGraph {}

impl DefaultGraph {
    pub fn new() -> Self {
        Self {}
    }
}

impl PartialEq for DefaultGraph {
    fn eq(&self, _other: &Self) -> bool {
        true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for DefaultGraph {
    fn eq(&self, other: &Term) -> bool {
        match other {
            Term::DefaultGraph(dg) => self == dg,
            _ => false,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<DefaultGraph> for Term {
    fn eq(&self, other: &DefaultGraph) -> bool {
        match self {
            Term::DefaultGraph(dg) => other == dg,
            _ => false,
        }
    }

    fn ne(&self, other: &DefaultGraph) -> bool {
        !self.eq(other)
    }
}

impl TermLike for DefaultGraph {
    fn value(&self) -> &str {
        ""
    }

    fn as_term(self) -> Term {
        Term::DefaultGraph(self)
    }

    fn to_term(&self) -> Term {
        Term::DefaultGraph(self.to_owned())
    }
}

#[derive(Clone, Eq, Debug)]
pub enum QuadSubject {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Variable(Variable),
    Quad(Box<Quad>),
}

impl PartialEq for QuadSubject {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::NamedNode(self_nn) => match other {
                Self::NamedNode(other_nn) => self_nn == other_nn,
                _ => false,
            },
            Self::BlankNode(self_bn) => match other {
                Self::BlankNode(other_bn) => self_bn == other_bn,
                _ => false,
            },
            Self::Variable(self_v) => match other {
                Self::Variable(other_v) => self_v == other_v,
                _ => false,
            },
            Self::Quad(self_q) => match other {
                Self::Quad(other_q) => self_q == other_q,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for QuadSubject {
    fn eq(&self, other: &Term) -> bool {
        match self {
            QuadSubject::NamedNode(named_node) => named_node == other,
            QuadSubject::BlankNode(blank_node) => blank_node == other,
            QuadSubject::Variable(variable) => variable == other,
            QuadSubject::Quad(quad) => **quad == *other,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<QuadSubject> for Term {
    fn eq(&self, other: &QuadSubject) -> bool {
        match other {
            QuadSubject::NamedNode(named_node) => named_node == self,
            QuadSubject::BlankNode(blank_node) => blank_node == self,
            QuadSubject::Variable(variable) => variable == self,
            QuadSubject::Quad(quad) => **quad == *self,
        }
    }

    fn ne(&self, other: &QuadSubject) -> bool {
        !self.eq(other)
    }
}

impl TermLike for QuadSubject {
    fn value(&self) -> &str {
        match self {
            QuadSubject::NamedNode(named_node) => named_node.value(),
            QuadSubject::BlankNode(blank_node) => blank_node.value(),
            QuadSubject::Variable(variable) => variable.value(),
            QuadSubject::Quad(quad) => quad.value(),
        }
    }

    fn as_term(self) -> Term {
        match self {
            QuadSubject::NamedNode(named_node) => named_node.as_term(),
            QuadSubject::BlankNode(blank_node) => blank_node.as_term(),
            QuadSubject::Variable(variable) => variable.as_term(),
            QuadSubject::Quad(quad) => quad.as_term(),
        }
    }

    fn to_term(&self) -> Term {
        match self {
            QuadSubject::NamedNode(named_node) => named_node.to_term(),
            QuadSubject::BlankNode(blank_node) => blank_node.to_term(),
            QuadSubject::Variable(variable) => variable.to_term(),
            QuadSubject::Quad(quad) => quad.to_term(),
        }
    }
}

#[derive(Clone, Eq, Debug)]
pub enum QuadPredicate {
    NamedNode(NamedNode),
    Variable(Variable),
}

impl PartialEq for QuadPredicate {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::NamedNode(self_nn) => match other {
                Self::NamedNode(other_nn) => self_nn == other_nn,
                _ => false,
            },
            Self::Variable(self_v) => match other {
                Self::Variable(other_v) => self_v == other_v,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for QuadPredicate {
    fn eq(&self, other: &Term) -> bool {
        match self {
            QuadPredicate::NamedNode(named_node) => named_node == other,
            QuadPredicate::Variable(variable) => variable == other,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<QuadPredicate> for Term {
    fn eq(&self, other: &QuadPredicate) -> bool {
        match other {
            QuadPredicate::NamedNode(named_node) => named_node == self,
            QuadPredicate::Variable(variable) => variable == self,
        }
    }

    fn ne(&self, other: &QuadPredicate) -> bool {
        !self.eq(other)
    }
}

impl TermLike for QuadPredicate {
    fn value(&self) -> &str {
        match self {
            QuadPredicate::NamedNode(named_node) => named_node.value(),
            QuadPredicate::Variable(variable) => variable.value(),
        }
    }

    fn as_term(self) -> Term {
        match self {
            QuadPredicate::NamedNode(named_node) => named_node.as_term(),
            QuadPredicate::Variable(variable) => variable.as_term(),
        }
    }

    fn to_term(&self) -> Term {
        match self {
            QuadPredicate::NamedNode(named_node) => named_node.to_term(),
            QuadPredicate::Variable(variable) => variable.to_term(),
        }
    }
}

#[derive(Clone, Eq, Debug)]
pub enum QuadObject {
    NamedNode(NamedNode),
    Literal(Literal),
    BlankNode(BlankNode),
    Variable(Variable),
}

impl PartialEq for QuadObject {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::NamedNode(self_nn) => match other {
                Self::NamedNode(other_nn) => self_nn == other_nn,
                _ => false,
            },
            Self::BlankNode(self_bn) => match other {
                Self::BlankNode(other_bn) => self_bn == other_bn,
                _ => false,
            },
            Self::Variable(self_v) => match other {
                Self::Variable(other_v) => self_v == other_v,
                _ => false,
            },
            Self::Literal(self_q) => match other {
                Self::Literal(other_q) => self_q == other_q,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for QuadObject {
    fn eq(&self, other: &Term) -> bool {
        match self {
            QuadObject::NamedNode(named_node) => named_node == other,
            QuadObject::Literal(literal) => literal == other,
            QuadObject::BlankNode(blank_node) => blank_node == other,
            QuadObject::Variable(variable) => variable == other,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<QuadObject> for Term {
    fn eq(&self, other: &QuadObject) -> bool {
        match other {
            QuadObject::NamedNode(named_node) => named_node == self,
            QuadObject::Literal(literal) => literal == self,
            QuadObject::BlankNode(blank_node) => blank_node == self,
            QuadObject::Variable(variable) => variable == self,
        }
    }

    fn ne(&self, other: &QuadObject) -> bool {
        !self.eq(other)
    }
}

impl TermLike for QuadObject {
    fn value(&self) -> &str {
        match self {
            QuadObject::NamedNode(named_node) => named_node.value(),
            QuadObject::Literal(literal) => literal.value(),
            QuadObject::BlankNode(blank_node) => blank_node.value(),
            QuadObject::Variable(variable) => variable.value(),
        }
    }

    fn as_term(self) -> Term {
        match self {
            QuadObject::NamedNode(named_node) => named_node.as_term(),
            QuadObject::Literal(literal) => literal.as_term(),
            QuadObject::BlankNode(blank_node) => blank_node.as_term(),
            QuadObject::Variable(variable) => variable.as_term(),
        }
    }

    fn to_term(&self) -> Term {
        match self {
            QuadObject::NamedNode(named_node) => named_node.to_term(),
            QuadObject::Literal(literal) => literal.to_term(),
            QuadObject::BlankNode(blank_node) => blank_node.to_term(),
            QuadObject::Variable(variable) => variable.to_term(),
        }
    }
}

#[derive(Clone, Eq, Debug)]
pub enum QuadGraph {
    DefaultGraph(DefaultGraph),
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Variable(Variable),
}

impl PartialEq for QuadGraph {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::NamedNode(self_nn) => match other {
                Self::NamedNode(other_nn) => self_nn == other_nn,
                _ => false,
            },
            Self::BlankNode(self_bn) => match other {
                Self::BlankNode(other_bn) => self_bn == other_bn,
                _ => false,
            },
            Self::Variable(self_v) => match other {
                Self::Variable(other_v) => self_v == other_v,
                _ => false,
            },
            Self::DefaultGraph(self_q) => match other {
                Self::DefaultGraph(other_q) => self_q == other_q,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for QuadGraph {
    fn eq(&self, other: &Term) -> bool {
        match self {
            QuadGraph::NamedNode(named_node) => named_node == other,
            QuadGraph::BlankNode(blank_node) => blank_node == other,
            QuadGraph::Variable(variable) => variable == other,
            QuadGraph::DefaultGraph(default_graph) => default_graph == other,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<QuadGraph> for Term {
    fn eq(&self, other: &QuadGraph) -> bool {
        match other {
            QuadGraph::NamedNode(named_node) => named_node == self,
            QuadGraph::BlankNode(blank_node) => blank_node == self,
            QuadGraph::Variable(variable) => variable == self,
            QuadGraph::DefaultGraph(default_graph) => default_graph == self,
        }
    }

    fn ne(&self, other: &QuadGraph) -> bool {
        !self.eq(other)
    }
}

impl TermLike for QuadGraph {
    fn value(&self) -> &str {
        match self {
            QuadGraph::NamedNode(named_node) => named_node.value(),
            QuadGraph::BlankNode(blank_node) => blank_node.value(),
            QuadGraph::Variable(variable) => variable.value(),
            QuadGraph::DefaultGraph(default_graph) => default_graph.value(),
        }
    }

    fn as_term(self) -> Term {
        match self {
            QuadGraph::NamedNode(named_node) => named_node.as_term(),
            QuadGraph::BlankNode(blank_node) => blank_node.as_term(),
            QuadGraph::Variable(variable) => variable.as_term(),
            QuadGraph::DefaultGraph(default_graph) => default_graph.as_term(),
        }
    }

    fn to_term(&self) -> Term {
        match self {
            QuadGraph::NamedNode(named_node) => named_node.to_term(),
            QuadGraph::BlankNode(blank_node) => blank_node.to_term(),
            QuadGraph::Variable(variable) => variable.to_term(),
            QuadGraph::DefaultGraph(default_graph) => default_graph.to_term(),
        }
    }
}

#[derive(Clone, Eq, Debug)]
pub struct Quad {
    pub subject: QuadSubject,
    pub predicate: QuadPredicate,
    pub object: QuadObject,
    pub graph: QuadGraph,
}

impl Quad {
    pub fn new(
        subject: &QuadSubject,
        predicate: &QuadPredicate,
        object: &QuadObject,
        graph: Option<&QuadGraph>,
    ) -> Self {
        Self {
            subject: subject.to_owned(),
            predicate: predicate.to_owned(),
            object: object.to_owned(),
            graph: if let Some(g) = graph {
                g.to_owned()
            } else {
                QuadGraph::DefaultGraph(DefaultGraph::new())
            },
        }
    }
}

impl PartialEq for Quad {
    fn eq(&self, other: &Self) -> bool {
        self.subject == other.subject
            && self.predicate == other.predicate
            && self.object == other.object
            && self.graph == other.graph
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Term> for Quad {
    fn eq(&self, other: &Term) -> bool {
        match other {
            Term::Quad(other_q) => *self == **other_q,
            _ => false,
        }
    }

    fn ne(&self, other: &Term) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<Quad> for Term {
    fn eq(&self, other: &Quad) -> bool {
        match self {
            Term::Quad(self_q) => *other == **self_q,
            _ => false,
        }
    }

    fn ne(&self, other: &Quad) -> bool {
        !self.eq(other)
    }
}

impl TermLike for Quad {
    fn value(&self) -> &str {
        ""
    }

    fn as_term(self) -> Term {
        Term::Quad(Box::new(self))
    }

    fn to_term(&self) -> Term {
        Term::Quad(Box::new(self.to_owned()))
    }
}

#[derive(Clone, Eq, Debug)]
pub enum Term {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
    Variable(Variable),
    DefaultGraph(DefaultGraph),
    Quad(Box<Quad>),
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Term::NamedNode(self_nn) => match other {
                Term::NamedNode(other_nn) => self_nn == other_nn,
                _ => false,
            },
            Term::BlankNode(self_bn) => match other {
                Term::BlankNode(other_bn) => self_bn == other_bn,
                _ => false,
            },
            Term::Literal(self_l) => match other {
                Term::Literal(other_l) => self_l == other_l,
                _ => false,
            },
            Term::Variable(self_v) => match other {
                Term::Variable(other_v) => self_v == other_v,
                _ => false,
            },
            Term::DefaultGraph(self_dg) => match other {
                Term::DefaultGraph(other_dg) => self_dg == other_dg,
                _ => false,
            },
            Term::Quad(self_q) => match other {
                Term::Quad(other_q) => self_q == other_q,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl TermLike for Term {
    fn value(&self) -> &str {
        match self {
            Term::NamedNode(nn) => nn.value(),
            Term::BlankNode(bn) => bn.value(),
            Term::Literal(l) => l.value(),
            Term::Variable(v) => v.value(),
            Term::DefaultGraph(dg) => dg.value(),
            Term::Quad(q) => q.value(),
        }
    }

    fn as_term(self) -> Term {
        self
    }

    fn to_term(&self) -> Term {
        self.clone()
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
