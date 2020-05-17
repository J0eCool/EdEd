// EdEditor program entry point + main loop

extern crate sdl2;
extern crate gl;

use anyhow::Result;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
};
use std::{
    time::Duration,
};

use wasmtime::*;

mod component;
mod renderer;
use component::{Component, Imports, ImportModule};
use renderer::Renderer;

fn main() -> Result<()> {
    // pixel_editor()
    notes_app()
}

fn notes_app() -> Result<()> {
    let render = Renderer::new();
    let store = Store::default();

    let input_rc = Component::init(&store);
    input_rc.borrow_mut().instance = Some(Component::initialize(&input_rc, "modules/out/input.wasm", Imports::new())?);
    let input_ref = input_rc.borrow();
    let input_instance = input_ref.instance.as_ref().unwrap();

    let notes_rc = Component::init(&store);
    let mut notes_imports = Imports::new();
    notes_imports.add_module("render", Renderer::import_module(&notes_rc));
    notes_imports.add_module("input", ImportModule::from_vec(vec![
        ("keyWentDown", input_instance.get_func("keyWentDown").unwrap()),
    ]));
    notes_rc.borrow_mut().instance = Some(Component::initialize(&notes_rc, "modules/out/notes.wasm", notes_imports)?);
    let notes_ref = notes_rc.borrow();
    let notes_instance = notes_ref.instance.as_ref().unwrap();

    println!("Extracting exports...");
    let notes_update = notes_instance
        .get_func("update")
        .ok_or(anyhow::format_err!("failed to find `update` function export"))?
        .get0::<()>()?;

    let input_update = input_instance
        .get_func("update")
        .ok_or(anyhow::format_err!("failed to find `update` function export"))?
        .get0::<()>()?;
    let mouse_event = input_instance
        .get_func("onMouseEvent")
        .ok_or(anyhow::format_err!("failed to find `onMouseEvent` function export"))?
        .get3::<i32, i32, i32, ()>()?;

    println!("Starting main loop");
    let mut event_pump = render.sdl_context.event_pump().unwrap();
    let canvas_x = 200;
    let canvas_y = 150;
    let to_canvas_space = |x: i32, y: i32| -> (i32, i32) {
        (x - canvas_x, 600 - y - canvas_y)
    };
    'mainloop: loop {
        unsafe {
            gl::Viewport(0, 0, 800, 600);
            gl::ClearColor(0.8, 0.8, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        input_update()?; // TODO: figure out generic timing on this
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                Event::KeyDown { keycode: Some(code), .. } => {
                    println!("Key pressed: {}", code);
                },
                Event::MouseMotion { x, y, .. } => {
                    let (x, y) = to_canvas_space(x, y);
                    mouse_event(0, x, y)?;
                },
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        let (x, y) = to_canvas_space(x, y);
                        mouse_event(1, x, y)?;
                    }
                },
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        let (x, y) = to_canvas_space(x, y);
                        mouse_event(2, x, y)?;
                    }
                },
                _ => {}
            }
        }

        render.pre_update();
        notes_update()?;
        render.post_update();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Done.");
    Ok(())
}

fn _pixel_editor() -> Result<()> {
    let render = Renderer::new();
    let store = Store::default();

    let input_rc = Component::init(&store);
    input_rc.borrow_mut().instance = Some(Component::initialize(&input_rc, "modules/out/input.wasm", Imports::new())?);
    let input_ref = input_rc.borrow();
    let input_instance = input_ref.instance.as_ref().unwrap();

    let canvas_rc = Component::init(&store);
    let mut canvas_imports = Imports::new();
    canvas_imports.add_module("render", Renderer::import_module(&canvas_rc));
    canvas_imports.add_module("input", ImportModule::from_vec(vec![
        ("mouseIsDown", input_instance.get_func("mouseIsDown").unwrap()),
        ("mouseWentDown", input_instance.get_func("mouseWentDown").unwrap()),
        ("mouseX", input_instance.get_func("mouseX").unwrap()),
        ("mouseY", input_instance.get_func("mouseY").unwrap()),
    ]));
    canvas_rc.borrow_mut().instance = Some(Component::initialize(&canvas_rc, "modules/out/canvas.wasm", canvas_imports)?);
    let canvas_ref = canvas_rc.borrow();
    let canvas_instance = canvas_ref.instance.as_ref().unwrap();

    println!("Extracting exports...");
    let init = canvas_instance
        .get_func("init")
        .ok_or(anyhow::format_err!("failed to find `init` function export"))?
        .get0::<()>()?;
    let canvas_update = canvas_instance
        .get_func("update")
        .ok_or(anyhow::format_err!("failed to find `update` function export"))?
        .get0::<()>()?;

    let input_update = input_instance
        .get_func("update")
        .ok_or(anyhow::format_err!("failed to find `update` function export"))?
        .get0::<()>()?;
    let mouse_event = input_instance
        .get_func("onMouseEvent")
        .ok_or(anyhow::format_err!("failed to find `onMouseEvent` function export"))?
        .get3::<i32, i32, i32, ()>()?;

    println!("Starting main loop");
    init()?;
    let mut event_pump = render.sdl_context.event_pump().unwrap();
    let canvas_x = 200;
    let canvas_y = 150;
    let to_canvas_space = |x: i32, y: i32| -> (i32, i32) {
        (x - canvas_x, 600 - y - canvas_y)
    };
    'mainloop: loop {
        unsafe {
            gl::Viewport(0, 0, 800, 600);
            gl::ClearColor(0.8, 0.8, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        input_update()?; // TODO: figure out generic timing on this
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                Event::MouseMotion { x, y, .. } => {
                    let (x, y) = to_canvas_space(x, y);
                    mouse_event(0, x, y)?;
                },
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        let (x, y) = to_canvas_space(x, y);
                        mouse_event(1, x, y)?;
                    }
                },
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        let (x, y) = to_canvas_space(x, y);
                        mouse_event(2, x, y)?;
                    }
                },
                _ => {}
            }
        }

        render.pre_update();
        canvas_update()?;
        render.post_update();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Done.");
    Ok(())
}
