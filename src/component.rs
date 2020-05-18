// Wasm Component

use anyhow::{Result, anyhow, format_err};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use wasmtime::*;

pub struct Component {
    filename: String,
    pub instance: Option<Instance>,
    pub store: Store,
}
impl<'a> Component {
    pub fn init(store: &Store) -> Rc<RefCell<Component>> {
        Rc::new(RefCell::new(Component {
            filename: String::new(),
            store: store.clone(),
            instance: None,
        }))
    }

    pub fn initialize(component: &Rc<RefCell<Component>>, filename: &str, mut imports: Imports) -> Result<Instance> {
        println!("Compiling module: {}", filename);
        // Store filename for later
        { component.borrow_mut().filename = filename.to_string(); }
        let store = &component.borrow().store;
        let module = Module::from_file(store, filename)?;

        // Hack simple wasi syscalls in until we can polyfill around them via composite components
        imports.add_module("wasi_snapshot_preview1", ImportModule::from_vec(vec![
            ("proc_exit", Func::wrap(&store, |code: i32| { panic!("wasi proc_exit called w/ code: {}", code); })),
        ]));

        println!("Instantiating module...");
        Instance::new(&module, &imports.to_extern_list(&module)?)
    }

    pub fn get_func(&self, name: &str) -> Result<Func> {
        let instance = self.instance.as_ref().ok_or(anyhow!("Instance not set"))?;
        let f = instance.get_func(name).ok_or(format_err!("Failed to find function: {} in component {}", name, self.filename))?;
        Ok(f)
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

    pub fn add_module(&mut self, name: &str, module: ImportModule) {
        self.modules.insert(name.to_string(), module);
    }

    fn to_extern_list(&self, module: &Module) -> Result<Vec<Extern>> {
        let mut imports = Vec::new();
        for import in module.imports() {
            let mod_name = import.module();
            let cur = self.modules.get(import.module())
                .ok_or(format_err!("No module found with name: {}", mod_name))?;
            let func_name = import.name();
            let func = cur.funcs.get(import.name())
                .ok_or(format_err!("Import not found: {}/{}", mod_name, func_name))?;
            imports.push(func.clone().into());
        }
        Ok(imports)
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
