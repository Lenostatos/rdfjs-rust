use rdfjs_rust::js;
use rdfjs_rust::rs;

use crate::rs::quad_object::QuadObject;
use crate::rs::quad_predicate::QuadPredicate;
use crate::rs::quad_subject::QuadSubject;

fn main() {
    // JS
    let mut factory = js::DataFactory::new();

    let nn = js::DataFactory::named_node("nn");
    let _bn = factory.blank_node(None);
    let l = js::DataFactory::literal("l", None);
    let _v = js::DataFactory::variable("v");
    let _dg = js::DataFactory::default_graph();

    let q = js::DataFactory::quad(&nn.to_term(), &nn.to_term(), &l.to_term(), None);

    print!("{:#?}", q);

    // RS
    let mut factory = rs::data_factory::DataFactory::new();

    let nn = rs::data_factory::DataFactory::named_node("nn");
    let _bn = factory.blank_node(None);
    let l = rs::data_factory::DataFactory::literal("l", None);
    let _v = rs::data_factory::DataFactory::variable("v");
    let _dg = rs::data_factory::DataFactory::default_graph();

    let q = rs::data_factory::DataFactory::quad(
        &QuadSubject::NamedNode(nn.clone()),
        &QuadPredicate::NamedNode(nn.clone()),
        &QuadObject::Literal(l.clone()),
        None,
    );

    print!("{:#?}", q);
}
