extern crate glutin_window;
extern crate graphics;
extern crate nalgebra;
extern crate opengl_graphics;
extern crate piston;

use std::usize;

use graphics::math::Matrix2d;
use graphics::Transformed;
// use glutin_window::GlutinWindow as Window;
use nalgebra::Vector2;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderArgs;
use piston::window::WindowSettings;
use piston::{RenderEvent, UpdateArgs, UpdateEvent};

use rand::{self, Rng};

use glutin_window::GlutinWindow;
// use opengl_graphics::{GlGraphics, OpenGL};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

struct Game {
    gl: GlGraphics,
    n: usize,
    rad: f64,
    points: Vec<Vector2<f64>>,
    at: i64,
    col: [f32; 4],
    update: bool,
}

impl Game {
    fn new(n: usize, rad: f64, opengl: OpenGL) -> Game {
        let mut g = Game {
            gl: GlGraphics::new(opengl),
            n,
            rad,
            points: Vec::new(),
            at: 0,
            update: false,
            col: [1.0, 1.0, 1.0, 1.0],
        };

        for i in 0..n {
            let v2 = g.getPointCoords(i);
            g.points.push(v2);
        }

        return g;
    }

    fn getPointCoords(&self, i: usize) -> Vector2<f64> {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / self.n as f64;

        let x = self.rad * angle.cos();
        let y = self.rad * angle.sin();

        return Vector2::new(x, y);
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
        });

        for p in &self.points {
            let circle = graphics::Ellipse::new(BLACK);
            self.gl.draw(args.viewport(), |c, gl| {
                let trans = c
                    .transform
                    .trans((WIDTH as f64) / 2.0, (HEIGHT as f64) / 2.0);

                circle.draw([p.x, p.y, 1.0, 1.0], &c.draw_state, trans, gl);
            });
        }

        let mut rng = rand::thread_rng();
        for i in 1..self.at {
            let from = self.getPointCoords(i as usize);
            let to = self.getPointCoords((i + i) as usize);

            let l = graphics::Line::new(self.col, 0.1);
            self.gl.draw(args.viewport(), |c, gl| {
                let trans = c
                    .transform
                    .trans((WIDTH as f64) / 2.0, (HEIGHT as f64) / 2.0);

                l.draw([from.x, from.y, to.x, to.y], &c.draw_state, trans, gl);
            });
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.at += 1;

        for i in 0..3 {
            let mut rng = rand::thread_rng();
            let rn: f32 = rng.gen();

            self.col[i] -= rn / 100.0;
            if self.col[i] < 0.0 {
                self.col[i] = 1.0;
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Hello Piston!", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = Game::new(500, 400.0, opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
}
