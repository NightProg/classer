use crate::{
    bytecode::{instr::*, *},
    descriptor::Descriptor,
};

fn compute_byte(byte: u16) -> (u8, u8) {
    (((byte >> 8) & 0xFF) as u8, (byte & 0xFF) as u8)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Builder {
    class_file: ClassFile,
    current_method: Option<u16>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            class_file: ClassFile {
                magic: JVM_MAGIC,
                minor_version: 0,
                major_version: 61,
                ..Default::default()
            },
            current_method: None,
        }
    }

    fn add_utf8(&mut self, string: &str) -> u16 {
        let cp_info = CpInfo {
            tag: 1,
            info: CpInfoType::Utf8 {
                length: string.len() as u16,
                bytes: string.to_string(),
            },
        };
        self.class_file.constant_pool.push(cp_info);
        self.class_file.constant_pool_count += 1;
        self.class_file.constant_pool_count
    }

    pub fn set_access_flags(&mut self, access_flags: u16) {
        self.class_file.access_flags = access_flags;
    }

    fn add_descriptor(&mut self, descriptor: Descriptor) -> u16 {
        self.add_utf8(&descriptor.serialize())
    }

    fn add_cp_info(&mut self, cp_info: CpInfo) -> u16 {
        self.class_file.constant_pool.push(cp_info);
        self.class_file.constant_pool_count += 1;
        self.class_file.constant_pool_count
    }

    pub fn load_class(&mut self, class_name: &str) -> u16 {
        let name_index = self.add_utf8(class_name);
        self.add_cp_info(CpInfo {
            tag: CP_TAG_CLASS,
            info: CpInfoType::Class { name_index },
        })
    }

    pub fn load_field(&mut self, class: u16, name: &str, descriptor: Descriptor) -> u16 {
        let name_index = self.add_utf8(name);
        let descriptor_index = self.add_descriptor(descriptor);
        let name_and_type_index = self.add_cp_info(CpInfo {
            tag: CP_TAG_NAMEANDTYPE,
            info: CpInfoType::NameAndType {
                name_index,
                descriptor_index,
            },
        });
        self.add_cp_info(CpInfo {
            tag: CP_TAG_FIELDREF,
            info: CpInfoType::Fieldref {
                class_index: class,
                name_and_type_index,
            },
        })
    }

    pub fn load_method(&mut self, class: u16, name: &str, descriptor: Descriptor) -> u16 {
        let name_index = self.add_utf8(name);
        let descriptor_index = self.add_descriptor(descriptor);
        let name_and_type_index = self.add_cp_info(CpInfo {
            tag: CP_TAG_NAMEANDTYPE,
            info: CpInfoType::NameAndType {
                name_index,
                descriptor_index,
            },
        });
        self.add_cp_info(CpInfo {
            tag: CP_TAG_METHODREF,
            info: CpInfoType::Methodref {
                class_index: class,
                name_and_type_index,
            },
        })
    }

    pub fn load_string(&mut self, string: &str) -> u16 {
        let utf8 = self.add_utf8(string);
        self.add_cp_info(CpInfo {
            tag: CP_TAG_STRING,
            info: CpInfoType::String { string_index: utf8 },
        })
    }

    pub fn set_super_class(&mut self, class: u16) {
        self.class_file.super_class = class;
    }

    pub fn set_class_name(&mut self, name: &str) {
        let name_index = self.add_utf8(name);
        self.class_file.this_class = self.add_cp_info(CpInfo {
            tag: CP_TAG_CLASS,
            info: CpInfoType::Class { name_index },
        });
    }

    pub fn add_field(&mut self, name: &str, descriptor: Descriptor, access_flags: u16) -> u16 {
        let name_index = self.add_utf8(name);
        let descriptor_index = self.add_descriptor(descriptor);
        self.class_file.fields.push(FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count: 0,
            attributes: vec![],
        });
        let name_and_type_index = self.add_cp_info(CpInfo {
            tag: CP_TAG_NAMEANDTYPE,
            info: CpInfoType::NameAndType {
                name_index,
                descriptor_index,
            },
        });

        self.add_cp_info(CpInfo {
            tag: CP_TAG_FIELDREF,
            info: CpInfoType::Fieldref {
                class_index: self.class_file.this_class,
                name_and_type_index,
            },
        })
    }

    pub fn add_method(&mut self, name: &str, descriptor: Descriptor, access_flags: u16) -> u16 {
        let name_index = self.add_utf8(name);
        let num_args;
        if let Descriptor::Function(l, _) = descriptor.clone() {
            num_args = l.len();
        } else {
            panic!("Invalid descriptor");
        }
        let descriptor_index = self.add_descriptor(descriptor);
        let code = self.add_utf8("Code");
        self.class_file.method_info.push(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count: 1,
            attributes: vec![AttributeInfo {
                attribute_name_index: code,
                attribute_length: 0,
                info: AttributeInfoKind::Code {
                    max_stack: 0,
                    max_locals: num_args as u16 + 1,
                    code_length: 0,
                    code: vec![],
                    exception_table_length: 0,
                    exception_table: vec![],
                    attributes_count: 0,
                    attributes: vec![],
                },
            }],
        });
        self.class_file.method_count += 1;
        let name_and_type_index = self.add_cp_info(CpInfo {
            tag: CP_TAG_NAMEANDTYPE,
            info: CpInfoType::NameAndType {
                name_index,
                descriptor_index,
            },
        });

        self.current_method = Some(self.class_file.method_count - 1);
        self.add_cp_info(CpInfo {
            tag: CP_TAG_METHODREF,
            info: CpInfoType::Methodref {
                class_index: self.class_file.this_class,
                name_and_type_index,
            },
        })
    }

    pub fn build_aaload(&mut self) {
        if let Some(ind) = self.current_method {
            let attr = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code {
                code, max_stack, ..
            } = attr
            {
                *max_stack -= 1;
                code.push(Opcode::Aaload);
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build_aastore(&mut self) {
        if let Some(ind) = self.current_method {
            let attr = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code {
                code, max_stack, ..
            } = attr
            {
                *max_stack -= 3;
                code.push(Opcode::Aastore);
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build_aconst_null(&mut self) {
        if let Some(ind) = self.current_method {
            let attr = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code {
                code, max_stack, ..
            } = attr
            {
                *max_stack += 1;
                code.push(Opcode::AconstNull);
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build_aload(&mut self, local: u8) {
        if let Some(ind) = self.current_method {
            let attr = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code {
                code, max_stack, ..
            } = attr
            {
                *max_stack += 1;
                code.push(Opcode::Aload(local));
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build_aload_n(&mut self, n: u8) {
        if let Some(ind) = self.current_method {
            let attr = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code {
                code, max_locals, ..
            } = attr
            {
                if n as u16 >= *max_locals {
                    *max_locals = n as u16 + 1;
                }
                code.push(Opcode::AloadN(n));
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build_anewarray(&mut self, class: u16) {
        if let Some(ind) = self.current_method {
            let attr = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code {
                code,
                
                
                ..
            } = attr
            {
                let (high, low) = compute_byte(class);
                code.push(Opcode::Anewarray(high, low));
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build_areturn(&mut self) {
        if let Some(ind) = self.current_method {
            let attr = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code {
                code, max_stack, ..
            } = attr
            {
                *max_stack -= 1;
                code.push(Opcode::Areturn);
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn buldd_arraylength(&mut self) {
        if let Some(ind) = self.current_method {
            let attr = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code { code, .. } = attr {
                code.push(Opcode::Arraylength);
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build_invoke_special(&mut self, method: u16) {
        if let Some(ind) = self.current_method {
            let code = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code {
                code, max_stack, ..
            } = code
            {
                *max_stack += 1;
                let (high, low) = compute_byte(method);
                code.push(Opcode::Invokespecial(high, low));
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build_invoke_virtual(&mut self, method: u16) {
        if let Some(ind) = self.current_method {
            let code = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code {
                code, max_stack, ..
            } = code
            {
                *max_stack += 1;
                let (high, low) = compute_byte(method);
                code.push(Opcode::Invokevirtual(high, low));
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build_getstatic(&mut self, field: u16) {
        if let Some(ind) = self.current_method {
            let code = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code {
                code, max_stack, ..
            } = code
            {
                let (high, low) = compute_byte(field);
                code.push(Opcode::Getstatic(high, low));
                *max_stack += 1;
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build_ldc(&mut self, constant: u16) {
        if let Some(ind) = self.current_method {
            let code = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code {
                code, max_stack, ..
            } = code
            {
                *max_stack += 1;
                code.push(Opcode::Ldc(constant as u8));
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build_return(&mut self) {
        if let Some(ind) = self.current_method {
            let attr = &mut self.class_file.method_info[ind as usize].attributes[0].info;
            if let AttributeInfoKind::Code { code, .. } = attr {
                code.push(Opcode::Return_);
            } else {
                panic!("Invalid code attribute");
            }
        } else {
            panic!("No method to build");
        }
    }

    pub fn build(&self) -> ClassFile {
        self.class_file.clone()
    }
}
