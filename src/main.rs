extern crate gl;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate sprite;

use gl::types::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::*;
use rand::Rng;
use sprite::*;

use std::rc::Rc;

const SQUARE_SIZE: f64 = 8.0;

pub struct Player {
    x: u32,
    y: u32,
}

pub struct Level {
    map: [[u8; 8]; 8],
}

impl Level {
    fn draw(&self, game: &Game, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        let mut n_x = 0.0;
        let mut n_y = 0.0;
        for i in &self.map {
            for j in i {
                draw_block(n_x, n_y, *j, game, c, gl);
                n_x += SQUARE_SIZE;
            }
            n_y = 0.0;
            n_y += SQUARE_SIZE;
        }
    }
}

pub struct Game {
    time: f64, // Rotation for the square.
    w: u32,
    h: u32,
    level: Level,
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    game: Game,
}

fn draw_block(
    x: f64,
    y: f64,
    b: u8,
    game: &Game,
    c: graphics::Context,
    gl: &mut opengl_graphics::GlGraphics,
) {
    use graphics::*;
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    let tx = f64::from(game.w / 2) + x * SQUARE_SIZE - 10.0 * SQUARE_SIZE / 2.0;
    let ty = 20.0 + y * SQUARE_SIZE;
    let square = rectangle::square(0.0, 0.0, SQUARE_SIZE);
    let transform = c.transform.trans(tx, ty);
    match b {
        1 => rectangle(RED, square, transform, gl),
        2 => rectangle(YELLOW, square, transform, gl),
        3 => rectangle(GREEN, square, transform, gl),
        4 => rectangle(BLUE, square, transform, gl),
        _ => (),
    };
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREY: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
        let game = &self.game;
        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            game.level.draw(game, ctx, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.game.time += args.dt;

        // self.game.board.update(args.dt);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let width = 1200;
    let height = 800;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Death + Taxes", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut map1 = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let level1 = Level { map: map1 };
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        game: Game {
            time: 0.0,
            w: width,
            h: height,
            level: level1,
        },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::A {
                println!("{:?}", key);
            }
            if key == Key::D {
                println!("{:?}", key);
            }
            if key == Key::S {
                println!("{:?}", key);
            }
            if key == Key::W {
                println!("{:?}", key);
            }
            if key == Key::Space {
                println!("{:?}", key);
            }
        };
        if let Some(Button::Keyboard(key)) = e.release_args() {
            if key == Key::A {}
            if key == Key::D {}
            if key == Key::S {}
        };
    }
}
