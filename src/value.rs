use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::hint::black_box;
use std::sync::{Arc, Mutex};
use bitflags::bitflags;
use crate::class::{Class, FieldIndex};

#[derive(Clone, Debug)]
pub enum Type {
    Boolean,
    Char,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Reference(Arc<Class>),
}

#[derive(Clone, Debug)]
pub struct Object {
    pub class: Arc<Class>,
    pub fields: Vec<RefCell<Value>>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Boolean(bool),
    Char(i16),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Reference(Arc<Object>),
}

impl Object {
    pub fn get_field(&self, index: FieldIndex) -> Value {
        match index {
            FieldIndex::Dynamic(index) => self.fields[index].borrow().clone(),
            FieldIndex::Static(index) => self.class.static_values[index].borrow().clone(),
        }
    }

    pub fn set_field(&self, index: FieldIndex, value: Value) {
        match index {
            FieldIndex::Dynamic(index) => self.fields[index].replace(value),
            FieldIndex::Static(index) => self.class.static_values[index].replace(value),
        };
    }

    pub fn get_field_name(&self, name: &str, descriptor: &str) -> Option<Value> {
        let field = self.class.field_index(name, descriptor)?;
        Some(self.get_field(field))
    }

    pub fn set_field_name(&self, name: &str, descriptor: &str, value: Value) -> Option<()> {
        let field = self.class.field_index(name, descriptor)?;
        self.set_field(field, value);
        Some(())
    }
}