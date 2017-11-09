extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

struct Block {
    color: Color,
    x: i32,
    y: i32
}

struct Ball {
    x: i32,
    y: i32,
    x_move: i32,
    y_move: i32
}

type Blocks=Vec<Block>;


fn make_blocks(left: u32, top: u32, width: u32, height: u32, x_count: u32, y_count: u32)->Blocks {
    let mut blocks: Blocks=vec![];
    for y in 0..y_count {
        for x in 0..x_count {
            let (dst_x, dst_y)=(left+x*width/x_count, top+y*height/y_count);
            blocks.push(Block{color: Color::RGB((x%2*255) as u8, ((x+1)%2*255) as u8, 0), x: dst_x as i32, y: dst_y as i32});
        }
    }
    blocks
}

struct Palka {
    x: i32
}

impl Block {
    fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
        renderer.set_draw_color(self.color);
        renderer.fill_rect(Rect::new(self.x, self.y, 20, 10));
    }
}

impl Ball {
    fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {


    }
}

struct App {
    blocks: Blocks,
    ball: Ball,
    palka: Palka
}

impl App {
    fn render(&mut self, renderer: &mut sdl2::render::WindowCanvas) {

        let blocks=&self.blocks;
        for block in blocks {
            block.render(renderer);
        }
    }

    fn update(&mut self) {

    }
}

fn main() {
    let sdl=sdl2::init().unwrap();
    let video=sdl.video().unwrap();
    let window=video.window("Arkanoid", 1000, 600).build().unwrap();
    let mut renderer=window.into_canvas().build().unwrap();

    let mut app = App {
        blocks: make_blocks(0, 0, 600, 600, 10, 10),
        ball: Ball{x: 300, y: 800, x_move: 0, y_move: 0},
        palka: Palka{x: 300}
    };

    let mut events = sdl.event_pump().unwrap();
    loop {
        for event in events.poll_iter() {

        }
        app.render(&mut renderer);
        app.update();
    }
}