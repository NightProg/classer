use std::rc::Rc;

use crate::bytecode::ClassFile;
use crate::bytecode::CpInfo;
use crate::bytecode::CpInfoType;
use crate::bytecode::FieldInfo;
use crate::descriptor::Descriptor;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassFileBuilder {
    class_file: ClassFile,
}

impl ClassFileBuilder {
    pub(crate) fn new(class_file: ClassFile) -> ClassFileBuilder {
        ClassFileBuilder { class_file }
    }

    pub fn set_access_flags(&mut self, access_flags: u16) -> &mut Self {
        self.class_file.access_flags = access_flags;
        self
    }

    pub fn build(&self) -> &ClassFile {
        &self.class_file
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfoBuilder<'a> {
    field_info: FieldInfo,
    class_file: Rc<RefCell<ClassFile>>,
}

impl<'a> FieldInfoBuilder<'a> {
    pub(crate) fn new(field_info: FieldInfo, class_file: &'a mut ClassFile) -> FieldInfoBuilder {
        FieldInfoBuilder {
            field_info,
            class_file: Rc::new(class_file),
        }
    }
    pub fn set_access_flags(&mut self, access_flags: u16) -> &mut Self {
        self.field_info.access_flags = access_flags;
        self
    }

    pub fn set_descriptor(&mut self, descriptor: Descriptor) -> &mut Self {
        (*Rc::make_mut(self.class_file)).constant_pool_count += 1;
        let serialized = descriptor.serialize();
        (*Rc::make_mut(self.class_file)).constant_pool.push(CpInfo {
            tag: 1,
            info: CpInfoType::Utf8 {
                length: serialized.len() as u16,
                bytes: descriptor.serialize(),
            },
        });
        self.field_info.descriptor_index = self.class_file.constant_pool_count;
        self
    }
    pub fn build(&self) -> &FieldInfo {
        &self.field_info
    }
}
