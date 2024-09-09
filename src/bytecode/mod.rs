pub mod instr;
use crate::flags::Flags;

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(u16)]
pub enum AccessFlag {
    #[default]
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
}

impl std::ops::BitAnd for AccessFlag {
    type Output = AccessFlag;
    fn bitand(self, rhs: Self) -> Self::Output {
        (self as u16 & rhs as u16).into()
    }
}

impl std::ops::BitOr for AccessFlag {
    type Output = AccessFlag;
    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u16 | rhs as u16).into()
    }
}

impl From<u16> for AccessFlag {
    fn from(value: u16) -> Self {
        match value {
            0x0001 => AccessFlag::Public,
            0x0010 => AccessFlag::Final,
            0x0020 => AccessFlag::Super,
            0x0200 => AccessFlag::Interface,
            0x0400 => AccessFlag::Abstract,
            0x1000 => AccessFlag::Synthetic,
            0x2000 => AccessFlag::Annotation,
            0x4000 => AccessFlag::Enum,
            e => panic!("Unknown access flag: {:b}", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<CpInfo>,
    pub access_flags: Flags<u16>,
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

impl ClassFile {
    pub fn display(&self) {
        println!("magic: {:x}", self.magic);
        println!("minor_version: {}", self.minor_version);
        println!("major_version: {}", self.major_version);
        println!("constant_pool_count: {}", self.constant_pool_count);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfo {
    pub access_flags: Flags<u16>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub info: AttributeInfoKind,
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
        code: Vec<u8>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterAnnotation {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementValue {
    pub tag: u8,
    pub value: ElementValueKind,
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
#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableTypeTable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableTable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineNumberTable {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: Flags<u16>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodInfo {
    pub access_flags: Flags<u16>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CpInfo {
    pub tag: u8,
    pub info: CpInfoType,
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
