#![allow(unused)]

use crate::reader::Reader;

use super::ToJvmBytecode;

const AALOAD: u8 = 0x32;
const AASTORE: u8 = 0x53;
const ACONST_NULL: u8 = 0x1;
const ALOAD: u8 = 0x19;
const ALOAD_N: u8 = 0x2a;

const ANEWARRAY: u8 = 0xbd;
const ARETURN: u8 = 0xb0;
const ARRAYLENGTH: u8 = 0xbe;
const ASTORE: u8 = 0x3a;
const ASTORE_N: u8 = 0x4b;

const ATHROW: u8 = 0xbf;
const BALOAD: u8 = 0x33;
const BASTORE: u8 = 0x54;
const BIPUSH: u8 = 0x10;
const CALOAD: u8 = 0x34;
const CASTORE: u8 = 0x55;
const CHECKCAST: u8 = 0xc0;
const D2F: u8 = 0x90;
const D2I: u8 = 0x8e;
const D2L: u8 = 0x8f;
const DADD: u8 = 0x63;
const DALOAD: u8 = 0x31;
const DASTORE: u8 = 0x52;
const DCM_N: u8 = 0x98;

const DCONST_N: u8 = 0xe;

const DDIV: u8 = 0x6f;
const DLOAD: u8 = 0x18;
const DLOAD_N: u8 = 0x26;

const DMUL: u8 = 0x6b;
const DNEG: u8 = 0x77;
const DREM: u8 = 0x73;
const DRETURN: u8 = 0xaf;
const DSTORE: u8 = 0x39;
const DSTORE_N: u8 = 0x47;

const DSUB: u8 = 0x67;
const DUP: u8 = 0x59;
const DUP_X1: u8 = 0x5a;
const DUP_X2: u8 = 0x5b;
const DUP2: u8 = 0x5c;
const DUP2_X1: u8 = 0x5d;
const DUP2_X2: u8 = 0x5e;
const F2D: u8 = 0x8d;
const F2I: u8 = 0x8b;
const F2L: u8 = 0x8c;
const FADD: u8 = 0x62;
const FALOAD: u8 = 0x30;
const FASTORE: u8 = 0x51;
const FCM_N: u8 = 0x96;

const FCONST_N: u8 = 0xb;

const FDIV: u8 = 0x6e;
const FLOAD: u8 = 0x17;
const FLOAD_N: u8 = 0x22;

const FMUL: u8 = 0x6a;
const FNEG: u8 = 0x76;
const FREM: u8 = 0x72;
const FRETURN: u8 = 0xae;
const FSTORE: u8 = 0x38;
const FSTORE_N: u8 = 0x43;

const FSUB: u8 = 0x66;
const GETFIELD: u8 = 0xb4;
const GETSTATIC: u8 = 0xb2;
const GOTO: u8 = 0xa7;
const GOTO_W: u8 = 0xc8;
const I2B: u8 = 0x91;
const I2C: u8 = 0x92;
const I2D: u8 = 0x87;
const I2F: u8 = 0x86;
const I2L: u8 = 0x85;
const I2S: u8 = 0x93;
const IADD: u8 = 0x60;
const IALOAD: u8 = 0x2e;
const IAND: u8 = 0x7e;
const IASTORE: u8 = 0x4f;
const ICONST_N: u8 = 0x2;

const IDIV: u8 = 0x6c;
const IF_ACM_N: u8 = 0xa5;

const IF_ICM_N: u8 = 0x9f;

const I_N: u8 = 0x99;

const IFNONNULL: u8 = 0xc7;
const IFNULL: u8 = 0xc6;
const IINC: u8 = 0x84;
const ILOAD: u8 = 0x15;
const ILOAD_N: u8 = 0x1a;

const IMUL: u8 = 0x68;
const INEG: u8 = 0x74;
const INSTANCEOF: u8 = 0xc1;
const INVOKEDYNAMIC: u8 = 0xba;
const INVOKEINTERFACE: u8 = 0xb9;
const INVOKESPECIAL: u8 = 0xb7;
const INVOKESTATIC: u8 = 0xb8;
const INVOKEVIRTUAL: u8 = 0xb6;
const IOR: u8 = 0x80;
const IREM: u8 = 0x70;
const IRETURN: u8 = 0xac;
const ISHL: u8 = 0x78;
const ISHR: u8 = 0x7a;
const ISTORE: u8 = 0x36;
const ISTORE_N: u8 = 0x3b;

const ISUB: u8 = 0x64;
const IUSHR: u8 = 0x7c;
const IXOR: u8 = 0x82;
const JSR: u8 = 0xa8;
const JSR_W: u8 = 0xc9;
const L2D: u8 = 0x8a;
const L2F: u8 = 0x89;
const L2I: u8 = 0x88;
const LADD: u8 = 0x61;
const LALOAD: u8 = 0x2f;
const LAND: u8 = 0x7f;
const LASTORE: u8 = 0x50;
const LCMP: u8 = 0x94;
const LCONST_N: u8 = 0x9;

const LDC: u8 = 0x12;
const LDC_W: u8 = 0x13;
const LDC2_W: u8 = 0x14;
const LDIV: u8 = 0x6d;
const LLOAD: u8 = 0x16;
const LLOAD_N: u8 = 0x1e;

const LMUL: u8 = 0x69;
const LNEG: u8 = 0x75;
const LOOKUPSWITCH: u8 = 0xab;
const LOR: u8 = 0x81;
const LREM: u8 = 0x71;
const LRETURN: u8 = 0xad;
const LSHL: u8 = 0x79;
const LSHR: u8 = 0x7b;
const LSTORE: u8 = 0x37;
const LSTORE_N: u8 = 0x3f;

const LSUB: u8 = 0x65;
const LUSHR: u8 = 0x7d;
const LXOR: u8 = 0x83;
const MONITORENTER: u8 = 0xc2;
const MONITOREXIT: u8 = 0xc3;
const MULTIANEWARRAY: u8 = 0xc5;
const NEW: u8 = 0xbb;
const NEWARRAY: u8 = 0xbc;
const NOP: u8 = 0x0;
const POP: u8 = 0x57;
const POP2: u8 = 0x58;
const PUTFIELD: u8 = 0xb5;
const PUTSTATIC: u8 = 0xb3;
const RET: u8 = 0xa9;
const RETURN: u8 = 0xb1;
const SALOAD: u8 = 0x35;
const SASTORE: u8 = 0x56;
const SIPUSH: u8 = 0x11;
const SWAP: u8 = 0x5f;
const TABLESWITCH: u8 = 0xaa;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Aaload,                                                              // aaload
    Aastore,                                                             // aastore
    AconstNull,                                                          // aconst_null
    Aload(u8),                                                           // aload index
    AloadN(u8),                                                          // aload_<n>
    Anewarray(u8, u8),               // anewarray indexbyte1 indexbyte2
    Areturn,                         // areturn
    Arraylength,                     // arraylength
    Astore(u8),                      // astore index
    AstoreN(u8),                     // astore_<n>
    Athrow,                          // athrow
    Baload,                          // baload
    Bastore,                         // bastore
    Bipush(u8),                      // bipush byte
    Caload,                          // caload
    Castore,                         // castore
    Checkcast(u8, u8),               // checkcast indexbyte1 indexbyte2
    D2f,                             // d2f
    D2i,                             // d2i
    D2l,                             // d2l
    Dadd,                            // dadd
    Daload,                          // daload
    Dastore,                         // dastore
    DcmOP,                           // dcmp<op>
    DconstD,                         // dconst_<d>
    Ddiv,                            // ddiv
    Dload(u8),                       // dload index
    DloadN,                          // dload_<n>
    Dmul,                            // dmul
    Dneg,                            // dneg
    Drem,                            // drem
    Dreturn,                         // dreturn
    Dstore(u8),                      // dstore index
    DstoreN,                         // dstore_<n>
    Dsub,                            // dsub
    Dup,                             // dup
    DupX1,                           // dup_x1
    DupX2,                           // dup_x2
    Dup2,                            // dup2
    Dup2X1,                          // dup2_x1
    Dup2X2,                          // dup2_x2
    F2d,                             // f2d
    F2i,                             // f2i
    F2l,                             // f2l
    Fadd,                            // fadd
    Faload,                          // faload
    Fastore,                         // fastore
    FcmOP,                           // fcmp<op>
    FconstF,                         // fconst_<f>
    Fdiv,                            // fdiv
    Fload(u8),                       // fload index
    FloadN,                          // fload_<n>
    Fmul,                            // fmul
    Fneg,                            // fneg
    Frem,                            // frem
    Freturn,                         // freturn
    Fstore(u8),                      // fstore index
    FstoreN,                         // fstore_<n>
    Fsub,                            // fsub
    Getfield(u8, u8),                // getfield indexbyte1 indexbyte2
    Getstatic(u8, u8),               // getstatic indexbyte1 indexbyte2
    Goto(u8, u8),                    // goto branchbyte1 branchbyte2
    GotoW(u8, u8, u8, u8),           // goto_w branchbyte1 branchbyte2 branchbyte3 branchbyte4
    I2b,                             // i2b
    I2c,                             // i2c
    I2d,                             // i2d
    I2f,                             // i2f
    I2l,                             // i2l
    I2s,                             // i2s
    Iadd,                            // iadd
    Iaload,                          // iaload
    Iand,                            // iand
    Iastore,                         // iastore
    IconstI,                         // iconst_<i>
    Idiv,                            // idiv
    IfAcmCond(u8, u8),               // if_acmp<cond> branchbyte1 branchbyte2
    IfIcmCond(u8, u8),               // if_icmp<cond> branchbyte1 branchbyte2
    ICOND(u8, u8),                   // if<cond> branchbyte1 branchbyte2
    Ifnonnull(u8, u8),               // ifnonnull branchbyte1 branchbyte2
    Ifnull(u8, u8),                  // ifnull branchbyte1 branchbyte2
    Iinc(u8, u8),                    // iinc index const
    Iload(u8),                       // iload index
    IloadN(u8),                      // iload_<n>
    Imul,                            // imul
    Ineg,                            // ineg
    Instanceof(u8, u8),              // instanceof indexbyte1 indexbyte2
    Invokedynamic(u8, u8, u8, u8),   // invokedynamic indexbyte1 indexbyte2 0 0
    Invokeinterface(u8, u8, u8, u8), // invokeinterface indexbyte1 indexbyte2 count 0
    Invokespecial(u8, u8),           // invokespecial indexbyte1 indexbyte2
    Invokestatic(u8, u8),            // invokestatic indexbyte1 indexbyte2
    Invokevirtual(u8, u8),           // invokevirtual indexbyte1 indexbyte2
    Ior,                             // ior
    Irem,                            // irem
    Ireturn,                         // ireturn
    Ishl,                            // ishl
    Ishr,                            // ishr
    Istore(u8),                      // istore index
    IstoreN(u8),                     // istore_<n>
    Isub,                            // isub
    Iushr,                           // iushr
    Ixor,                            // ixor
    Jsr(u8, u8),                     // jsr branchbyte1 branchbyte2
    JsrW(u8, u8, u8, u8),            // jsr_w branchbyte1 branchbyte2 branchbyte3 branchbyte4
    L2d,                             // l2d
    L2f,                             // l2f
    L2i,                             // l2i
    Ladd,                            // ladd
    Laload,                          // laload
    Land,                            // land
    Lastore,                         // lastore
    Lcmp,                            // lcmp
    LconstL(u8),                     // lconst_<l>
    Ldc(u8),                         // ldc index
    LdcW(u8, u8),                    // ldc_w indexbyte1 indexbyte2
    Ldc2W(u8, u8),                   // ldc2_w indexbyte1 indexbyte2
    Ldiv,                            // ldiv
    Lload(u8),                       // lload index
    LloadN(u8),                      // lload_<n>
    Lmul,                            // lmul
    Lneg,                            // lneg
    Lookupswitch(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), // lookupswitch <0-3 byte pad> defaultbyte1 defaultbyte2 defaultbyte3 defaultbyte4 npairs1 npairs2 npairs3 npairs4 match-offset pairs...
    Lor,                                                  // lor
    Lrem,                                                 // lrem
    Lreturn,                                              // lreturn
    Lshl,                                                 // lshl
    Lshr,                                                 // lshr
    Lstore(u8),                                           // lstore index
    LstoreN(u8),                                          // lstore_<n>
    Lsub,                                                 // lsub
    Lushr,                                                // lushr
    Lxor,                                                 // lxor
    Monitorenter,                                         // monitorenter
    Monitorexit,                                          // monitorexit
    Multianewarray(u8, u8, u8), // multianewarray indexbyte1 indexbyte2 dimensions
    New(u8, u8),                // new indexbyte1 indexbyte2
    Newarray(u8),               // newarray atype
    Nop,                        // nop
    Pop,                        // pop
    Pop2,                       // pop2
    Putfield(u8, u8),           // putfield indexbyte1 indexbyte2
    Putstatic(u8, u8),          // putstatic indexbyte1 indexbyte2
    Ret(u8),                    // ret index
    Return_,                    // return
    Saload,                     // saload
    Sastore,                    // sastore
    Sipush(u8, u8),             // sipush byte1 byte2
    Swap,                       // swap
    Tableswitch(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), // tableswitch <0-3 byte pad> defaultbyte1 defaultbyte2 defaultbyte3 defaultbyte4 lowbyte1 lowbyte2 lowbyte3 lowbyte4 highbyte1 highbyte2 highbyte3 highbyte4 jump offsets...
}
impl ToJvmBytecode for Opcode {
    fn to_jvm_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];

        match self {
            Opcode::Aaload => bytecode.push(AALOAD),
            Opcode::Aastore => bytecode.push(AASTORE),
            Opcode::AconstNull => bytecode.push(ACONST_NULL),
            Opcode::Aload(index) => {
                bytecode.push(ALOAD);
                bytecode.push(*index);
            }
            Opcode::AloadN(n) => {
                bytecode.push(ALOAD_N + n);
            }
            Opcode::Anewarray(index1, index2) => {
                bytecode.push(ANEWARRAY);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Arraylength => bytecode.push(ARRAYLENGTH),
            Opcode::Astore(index) => {
                bytecode.push(ASTORE);
                bytecode.push(*index);
            }
            Opcode::AstoreN(n) => {
                bytecode.push(ASTORE_N + n);
            }
            Opcode::Athrow => bytecode.push(ATHROW),
            Opcode::Areturn => bytecode.push(ARETURN),
            Opcode::Baload => bytecode.push(BALOAD),
            Opcode::Bastore => bytecode.push(BASTORE),
            Opcode::Bipush(byte) => {
                bytecode.push(BIPUSH);
                bytecode.push(*byte);
            }
            Opcode::Caload => bytecode.push(CALOAD),
            Opcode::Castore => bytecode.push(CASTORE),
            Opcode::Checkcast(index1, index2) => {
                bytecode.push(CHECKCAST);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::D2f => bytecode.push(D2F),
            Opcode::D2i => bytecode.push(D2I),
            Opcode::D2l => bytecode.push(D2L),
            Opcode::Dadd => bytecode.push(DADD),
            Opcode::Daload => bytecode.push(DALOAD),
            Opcode::Dastore => bytecode.push(DASTORE),
            Opcode::DcmOP => bytecode.push(DCM_N),
            Opcode::DconstD => bytecode.push(DCONST_N),
            Opcode::Ddiv => bytecode.push(DDIV),
            Opcode::Dload(index) => {
                bytecode.push(DLOAD);
                bytecode.push(*index);
            }
            Opcode::DloadN => bytecode.push(DLOAD_N),
            Opcode::Dmul => bytecode.push(DMUL),
            Opcode::Dneg => bytecode.push(DNEG),
            Opcode::Drem => bytecode.push(DREM),
            Opcode::Dreturn => bytecode.push(DRETURN),
            Opcode::Dstore(index) => {
                bytecode.push(DSTORE);
                bytecode.push(*index);
            }
            Opcode::DstoreN => bytecode.push(DSTORE_N),
            Opcode::Dsub => bytecode.push(DSUB),
            Opcode::Dup => bytecode.push(DUP),
            Opcode::DupX1 => bytecode.push(DUP_X1),
            Opcode::DupX2 => bytecode.push(DUP_X2),
            Opcode::Dup2 => bytecode.push(DUP2),
            Opcode::Dup2X1 => bytecode.push(DUP2_X1),
            Opcode::Dup2X2 => bytecode.push(DUP2_X2),
            Opcode::F2d => bytecode.push(F2D),
            Opcode::F2i => bytecode.push(F2I),
            Opcode::F2l => bytecode.push(F2L),
            Opcode::Fadd => bytecode.push(FADD),
            Opcode::Faload => bytecode.push(FALOAD),
            Opcode::Fastore => bytecode.push(FASTORE),
            Opcode::FcmOP => bytecode.push(FCM_N),
            Opcode::FconstF => bytecode.push(FCONST_N),
            Opcode::Fdiv => bytecode.push(FDIV),
            Opcode::Fload(index) => {
                bytecode.push(FLOAD);
                bytecode.push(*index);
            }
            Opcode::FloadN => bytecode.push(FLOAD_N),
            Opcode::Fmul => bytecode.push(FMUL),
            Opcode::Fneg => bytecode.push(FNEG),
            Opcode::Frem => bytecode.push(FREM),
            Opcode::Freturn => bytecode.push(FRETURN),
            Opcode::Fstore(index) => {
                bytecode.push(FSTORE);
                bytecode.push(*index);
            }
            Opcode::FstoreN => bytecode.push(FSTORE_N),
            Opcode::Fsub => bytecode.push(FSUB),
            Opcode::Getfield(index1, index2) => {
                bytecode.push(GETFIELD);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Getstatic(index1, index2) => {
                bytecode.push(GETSTATIC);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Goto(branch1, branch2) => {
                bytecode.push(GOTO);
                bytecode.push(*branch1);
                bytecode.push(*branch2);
            }
            Opcode::GotoW(branch1, branch2, branch3, branch4) => {
                bytecode.push(GOTO_W);
                bytecode.push(*branch1);
                bytecode.push(*branch2);
                bytecode.push(*branch3);
                bytecode.push(*branch4);
            }
            Opcode::I2b => bytecode.push(I2B),
            Opcode::I2c => bytecode.push(I2C),
            Opcode::I2d => bytecode.push(I2D),
            Opcode::I2f => bytecode.push(I2F),
            Opcode::I2l => bytecode.push(I2L),
            Opcode::I2s => bytecode.push(I2S),
            Opcode::Iadd => bytecode.push(IADD),
            Opcode::Iaload => bytecode.push(IALOAD),
            Opcode::Iand => bytecode.push(IAND),
            Opcode::Iastore => bytecode.push(IASTORE),
            Opcode::IconstI => bytecode.push(ICONST_N),
            Opcode::Idiv => bytecode.push(IDIV),
            Opcode::IfAcmCond(branch1, branch2) => {
                bytecode.push(IF_ACM_N);
                bytecode.push(*branch1);
                bytecode.push(*branch2);
            }
            Opcode::IfIcmCond(branch1, branch2) => {
                bytecode.push(IF_ICM_N);
                bytecode.push(*branch1);
                bytecode.push(*branch2);
            }
            Opcode::ICOND(branch1, branch2) => {
                bytecode.push(I_N);
                bytecode.push(*branch1);
                bytecode.push(*branch2);
            }
            Opcode::Ifnonnull(branch1, branch2) => {
                bytecode.push(IFNONNULL);
                bytecode.push(*branch1);
                bytecode.push(*branch2);
            }
            Opcode::Ifnull(branch1, branch2) => {
                bytecode.push(IFNULL);
                bytecode.push(*branch1);
                bytecode.push(*branch2);
            }
            Opcode::Iinc(index, const_) => {
                bytecode.push(IINC);
                bytecode.push(*index);
                bytecode.push(*const_);
            }
            Opcode::Iload(index) => {
                bytecode.push(ILOAD);
                bytecode.push(*index);
            }
            Opcode::IloadN(n) => {
                bytecode.push(ILOAD_N + n);
            }
            Opcode::Imul => bytecode.push(IMUL),
            Opcode::Ineg => bytecode.push(INEG),
            Opcode::Instanceof(index1, index2) => {
                bytecode.push(INSTANCEOF);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Invokedynamic(index1, index2, _, _) => {
                bytecode.push(INVOKEDYNAMIC);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Invokeinterface(index1, index2, count, _) => {
                bytecode.push(INVOKEINTERFACE);
                bytecode.push(*index1);
                bytecode.push(*index2);
                bytecode.push(*count);
            }
            Opcode::Invokespecial(index1, index2) => {
                bytecode.push(INVOKESPECIAL);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Invokestatic(index1, index2) => {
                bytecode.push(INVOKESTATIC);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Invokevirtual(index1, index2) => {
                bytecode.push(INVOKEVIRTUAL);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Ior => bytecode.push(IOR),
            Opcode::Irem => bytecode.push(IREM),
            Opcode::Ireturn => bytecode.push(IRETURN),
            Opcode::Ishl => bytecode.push(ISHL),
            Opcode::Ishr => bytecode.push(ISHR),
            Opcode::Istore(index) => {
                bytecode.push(ISTORE);
                bytecode.push(*index);
            }
            Opcode::IstoreN(n) => {
                bytecode.push(ISTORE_N + n);
            }
            Opcode::Isub => bytecode.push(ISUB),
            Opcode::Iushr => bytecode.push(IUSHR),
            Opcode::Ixor => bytecode.push(IXOR),
            Opcode::Jsr(branch1, branch2) => {
                bytecode.push(JSR);
                bytecode.push(*branch1);
                bytecode.push(*branch2);
            }
            Opcode::JsrW(branch1, branch2, branch3, branch4) => {
                bytecode.push(JSR_W);
                bytecode.push(*branch1);
                bytecode.push(*branch2);
                bytecode.push(*branch3);
                bytecode.push(*branch4);
            }
            Opcode::L2d => bytecode.push(L2D),
            Opcode::L2f => bytecode.push(L2F),
            Opcode::L2i => bytecode.push(L2I),
            Opcode::Ladd => bytecode.push(LADD),
            Opcode::Laload => bytecode.push(LALOAD),
            Opcode::Land => bytecode.push(LAND),
            Opcode::Lastore => bytecode.push(LASTORE),
            Opcode::Lcmp => bytecode.push(LCMP),
            Opcode::LconstL(n) => bytecode.push(LCONST_N + n),
            Opcode::Ldc(index) => {
                bytecode.push(LDC);
                bytecode.push(*index);
            }
            Opcode::LdcW(index1, index2) => {
                bytecode.push(LDC_W);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Ldc2W(index1, index2) => {
                bytecode.push(LDC2_W);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Ldiv => bytecode.push(LDIV),
            Opcode::Lload(index) => {
                bytecode.push(LLOAD);
                bytecode.push(*index);
            }
            Opcode::LloadN(n) => bytecode.push(LLOAD_N + n),
            Opcode::Lmul => bytecode.push(LMUL),
            Opcode::Lneg => bytecode.push(LNEG),
            Opcode::Lookupswitch(
                defaultbyte1,
                defaultbyte2,
                defaultbyte3,
                defaultbyte4,
                npairs1,
                npairs2,
                npairs3,
                npairs4,
                match_offset,
                pairs,
            ) => {
                todo!()
            }
            Opcode::Lor => bytecode.push(LOR),
            Opcode::Lrem => bytecode.push(LREM),
            Opcode::Lreturn => bytecode.push(LRETURN),
            Opcode::Lshl => bytecode.push(LSHL),
            Opcode::Lshr => bytecode.push(LSHR),
            Opcode::Lstore(index) => {
                bytecode.push(LSTORE);
                bytecode.push(*index);
            }
            Opcode::LstoreN(n) => bytecode.push(LSTORE_N + n),
            Opcode::Lsub => bytecode.push(LSUB),
            Opcode::Lushr => bytecode.push(LUSHR),
            Opcode::Lxor => bytecode.push(LXOR),
            Opcode::Monitorenter => bytecode.push(MONITORENTER),
            Opcode::Monitorexit => bytecode.push(MONITOREXIT),
            Opcode::Multianewarray(index1, index2, dimensions) => {
                bytecode.push(MULTIANEWARRAY);
                bytecode.push(*index1);
                bytecode.push(*index2);
                bytecode.push(*dimensions);
            }
            Opcode::New(index1, index2) => {
                bytecode.push(NEW);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Newarray(atype) => {
                bytecode.push(NEWARRAY);
                bytecode.push(*atype);
            }
            Opcode::Nop => bytecode.push(NOP),
            Opcode::Pop => bytecode.push(POP),
            Opcode::Pop2 => bytecode.push(POP2),
            Opcode::Putfield(index1, index2) => {
                bytecode.push(PUTFIELD);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Putstatic(index1, index2) => {
                bytecode.push(PUTSTATIC);
                bytecode.push(*index1);
                bytecode.push(*index2);
            }
            Opcode::Ret(index) => {
                bytecode.push(RET);
                bytecode.push(*index);
            }
            Opcode::Return_ => bytecode.push(RETURN),
            Opcode::Saload => bytecode.push(SALOAD),
            Opcode::Sastore => bytecode.push(SASTORE),
            Opcode::Sipush(byte1, byte2) => {
                bytecode.push(SIPUSH);
                bytecode.push(*byte1);
                bytecode.push(*byte2);
            }
            Opcode::Swap => bytecode.push(SWAP),
            Opcode::Tableswitch(..) => {
                todo!()
            }
        }
        bytecode
    }
}
impl Opcode {
    pub fn from_reader(reader: &mut Reader) -> Option<Opcode> {
        let opcode = reader.read_int1();
        match opcode {
            AALOAD => Some(Opcode::Aaload),
            AASTORE => Some(Opcode::Aastore),
            ACONST_NULL => Some(Opcode::AconstNull),
            ALOAD => Some(Opcode::Aload(reader.read_int1())),
            n if n >= ALOAD_N && n - ALOAD_N <= 3 => Some(Opcode::AloadN(n - ALOAD_N)),
            ANEWARRAY => Some(Opcode::Anewarray(reader.read_int1(), reader.read_int1())),
            ARRAYLENGTH => Some(Opcode::Arraylength),
            ASTORE => Some(Opcode::Astore(reader.read_int1())),
            n if n >= ASTORE_N && n - ASTORE_N <= 3 => Some(Opcode::AstoreN(n - ASTORE_N)),
            ATHROW => Some(Opcode::Athrow),
            BALOAD => Some(Opcode::Baload),
            BASTORE => Some(Opcode::Bastore),
            BIPUSH => Some(Opcode::Bipush(reader.read_int1())),
            CALOAD => Some(Opcode::Caload),
            CASTORE => Some(Opcode::Castore),
            CHECKCAST => Some(Opcode::Checkcast(reader.read_int1(), reader.read_int1())),
            D2F => Some(Opcode::D2f),
            D2I => Some(Opcode::D2i),
            D2L => Some(Opcode::D2l),
            DADD => Some(Opcode::Dadd),
            DALOAD => Some(Opcode::Daload),
            DASTORE => Some(Opcode::Dastore),
            DCM_N => Some(Opcode::DcmOP),
            n if n >= DCONST_N && n - DCONST_N <= 1 => Some(Opcode::DconstD),
            DDIV => Some(Opcode::Ddiv),
            DLOAD => Some(Opcode::Dload(reader.read_int1())),
            n if n >= DLOAD_N && n - DLOAD_N <= 3 => Some(Opcode::DloadN),
            DMUL => Some(Opcode::Dmul),
            DNEG => Some(Opcode::Dneg),
            DREM => Some(Opcode::Drem),
            DRETURN => Some(Opcode::Dreturn),
            DSTORE => Some(Opcode::Dstore(reader.read_int1())),
            n if n >= DSTORE_N && n - DSTORE_N <= 3 => Some(Opcode::DstoreN),
            DSUB => Some(Opcode::Dsub),
            DUP => Some(Opcode::Dup),
            DUP_X1 => Some(Opcode::DupX1),
            DUP_X2 => Some(Opcode::DupX2),
            DUP2 => Some(Opcode::Dup2),
            DUP2_X1 => Some(Opcode::Dup2X1),
            DUP2_X2 => Some(Opcode::Dup2X2),
            F2D => Some(Opcode::F2d),
            F2I => Some(Opcode::F2i),
            F2L => Some(Opcode::F2l),
            FADD => Some(Opcode::Fadd),
            FALOAD => Some(Opcode::Faload),
            FASTORE => Some(Opcode::Fastore),
            FCM_N => Some(Opcode::FcmOP),
            n if n >= FCONST_N && n - FCONST_N <= 2 => Some(Opcode::FconstF),
            FDIV => Some(Opcode::Fdiv),
            FLOAD => Some(Opcode::Fload(reader.read_int1())),
            n if n >= FLOAD_N && n - FLOAD_N <= 3 => Some(Opcode::FloadN),
            FMUL => Some(Opcode::Fmul),
            FNEG => Some(Opcode::Fneg),
            FREM => Some(Opcode::Frem),
            FRETURN => Some(Opcode::Freturn),
            FSTORE => Some(Opcode::Fstore(reader.read_int1())),
            n if n >= FSTORE_N && n - FSTORE_N <= 3 => Some(Opcode::FstoreN),
            FSUB => Some(Opcode::Fsub),
            GETFIELD => Some(Opcode::Getfield(reader.read_int1(), reader.read_int1())),
            GETSTATIC => Some(Opcode::Getstatic(reader.read_int1(), reader.read_int1())),
            GOTO => Some(Opcode::Goto(reader.read_int1(), reader.read_int1())),
            GOTO_W => Some(Opcode::GotoW(
                reader.read_int1(),
                reader.read_int1(),
                reader.read_int1(),
                reader.read_int1(),
            )),
            I2B => Some(Opcode::I2b),
            I2C => Some(Opcode::I2c),
            I2D => Some(Opcode::I2d),
            I2F => Some(Opcode::I2f),
            I2L => Some(Opcode::I2l),
            I2S => Some(Opcode::I2s),
            IADD => Some(Opcode::Iadd),
            IALOAD => Some(Opcode::Iaload),
            IAND => Some(Opcode::Iand),
            IASTORE => Some(Opcode::Iastore),
            n if n >= ICONST_N && n - ICONST_N <= 5 => Some(Opcode::IconstI),
            IDIV => Some(Opcode::Idiv),
            IF_ACM_N => Some(Opcode::IfAcmCond(reader.read_int1(), reader.read_int1())),
            IF_ICM_N => Some(Opcode::IfIcmCond(reader.read_int1(), reader.read_int1())),
            I_N => Some(Opcode::ICOND(reader.read_int1(), reader.read_int1())),
            IFNONNULL => Some(Opcode::Ifnonnull(reader.read_int1(), reader.read_int1())),
            IFNULL => Some(Opcode::Ifnull(reader.read_int1(), reader.read_int1())),
            IINC => Some(Opcode::Iinc(reader.read_int1(), reader.read_int1())),
            ILOAD => Some(Opcode::Iload(reader.read_int1())),
            n if n >= ILOAD_N && n - ILOAD_N <= 3 => Some(Opcode::IloadN(n - ILOAD_N)),
            IMUL => Some(Opcode::Imul),
            INEG => Some(Opcode::Ineg),
            INSTANCEOF => Some(Opcode::Instanceof(reader.read_int1(), reader.read_int1())),
            INVOKEDYNAMIC => Some(Opcode::Invokedynamic(
                reader.read_int1(),
                reader.read_int1(),
                0,
                0,
            )),
            INVOKEINTERFACE => Some(Opcode::Invokeinterface(
                reader.read_int1(),
                reader.read_int1(),
                reader.read_int1(),
                0,
            )),
            INVOKESPECIAL => Some(Opcode::Invokespecial(
                reader.read_int1(),
                reader.read_int1(),
            )),
            INVOKESTATIC => Some(Opcode::Invokestatic(reader.read_int1(), reader.read_int1())),
            INVOKEVIRTUAL => Some(Opcode::Invokevirtual(
                reader.read_int1(),
                reader.read_int1(),
            )),
            IOR => Some(Opcode::Ior),
            IREM => Some(Opcode::Irem),
            IRETURN => Some(Opcode::Ireturn),
            ISHL => Some(Opcode::Ishl),
            ISHR => Some(Opcode::Ishr),
            ISTORE => Some(Opcode::Istore(reader.read_int1())),
            n if n >= ISTORE_N && n - ISTORE_N <= 3 => Some(Opcode::IstoreN(n - ISTORE)),
            ISUB => Some(Opcode::Isub),
            IUSHR => Some(Opcode::Iushr),
            IXOR => Some(Opcode::Ixor),
            JSR => Some(Opcode::Jsr(reader.read_int1(), reader.read_int1())),
            JSR_W => Some(Opcode::JsrW(
                reader.read_int1(),
                reader.read_int1(),
                reader.read_int1(),
                reader.read_int1(),
            )),
            L2D => Some(Opcode::L2d),
            L2F => Some(Opcode::L2f),
            L2I => Some(Opcode::L2i),
            LADD => Some(Opcode::Ladd),
            LALOAD => Some(Opcode::Laload),
            LAND => Some(Opcode::Land),
            LASTORE => Some(Opcode::Lastore),
            LCMP => Some(Opcode::Lcmp),
            n if n >= LCONST_N && n - LCONST_N <= 3 => Some(Opcode::LconstL(n - LCONST_N)),
            LDC => Some(Opcode::Ldc(reader.read_int1())),
            LDC_W => Some(Opcode::LdcW(reader.read_int1(), reader.read_int1())),
            LDC2_W => Some(Opcode::Ldc2W(reader.read_int1(), reader.read_int1())),
            LDIV => Some(Opcode::Ldiv),
            LLOAD => Some(Opcode::Lload(reader.read_int1())),
            n if n >= LLOAD_N && n - LLOAD_N <= 3 => Some(Opcode::LloadN(n - LLOAD_N)),
            LMUL => Some(Opcode::Lmul),
            LNEG => Some(Opcode::Lneg),
            LOOKUPSWITCH => todo!(),
            LOR => Some(Opcode::Lor),
            LREM => Some(Opcode::Lrem),
            LRETURN => Some(Opcode::Lreturn),
            LSHL => Some(Opcode::Lshl),
            LSHR => Some(Opcode::Lshr),
            LSTORE => Some(Opcode::Lstore(reader.read_int1())),
            n if n >= LSTORE_N && n - LSTORE_N <= 3 => Some(Opcode::LstoreN(n - LSTORE_N)),
            LSUB => Some(Opcode::Lsub),
            LUSHR => Some(Opcode::Lushr),
            LXOR => Some(Opcode::Lxor),
            MONITORENTER => Some(Opcode::Monitorenter),
            MONITOREXIT => Some(Opcode::Monitorexit),
            MULTIANEWARRAY => Some(Opcode::Multianewarray(
                reader.read_int1(),
                reader.read_int1(),
                reader.read_int1(),
            )),
            NEW => Some(Opcode::New(reader.read_int1(), reader.read_int1())),
            NEWARRAY => Some(Opcode::Newarray(reader.read_int1())),
            NOP => Some(Opcode::Nop),
            POP => Some(Opcode::Pop),
            POP2 => Some(Opcode::Pop2),
            PUTFIELD => Some(Opcode::Putfield(reader.read_int1(), reader.read_int1())),
            PUTSTATIC => Some(Opcode::Putstatic(reader.read_int1(), reader.read_int1())),
            RET => Some(Opcode::Ret(reader.read_int1())),
            RETURN => Some(Opcode::Return_),
            SALOAD => Some(Opcode::Saload),
            SASTORE => Some(Opcode::Sastore),
            SIPUSH => Some(Opcode::Sipush(reader.read_int1(), reader.read_int1())),
            SWAP => Some(Opcode::Swap),
            TABLESWITCH => todo!(),
            _ => None,
        }
    }
}
