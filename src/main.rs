use rdfjs_rust::DataFactory;

fn main() {
    let mut factory = DataFactory::new();

    let nn = DataFactory::named_node("nn");
    let bn = factory.blank_node(None);
    let l = DataFactory::literal("l", None);
    let v = DataFactory::variable("v");
    let dg = DataFactory::default_graph();

    let q = DataFactory::quad(&nn.to_term(), &nn.to_term(), &l.to_term(), None);

    print!("{:#?}", q);
}
