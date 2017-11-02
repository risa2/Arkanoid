extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::vec::Vec;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

struct Block {
    color: [f32; 4],
    x: f64,
    y: f64
}

struct Ball {
    x: f64,
    y: f64,
    x_move: f64,
    y_move: f64
}

type Blocks=Vec<Block>;

trait BlocksTrait{
    fn make(left: f64, top: f64, width: f64, height: f64, x_count: u32, y_count: u32)->Blocks;
}

impl BlocksTrait for Blocks {
    fn make(left: f64, top: f64, width: f64, height: f64, x_count: u32, y_count: u32)->Blocks {
        let mut blocks: Blocks=vec![];
        for x in 0..x_count {
            for y in 0..y_count {
                let (dst_x, dst_y)=(left+(x as f64)*width/(x_count as f64), top+(y as f64)*width/(y_count as f64));
                blocks.push(Block{color: [(x%2) as f32, ((x+1)%2) as f32, 0.0, 1.0], x: dst_x, y: dst_y});
            }
        }
        blocks
    }
}

struct Palka {
    x: f64
}

impl Block {
    fn render(&self, c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;
        rectangle(self.color, [self.x, self.y, 25.0, 10.0], c.transform, gl);
    }
}

impl Ball {
    fn render(&self, c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;


    }
}

struct App {
    gl: GlGraphics,
    blocks: Blocks,
    ball: Ball,
    palka: Palka
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let blocks=&self.blocks;
        self.gl.draw(args.viewport(), |c, glg| {
            clear([0.0, 1.0, 0.0, 1.0], glg);

            for block in blocks {
                block.render(c, glg);
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs) {

    }
}

fn main() {
    let opengl = OpenGL::V3_0;

    let mut window: Window = WindowSettings::new(
        "Arkanoid",
        [1000, 600]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        blocks: Blocks::make(0.0, 0.0, 150.0, 600.0, 16, 10),
        ball: Ball{x: 300.0, y: 800.0, x_move: 0.0, y_move: 0.0},
        palka: Palka{x: 300.0}
    };

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