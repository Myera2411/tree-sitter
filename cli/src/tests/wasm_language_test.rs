use crate::tests::helpers::fixtures::WASM_DIR;
use lazy_static::lazy_static;
use std::fs;
use tree_sitter::{wasmtime::Engine, Parser, WasmStore};

lazy_static! {
    static ref ENGINE: Engine = Engine::default();
}

#[test]
fn test_wasm_store() {
    let mut store = WasmStore::new(ENGINE.clone());
    let mut parser = Parser::new();

    let wasm_cpp = fs::read(&WASM_DIR.join(format!("tree-sitter-cpp.wasm"))).unwrap();
    let wasm_rs = fs::read(&WASM_DIR.join(format!("tree-sitter-rust.wasm"))).unwrap();
    let wasm_rb = fs::read(&WASM_DIR.join(format!("tree-sitter-ruby.wasm"))).unwrap();

    let language_rust = store.load_language("rust", &wasm_rs);
    let language_cpp = store.load_language("cpp", &wasm_cpp);
    let language_ruby = store.load_language("ruby", &wasm_rb);
    parser.set_wasm_store(store).unwrap();

    for _ in 0..2 {
        parser.set_language(language_cpp).unwrap();
        let tree = parser.parse("A<B> c = d();", None).unwrap();
        assert_eq!(
            tree.root_node().to_sexp(),
            "(translation_unit (declaration type: (template_type name: (type_identifier) arguments: (template_argument_list (type_descriptor type: (type_identifier)))) declarator: (init_declarator declarator: (identifier) value: (call_expression function: (identifier) arguments: (argument_list)))))"
        );

        parser.set_language(language_rust).unwrap();
        let tree = parser.parse("const A: B = c();", None).unwrap();
        assert_eq!(
            tree.root_node().to_sexp(),
            "(source_file (const_item name: (identifier) type: (type_identifier) value: (call_expression function: (identifier) arguments: (arguments))))"
        );

        parser.set_language(language_ruby).unwrap();
        let tree = parser.parse("class A; end", None).unwrap();
        assert_eq!(
            tree.root_node().to_sexp(),
            "(program (class name: (constant)))"
        );
    }
}
