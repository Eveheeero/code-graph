fn main() {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_rust::language()).unwrap();

    let file = RustFile::new("./src/main.rs").unwrap();
    let tree = file.get_tree();

    let mut cursor = tree.walk();
    cursor.goto_first_child();

    if file.is_function(&cursor.node()) {
        dbg!(file.get_function_name(&cursor.node()).unwrap());
    }
}

struct RustFile {
    path: std::path::PathBuf,
    content: String,
}

impl RustFile {
    fn new<T>(path: T) -> Result<Self, std::io::Error>
    where
        T: Into<std::path::PathBuf>,
    {
        let path = path.into();
        let content = std::fs::read_to_string(&path)?;
        Ok(Self { path, content })
    }

    fn get_tree(&self) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_rust::language()).unwrap();
        parser.parse(&self.content, None).unwrap()
    }

    #[inline]
    fn is_function(&self, node: &tree_sitter::Node) -> bool {
        node.kind() == "function_item"
    }

    #[inline]
    fn get_function_name(&self, node: &tree_sitter::Node) -> Result<String, &'static str> {
        let range = node
            .child(1)
            .and_then(|n| Some((n.range().start_byte, n.range().end_byte)));
        if let Some(range) = range {
            Ok(self.content[range.0..range.1].to_string())
        } else {
            Err("No function name found")
        }
    }
}
