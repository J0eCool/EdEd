//! Small example of how to instantiate a wasm module that imports one function,
//! showing how you can fill in host functionality for a wasm module.

// You can execute this example with `cargo run --example hello`

extern crate sdl2;
extern crate gl;

use anyhow::Result;
use gl::types::*;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    video::GLProfile,
};
use std::{
    collections::HashMap,
    ffi::CString,
    time::Duration,
};

use wasmtime::*;

fn main() -> Result<()> {
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
    let _ctx = window.gl_create_context().unwrap();
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

    let tex_w: i32 = 32;
    let tex_h: i32 = 32;
    let mut tex_id: GLuint = 0;
    let mut tex_data: Vec<u8> = vec![];
    for _ in 0..(tex_w * tex_h) {
        tex_data.push(0);
    }
    unsafe {
        gl::GenTextures(1, &mut tex_id);
    }
    let mut set_tex_pixel = |i: usize, val: u8| {
        tex_data[i] = val;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED as i32, tex_w, tex_h, 0, gl::RED as u32,
                gl::UNSIGNED_BYTE, tex_data.as_ptr() as *const GLvoid);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    };
    // Borrow checker won't allow for two mutable borrows; should go away with better abstraction?
    set_tex_pixel(0, 0);

    // Configure the initial compilation environment, creating the global
    // `Store` structure. Note that you can also tweak configuration settings
    // with a `Config` and an `Engine` if desired.
    println!("Compiling wasm module...");
    let store = Store::default();
    // Compile the wasm binary into an in-memory instance of a `Module`.
    let module = Module::from_file(&store, "modules/out/game.wasm")?;

    // Here we handle the imports of the module, which in this case is our
    // `HelloCallback` type and its associated implementation of `Callback.
    println!("Creating callback...");

    // Dictionary of imports for "render" module
    let mut render_imports: HashMap<String, Func> = HashMap::new();
    render_imports.insert("draw".to_string(), Func::wrap(&store, move |_x: i32| {
        unsafe {
            gl::BindVertexArray(vao);
            // gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // // Set color
            // // gl::EnableVertexAttribArray(1);
            // gl::VertexAttrib3f(1, x as f32 / 400.0, 0.2, 0.2);

            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }));
    render_imports.insert("sin".to_string(), Func::wrap(&store, f32::sin));

    let mut imports: Vec<Extern> = Vec::new();
    for import in module.imports() {
        assert_eq!(import.module(), "render");
        let func = render_imports.get(import.name()).unwrap();
        imports.push(func.clone().into());
    }


    // Once we've got that all set up we can then move to the instantiation
    // phase, pairing together a compiled module as well as a set of imports.
    // Note that this is where the wasm `start` function, if any, would run.
    println!("Instantiating module...");
    // let imports = [
    //     wasm_sin.into(),
    //     draw.into(),
    // ];
    let instance = Instance::new(&module, &imports)?;

    // Next we poke around a bit to extract the `frame` function from the module.
    println!("Extracting export...");
    let frame = instance
        .get_func("frame")
        .ok_or(anyhow::format_err!("failed to find `frame` function export"))?
        .get0::<()>()?;

    println!("Loading shaders");
    let vert_shader = Shader::from_source_vert(include_str!("../resources/shaders/textured.vert")).unwrap();
    let frag_shader = Shader::from_source_frag(include_str!("../resources/shaders/textured.frag")).unwrap();
    let shader_program = ShaderProgram::from_shaders(&[vert_shader, frag_shader]).unwrap();

    println!("Starting main loop");
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut is_mouse_down = false;
    'mainloop: loop {
        unsafe {
            gl::Viewport(0, 0, 800, 600);
            gl::ClearColor(0.8, 0.8, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                Event::MouseMotion { x, y, .. } => {
                    if is_mouse_down {
                        // Note: gross
                        let i = ((x - 200) as f32 / 400.0 * tex_w as f32) as usize;
                        let j = (1.0 - ((y - 450) as f32 / 300.0) * tex_h as f32) as usize;
                        if i < tex_w as usize && j < tex_h as usize {
                            set_tex_pixel(i + j * tex_w as usize, 0xff);
                        }
                    }
                },
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left { is_mouse_down = true; }
                },
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left { is_mouse_down = false; }
                },
                _ => {}
            }
        }

        shader_program.set_used();
        unsafe {
            // TODO: I have no idea why this needs println! to function, ignoring for now
            // let loc = gl::GetUniformLocation(shader_program.id, CString::new("Texture").unwrap().as_ptr());
            // println!("Location: {}", loc);
            let loc = -1;
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::Uniform1i(loc, tex_id as i32);
        }
        frame()?;

        // canvas.present();
        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Done.");
    Ok(())
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
// Program class
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