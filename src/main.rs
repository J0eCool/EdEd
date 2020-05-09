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
    video::GLProfile,
};
use std::{
    ffi::{CString, CStr},
    time::Duration,
};

use wasmtime::*;

fn main() -> Result<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .opengl()
        .build().unwrap();

    let ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    // let mut canvas = window.into_canvas()
    //     .index(find_sdl_gl_driver().unwrap())
    //     .build().unwrap();

    // Configure the initial compilation environment, creating the global
    // `Store` structure. Note that you can also tweak configuration settings
    // with a `Config` and an `Engine` if desired.
    println!("Initializing...");
    let store = Store::default();

    // Compile the wasm binary into an in-memory instance of a `Module`.
    println!("Compiling module...");
    let module = Module::from_file(&store, "modules/out/game.wasm")?;

    // Here we handle the imports of the module, which in this case is our
    // `HelloCallback` type and its associated implementation of `Callback.
    println!("Creating callback...");

    let draw = Func::wrap(&store, |x: i32| {
        // canvas.fill_rect(Rect::new(x, 100, 50, 50));
    });
    let wasm_sin = Func::wrap(&store, |x: f32| -> f32 {
        f32::sin(x)
    });

    // Once we've got that all set up we can then move to the instantiation
    // phase, pairing together a compiled module as well as a set of imports.
    // Note that this is where the wasm `start` function, if any, would run.
    println!("Instantiating module...");
    let imports = [
        wasm_sin.into(),
        draw.into(),
    ];
    let instance = Instance::new(&module, &imports)?;

    // Next we poke around a bit to extract the `frame` function from the module.
    println!("Extracting export...");
    let frame = instance
        .get_func("frame")
        .ok_or(anyhow::format_err!("failed to find `frame` function export"))?
        .get0::<()>()?;

    // canvas.set_draw_color(Color::RGB(192, 192, 192));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    println!("Starting main loop");
    'mainloop: loop {
        unsafe {
            gl::Viewport(0, 0, 900, 700);
            gl::ClearColor(0.8, 0.8, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                _ => {}
            }
        }
        
        frame()?;

        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Done.");
    Ok(())
}

fn shader_from_source(source: &CStr, kind: GLuint) -> Result<GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        let mut success: GLint = 1;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            println!("Oh no! Shader failed with message: {}", "[TODO]");
        }
    }

    Ok(id)
}
