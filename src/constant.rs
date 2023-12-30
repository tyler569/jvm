use std::sync::Arc;
use bytes::{Buf, Bytes};
use crate::class::Class;

#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    Class { name_index: u16 },
    FieldRef { class_index: u16, name_and_type_index: u16 },
    MethodRef { class_index: u16, name_and_type_index: u16 },
    InterfaceMethodRef { class_index: u16, name_and_type_index: u16 },
    String { string_index: u16 },
    Integer(i32),
    Float(f32),
    Double(f64),
    Long(i64),
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8(Arc<str>),
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType { descriptor_index: u16 },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Module { name_index: u16 },
    Package { name_index: u16 },
}

impl Constant {
    pub fn from_bytes(bytes: &mut Bytes) -> Self {
        match bytes.get_u8() {
            7 => Self::Class { name_index: bytes.get_u16() },
            9 => Self::FieldRef {
                class_index: bytes.get_u16(),
                name_and_type_index: bytes.get_u16(),
            },
            10 => Self::MethodRef {
                class_index: bytes.get_u16(),
                name_and_type_index: bytes.get_u16(),
            },
            11 => Self::InterfaceMethodRef {
                class_index: bytes.get_u16(),
                name_and_type_index: bytes.get_u16(),
            },
            8 => Self::String { string_index: bytes.get_u16() },
            3 => Self::Integer(bytes.get_i32()),
            4 => Self::Float(bytes.get_f32()),
            5 => Self::Long(bytes.get_i64()),
            6 => Self::Double(bytes.get_f64()),
            12 => Self::NameAndType {
                name_index: bytes.get_u16(),
                descriptor_index: bytes.get_u16(),
            },
            1 => {
                let length = bytes.get_u16() as usize;
                let mut buf = vec![0; length];
                bytes.copy_to_slice(&mut buf);
                let string = String::from_utf8(buf).unwrap();
                Self::Utf8(Arc::from(string))
            }
            15 => Self::MethodHandle {
                reference_kind: bytes.get_u8(),
                reference_index: bytes.get_u16(),
            },
            16 => Self::MethodType { descriptor_index: bytes.get_u16(), },
            17 => Self::Dynamic {
                bootstrap_method_attr_index: bytes.get_u16(),
                name_and_type_index: bytes.get_u16(),
            },
            18 => Self::InvokeDynamic {
                bootstrap_method_attr_index: bytes.get_u16(),
                name_and_type_index: bytes.get_u16(),
            },
            19 => Self::Module { name_index: bytes.get_u16() },
            20 => Self::Package { name_index: bytes.get_u16() },
            _ => panic!("invalid constant tag"),
        }
    }
}