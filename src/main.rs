extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
#[macro_use]
extern crate lazy_static;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop;
use piston::input;
use piston::input::{ButtonEvent, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod buttons;
mod draw;
mod state;

pub struct App {
    /// OpenGL drawing backend.
    gl: GlGraphics,
    /// Tracks which buttons are being held down.
    buttons: buttons::ButtonTracker,
    /// Game state.
    state: state::State,
}

impl App {
    fn new(gl: GlGraphics) -> Self {
        App {
            gl,
            buttons: buttons::ButtonTracker::new(),
            state: state::State::new(),
        }
    }
    fn render(&mut self, args: &input::RenderArgs) {
        draw::draw(&mut self.gl, args,  & self.state);
    }

    fn update(&mut self, args: &UpdateArgs) {
        for (i, p) in self.state.players.iter_mut().enumerate() {
            p.update(
                args.dt,
                self.buttons.is_key_held(i, buttons::Key::Left),
                self.buttons.is_key_held(i, buttons::Key::Right),
                self.buttons.is_key_held(i, buttons::Key::Up),
            );
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Rusty Lance", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = event_loop::Events::new(event_loop::EventSettings::new());
    // Main event loop.
    while let Some(e) = events.next(&mut window) {
        // For each of the following methods, the closure will be called if the
        // event has the right type.
        e.button(|button_args| app.buttons.handle(&button_args));
        e.render(|render_args| app.render(&render_args));
        e.update(|update_args| app.update(&update_args));
    }
}
