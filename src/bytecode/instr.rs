pub const AALOAD: u8 = 0x32;
pub const AASTORE: u8 = 0x53;
pub const ACONST_NULL: u8 = 0x01;
pub const ALOAD: u8 = 0x19;
pub const ANEWARRAY: u8 = 0xbd;
pub const ARETURN: u8 = 0xb0;
pub const ARRAYLENGTH: u8 = 0xbe;
pub const ASTORE: u8 = 0x3a;

pub const fn aloadN(n: u8) -> u8 {
    0x2a + n
}

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Aaload,
    Aastore,
    AaconstNull,
    Aload(u8),
    AloadN(u8),
    ANewArray(u8, u8),
    Areturn,
    ArrayLength,
    AStore(u8),
}
