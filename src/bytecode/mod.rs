pub mod instr;

use std::default;

use instr::Opcode;

use crate::{builder::*, flags::Flags};

pub const PUBLIC: u16 = 0x0001;
pub const FINAL: u16 = 0x0010;
pub const SUPER: u16 = 0x0020;
pub const INTERFACE: u16 = 0x0200;
pub const ABSTRACT: u16 = 0x0400;
pub const SYNTHETIC: u16 = 0x1000;
pub const ANNOTATION: u16 = 0x2000;
pub const ENUM: u16 = 0x4000;

pub const JVM_MAGIC: u32 = 0xCAFEBABE;

pub trait ToJvmBytecode {
    fn to_jvm_bytecode(&self) -> Vec<u8>;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<CpInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub method_count: u16,
    pub method_info: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl ToJvmBytecode for ClassFile {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.magic.to_be_bytes());
        bytes.extend_from_slice(&self.minor_version.to_be_bytes());
        bytes.extend_from_slice(&self.major_version.to_be_bytes());
        bytes.extend_from_slice(&(self.constant_pool_count + 1).to_be_bytes());

        bytes.extend_from_slice(
            &self
                .constant_pool
                .iter()
                .flat_map(|cp| cp.to_jvm_bytecode())
                .collect::<Vec<u8>>(),
        );
        bytes.extend_from_slice(&self.access_flags.to_be_bytes());
        bytes.extend_from_slice(&self.this_class.to_be_bytes());
        bytes.extend_from_slice(&self.super_class.to_be_bytes());
        bytes.extend_from_slice(&self.interfaces_count.to_be_bytes());
        bytes.extend_from_slice(
            &self
                .interfaces
                .iter()
                .map(|i| i.to_be_bytes())
                .flatten()
                .collect::<Vec<u8>>(),
        );
        bytes.extend_from_slice(&self.fields_count.to_be_bytes());
        bytes.extend_from_slice(
            &self
                .fields
                .iter()
                .map(|f| f.to_jvm_bytecode())
                .flatten()
                .collect::<Vec<u8>>(),
        );
        bytes.extend_from_slice(&self.method_count.to_be_bytes());
        bytes.extend_from_slice(
            &self
                .method_info
                .iter()
                .map(|m| m.to_jvm_bytecode())
                .flatten()
                .collect::<Vec<u8>>(),
        );
        bytes.extend_from_slice(&self.attributes_count.to_be_bytes());
        bytes.extend_from_slice(
            &self
                .attributes
                .iter()
                .map(|a| a.to_jvm_bytecode())
                .flatten()
                .collect::<Vec<u8>>(),
        );
        bytes
    }
}

pub const FIELD_ACC_PUBLIC: u16 = 0x0001;
pub const FIELD_ACC_PRIVATE: u16 = 0x0002;
pub const FIELD_ACC_PROTECTED: u16 = 0x0004;
pub const FIELD_ACC_STATIC: u16 = 0x0008;
pub const FIELD_ACC_FINAL: u16 = 0x0010;
pub const FIELD_ACC_VOLATILE: u16 = 0x0040;
pub const FIELD_ACC_TRANSIENT: u16 = 0x0080;
pub const FIELD_ACC_SYNTHETIC: u16 = 0x1000;
pub const FIELD_ACC_ENUM: u16 = 0x4000;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl ToJvmBytecode for FieldInfo {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.access_flags.to_be_bytes());
        bytes.extend_from_slice(&self.name_index.to_be_bytes());
        bytes.extend_from_slice(&self.descriptor_index.to_be_bytes());
        bytes.extend_from_slice(&self.attributes_count.to_be_bytes());
        bytes.extend_from_slice(
            &self
                .attributes
                .iter()
                .map(|a| a.to_jvm_bytecode())
                .flatten()
                .collect::<Vec<u8>>(),
        );
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub info: AttributeInfoKind,
}

impl ToJvmBytecode for AttributeInfo {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        let b = self.info.to_jvm_bytecode();
        let length = b.len() as u32;
        bytes.extend_from_slice(&self.attribute_name_index.to_be_bytes());
        bytes.extend_from_slice(&length.to_be_bytes());
        bytes.extend_from_slice(&b);
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeInfoKind {
    ConstantValue {
        constantvalue_index: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<Opcode>,
        exception_table_length: u16,
        exception_table: Vec<ExceptionTable>,
        attributes_count: u16,
        attributes: Vec<AttributeInfo>,
    },
    StackMapTable {
        number_of_entries: u16,
        entries: Vec<StackMapFrame>,
    },
    Exceptions {
        number_of_exceptions: u16,
        exception_index_table: Vec<u16>,
    },
    InnerClasses {
        number_of_classes: u16,
        classes: Vec<InnerClass>,
    },
    EnclosingMethod {
        class_index: u16,
        method_index: u16,
    },
    Synthetic {
        attribute_name_index: u16,
        attribute_length: u32,
    },
    Signature {
        signature_index: u16,
    },
    SourceFile {
        sourcefile_index: u16,
    },
    SourceDebugExtension {
        debug_extension: Vec<u8>,
    },
    LineNumberTable {
        line_number_table_length: u16,
        line_number_table: Vec<LineNumberTable>,
    },
    LocalVariableTable {
        local_variable_table_length: u16,
        local_variable_table: Vec<LocalVariableTable>,
    },
    LocalVariableTypeTable {
        local_variable_type_table_length: u16,
        local_variable_type_table: Vec<LocalVariableTypeTable>,
    },
    Deprecated,
    RuntimeVisibleAnnotations {
        num_annotations: u16,
        annotations: Vec<Annotation>,
    },
    RuntimeInvisibleAnnotations {
        num_annotations: u16,
        annotations: Vec<Annotation>,
    },
    RuntimeVisibleParameterAnnotations {
        num_parameters: u8,
        parameter_annotations: Vec<ParameterAnnotation>,
    },
    RuntimeInvisibleParameterAnnotations {
        num_parameters: u8,
        parameter_annotations: Vec<ParameterAnnotation>,
    },
    AnnotationDefault {
        default_value: ElementValue,
    },
    BootstrapMethods {
        num_bootstrap_methods: u16,
        bootstrap_methods: Vec<BootstrapMethod>,
    },
}

impl ToJvmBytecode for AttributeInfoKind {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        match self {
            AttributeInfoKind::ConstantValue {
                constantvalue_index,
            } => {
                bytes.extend_from_slice(&constantvalue_index.to_be_bytes());
            }
            AttributeInfoKind::Code {
                max_stack,
                max_locals,
                code,
                exception_table_length,
                exception_table,
                attributes_count,
                attributes,
                ..
            } => {
                let c = code
                    .iter()
                    .map(|c| c.to_jvm_bytecode())
                    .flatten()
                    .collect::<Vec<u8>>();
                bytes.extend_from_slice(&max_stack.to_be_bytes());
                bytes.extend_from_slice(&max_locals.to_be_bytes());
                bytes.extend_from_slice(&(c.len() as u32).to_be_bytes());
                bytes.extend_from_slice(&c);
                bytes.extend_from_slice(&exception_table_length.to_be_bytes());
                bytes.extend_from_slice(
                    &exception_table
                        .iter()
                        .map(|e| e.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
                bytes.extend_from_slice(&attributes_count.to_be_bytes());
                bytes.extend_from_slice(
                    &attributes
                        .iter()
                        .map(|a| a.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            AttributeInfoKind::StackMapTable {
                number_of_entries,
                entries,
            } => {
                bytes.extend_from_slice(&number_of_entries.to_be_bytes());
                bytes.extend_from_slice(
                    &entries
                        .iter()
                        .map(|e| e.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            AttributeInfoKind::Exceptions {
                number_of_exceptions,
                exception_index_table,
            } => {
                bytes.extend_from_slice(&number_of_exceptions.to_be_bytes());
                bytes.extend_from_slice(
                    &exception_index_table
                        .iter()
                        .map(|e| e.to_be_bytes())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            AttributeInfoKind::InnerClasses {
                number_of_classes,
                classes,
            } => {
                bytes.extend_from_slice(&number_of_classes.to_be_bytes());
                bytes.extend_from_slice(
                    &classes
                        .iter()
                        .map(|c| c.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            AttributeInfoKind::EnclosingMethod {
                class_index,
                method_index,
            } => {
                bytes.extend_from_slice(&class_index.to_be_bytes());
                bytes.extend_from_slice(&method_index.to_be_bytes());
            }
            AttributeInfoKind::Synthetic {
                attribute_name_index,
                attribute_length,
            } => {
                bytes.extend_from_slice(&attribute_name_index.to_be_bytes());
                bytes.extend_from_slice(&attribute_length.to_be_bytes());
            }
            AttributeInfoKind::Signature { signature_index } => {
                bytes.extend_from_slice(&signature_index.to_be_bytes());
            }
            AttributeInfoKind::SourceFile { sourcefile_index } => {
                bytes.extend_from_slice(&sourcefile_index.to_be_bytes());
            }
            AttributeInfoKind::SourceDebugExtension { debug_extension } => {
                bytes.extend_from_slice(&debug_extension);
            }
            AttributeInfoKind::LineNumberTable {
                line_number_table_length,
                line_number_table,
            } => {
                bytes.extend_from_slice(&line_number_table_length.to_be_bytes());
                bytes.extend_from_slice(
                    &line_number_table
                        .iter()
                        .map(|l| l.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            AttributeInfoKind::LocalVariableTable {
                local_variable_table_length,
                local_variable_table,
            } => {
                bytes.extend_from_slice(&local_variable_table_length.to_be_bytes());
                bytes.extend_from_slice(
                    &local_variable_table
                        .iter()
                        .map(|l| l.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            AttributeInfoKind::LocalVariableTypeTable {
                local_variable_type_table_length,
                local_variable_type_table,
            } => {
                bytes.extend_from_slice(&local_variable_type_table_length.to_be_bytes());
                bytes.extend_from_slice(
                    &local_variable_type_table
                        .iter()
                        .map(|l| l.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            AttributeInfoKind::Deprecated => {}
            AttributeInfoKind::RuntimeVisibleAnnotations {
                num_annotations,
                annotations,
            } => {
                bytes.extend_from_slice(&num_annotations.to_be_bytes());
                bytes.extend_from_slice(
                    &annotations
                        .iter()
                        .map(|a| a.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            AttributeInfoKind::RuntimeInvisibleAnnotations {
                num_annotations,
                annotations,
            } => {
                bytes.extend_from_slice(&num_annotations.to_be_bytes());
                bytes.extend_from_slice(
                    &annotations
                        .iter()
                        .map(|a| a.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            AttributeInfoKind::RuntimeVisibleParameterAnnotations {
                num_parameters,
                parameter_annotations,
            } => {
                bytes.extend_from_slice(&num_parameters.to_be_bytes());
                bytes.extend_from_slice(
                    &parameter_annotations
                        .iter()
                        .map(|p| p.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            AttributeInfoKind::RuntimeInvisibleParameterAnnotations {
                num_parameters,
                parameter_annotations,
            } => {
                bytes.push(*num_parameters);
                bytes.extend_from_slice(
                    &parameter_annotations
                        .iter()
                        .map(|p| p.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            AttributeInfoKind::AnnotationDefault { default_value } => {
                bytes.extend_from_slice(&default_value.to_jvm_bytecode());
            }
            AttributeInfoKind::BootstrapMethods {
                num_bootstrap_methods,
                bootstrap_methods,
            } => {
                bytes.extend_from_slice(&num_bootstrap_methods.to_be_bytes());
                bytes.extend_from_slice(
                    &bootstrap_methods
                        .iter()
                        .map(|b| b.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
        }
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>,
}

impl ToJvmBytecode for BootstrapMethod {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.bootstrap_method_ref.to_be_bytes());
        bytes.extend_from_slice(&self.num_bootstrap_arguments.to_be_bytes());
        bytes.extend_from_slice(
            &self
                .bootstrap_arguments
                .iter()
                .map(|a| a.to_be_bytes())
                .flatten()
                .collect::<Vec<u8>>(),
        );
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterAnnotation {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

impl ToJvmBytecode for ParameterAnnotation {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.num_annotations.to_be_bytes());
        bytes.extend_from_slice(
            &self
                .annotations
                .iter()
                .map(|a| a.to_jvm_bytecode())
                .flatten()
                .collect::<Vec<u8>>(),
        );
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

impl ToJvmBytecode for Annotation {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.type_index.to_be_bytes());
        bytes.extend_from_slice(&self.num_element_value_pairs.to_be_bytes());
        bytes.extend_from_slice(
            &self
                .element_value_pairs
                .iter()
                .map(|e| e.to_jvm_bytecode())
                .flatten()
                .collect::<Vec<u8>>(),
        );
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

impl ToJvmBytecode for ElementValuePair {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.element_name_index.to_be_bytes());
        bytes.extend_from_slice(&self.value.to_jvm_bytecode());
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementValue {
    pub tag: u8,
    pub value: ElementValueKind,
}

impl ToJvmBytecode for ElementValue {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.tag);
        bytes.extend_from_slice(&self.value.to_jvm_bytecode());
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementValueKind {
    ConstValueIndex(u16),
    EnumConstValue {
        type_name_index: u16,
        const_name_index: u16,
    },
    ClassInfoIndex(u16),
    AnnotationValue(Annotation),
    ArrayValue {
        num_values: u16,
        values: Vec<ElementValue>,
    },
}

impl ToJvmBytecode for ElementValueKind {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        match self {
            ElementValueKind::ConstValueIndex(i) => bytes.extend_from_slice(&i.to_be_bytes()),
            ElementValueKind::EnumConstValue {
                type_name_index,
                const_name_index,
            } => {
                bytes.extend_from_slice(&type_name_index.to_be_bytes());
                bytes.extend_from_slice(&const_name_index.to_be_bytes());
            }
            ElementValueKind::ClassInfoIndex(i) => bytes.extend_from_slice(&i.to_be_bytes()),
            ElementValueKind::AnnotationValue(a) => bytes.extend_from_slice(&a.to_jvm_bytecode()),
            ElementValueKind::ArrayValue { num_values, values } => {
                bytes.extend_from_slice(&num_values.to_be_bytes());
                bytes.extend_from_slice(
                    &values
                        .iter()
                        .map(|v| v.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
        }
        bytes
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableTypeTable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

impl ToJvmBytecode for LocalVariableTypeTable {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.start_pc.to_be_bytes());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.name_index.to_be_bytes());
        bytes.extend_from_slice(&self.signature_index.to_be_bytes());
        bytes.extend_from_slice(&self.index.to_be_bytes());
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableTable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

impl ToJvmBytecode for LocalVariableTable {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.start_pc.to_be_bytes());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.name_index.to_be_bytes());
        bytes.extend_from_slice(&self.descriptor_index.to_be_bytes());
        bytes.extend_from_slice(&self.index.to_be_bytes());
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineNumberTable {
    pub start_pc: u16,
    pub line_number: u16,
}

impl ToJvmBytecode for LineNumberTable {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.start_pc.to_be_bytes());
        bytes.extend_from_slice(&self.line_number.to_be_bytes());
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: Flags<u16>,
}

impl ToJvmBytecode for InnerClass {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.inner_class_info_index.to_be_bytes());
        bytes.extend_from_slice(&self.outer_class_info_index.to_be_bytes());
        bytes.extend_from_slice(&self.inner_name_index.to_be_bytes());
        bytes.extend_from_slice(&self.inner_class_access_flags.flags.to_be_bytes());
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackMapFrame {
    SameFrame {
        frame_type: u8,
    },
    SameLocals1StackItemFrame {
        frame_type: u8,
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemFrameExtended {
        frame_type: u8,
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    ChopFrame {
        frame_type: u8,
        offset_delta: u16,
    },
    SameFrameExtended {
        frame_type: u8,
        offset_delta: u16,
    },
    AppendFrame {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

impl ToJvmBytecode for StackMapFrame {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        match self {
            StackMapFrame::SameFrame { frame_type } => bytes.push(*frame_type),
            StackMapFrame::SameLocals1StackItemFrame { frame_type, stack } => {
                bytes.push(*frame_type);
                bytes.extend_from_slice(&stack.to_jvm_bytecode());
            }
            StackMapFrame::SameLocals1StackItemFrameExtended {
                frame_type,
                offset_delta,
                stack,
            } => {
                bytes.push(*frame_type);
                bytes.extend_from_slice(&offset_delta.to_be_bytes());
                bytes.extend_from_slice(&stack.to_jvm_bytecode());
            }
            StackMapFrame::ChopFrame {
                frame_type,
                offset_delta,
            } => {
                bytes.push(*frame_type);
                bytes.extend_from_slice(&offset_delta.to_be_bytes());
            }
            StackMapFrame::SameFrameExtended {
                frame_type,
                offset_delta,
            } => {
                bytes.push(*frame_type);
                bytes.extend_from_slice(&offset_delta.to_be_bytes());
            }
            StackMapFrame::AppendFrame {
                frame_type,
                offset_delta,
                locals,
            } => {
                bytes.push(*frame_type);
                bytes.extend_from_slice(&offset_delta.to_be_bytes());
                bytes.extend_from_slice(
                    &locals
                        .iter()
                        .map(|l| l.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
            StackMapFrame::FullFrame {
                frame_type,
                offset_delta,
                locals,
                stack,
            } => {
                bytes.push(*frame_type);
                bytes.extend_from_slice(&offset_delta.to_be_bytes());
                bytes.extend_from_slice(
                    &locals
                        .iter()
                        .map(|l| l.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
                bytes.extend_from_slice(
                    &stack
                        .iter()
                        .map(|s| s.to_jvm_bytecode())
                        .flatten()
                        .collect::<Vec<u8>>(),
                );
            }
        }
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object { cpool_index: u16 },
    Uninitialized { offset: u16 },
}

impl ToJvmBytecode for VerificationTypeInfo {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        match self {
            VerificationTypeInfo::Top => bytes.push(0),
            VerificationTypeInfo::Integer => bytes.push(1),
            VerificationTypeInfo::Float => bytes.push(2),
            VerificationTypeInfo::Long => bytes.push(4),
            VerificationTypeInfo::Double => bytes.push(3),
            VerificationTypeInfo::Null => bytes.push(5),
            VerificationTypeInfo::UninitializedThis => bytes.push(6),
            VerificationTypeInfo::Object { cpool_index } => {
                bytes.push(7);
                bytes.extend_from_slice(&cpool_index.to_be_bytes());
            }
            VerificationTypeInfo::Uninitialized { offset } => {
                bytes.push(8);
                bytes.extend_from_slice(&offset.to_be_bytes());
            }
        }
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ToJvmBytecode for ExceptionTable {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.start_pc.to_be_bytes());
        bytes.extend_from_slice(&self.end_pc.to_be_bytes());
        bytes.extend_from_slice(&self.handler_pc.to_be_bytes());
        bytes.extend_from_slice(&self.catch_type.to_be_bytes());
        bytes
    }
}

pub const METHOD_ACC_PUBLIC: u16 = 0x0001;
pub const METHOD_ACC_PRIVATE: u16 = 0x0002;
pub const METHOD_ACC_PROTECTED: u16 = 0x0004;
pub const METHOD_ACC_STATIC: u16 = 0x0008;
pub const METHOD_ACC_FINAL: u16 = 0x0010;
pub const METHOD_ACC_SYNCHRONIZED: u16 = 0x0020;
pub const METHOD_ACC_BRIDGE: u16 = 0x0040;
pub const METHOD_ACC_VARARGS: u16 = 0x0080;
pub const METHOD_ACC_NATIVE: u16 = 0x0100;
pub const METHOD_ACC_ABSTRACT: u16 = 0x0400;
pub const METHOD_ACC_STRICT: u16 = 0x0800;
pub const METHOD_ACC_SYNTHETIC: u16 = 0x1000;

#[derive(Debug, Clone, PartialEq)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl ToJvmBytecode for MethodInfo {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.access_flags.to_be_bytes());
        bytes.extend_from_slice(&self.name_index.to_be_bytes());
        bytes.extend_from_slice(&self.descriptor_index.to_be_bytes());
        bytes.extend_from_slice(&self.attributes_count.to_be_bytes());
        bytes.extend_from_slice(
            &self
                .attributes
                .iter()
                .flat_map(|a| a.to_jvm_bytecode())
                .collect::<Vec<u8>>(),
        );
        bytes
    }
}

pub const CP_TAG_UTF8: u8 = 1;
pub const CP_TAG_INTEGER: u8 = 3;
pub const CP_TAG_FLOAT: u8 = 4;
pub const CP_TAG_LONG: u8 = 5;
pub const CP_TAG_DOUBLE: u8 = 6;
pub const CP_TAG_CLASS: u8 = 7;
pub const CP_TAG_STRING: u8 = 8;
pub const CP_TAG_FIELDREF: u8 = 9;
pub const CP_TAG_METHODREF: u8 = 10;
pub const CP_TAG_INTERFACEMETHODREF: u8 = 11;
pub const CP_TAG_NAMEANDTYPE: u8 = 12;
pub const CP_TAG_METHODHANDLE: u8 = 15;
pub const CP_TAG_METHODTYPE: u8 = 16;
pub const CP_TAG_INVOKEDYNAMIC: u8 = 18;

#[derive(Debug, Clone, PartialEq)]
pub struct CpInfo {
    pub tag: u8,
    pub info: CpInfoType,
}

impl ToJvmBytecode for CpInfo {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.tag.to_be_bytes());
        match &self.info {
            CpInfoType::Class { name_index } => bytes.extend_from_slice(&name_index.to_be_bytes()),
            CpInfoType::Fieldref {
                class_index,
                name_and_type_index,
            } => {
                bytes.extend_from_slice(&class_index.to_be_bytes());
                bytes.extend_from_slice(&name_and_type_index.to_be_bytes());
            }
            CpInfoType::Methodref {
                class_index,
                name_and_type_index,
            } => {
                bytes.extend_from_slice(&class_index.to_be_bytes());
                bytes.extend_from_slice(&name_and_type_index.to_be_bytes());
            }
            CpInfoType::InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => {
                bytes.extend_from_slice(&class_index.to_be_bytes());
                bytes.extend_from_slice(&name_and_type_index.to_be_bytes());
            }
            CpInfoType::String { string_index } => {
                bytes.extend_from_slice(&string_index.to_be_bytes())
            }
            CpInfoType::Integer { bytes: i } => bytes.extend_from_slice(&i.to_be_bytes()),
            CpInfoType::Float { bytes: f } => bytes.extend_from_slice(&f.to_be_bytes()),
            CpInfoType::Long {
                high_bytes,
                low_bytes,
            } => {
                bytes.extend_from_slice(&high_bytes.to_be_bytes());
                bytes.extend_from_slice(&low_bytes.to_be_bytes());
            }
            CpInfoType::Double {
                high_bytes,
                low_bytes,
            } => {
                bytes.extend_from_slice(&high_bytes.to_be_bytes());
                bytes.extend_from_slice(&low_bytes.to_be_bytes());
            }
            CpInfoType::NameAndType {
                name_index,
                descriptor_index,
            } => {
                bytes.extend_from_slice(&name_index.to_be_bytes());
                bytes.extend_from_slice(&descriptor_index.to_be_bytes());
            }
            CpInfoType::Utf8 { length, bytes: str } => {
                bytes.extend_from_slice(&length.to_be_bytes());
                bytes.extend_from_slice(str.as_bytes());
            }
            CpInfoType::MethodHandle {
                reference_kind,
                reference_index,
            } => {
                bytes.extend_from_slice(&reference_kind.to_be_bytes());
                bytes.extend_from_slice(&reference_index.to_be_bytes());
            }
            CpInfoType::MethodType { descriptor_index } => {
                bytes.extend_from_slice(&descriptor_index.to_be_bytes());
            }
            CpInfoType::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                bytes.extend_from_slice(&bootstrap_method_attr_index.to_be_bytes());
                bytes.extend_from_slice(&name_and_type_index.to_be_bytes());
            }
        }

        bytes
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum CpInfoType {
    Class {
        name_index: u16,
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer {
        bytes: u32,
    },
    Float {
        bytes: f32,
    },
    Long {
        high_bytes: u32,
        low_bytes: u32,
    },
    Double {
        high_bytes: u32,
        low_bytes: u32,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8 {
        length: u16,
        bytes: String,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}
