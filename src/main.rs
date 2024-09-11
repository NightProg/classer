use just::{bytecode::*, descriptor::Descriptor};

fn main() {
    let mut parser = just::parser::Parser::new("Main.class");
    //let class_file = parser.parse();
    //println!("{:#?}", class_file);
    let mut builder = just::builder::Builder::new();
    builder.set_class_name("Main");
    let obj = builder.load_class("java/lang/Object");
    builder.set_super_class(obj);
    //let f = builder.add_field("field", Descriptor::Int, FIELD_ACC_PUBLIC);
    let init = builder.add_method("<init>", Descriptor::Function(Vec::new(), Box::new(Descriptor::Void)), METHOD_ACC_PUBLIC);
    builder.build_aload_n(0);
    builder.build_invoke_special(1);
    builder.build_return();
    let main = builder.add_method(
        "main", 
        Descriptor::Function(vec![], Box::new(Descriptor::Void)), 
        METHOD_ACC_STATIC
    );
    let class = builder.load_class("java/lang/System");
    let out = builder.load_field(class, "out", Descriptor::Object("java/io/PrintStream".to_string()));
    builder.build_getstatic(out);
    let string = builder.load_string("Hello, World!");
    builder.build_ldc(string);
    let print_stream_class = builder.load_class("java/io/PrintStream");
    let println = builder.load_method(
        print_stream_class, 
        "println", 
        Descriptor::Function(vec![Descriptor::Object("java/lang/String".to_string())], Box::new(Descriptor::Void))
    );
    builder.build_invoke_virtual(println);
    builder.build_return();
    let c = builder.build();
    std::fs::write("Main.class", c.to_jvm_bytecode()).unwrap();
    println!("{:#?}", c);
}
