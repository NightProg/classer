fn main() {
    let mut parser = just::parser::Parser::new("Main.class");
    let class_file = parser.parse();
    println!("{:#?}", class_file);
}
