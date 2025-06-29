# RDF/JS mapping to Rust

I wanted to learn some Rust, so I tried to map the [RDF/JS data model specification](https://rdf.js.org/data-model-spec/) to an equivalent Rust API.

This is not production-ready but feel free to contact me, if you have a specific use case for the code and need some adjustments.

The main stuff is in [/src/rs](/src/rs/). The base node types of [`NamedNode`](/src/rs/named_node.rs), [`BlankNode`](/src/rs/blank_node.rs), [`Literal`](/src/rs/literal.rs), [`Variable`](/src/rs/variable.rs), [`DefaultGraph`](/src/rs/default_graph.rs), and [`Quad`](/src/rs/quad.rs) are collected under the [`Term`](/src/rs/term.rs) enum. Quad subject, predicate, object, and graph types are collected under the respective enums [`QuadSubject`](/src/rs/quad_subject.rs), [`QuadPredicate`](/src/rs/quad_predicate.rs), [`QuadObject`](/src/rs/quad_object.rs), and [`QuadGraph`](/src/rs/quad_graph.rs).

In [/src/js.rs](/src/js.rs) I tried to mimic the JS API as closely as possible without any regard for idiomatic patterns. Probably not useful, but an interesting learning experience 🙂
