use rdfjs_rust::js;
use rdfjs_rust::rs;

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
    let mut factory = rs::DataFactory::new();

    let nn = rs::DataFactory::named_node("nn");
    let _bn = factory.blank_node(None);
    let l = rs::DataFactory::literal("l", None);
    let _v = rs::DataFactory::variable("v");
    let _dg = rs::DataFactory::default_graph();

    let q = rs::DataFactory::quad(
        &rs::QuadSubject::NamedNode(nn.clone()),
        &rs::QuadPredicate::NamedNode(nn.clone()),
        &rs::QuadObject::Literal(l.clone()),
        None,
    );

    print!("{:#?}", q);
}
