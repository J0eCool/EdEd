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
}
impl<'a> Component {
    pub fn from_file(filename: &str) -> Result<Rc<RefCell<Component>>> {
        let ret = Rc::new(RefCell::new(Component { instance: None }));
        ret.borrow_mut().instance = Some(Component::initialize(&ret, filename)?);
        Ok(ret)
    }

    fn initialize(component: &Rc<RefCell<Component>>, filename: &str) -> Result<Instance> {
        println!("Compiling wasm module...");
        let store = Store::default();
        // Compile the wasm binary into an in-memory instance of a `Module`.
        let module = Module::from_file(&store, filename)?;

        // Dictionary of imports for "render" module
        // let ptr = Rc::new(self);
        // let render_imports = Component::imports_for_render(ptr.clone(), &store);
        // let render_imports = Component::imports_for_render(component, &store);

        let mut render_imports = HashMap::new();
        render_imports.insert("drawImage".to_string(), Func::wrap(&store, |tex_id: i32| {
            unsafe {
                // TODO: I have no idea why this needs println! to function, ignoring for now
                // let loc = gl::GetUniformLocation(shader_program.id, CString::new("Texture").unwrap().as_ptr());
                // println!("Location: {}", loc);
                let loc = -1;
                gl::BindTexture(gl::TEXTURE_2D, tex_id as u32);
                gl::Uniform1i(loc, tex_id);

                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }
        }));
        render_imports.insert("allocImage".to_string(), Func::wrap(&store, || {
            let mut tex_id: GLuint = 0;
            unsafe {
                gl::GenTextures(1, &mut tex_id);
            }
            tex_id as i32
        }));
        let feee = component.clone();
        render_imports.insert("updateImage".to_string(), Func::wrap(&store, move |tex_id: i32, image_ptr: i32, image_size: i32| {
            // TODO: pass these in
            let tex_w: i32 = 16;
            let tex_h: i32 = 16;
            let mut tex_data: Vec<u8> = vec![];
            // let memory = unsafe { rcself.clone().instance.as_ref().unwrap().get_memory("memory").unwrap() };
            let component_ref = feee.borrow();
            let memory = component_ref.instance.as_ref().unwrap().get_memory("memory").unwrap();
            for i in 0..image_size {
                // tex_data.push(i as u8);
                unsafe { tex_data.push(memory.data_unchecked()[(image_ptr + i) as usize]); }
            }
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, tex_id as u32);
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED as i32, tex_w, tex_h, 0, gl::RED as u32,
                    gl::UNSIGNED_BYTE, tex_data.as_ptr() as *const GLvoid);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                // unbind
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }));

        let mut imports: Vec<Extern> = Vec::new();
        for import in module.imports() {
            assert_eq!(import.module(), "render");
            let func = render_imports.get(import.name()).unwrap();
            imports.push(func.clone().into());
        }

        println!("Instantiating module...");
        Instance::new(&module, &imports)
    }
}
