fn main() {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_toml::language()).unwrap();

    let file = std::fs::read_to_string("./Cargo.toml").unwrap();
    let parsed = parser.parse(&file, None).unwrap();

    dbg!(parsed.root_node().child(1));
}
