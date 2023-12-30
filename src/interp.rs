use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::task::Wake;
use bytes::Bytes;
use crate::class::Class;
use crate::error::{Error, Result};
use crate::thread::Thread;
use crate::value::Value;

pub struct InterpContext {
    pub classes: HashMap<Arc<str>, Arc<Class>>,
}

impl InterpContext {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
        }
    }

    pub fn class(&self, name: &str) -> Option<Arc<Class>> {
        self.classes.get(name).cloned()
    }
}

pub struct Interp {
    pub context: InterpContext,
    pub threads: Vec<Thread>,
}

impl Interp {
    pub fn new() -> Self {
        Self {
            context: InterpContext::new(),
            threads: Vec::new(),
        }
    }

    pub fn load_class<P: AsRef<Path>>(&mut self, file: P) -> Result<()> {
        let bytes = std::fs::read(file)?;
        let mut bytes = Bytes::from(bytes);
        let class = match Class::parse(&mut bytes) {
            Ok(class) => class,
            Err(_) => return Err(Error::ClassNotFound),
        };

        let class = Arc::new(class);
        let name = class.name();

        self.context.classes.insert(name.into(), class);

        Ok(())
    }

    pub fn new_thread_runnable<C: AsRef<str>>(&mut self, class_name: C) -> Result<()> {
        let class = self.context.class(class_name.as_ref()).ok_or(Error::ClassNotFound)?;
        let method_index = class.method_index("run", "()V").ok_or(Error::ClassNotRunnable)?;
        let thread = Thread::new(class, method_index);
        self.threads.push(thread);
        Ok(())
    }

    pub fn new_thread_main<C: AsRef<str>>(&mut self, class_name: C) -> Result<()> {
        let class = self.context.class(class_name.as_ref()).ok_or(Error::ClassNotFound)?;
        let method_index = class
            .static_method_index("main", "([Ljava/lang/String;)V")
            .ok_or(Error::ClassNotMain)?;
        let thread = Thread::new(class, method_index);
        self.threads.push(thread);
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            for thread in self.threads.iter_mut() {
                thread.exec_one(&mut self.context)?;
            }
        }
    }
}
