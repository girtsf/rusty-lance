extern crate graphics;
extern crate opengl_graphics;

use super::state;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
// use graphics::Context;

struct Draw<'a> {
    gl_graphics: &'a mut GlGraphics,
    render_args: &'a RenderArgs,
    state: &'a state::State,
}

impl<'a> Draw<'a> {
    fn draw_player(&mut self, player: &state::Player, color: &[f32; 4]) {
        use self::graphics::{rectangle, Transformed};

        let context = self.gl_graphics.draw_begin(self.render_args.viewport());
        let square = rectangle::centered_square(0.0, 0.0, 25.0);
        let state::Player {
            x, y, lean_angle, ..
        } = *player;

        let transform = context
            .transform
            .trans(x, self.render_args.height as f64 - y - 25.0)
            .rot_rad(lean_angle);

        rectangle(*color, square, transform, self.gl_graphics);
        self.gl_graphics.draw_end();
    }
    fn draw(&mut self) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        self.gl_graphics
            .draw(self.render_args.viewport(), |context, gl| {
                use self::graphics::clear;
                clear(BLACK, gl);
            });

        self.draw_player(&self.state.players[0], &RED);
        self.draw_player(&self.state.players[1], &GREEN);
    }
}

pub fn draw(gl_graphics: &mut GlGraphics, render_args: &RenderArgs, state: &state::State) {
    Draw {
        gl_graphics,
        render_args,
        state,
    }.draw();
}
