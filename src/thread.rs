use std::sync::Arc;
use crate::class::{Class, Method};
use crate::code::Instr;
use crate::constant::Constant;
use crate::error::Result;
use crate::interp::Interp;
use crate::value::Value;

pub struct Thread {
    pc: Pc,
    stack: Vec<Frame>,
}

impl Thread {
    pub fn new(class: Arc<Class>, method_index: usize) -> Self {
        let pc = Pc::new(class.clone(), method_index);
        let frame = Frame::for_method(class.clone(), method_index);
        let stack = vec![frame];

        Self {
            pc,
            stack,
        }
    }

    pub fn current_frame(&mut self) -> &mut Frame {
        self.stack.last_mut().unwrap()
    }

    pub fn method(&self) -> &Method {
        &self.pc.class.methods()[self.pc.method_index]
    }

    pub fn exec_one(&mut self, interp: &mut Interp) -> Result<()> {
        let instr = self.method().code().unwrap().code[self.pc.instr].clone();
        let class = self.pc.class.clone();
        let frame = self.current_frame();
        eprintln!("exec: {:?}", instr);
        match instr {
            Instr::Aaload => todo!(),
            Instr::Aastore => todo!(),
            Instr::AconstNull => todo!(),
            Instr::Aload(ix) => frame.stack.push(frame.locals[ix as usize].clone()),
            Instr::Aload0 => frame.stack.push(frame.locals[0].clone()),
            Instr::Aload1 => frame.stack.push(frame.locals[1].clone()),
            Instr::Aload2 => frame.stack.push(frame.locals[2].clone()),
            Instr::Aload3 => frame.stack.push(frame.locals[3].clone()),
            Instr::Anewarray(_) => todo!(),
            Instr::Areturn => todo!(),
            Instr::Arraylength => todo!(),
            Instr::Astore(_) => todo!(),
            Instr::Astore0 => todo!(),
            Instr::Astore1 => todo!(),
            Instr::Astore2 => todo!(),
            Instr::Astore3 => todo!(),
            Instr::Athrow => todo!(),
            Instr::Baload => todo!(),
            Instr::Bastore => todo!(),
            Instr::Bipush(_) => todo!(),
            Instr::Caload => todo!(),
            Instr::Castore => todo!(),
            Instr::Checkcast(_) => todo!(),
            Instr::D2f => todo!(),
            Instr::D2i => todo!(),
            Instr::D2l => todo!(),
            Instr::Dadd => todo!(),
            Instr::Daload => todo!(),
            Instr::Dastore => todo!(),
            Instr::Dcmpg => todo!(),
            Instr::Dcmpl => todo!(),
            Instr::Dconst0 => todo!(),
            Instr::Dconst1 => todo!(),
            Instr::Ddiv => todo!(),
            Instr::Dload(_) => todo!(),
            Instr::Dload0 => todo!(),
            Instr::Dload1 => todo!(),
            Instr::Dload2 => todo!(),
            Instr::Dload3 => todo!(),
            Instr::Dmul => todo!(),
            Instr::Dneg => todo!(),
            Instr::Drem => todo!(),
            Instr::Dreturn => todo!(),
            Instr::Dstore(_) => todo!(),
            Instr::Dstore0 => todo!(),
            Instr::Dstore1 => todo!(),
            Instr::Dstore2 => todo!(),
            Instr::Dstore3 => todo!(),
            Instr::Dsub => todo!(),
            Instr::Dup => todo!(),
            Instr::DupX1 => todo!(),
            Instr::DupX2 => todo!(),
            Instr::Dup2 => todo!(),
            Instr::Dup2X1 => todo!(),
            Instr::Dup2X2 => todo!(),
            Instr::F2d => todo!(),
            Instr::F2i => todo!(),
            Instr::F2l => todo!(),
            Instr::Fadd => todo!(),
            Instr::Faload => todo!(),
            Instr::Fastore => todo!(),
            Instr::Fcmpg => todo!(),
            Instr::Fcmpl => todo!(),
            Instr::Fconst0 => todo!(),
            Instr::Fconst1 => todo!(),
            Instr::Fconst2 => todo!(),
            Instr::Fdiv => todo!(),
            Instr::Fload(_) => todo!(),
            Instr::Fload0 => todo!(),
            Instr::Fload1 => todo!(),
            Instr::Fload2 => todo!(),
            Instr::Fload3 => todo!(),
            Instr::Fmul => todo!(),
            Instr::Fneg => todo!(),
            Instr::Frem => todo!(),
            Instr::Freturn => todo!(),
            Instr::Fstore(_) => todo!(),
            Instr::Fstore0 => todo!(),
            Instr::Fstore1 => todo!(),
            Instr::Fstore2 => todo!(),
            Instr::Fstore3 => todo!(),
            Instr::Fsub => todo!(),
            Instr::Getfield(_) => todo!(),
            Instr::Getstatic(n) => {
                let Some(Constant::FieldRef { class_index, name_and_type_index }) = class.constant(n as usize) else {
                    panic!("Invalid constant")
                };
                let Some(Constant::Class { name_index }) = class.constant(*class_index as usize) else {
                    panic!("Invalid constant")
                };
                let Some(Constant::NameAndType { name_index, descriptor_index }) = class.constant(*name_and_type_index as usize) else {
                    panic!("Invalid constant")
                };
                let Some(Constant::Utf8(name)) = class.constant(*name_index as usize) else {
                    panic!("Invalid constant")
                };
                let Some(Constant::Utf8(descriptor)) = class.constant(*descriptor_index as usize) else {
                    panic!("Invalid constant")
                };

                let class = interp.class(name).unwrap();
                let field_index = class.static_field_index(name, descriptor.as_ref()).unwrap();
                let value = class.static_field_value(field_index).unwrap();

                frame.stack.push(value);
            },
            Instr::Goto(_) => todo!(),
            Instr::GotoW(_) => todo!(),
            Instr::I2b => todo!(),
            Instr::I2c => todo!(),
            Instr::I2d => todo!(),
            Instr::I2f => todo!(),
            Instr::I2l => todo!(),
            Instr::I2s => todo!(),
            Instr::Iadd => todo!(),
            Instr::Iaload => todo!(),
            Instr::Iand => todo!(),
            Instr::Iastore => todo!(),
            Instr::IconstM1 => todo!(),
            Instr::Iconst0 => todo!(),
            Instr::Iconst1 => todo!(),
            Instr::Iconst2 => todo!(),
            Instr::Iconst3 => todo!(),
            Instr::Iconst4 => todo!(),
            Instr::Iconst5 => todo!(),
            Instr::Idiv => todo!(),
            Instr::IfAcmpeq(_) => todo!(),
            Instr::IfAcmpne(_) => todo!(),
            Instr::IfIcmpeq(_) => todo!(),
            Instr::IfIcmpne(_) => todo!(),
            Instr::IfIcmplt(_) => todo!(),
            Instr::IfIcmpge(_) => todo!(),
            Instr::IfIcmpgt(_) => todo!(),
            Instr::IfIcmple(_) => todo!(),
            Instr::Ifeq(_) => todo!(),
            Instr::Ifne(_) => todo!(),
            Instr::Iflt(_) => todo!(),
            Instr::Ifge(_) => todo!(),
            Instr::Ifgt(_) => todo!(),
            Instr::Ifle(_) => todo!(),
            Instr::Ifnonnull(_) => todo!(),
            Instr::Ifnull(_) => todo!(),
            Instr::Iinc(_, _) => todo!(),
            Instr::Iload(_) => todo!(),
            Instr::Iload0 => todo!(),
            Instr::Iload1 => todo!(),
            Instr::Iload2 => todo!(),
            Instr::Iload3 => todo!(),
            Instr::Imul => todo!(),
            Instr::Ineg => todo!(),
            Instr::Instanceof(_) => todo!(),
            Instr::Invokedynamic(_) => todo!(),
            Instr::Invokeinterface(_, _) => todo!(),
            Instr::Invokespecial(_) => todo!(),
            Instr::Invokestatic(_) => todo!(),
            Instr::Invokevirtual(_) => todo!(),
            Instr::Ior => todo!(),
            Instr::Irem => todo!(),
            Instr::Ireturn => todo!(),
            Instr::Ishl => todo!(),
            Instr::Ishr => todo!(),
            Instr::Istore(_) => todo!(),
            Instr::Istore0 => todo!(),
            Instr::Istore1 => todo!(),
            Instr::Istore2 => todo!(),
            Instr::Istore3 => todo!(),
            Instr::Isub => todo!(),
            Instr::Iushr => todo!(),
            Instr::Ixor => todo!(),
            Instr::Jsr(_) => todo!(),
            Instr::JsrW(_) => todo!(),
            Instr::L2d => todo!(),
            Instr::L2f => todo!(),
            Instr::L2i => todo!(),
            Instr::Ladd => todo!(),
            Instr::Laload => todo!(),
            Instr::Land => todo!(),
            Instr::Lastore => todo!(),
            Instr::Lcmp => todo!(),
            Instr::Lconst0 => todo!(),
            Instr::Lconst1 => todo!(),
            Instr::Ldc(_) => todo!(),
            Instr::LdcW(_) => todo!(),
            Instr::Ldc2W(_) => todo!(),
            Instr::Ldiv => todo!(),
            Instr::Lload(_) => todo!(),
            Instr::Lload0 => todo!(),
            Instr::Lload1 => todo!(),
            Instr::Lload2 => todo!(),
            Instr::Lload3 => todo!(),
            Instr::Lmul => todo!(),
            Instr::Lneg => todo!(),
            Instr::Lookupswitch { .. } => todo!(),
            Instr::Lor => todo!(),
            Instr::Lrem => todo!(),
            Instr::Lreturn => todo!(),
            Instr::Lshl => todo!(),
            Instr::Lshr => todo!(),
            Instr::Lstore(_) => todo!(),
            Instr::Lstore0 => todo!(),
            Instr::Lstore1 => todo!(),
            Instr::Lstore2 => todo!(),
            Instr::Lstore3 => todo!(),
            Instr::Lsub => todo!(),
            Instr::Lushr => todo!(),
            Instr::Lxor => todo!(),
            Instr::Monitorenter => todo!(),
            Instr::Monitorexit => todo!(),
            Instr::Multianewarray(_, _) => todo!(),
            Instr::New(_) => todo!(),
            Instr::Newarray(_) => todo!(),
            Instr::Nop => todo!(),
            Instr::Pop => todo!(),
            Instr::Pop2 => todo!(),
            Instr::Putfield(_) => todo!(),
            Instr::Putstatic(_) => todo!(),
            Instr::Ret(_) => todo!(),
            Instr::Return => todo!(),
            Instr::Saload => todo!(),
            Instr::Sastore => todo!(),
            Instr::Sipush(_) => todo!(),
            Instr::Swap => todo!(),
            Instr::Tableswitch { .. } => todo!(),
            Instr::Wide { .. } => todo!(),
        }
        self.pc.instr += 1;

        Ok(())
    }
}

pub struct Pc {
    class: Arc<Class>,
    method_index: usize,
    instr: usize,
}

impl Pc {
    pub fn new(class: Arc<Class>, method_index: usize) -> Self {
        Self {
            class,
            method_index,
            instr: 0,
        }
    }

    pub fn next(&mut self) -> usize {
        self.instr += 1;
        self.instr
    }

    pub fn pc(&self) -> usize {
        self.instr
    }
}

pub struct Frame {
    return_pc: Option<Pc>,

    locals: Vec<Value>,
    stack: Vec<Value>,
}

impl Frame {
    pub fn for_method(class: Arc<Class>, method_index: usize) -> Self {
        let method = &class.methods()[method_index];
        if !method.has_code() {
            panic!("Method has no code");
        }
        let max_locals = method.max_locals().unwrap();
        let max_stack = method.max_stack().unwrap();

        Self {
            return_pc: None,
            locals: vec![Value::Int(0); max_locals],
            stack: Vec::with_capacity(max_stack),
        }
    }

    pub fn set_local(&mut self, index: u16, value: Value) {
        self.locals[index as usize] = value;
    }

    pub fn get_local(&self, index: u16) -> Value {
        self.locals[index as usize].clone()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn new(return_pc: Pc, call_class: Arc<Class>, call_method: usize) -> Self {
        let method = &call_class.methods()[call_method];
        if !method.has_code() {
            panic!("Method has no code");
        }
        Self {
            return_pc: Some(return_pc),
            locals: Vec::with_capacity(method.max_locals().unwrap()),
            stack: Vec::with_capacity(method.max_stack().unwrap()),
        }
    }
}