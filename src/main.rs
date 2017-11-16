extern crate sdl2;

use std::thread;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event;

struct Scene {
    width: i32,
    height: i32
}

struct Block {
    color: Color,
    x: i32,
    y: i32
}

struct Ball<'a> {
    x: i32,
    y: i32,
    x_move: i32,
    y_move: i32,
    scene: &'a Scene
}

struct Palka {
    x: i32
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

impl Block {
    const WIDTH:u32=20;
    const HEIGHT:u32=10;
    fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
        renderer.set_draw_color(self.color);
        renderer.fill_rect(Rect::new(self.x, self.y, Block::WIDTH, Block::HEIGHT));
    }
}

impl<'a> Ball<'a> {
    const SIZE: i32=20;
    fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
        renderer.set_draw_color(Color::RGB(0,0,255));
        renderer.fill_rect(Rect::new(self.x-Ball::SIZE/2, self.y-Ball::SIZE/2, Ball::SIZE as u32, Ball::SIZE as u32));

    }

    fn update(&mut self) {
        self.x+=self.x_move;
        self.y+=self.y_move;

        if self.x>self.scene.width-Ball::SIZE/2 || self.x<Ball::SIZE/2 {
            self.x_move=-self.x_move;
        }

        if self.y>self.scene.height-Ball::SIZE/2 || self.y<Ball::SIZE/2 {
            self.y_move=-self.y_move;
        }
    }
}

struct App<'a> {
    blocks: Blocks,
    ball: Ball<'a>,
    palka: Palka,
    scene: &'a Scene
}

impl<'a> App<'a> {
    fn render(&mut self, renderer: &mut sdl2::render::WindowCanvas) {

        renderer.set_draw_color(Color::RGB(0,200,0));
        renderer.clear();

        let blocks=&self.blocks;
        for block in blocks {
            block.render(renderer);
        }
        self.ball.render(renderer);
        renderer.present();
    }

    fn update(&mut self) {
        self.ball.update();
    }
}

fn main() {
    let sdl=sdl2::init().unwrap();
    let video=sdl.video().unwrap();
    let window=video.window("Arkanoid", 1000, 600).build().unwrap();
    let mut renderer=window.into_canvas().build().unwrap();

    let scene=Scene{width: 1000, height: 600};
    let mut app=App {
        blocks: make_blocks(10, 0, 980, 600, 10, 10),
        ball: Ball{x: 300, y: 400, x_move: 2, y_move: -1, scene: &scene},
        palka: Palka{x: 300},
        scene: &scene
    };

    let mut events=sdl.event_pump().unwrap();
    'outer: loop {
        for event in events.poll_iter() {

            match event {
                event::Event::Quit{timestamp} => {break 'outer}
                _ => ()
            }
        }
        app.render(&mut renderer);
        app.update();
        std::thread::sleep_ms(20);
    }
}