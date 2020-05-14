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

mod component;
mod renderer;
use component::Component;
use renderer::Renderer;

fn main() -> Result<()> {
    let render = Renderer::new();

    let canvas_rc = Component::from_file("modules/out/canvas.wasm")?;
    let canvas_ref = canvas_rc.borrow();
    // let instance = Instance::new(&module, &imports)?;
    let canvas_instance = canvas_ref.instance.as_ref().unwrap();

    // Next we poke around a bit to extract the `frame` function from the module.
    println!("Extracting export...");
    let init = canvas_instance
        .get_func("init")
        .ok_or(anyhow::format_err!("failed to find `init` function export"))?
        .get0::<()>()?;
    let update = canvas_instance
        .get_func("update")
        .ok_or(anyhow::format_err!("failed to find `update` function export"))?
        .get0::<()>()?;
    let mouse_event = canvas_instance
        .get_func("mouseEvent")
        .ok_or(anyhow::format_err!("failed to find `mouseEvent` function export"))?
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
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
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
        update()?;
        render.post_update();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Done.");
    Ok(())
}
