// Wasm Component

use anyhow::Result;
use std::{
    cell::RefCell,
    collections::HashMap,
    error,
    fmt,
    rc::Rc,
};

use wasmtime::*;

pub struct Component {
    pub instance: Option<Instance>,
    pub store: Store,
}
impl<'a> Component {
    pub fn init(store: &Store) -> Rc<RefCell<Component>> {
        Rc::new(RefCell::new(Component {
            store: store.clone(),
            instance: None,
        }))
    }

    pub fn initialize(component: &Rc<RefCell<Component>>, filename: &str, imports: &Imports) -> Result<Instance> {
        println!("Compiling wasm module...");
        let store = &component.borrow().store;
        let module = Module::from_file(store, filename)?;

        println!("Instantiating module...");
        Instance::new(&module, &imports.to_list(&module)?)
    }
}

fn unwrap_err<T>(opt: Option<T>, msg: &str) -> Result<T> {
    opt.map_or_else(|| Err(anyhow::anyhow!(msg.to_string())), |x| Ok(x))
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

    fn to_list(&self, module: &Module) -> Result<Vec<Extern>> {
        let mut imports = Vec::new();
        for import in module.imports() {
            let cur = unwrap_err(self.modules.get(import.module()), "beep")?;
            let func = unwrap_err(cur.funcs.get(import.name()), "Doop")?;
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

//-----------
// Nicer error handling
#[derive(Debug, Clone)]
struct ComponentError {
    msg: String,
}
impl fmt::Display for ComponentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl error::Error for ComponentError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
