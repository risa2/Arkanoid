extern crate sdl2;

use std::thread;
use std::f32;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event;


struct Block {
    color: Color,
    x: i32,
    y: i32
}

type Blocks=Vec<Block>;

struct Scene {
    width: i32,
    height: i32,
    blocks: Blocks
}

struct Ball<'a> {
    x: f32,
    y: f32,
    direction: f32,
    speed: f32,
    scene: &'a Scene
}

struct Palka {
    x: i32
}


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
    fn to_rect(&self)->Rect {
        Rect::new(self.x, self.y, Block::WIDTH, Block::HEIGHT)
    }
    fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
        renderer.set_draw_color(self.color);
        renderer.fill_rect(Rect::new(self.x, self.y, Block::WIDTH, Block::HEIGHT));
    }
}

impl Scene {
    fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
        renderer.set_draw_color(Color::RGB(0,200,0));
        renderer.clear();

        let blocks=&self.blocks;
        for block in blocks {
            block.render(renderer);
        }
    }
}

impl<'a> Ball<'a> {
    const SIZE: f32=20.0;
    fn to_rect(&self)->Rect {
        Rect::new(self.x, self.y, Ball::SIZE as u32, Ball::SIZE as u32)
    }
    fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
        renderer.set_draw_color(Color::RGB(0,0,255));
        renderer.fill_rect(self.to_rect());

    }

    fn update(&mut self) {
        self.x+=self.speed*self.direction.cos();
        self.y+=self.speed*self.direction.sin();

        if self.x>self.scene.width as f32-Ball::SIZE || self.x<0.0 {
            self.direction=std::f32::consts::PI-self.direction;
        }

        if self.y>self.scene.height as f32-Ball::SIZE || self.y<0.0 {
            self.direction=-self.direction;
        }
        if self.direction<0.0 {
            self.direction+=2.0*std::f32::consts::PI;
        }

        for block in self.scene.blocks {
            if self.to_rect().has_intersection(block.to_rect()) {

            }
        }
    }
}

struct App<'a> {
    ball: Ball<'a>,
    palka: Palka,
    scene: &'a Scene
}

impl<'a> App<'a> {
    fn render(&mut self, renderer: &mut sdl2::render::WindowCanvas) {

        scene.render(renderer);
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
        ball: Ball{x: 300.0, y: 400.0, direction: 3.0, speed: 2.0, scene: &scene},
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