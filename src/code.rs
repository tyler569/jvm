use bytes::{Buf, Bytes};
use crate::class::{Attribute};
use crate::constant::Constant;

#[derive(Clone, Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<Instr>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<Attribute>,
}

impl Code {
    pub fn from_bytes(bytes: &mut Bytes, constant_pool: &[Constant]) -> Self {
        let max_stack = bytes.get_u16();
        let max_locals = bytes.get_u16();
        let code_length = bytes.get_u32() as usize;
        let mut code_bytes = bytes.copy_to_bytes(code_length);
        let exception_table = parse_exception_table(bytes);
        let attributes = Attribute::parse_list(bytes, constant_pool);

        let mut code = Vec::new();

        while !code_bytes.is_empty() {
            code.push(Instr::from_bytes(&mut code_bytes));
        }

        Self {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Instr {
    Aaload,
    Aastore,
    AconstNull,
    Aload(u8),
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Anewarray(u16),
    Areturn,
    Arraylength,
    Astore(u8),
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Athrow,
    Baload,
    Bastore,
    Bipush(i8),
    Caload,
    Castore,
    Checkcast(u16),
    D2f,
    D2i,
    D2l,
    Dadd,
    Daload,
    Dastore,
    Dcmpg,
    Dcmpl,
    Dconst0,
    Dconst1,
    Ddiv,
    Dload(u8),
    Dload0,
    Dload1,
    Dload2,
    Dload3,
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    Dstore(u8),
    Dstore0,
    Dstore1,
    Dstore2,
    Dstore3,
    Dsub,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    F2d,
    F2i,
    F2l,
    Fadd,
    Faload,
    Fastore,
    Fcmpg,
    Fcmpl,
    Fconst0,
    Fconst1,
    Fconst2,
    Fdiv,
    Fload(u8),
    Fload0,
    Fload1,
    Fload2,
    Fload3,
    Fmul,
    Fneg,
    Frem,
    Freturn,
    Fstore(u8),
    Fstore0,
    Fstore1,
    Fstore2,
    Fstore3,
    Fsub,
    Getfield(u16),
    Getstatic(u16),
    Goto(i16),
    GotoW(i32),
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iaload,
    Iand,
    Iastore,
    IconstM1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Idiv,
    IfAcmpeq(i16),
    IfAcmpne(i16),
    IfIcmpeq(i16),
    IfIcmpne(i16),
    IfIcmplt(i16),
    IfIcmpge(i16),
    IfIcmpgt(i16),
    IfIcmple(i16),
    Ifeq(i16),
    Ifne(i16),
    Iflt(i16),
    Ifge(i16),
    Ifgt(i16),
    Ifle(i16),
    Ifnonnull(i16),
    Ifnull(i16),
    Iinc(u8, i8),
    Iload(u8),
    Iload0,
    Iload1,
    Iload2,
    Iload3,
    Imul,
    Ineg,
    Instanceof(u16),
    Invokedynamic(u16),
    Invokeinterface(u16, u8),
    Invokespecial(u16),
    Invokestatic(u16),
    Invokevirtual(u16),
    Ior,
    Irem,
    Ireturn,
    Ishl,
    Ishr,
    Istore(u8),
    Istore0,
    Istore1,
    Istore2,
    Istore3,
    Isub,
    Iushr,
    Ixor,
    Jsr(i16),
    JsrW(i32),
    L2d,
    L2f,
    L2i,
    Ladd,
    Laload,
    Land,
    Lastore,
    Lcmp,
    Lconst0,
    Lconst1,
    Ldc(u8),
    LdcW(u16),
    Ldc2W(u16),
    Ldiv,
    Lload(u8),
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Lmul,
    Lneg,
    Lookupswitch {
        default: i32,
        npairs: u32,
        match_offsets: Vec<(i32, i32)>,
    },
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    Lstore(u8),
    Lstore0,
    Lstore1,
    Lstore2,
    Lstore3,
    Lsub,
    Lushr,
    Lxor,
    Monitorenter,
    Monitorexit,
    Multianewarray(u16, u8),
    New(u16),
    Newarray(u8),
    Nop,
    Pop,
    Pop2,
    Putfield(u16),
    Putstatic(u16),
    Ret(u8),
    Return,
    Saload,
    Sastore,
    Sipush(i16),
    Swap,
    Tableswitch {
        default: i32,
        low: i32,
        high: i32,
        jump_offsets: Vec<i32>,
    },
    Wide {
        instr: Box<Instr>,
        index: u16,
    },
}

impl Instr {
    pub fn from_bytes(bytes: &mut Bytes) -> Self {
        match bytes.get_u8() {
            0x32 => Self::Aaload,
            0x53 => Self::Aastore,
            0x01 => Self::AconstNull,
            0x19 => Self::Aload(bytes.get_u8()),
            0x2a => Self::Aload0,
            0x2b => Self::Aload1,
            0x2c => Self::Aload2,
            0x2d => Self::Aload3,
            0xbd => Self::Anewarray(bytes.get_u16()),
            0xb0 => Self::Areturn,
            0xbe => Self::Arraylength,
            0x3a => Self::Astore(bytes.get_u8()),
            0x4b => Self::Astore0,
            0x4c => Self::Astore1,
            0x4d => Self::Astore2,
            0x4e => Self::Astore3,
            0xbf => Self::Athrow,
            0x33 => Self::Baload,
            0x54 => Self::Bastore,
            0x10 => Self::Bipush(bytes.get_i8()),
            0x34 => Self::Caload,
            0x55 => Self::Castore,
            0xc0 => Self::Checkcast(bytes.get_u16()),
            0x8e => Self::D2f,
            0x8d => Self::D2i,
            0x8f => Self::D2l,
            0x63 => Self::Dadd,
            0x31 => Self::Daload,
            0x52 => Self::Dastore,
            0x98 => Self::Dcmpg,
            0x97 => Self::Dcmpl,
            0x0e => Self::Dconst0,
            0x0f => Self::Dconst1,
            0x6f => Self::Ddiv,
            0x18 => Self::Dload(bytes.get_u8()),
            0x26 => Self::Dload0,
            0x27 => Self::Dload1,
            0x28 => Self::Dload2,
            0x29 => Self::Dload3,
            0x6b => Self::Dmul,
            0x77 => Self::Dneg,
            0x73 => Self::Drem,
            0xaf => Self::Dreturn,
            0x39 => Self::Dstore(bytes.get_u8()),
            0x47 => Self::Dstore0,
            0x48 => Self::Dstore1,
            0x49 => Self::Dstore2,
            0x4a => Self::Dstore3,
            0x67 => Self::Dsub,
            0x59 => Self::Dup,
            0x5a => Self::DupX1,
            0x5b => Self::DupX2,
            0x5c => Self::Dup2,
            0x5d => Self::Dup2X1,
            0x5e => Self::Dup2X2,
            0x8e => Self::F2d,
            0x8b => Self::F2i,
            0x8c => Self::F2l,
            0x62 => Self::Fadd,
            0x30 => Self::Faload,
            0x51 => Self::Fastore,
            0x96 => Self::Fcmpg,
            0x95 => Self::Fcmpl,
            0x0b => Self::Fconst0,
            0x0c => Self::Fconst1,
            0x0d => Self::Fconst2,
            0x6e => Self::Fdiv,
            0x17 => Self::Fload(bytes.get_u8()),
            0x22 => Self::Fload0,
            0x23 => Self::Fload1,
            0x24 => Self::Fload2,
            0x25 => Self::Fload3,
            0x6a => Self::Fmul,
            0x76 => Self::Fneg,
            0x72 => Self::Frem,
            0xae => Self::Freturn,
            0x38 => Self::Fstore(bytes.get_u8()),
            0x43 => Self::Fstore0,
            0x44 => Self::Fstore1,
            0x45 => Self::Fstore2,
            0x46 => Self::Fstore3,
            0x66 => Self::Fsub,
            0xb4 => Self::Getfield(bytes.get_u16()),
            0xb2 => Self::Getstatic(bytes.get_u16()),
            0xa7 => Self::Goto(bytes.get_i16()),
            0xc8 => Self::GotoW(bytes.get_i32()),
            0x91 => Self::I2b,
            0x92 => Self::I2c,
            0x87 => Self::I2d,
            0x86 => Self::I2f,
            0x85 => Self::I2l,
            0x93 => Self::I2s,
            0x60 => Self::Iadd,
            0x2e => Self::Iaload,
            0x7e => Self::Iand,
            0x4f => Self::Iastore,
            0x02 => Self::IconstM1,
            0x03 => Self::Iconst0,
            0x04 => Self::Iconst1,
            0x05 => Self::Iconst2,
            0x06 => Self::Iconst3,
            0x07 => Self::Iconst4,
            0x08 => Self::Iconst5,
            0x6c => Self::Idiv,
            0xa5 => Self::IfAcmpeq(bytes.get_i16()),
            0xa6 => Self::IfAcmpne(bytes.get_i16()),
            0x9f => Self::IfIcmpeq(bytes.get_i16()),
            0xa0 => Self::IfIcmpne(bytes.get_i16()),
            0xa1 => Self::IfIcmplt(bytes.get_i16()),
            0xa2 => Self::IfIcmpge(bytes.get_i16()),
            0xa3 => Self::IfIcmpgt(bytes.get_i16()),
            0xa4 => Self::IfIcmple(bytes.get_i16()),
            0x99 => Self::Ifeq(bytes.get_i16()),
            0x9a => Self::Ifne(bytes.get_i16()),
            0x9b => Self::Iflt(bytes.get_i16()),
            0x9c => Self::Ifge(bytes.get_i16()),
            0x9d => Self::Ifgt(bytes.get_i16()),
            0x9e => Self::Ifle(bytes.get_i16()),
            0xc7 => Self::Ifnonnull(bytes.get_i16()),
            0xc6 => Self::Ifnull(bytes.get_i16()),
            0x84 => Self::Iinc(bytes.get_u8(), bytes.get_i8()),
            0x15 => Self::Iload(bytes.get_u8()),
            0x1a => Self::Iload0,
            0x1b => Self::Iload1,
            0x1c => Self::Iload2,
            0x1d => Self::Iload3,
            0x68 => Self::Imul,
            0x74 => Self::Ineg,
            0xc1 => Self::Instanceof(bytes.get_u16()),
            0xba => Self::Invokedynamic(bytes.get_u16()),
            0xb9 => Self::Invokeinterface(bytes.get_u16(), bytes.get_u8()),
            0xb7 => Self::Invokespecial(bytes.get_u16()),
            0xb8 => Self::Invokestatic(bytes.get_u16()),
            0xb6 => Self::Invokevirtual(bytes.get_u16()),
            0x80 => Self::Ior,
            0x70 => Self::Irem,
            0xac => Self::Ireturn,
            0x78 => Self::Ishl,
            0x7a => Self::Ishr,
            0x36 => Self::Istore(bytes.get_u8()),
            0x3b => Self::Istore0,
            0x3c => Self::Istore1,
            0x3d => Self::Istore2,
            0x3e => Self::Istore3,
            0x64 => Self::Isub,
            0x7c => Self::Iushr,
            0x82 => Self::Ixor,
            0xa8 => Self::Jsr(bytes.get_i16()),
            0xc9 => Self::JsrW(bytes.get_i32()),
            0x8a => Self::L2d,
            0x89 => Self::L2f,
            0x88 => Self::L2i,
            0x61 => Self::Ladd,
            0x2f => Self::Laload,
            0x7f => Self::Land,
            0x50 => Self::Lastore,
            0x94 => Self::Lcmp,
            0x09 => Self::Lconst0,
            0x0a => Self::Lconst1,
            0x12 => Self::Ldc(bytes.get_u8()),
            0x13 => Self::LdcW(bytes.get_u16()),
            0x14 => Self::Ldc2W(bytes.get_u16()),
            0x6d => Self::Ldiv,
            0x16 => Self::Lload(bytes.get_u8()),
            0x1e => Self::Lload0,
            0x1f => Self::Lload1,
            0x20 => Self::Lload2,
            0x21 => Self::Lload3,
            0x69 => Self::Lmul,
            0x75 => Self::Lneg,
            0xab => {
                let default = bytes.get_i32();
                let npairs = bytes.get_u32();
                let match_offsets = (0..npairs)
                    .map(|_| (bytes.get_i32(), bytes.get_i32()))
                    .collect();
                Self::Lookupswitch {
                    default,
                    npairs,
                    match_offsets,
                }
            },
            0x81 => Self::Lor,
            0x71 => Self::Lrem,
            0xad => Self::Lreturn,
            0x79 => Self::Lshl,
            0x7b => Self::Lshr,
            0x37 => Self::Lstore(bytes.get_u8()),
            0x3f => Self::Lstore0,
            0x40 => Self::Lstore1,
            0x41 => Self::Lstore2,
            0x42 => Self::Lstore3,
            0x65 => Self::Lsub,
            0x7d => Self::Lushr,
            0x83 => Self::Lxor,
            0xc2 => Self::Monitorenter,
            0xc3 => Self::Monitorexit,
            0xc5 => Self::Multianewarray(bytes.get_u16(), bytes.get_u8()),
            0xbb => Self::New(bytes.get_u16()),
            0xbc => Self::Newarray(bytes.get_u8()),
            0x00 => Self::Nop,
            0x57 => Self::Pop,
            0x58 => Self::Pop2,
            0xb5 => Self::Putfield(bytes.get_u16()),
            0xb3 => Self::Putstatic(bytes.get_u16()),
            0xa9 => Self::Ret(bytes.get_u8()),
            0xb1 => Self::Return,
            0x35 => Self::Saload,
            0x56 => Self::Sastore,
            0x11 => Self::Sipush(bytes.get_i16()),
            0x5f => Self::Swap,
            0xaa => {
                let default = bytes.get_i32();
                let low = bytes.get_i32();
                let high = bytes.get_i32();
                let jump_offsets = (0..high - low + 1)
                    .map(|_| bytes.get_i32())
                    .collect();
                Self::Tableswitch {
                    default,
                    low,
                    high,
                    jump_offsets,
                }
            },
            0xc4 => Self::Wide {
                instr: Box::new(Self::from_bytes(bytes)),
                index: bytes.get_u16(),
            },
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionTableEntry {
    pub fn from_bytes(bytes: &mut Bytes) -> Self {
        let start_pc = bytes.get_u16();
        let end_pc = bytes.get_u16();
        let handler_pc = bytes.get_u16();
        let catch_type = bytes.get_u16();

        Self {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}

fn parse_exception_table(bytes: &mut Bytes) -> Vec<ExceptionTableEntry> {
    let exception_table_length = bytes.get_u16() as usize;
    (0..exception_table_length)
        .map(|_| ExceptionTableEntry::from_bytes(bytes))
        .collect::<Vec<_>>()
}