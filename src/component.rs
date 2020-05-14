// Wasm Component

extern crate gl;

use anyhow::Result;
use gl::types::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use wasmtime::*;

pub struct Component {
    pub instance: Option<Instance>,
    pub store: Store,
}
impl<'a> Component {
    pub fn init() -> Rc<RefCell<Component>> {
        Rc::new(RefCell::new(Component {
            store: Store::default(),
            instance: None,
        }))
    }

    pub fn initialize(component: &Rc<RefCell<Component>>, filename: &str, imports: &Imports) -> Result<Instance> {
        println!("Compiling wasm module...");
        let store = &component.borrow().store;
        let module = Module::from_file(store, filename)?;

        println!("Instantiating module...");
        Instance::new(&module, &imports.to_list(&module))
    }
}

// An import dictionary
pub struct Imports {
    modules: HashMap<String, ImportModule>,
}
impl Imports {
    pub fn new() -> Imports {
        Imports { modules: HashMap::new() }
    }

    pub fn from_vec(list: Vec<(&str, ImportModule)>) -> Imports {
        let mut modules = HashMap::new();
        for (name, module) in list {
            modules.insert(name.to_string(), module);
        }
        Imports { modules }
    }

    pub fn add_module(&mut self, name: &str, module: ImportModule) {
        self.modules.insert(name.to_string(), module);
    }

    fn to_list(&self, module: &Module) -> Vec<Extern> {
        let mut imports = Vec::new();
        for import in module.imports() {
            let cur = self.modules.get(import.module()).unwrap();
            let func = cur.funcs.get(import.name()).unwrap();
            imports.push(func.clone().into());
        }
        imports
    }
}

// A set of imports for one module in an import dictionary
pub struct ImportModule {
    funcs: HashMap<String, Func>,
}
impl ImportModule {
    pub fn from_vec(list: Vec<(&str, Func)>) -> ImportModule {
        let mut funcs = HashMap::new();
        for (name, func) in list {
            funcs.insert(name.to_string(), func);
        }
        ImportModule { funcs }
    }
}
