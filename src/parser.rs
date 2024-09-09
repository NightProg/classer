use crate::{
    bytecode::{
        AccessFlag, AttributeInfo, AttributeInfoKind, ClassFile, CpInfo, CpInfoType,
        ExceptionTable, FieldInfo, MethodInfo,
    },
    flags::Flags,
    reader::Reader,
};

pub const CONSTANT_CLASS: u8 = 7;
pub const CONSTANT_FIELDREF: u8 = 9;
pub const CONSTANT_METHODREF: u8 = 10;
pub const CONSTANT_INTERFACEMETHODREF: u8 = 11;
pub const CONSTANT_STRING: u8 = 8;
pub const CONSTANT_INTEGER: u8 = 3;
pub const CONSTANT_FLOAT: u8 = 4;
pub const CONSTANT_LONG: u8 = 5;
pub const CONSTANT_DOUBLE: u8 = 6;
pub const CONSTANT_NAMEANDTYPE: u8 = 12;
pub const CONSTANT_UTF8: u8 = 1;
pub const CONSTANT_METHODHANDLE: u8 = 15;
pub const CONSTANT_METHODTYPE: u8 = 16;
pub const CONSTANT_INVOKEDYNAMIC: u8 = 18;

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    pub reader: Reader,
}

impl Parser {
    pub fn new(class_file: impl AsRef<std::path::Path>) -> Parser {
        let file = std::fs::read(class_file).unwrap();
        Parser {
            reader: Reader::new(file),
        }
    }

    pub fn parse(&mut self) -> ClassFile {
        let magic = self.reader.read_int4();
        let minor_version = self.reader.read_int2();
        let major_version = self.reader.read_int2();
        let constant_pool_count = self.reader.read_int2();
        let mut class_file = ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            ..Default::default()
        };
        for _ in 1..constant_pool_count {
            let tag = self.reader.read_int1();
            match tag {
                CONSTANT_CLASS => class_file.constant_pool.push(CpInfo {
                    tag,
                    info: CpInfoType::Class {
                        name_index: self.reader.read_int2(),
                    },
                }),
                CONSTANT_METHODREF => class_file.constant_pool.push(CpInfo {
                    tag,
                    info: CpInfoType::Methodref {
                        class_index: self.reader.read_int2(),
                        name_and_type_index: self.reader.read_int2(),
                    },
                }),
                CONSTANT_FIELDREF => class_file.constant_pool.push(CpInfo {
                    tag,
                    info: CpInfoType::Fieldref {
                        class_index: self.reader.read_int2(),
                        name_and_type_index: self.reader.read_int2(),
                    },
                }),
                CONSTANT_NAMEANDTYPE => class_file.constant_pool.push(CpInfo {
                    tag,
                    info: CpInfoType::NameAndType {
                        name_index: self.reader.read_int2(),
                        descriptor_index: self.reader.read_int2(),
                    },
                }),
                CONSTANT_UTF8 => {
                    let length = self.reader.read_int2();
                    class_file.constant_pool.push(CpInfo {
                        tag,
                        info: CpInfoType::Utf8 {
                            length,
                            bytes: self.reader.read_string(length as usize),
                        },
                    })
                }
                CONSTANT_STRING => class_file.constant_pool.push(CpInfo {
                    tag,
                    info: CpInfoType::String {
                        string_index: self.reader.read_int2(),
                    },
                }),
                e => todo!("{:?} {}", class_file, e),
            }
        }
        class_file.access_flags = Flags::new(self.reader.read_int2());
        class_file.this_class = self.reader.read_int2();
        class_file.super_class = self.reader.read_int2();
        class_file.interfaces_count = self.reader.read_int2();
        for _ in 1..class_file.interfaces_count {
            class_file.interfaces.push(self.reader.read_int2());
        }
        class_file.fields_count = self.reader.read_int2();
        for _ in 0..class_file.fields_count {
            let access_flags = Flags::new(self.reader.read_int2());
            let name_index = self.reader.read_int2();
            let descriptor_index = self.reader.read_int2();
            let attributes_count = self.reader.read_int2();
            let attributes = self.parse_attribute(&class_file, attributes_count);
            class_file.fields.push(FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            });
        }
        class_file.method_count = self.reader.read_int2();
        for _ in 0..class_file.method_count {
            let access_flags = Flags::new(self.reader.read_int2());
            let name_index = self.reader.read_int2();
            let descriptor_index = self.reader.read_int2();
            let attributes_count = self.reader.read_int2();
            let attributes = self.parse_attribute(&class_file, attributes_count);
            class_file.method_info.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            });
        }

        class_file.attributes_count = self.reader.read_int2();
        class_file.attributes = self.parse_attribute(&class_file, class_file.attributes_count);

        class_file
    }

    fn parse_attribute(&mut self, class: &ClassFile, count: u16) -> Vec<AttributeInfo> {
        let mut attributes = vec![];
        for _ in 0..count {
            let attribute_name_index = self.reader.read_int2();
            let attribute_length = self.reader.read_int4();
            let attribute;
            if let CpInfoType::Utf8 { bytes, .. } = class.constant_pool
                [attribute_name_index as usize - 1]
                .clone()
                .info
            {
                match bytes.as_str() {
                    "ConstantValue" => {
                        attribute = AttributeInfoKind::ConstantValue {
                            constantvalue_index: self.reader.read_int2(),
                        };
                    }
                    "Code" => {
                        let max_stack = self.reader.read_int2();
                        let max_locals = self.reader.read_int2();
                        let code_length = self.reader.read_int4();
                        let code = self.reader.read_bytes(code_length as usize);
                        let exception_table_length = self.reader.read_int2();
                        let mut exception_table = vec![];
                        for _ in 0..exception_table_length {
                            exception_table.push(ExceptionTable {
                                start_pc: self.reader.read_int2(),
                                end_pc: self.reader.read_int2(),
                                handler_pc: self.reader.read_int2(),
                                catch_type: self.reader.read_int2(),
                            });
                        }
                        let attributes_count = self.reader.read_int2();
                        let attributes = self.parse_attribute(class, attributes_count);
                        attribute = AttributeInfoKind::Code {
                            max_stack,
                            max_locals,
                            code_length,
                            code,
                            exception_table_length,
                            exception_table,
                            attributes_count,
                            attributes,
                        };
                    }
                    "StackMapTable" => {
                        todo!()
                    }
                    "Exceptions" => todo!(),
                    e => todo!("{}", e),
                }
            } else {
                panic!("Expected Utf8")
            }
            attributes.push(AttributeInfo {
                attribute_name_index,
                attribute_length,
                info: attribute, //info.into_iter().map(|x| x as i8).collect(),
            });
        }

        attributes
    }
}
