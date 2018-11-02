extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod ibb;
use ibb::view::*;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "Organims",
            [640, 480]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut app    = View::new(GlGraphics::new(opengl));
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}