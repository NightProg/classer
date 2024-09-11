#[derive(Debug, Clone, PartialEq)]
pub enum Descriptor {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    Array(Box<Descriptor>),
    Object(String),
    Function(Vec<Descriptor>, Box<Descriptor>),
    Void,
}

impl Descriptor {
    pub fn parse(descriptor: &str) -> (Descriptor, String) {
        let mut chars = descriptor.chars();
        (
            match chars.next().unwrap() {
                'B' => Descriptor::Byte,
                'C' => Descriptor::Char,
                'D' => Descriptor::Double,
                'F' => Descriptor::Float,
                'I' => Descriptor::Int,
                'J' => Descriptor::Long,
                'S' => Descriptor::Short,
                'Z' => Descriptor::Boolean,
                'L' => {
                    let mut object = String::new();
                    loop {
                        match chars.next().unwrap() {
                            ';' => break,
                            c => object.push(c),
                        }
                    }
                    Descriptor::Object(object)
                }
                '(' => {
                    let mut parameters = Vec::new();
                    loop {
                        let c = chars.next().unwrap();
                        if c == ')' {
                            break;
                        }
                        let mut s = String::new();
                        s.push(c);
                        s.extend(chars.clone());
                        let d = Descriptor::parse(&s);
                        parameters.push(d.0);
                        let var_name = Box::leak(Box::new(d.1.clone()));
                        chars = var_name.chars();
                    }
                    Descriptor::Function(parameters, Box::new(Descriptor::parse(chars.as_str()).0))
                }
                '[' => {
                    let res = Descriptor::parse(chars.as_str());
                    let arr = Descriptor::Array(Box::new(res.0));
                    return (arr, res.1);
                }
                'V' => Descriptor::Void,
                _ => panic!("Invalid descriptor"),
            },
            chars.as_str().to_string(),
        )
    }

    pub fn serialize(&self) -> String {
        match self {
            Descriptor::Byte => "B".to_string(),
            Descriptor::Char => "C".to_string(),
            Descriptor::Double => "D".to_string(),
            Descriptor::Float => "F".to_string(),
            Descriptor::Int => "I".to_string(),
            Descriptor::Long => "J".to_string(),
            Descriptor::Short => "S".to_string(),
            Descriptor::Boolean => "Z".to_string(),
            Descriptor::Object(object) => format!("L{};", object),
            Descriptor::Function(parameters, return_type) => {
                let mut s = "(".to_string();
                for p in parameters {
                    s.push_str(&p.serialize());
                }
                s.push(')');
                s.push_str(&return_type.serialize());
                s
            }
            Descriptor::Array(inner) => format!("[{}", inner.serialize()),
            Descriptor::Void => "V".to_string(),
        }
    }
}
