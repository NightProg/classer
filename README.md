# classer - A Rust JVM tool

This is a crate for manipulating Java class files.

## Example

### Read a class file

```rust
use classer::parser::Parser;

fn main() {
    let class_file: classer::bytecode::ClassFile = Parser::new("HelloWorld.class").parse();
    println!("{:?}", class_file);
}
```

### Write a class file

```rust
use classer::parser::Parser;

fn main() {
    let class_file = Parser::new().parse("Hello.class");
    class_file.write("HelloWorld.class");
}
```

### Generate a class file

```rust
use classer::builder::Builder;
use classer::descriptor::Descriptor;
use classer::bytecode::*;

fn main() {
    let mut builder = Builder::new();
    let obj = builder.load_class("java/lang/Object");
    let init = builder.load_method(obj, "<init>", Descriptor::Function(vec![], Box::new(Descriptor::Void)), METHOD_ACC_PUBLIC);
    let system = builder.load_class("java/lang/System");
    let out = builder.load_field(system, "out", Descriptor::Object("java/io/PrintStream".to_string()));
    let print_stream = builder.load_class("java/io/PrintStream");
    let println = builder.load_method(print_stream, "println", Descriptor::Function(vec![Descriptor::Object("java/lang/String".to_string())], Box::new(Descriptor::Void)), METHOD_ACC_PUBLIC);
    let hello_world = builder.load_string("Hello, World");
    builder.set_class_name("HelloWorld");
    builder.set_super_class(obj);

    builder.add_field("hello", Descriptor::Object("java/lang/String".to_string()));
    builder.add_method("<init>", Descriptor::Function(vec![], Box::new(Descriptor::Void)), METHOD_ACC_PUBLIC);
    builder.build_aload_n(0);
    builder.build_invokespecial(init);
    builder.build_return();
    
    builder.add_method(
      "main", 
      Descriptor::Function(
          vec![
              Descriptor::Array(
                  Box::new(
                      Descriptor::Object("java/lang/String".to_string())
                  )
              )
          ], Box::new(Descriptor::Void)), METHOD_ACC_PUBLIC | METHOD_ACC_STATIC
    );
    
    builder.build_getstatic(out);
    builder.build_ldc(hello_world);
    builder.build_invokevirtual(println);
    builder.build_return();
    
    let class_file = builder.build();
    
    class_file.write("HelloWorld.class");
}
```

```bash
java HelloWorld # will print Hello, World


```
