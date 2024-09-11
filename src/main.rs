use just::{bytecode::*, descriptor::Descriptor};

fn main() {
    let mut parser = just::parser::Parser::new("Main.class");
    let class_file = parser.parse();
    if let CpInfoType::Utf8 { length, bytes } = class_file.constant_pool[30 - 1].clone().info {
        println!("{}", bytes);
        println!(
            "{:?}",
            Descriptor::Function(
                vec![Descriptor::Int, Descriptor::Int],
                Box::new(Descriptor::Int)
            )
            .serialize()
        );
    }
    println!("{:#?}", class_file);
    std::fs::write("tmp/Main.class", class_file.to_jvm_bytecode()).unwrap();
}
