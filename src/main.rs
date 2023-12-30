#![allow(unused)]

use std::fs::{read};
use bytes::{Buf, Bytes};
use crate::class::Class;
use crate::interp::Interp;

mod value;
mod constant;
mod class;
mod interp;
mod code;
mod thread;
mod error;

fn main() -> anyhow::Result<()> {
    let file = read("Hello.class")?;
    let mut br = Bytes::from(file);

    let class = Class::parse(&mut br);
    println!("{:#?}", class);

    let mut interp = Interp::new();
    interp.load_class("Hello.class")?;
    interp.new_thread_main("Hello")?;
    interp.run()?;

    Ok(())
}
