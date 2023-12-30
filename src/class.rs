use std::cell::RefCell;
use std::sync::{Arc, Condvar, Mutex};
use std::task::Context;
use bitflags::{bitflags, Flags};
use bytes::{Buf, Bytes};
use crate::code::Code;
use crate::constant::Constant;
use crate::error::{Error, Result};
use crate::interp::{Interp, InterpContext};
use crate::value::{Object, Type, Value};

#[derive(Debug)]
pub struct Class {
    pub name: Arc<str>,
    pub superclass_name: Arc<str>,

    pub constant_pool: Vec<Constant>,

    pub static_fields: Vec<Field>,
    pub static_values: Vec<RefCell<Value>>,
    pub static_methods: Vec<Method>,

    pub fields: Vec<Field>,
    pub methods: Vec<Method>,

    pub interfaces: Vec<u16>,
    pub attributes: Vec<Attribute>,

    pub monitor: (Mutex<()>, Condvar),
}

#[derive(Copy, Clone, Debug)]
pub enum MethodIndex {
    Dynamic(usize),
    Static(usize),
}

#[derive(Copy, Clone, Debug)]
pub enum FieldIndex {
    Dynamic(usize),
    Static(usize),
}

impl Class {
    pub fn parse(bytes: &mut Bytes) -> Result<Self> {
        let magic: u32 = bytes.get_u32();
        if magic != 0xCAFEBABE {
            return Err(Error::InvalidClass);
        }

        let minor: u16 = bytes.get_u16();
        let major: u16 = bytes.get_u16();

        if major != 64 || minor != 0 {
            return Err(Error::InvalidClass);
        }

        let constant_pool = parse_constants(bytes);

        let access_flags: u16 = bytes.get_u16();
        let this_class: u16 = bytes.get_u16();
        let super_class: u16 = bytes.get_u16();

        let interfaces = parse_interfaces(bytes);
        let all_fields = parse_fields(bytes, &constant_pool);
        let all_methods = parse_methods(bytes, &constant_pool);
        let attributes = parse_attributes(bytes, &constant_pool);

        let static_fields = all_fields.iter()
            .filter(|field| field.access.contains(FieldAccessFlags::Static))
            .cloned()
            .collect::<Vec<_>>();
        let static_values = vec![RefCell::new(Value::Int(0)); static_fields.len()];
        let static_methods = all_methods.iter()
            .filter(|method| method.access.contains(MethodAccessFlags::Static))
            .cloned()
            .collect::<Vec<_>>();

        let fields = all_fields.iter()
            .filter(|field| !field.access.contains(FieldAccessFlags::Static))
            .cloned()
            .collect::<Vec<_>>();
        let methods = all_methods.iter()
            .filter(|method| !method.access.contains(MethodAccessFlags::Static))
            .cloned()
            .collect::<Vec<_>>();

        let name = get_class_name(&constant_pool, this_class);
        let superclass_name = get_class_name(&constant_pool, super_class);

        Ok(Self {
            name,
            superclass_name,
            constant_pool,
            interfaces,
            static_methods,
            static_fields,
            static_values,
            fields,
            methods,
            attributes,
            monitor: (Mutex::new(()), Condvar::new()),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn superclass(&self, context: &InterpContext) -> Arc<Class> {
        context.class(&self.superclass_name).unwrap()
    }

    pub fn method(&self, index: MethodIndex) -> &Method {
        match index {
            MethodIndex::Dynamic(index) => &self.methods[index],
            MethodIndex::Static(index) => &self.static_methods[index],
        }
    }

    pub fn wait(&self) {
        self.monitor.1.wait(self.monitor.0.lock().unwrap()).unwrap();
    }

    pub fn notify(&self) {
        self.monitor.1.notify_one();
    }

    pub fn method_index(&self, name: &str, descriptor: &str) -> Option<MethodIndex> {
        self.methods.iter().position(|method|
            method.name.as_ref() == name && method.descriptor.as_ref() == descriptor
        ).map(MethodIndex::Dynamic)
    }

    pub fn static_method_index(&self, name: &str, descriptor: &str) -> Option<MethodIndex> {
        self.static_methods.iter().position(|method|
            method.name.as_ref() == name && method.descriptor.as_ref() == descriptor
        ).map(MethodIndex::Static)
    }

    pub fn field_index(&self, name: &str, descriptor: &str) -> Option<FieldIndex> {
        self.fields.iter().position(|field|
            field.name.as_ref() == name && field.typ.as_ref() == descriptor
        ).map(FieldIndex::Dynamic)
    }

    pub fn static_field_index(&self, name: &str, descriptor: &str) -> Option<FieldIndex> {
        self.static_fields.iter().position(|field|
            field.name.as_ref() == name && field.typ.as_ref() == descriptor
        ).map(FieldIndex::Static)
    }

    pub fn static_field_value(&self, index: FieldIndex) -> Option<Value> {
        match index {
            FieldIndex::Static(index) =>
                self.static_values.get(index).map(|p| p.borrow().clone()),
            _ => panic!(),
        }
    }

    pub fn static_field_value_name(&self, name: &str, descriptor: &str) -> Option<Value> {
        let index = self.static_field_index(name, descriptor)?;
        self.static_field_value(index)
    }

    pub fn set_static_field_value(&mut self, index: FieldIndex, value: Value) {
        match index {
            FieldIndex::Static(index) => self.static_values[index].replace(value),
            _ => panic!(),
        };
    }

    pub fn set_static_field_value_name(&mut self, name: &str, descriptor: &str, value: Value) {
        let index = self.static_field_index(name, descriptor).unwrap();
        self.set_static_field_value(index, value);
    }

    pub fn constant(&self, n: usize) -> Option<&Constant> {
        if n == 0 {
            return None
        }
        self.constant_pool.get(n - 1)
    }

    pub fn get_static_from_constant(&self, context: &InterpContext, constant_index: usize) -> Result<Value> {
        let Some(Constant::FieldRef { class_index, name_and_type_index }) = self.constant(constant_index) else {
            return Err(Error::InvalidClass)
        };
        let Some(Constant::Class { name_index }) = self.constant(*class_index as usize) else {
            return Err(Error::InvalidClass)
        };
        let Some(Constant::NameAndType { name_index, descriptor_index }) = self.constant(*name_and_type_index as usize) else {
            return Err(Error::InvalidClass)
        };
        let Some(Constant::Utf8(name)) = self.constant(*name_index as usize) else {
            return Err(Error::InvalidClass)
        };
        let Some(Constant::Utf8(descriptor)) = self.constant(*descriptor_index as usize) else {
            return Err(Error::InvalidClass)
        };

        let Some(class) = context.class(name) else {
            eprintln!("Class not found: name: {name}, descriptor: {descriptor}");
            return Err(Error::ClassNotFound);
        };
        let field_index = class.static_field_index(name, descriptor.as_ref()).unwrap();
        Ok(class.static_field_value(field_index).unwrap())
    }
}

fn get_class_name(constant_pool: &[Constant], class_index: u16) -> Arc<str> {
    let class_info = &constant_pool[class_index as usize - 1];
    if let Constant::Class { name_index } = class_info {
        let utf8_info = &constant_pool[*name_index as usize - 1];
        if let Constant::Utf8(name) = utf8_info {
            name.clone()
        } else {
            panic!("Invalid class name");
        }
    } else {
        panic!("Invalid class info");
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct FieldAccessFlags: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Volatile = 0x0040;
        const Transient = 0x0080;
        const Synthetic = 0x1000;
        const Enum = 0x4000;
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct MethodAccessFlags: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Synchronized = 0x0020;
        const Bridge = 0x0040;
        const Varargs = 0x0080;
        const Native = 0x0100;
        const Abstract = 0x0400;
        const Struct = 0x0800;
        const Synthetic = 0x1000;
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: Arc<str>,
    pub typ: Arc<str>,
    pub access: FieldAccessFlags,
    pub attributes: Vec<Attribute>,
}

impl Field {
    pub fn from_bytes(bytes: &mut Bytes, constant_pool: &[Constant]) -> Self {
        let access_flags: u16 = bytes.get_u16();
        let name_index: u16 = bytes.get_u16();
        let descriptor_index: u16 = bytes.get_u16();

        let name = match &constant_pool[name_index as usize - 1] {
            Constant::Utf8(name) => name.clone(),
            _ => panic!("Invalid field name"),
        };
        let typ = match &constant_pool[descriptor_index as usize - 1] {
            Constant::Utf8(typ) => name.clone(),
            _ => panic!("Invalid field type"),
        };

        let attributes = parse_attributes(bytes, &constant_pool);

        Self {
            name,
            typ,
            access: FieldAccessFlags::from_bits_retain(access_flags),
            attributes,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Method {
    pub name: Arc<str>,
    pub descriptor: Arc<str>,
    pub access: MethodAccessFlags,
    pub attributes: Vec<Attribute>,
    pub code: Option<Code>,
}

impl Method {
    pub fn from_bytes(bytes: &mut Bytes, constant_pool: &[Constant]) -> Self {
        let access_flags: u16 = bytes.get_u16();
        let name_index: u16 = bytes.get_u16();
        let descriptor_index: u16 = bytes.get_u16();

        let name = match &constant_pool[name_index as usize - 1] {
            Constant::Utf8(name) => name.clone(),
            _ => panic!("Invalid method name"),
        };
        let descriptor = match &constant_pool[descriptor_index as usize - 1] {
            Constant::Utf8(descriptor) => descriptor.clone(),
            _ => panic!("Invalid method descriptor"),
        };

        let attributes = parse_attributes(bytes, &constant_pool);

        let code = attributes.iter()
            .find(|attr| attr.name.as_ref() == "Code")
            .map(|attr| Code::from_bytes(
                &mut Bytes::from(attr.info.clone()), constant_pool)
            );

        Self {
            name,
            descriptor,
            access: MethodAccessFlags::from_bits_retain(access_flags),
            attributes,
            code,
        }
    }

    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }

    pub fn max_locals(&self) -> Option<usize> {
        self.code.as_ref().map(|c| c.max_locals as usize)
    }

    pub fn max_stack(&self) -> Option<usize> {
        self.code.as_ref().map(|c| c.max_stack as usize)
    }

    pub fn code(&self) -> Option<&Code> {
        self.code.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: Arc<str>,
    pub info: Vec<u8>,
}

impl Attribute {
    pub fn from_bytes(bytes: &mut Bytes, constant_pool: &[Constant]) -> Self {
        let name_index: u16 = bytes.get_u16();
        let length: u32 = bytes.get_u32();
        let info = bytes.copy_to_bytes(length as usize);

        let name = match &constant_pool[name_index as usize - 1] {
            Constant::Utf8(name) => name.clone(),
            _ => panic!("Invalid attribute name"),
        };

        Self {
            name,
            info: info.to_vec(),
        }
    }

    pub fn parse_list(bytes: &mut Bytes, constant_pool: &[Constant]) -> Vec<Self> {
        parse_attributes(bytes, constant_pool)
    }
}

fn parse_constants(bytes: &mut Bytes) -> Vec<Constant> {
    let constants_count: u16 = bytes.get_u16();
    (0..constants_count - 1)
        .map(|_| Constant::from_bytes(bytes))
        .collect::<Vec<_>>()
}

fn parse_interfaces(bytes: &mut Bytes) -> Vec<u16> {
    let interfaces_count: u16 = bytes.get_u16();
    (0..interfaces_count)
        .map(|_| bytes.get_u16())
        .collect::<Vec<_>>()
}

fn parse_fields(bytes: &mut Bytes, constant_pool: &[Constant]) -> Vec<Field> {
    let fields_count: u16 = bytes.get_u16();
    (0..fields_count)
        .map(|_| Field::from_bytes(bytes, constant_pool))
        .collect::<Vec<_>>()
}

fn parse_methods(bytes: &mut Bytes, constant_pool: &[Constant]) -> Vec<Method> {
    let methods_count: u16 = bytes.get_u16();
    (0..methods_count)
        .map(|_| Method::from_bytes(bytes, constant_pool))
        .collect::<Vec<_>>()
}

fn parse_attributes(bytes: &mut Bytes, constant_pool: &[Constant]) -> Vec<Attribute> {
    let attributes_count: u16 = bytes.get_u16();
    (0..attributes_count)
        .map(|_| Attribute::from_bytes(bytes, constant_pool))
        .collect::<Vec<_>>()
}


