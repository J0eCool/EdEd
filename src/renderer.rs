extern crate sdl2;
extern crate gl;

use anyhow::Result;
use gl::types::*;
use sdl2::{
    video::{GLContext, GLProfile, Window},
};
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::CString,
    rc::Rc,
};

use wasmtime::*;

use crate::component::{Component, ImportModule};

pub struct Renderer {
    pub sdl_context: sdl2::Sdl,
    shader_program: ShaderProgram,
    vao: GLuint,
    window: Window,

    // Need to capture this so that it doesn't get Drop'd
    #[allow(dead_code)]
    gl_context: GLContext,
}

impl Renderer {
    pub fn new() -> Renderer {
        println!("Initializing SDL...");
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);
     
        let window = video_subsystem.window("EdEditor", 800, 600)
            .position_centered()
            .opengl()
            .resizable()
            .build().unwrap();

        println!("Initializing GL...");
        let gl_context = window.gl_create_context().unwrap();
        let _gl = gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (3, 3));

        // Square model, 1x1, centered at 0
        let vertices: Vec<f32> = vec![
            // Position           UV
            -0.5, -0.5, 0.0,      0.0, 0.0,
            0.5, -0.5, 0.0,       1.0, 0.0,
            0.5, 0.5, 0.0,        1.0, 1.0,

            0.5, 0.5, 0.0,        1.0, 1.0,
            -0.5, 0.5, 0.0,       0.0, 1.0,
            -0.5, -0.5, 0.0,      0.0, 0.0,
        ];
        let mut vbo: GLuint = 0; // VBO to store vertex data
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
        }
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Bind Position
            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                0, // index of the generic vertex attribute ("layout (location = 0)")
                3, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (5 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null() // offset of the first component
            );

            // Bind UV Coords
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1, // index of the generic vertex attribute ("layout (location = 0)")
                2, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (5 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                (3 * std::mem::size_of::<f32>()) as *const GLvoid // offset of the first component
            );

            // unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        println!("Loading shaders");
        let vert_shader = Shader::from_source_vert(include_str!("../resources/shaders/textured.vert")).unwrap();
        let frag_shader = Shader::from_source_frag(include_str!("../resources/shaders/textured.frag")).unwrap();
        let shader_program = ShaderProgram::from_shaders(&[vert_shader, frag_shader]).unwrap();

        Renderer {
            sdl_context,
            shader_program,
            gl_context,
            vao,
            // vbo,
            window,
        }
    }

    pub fn pre_update(&self) {
        self.shader_program.set_used();
        unsafe { gl::BindVertexArray(self.vao); }
    }
    pub fn post_update(&self) {
        self.window.gl_swap_window();
    }

    pub fn import_module(component: &Rc<RefCell<Component>>) -> ImportModule {
        let store = &component.borrow().store;
        let component_rc = component.clone();
        ImportModule::from_vec(vec![
            ("drawImage", Func::wrap(&store, |tex_id: i32| {
                unsafe {
                    // TODO: I have no idea why this needs println! to function, ignoring for now
                    // let loc = gl::GetUniformLocation(shader_program.id, CString::new("Texture").unwrap().as_ptr());
                    // println!("Location: {}", loc);
                    let loc = -1;
                    gl::BindTexture(gl::TEXTURE_2D, tex_id as u32);
                    gl::Uniform1i(loc, tex_id);

                    gl::DrawArrays(gl::TRIANGLES, 0, 6);
                }
            })),
            ("allocImage", Func::wrap(&store, || {
                let mut tex_id: GLuint = 0;
                unsafe {
                    gl::GenTextures(1, &mut tex_id);
                }
                tex_id as i32
            })),
            ("updateImage", Func::wrap(&store, move |tex_id: i32, image_ptr: i32, image_size: i32| {
                // TODO: pass these in
                let tex_w: i32 = 16;
                let tex_h: i32 = 16;
                let mut tex_data: Vec<u8> = vec![];
                let component_ref = component_rc.borrow();
                let memory = component_ref.instance.as_ref().unwrap().get_memory("memory").unwrap();
                for i in 0..image_size {
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
            })),
        ])
    }
}

// -------------------------
// Shader class
struct Shader {
    id: GLuint,
}
impl Shader {
    pub fn from_source(source: &str, kind: GLuint) -> Result<Shader, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &CString::new(source).unwrap().as_ptr(), std::ptr::null());
            gl::CompileShader(id);
            let mut success: GLint = 1;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len: GLint = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buffer.push(b' ');
                }
                let error: CString = CString::from_vec_unchecked(buffer);
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);

                println!("Oh no! Shader \"{}\" failed with message: {}", "[FILENAME]", error.to_string_lossy());
            }
        }
        Ok(Shader { id })
    }
    pub fn from_source_vert(source: &str) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }
    pub fn from_source_frag(source: &str) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
    fn id(&self) -> GLuint {
        self.id
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

// -------------------------
// Shader Programs
struct ShaderProgram {
    id: GLuint,
}
impl ShaderProgram {
    pub fn from_shaders(shaders: &[Shader]) -> Result<ShaderProgram, String> {
        let program_id = unsafe { gl::CreateProgram() };

        unsafe {
            for shader in shaders {
                gl::AttachShader(program_id, shader.id());
            }

            gl::LinkProgram(program_id);

            // continue with error handling here

            for shader in shaders {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(ShaderProgram { id: program_id })
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
