extern crate sdl2;

mod geometry;

use std::f32;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
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
            let (dst_x, dst_y)=(left+geometry::split(width, x_count, x), top+geometry::split(height, y_count, y));
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
        renderer.fill_rect(self.to_rect());
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
        Rect::new(self.x as i32, self.y as i32, Ball::SIZE as u32, Ball::SIZE as u32)
    }
    fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
        renderer.set_draw_color(Color::RGB(0,0,255));
        renderer.fill_rect(self.to_rect());

    }

    fn update(&mut self) {
        let shift=geometry::to_cartesian(self.speed, self.direction);
        self.x+=shift.0;
        self.y+=shift.1;

        if self.x>self.scene.width as f32-Ball::SIZE || self.x<0.0 {
            self.direction=geometry::horizontal_bounce(self.direction);
        }

        if self.y>self.scene.height as f32-Ball::SIZE || self.y<0.0 {
            self.direction=geometry::vertical_bounce(self.direction);
        }

		let blocks=&self.scene.blocks;
        for block in blocks {
                if self.to_rect().has_intersection(block.to_rect()) {
                    let left_up=Rect::new(block.x - Ball::SIZE, block.y - Ball::SIZE, Ball::SIZE, Ball::SIZE);
                    let right_up=Rect::new(block.x, block.y - Ball::SIZE, Ball::SIZE, Ball::SIZE);
                    let left_down=Rect::new(block.x - Ball::SIZE, block.y, Ball::SIZE, Ball::SIZE);
                    let right_down=Rect::new(block.x, block.y, Ball::SIZE, Ball::SIZE);

                    let left_up_corner=(block.x-Ball::SIZE, block.y - Ball::SIZE);
                    let right_up_corner=(block.x, block.y - Ball::SIZE);
                    let left_down_corner=(block.x - Ball::SIZE, block.y);
                    let right_down=(block.x, block.y);

                    let center=(self.x + Ball::SIZE / 2, self.y + Ball::SIZE / 2);
                    let this=Point::new(self.x, self.y);

                    let not_touch=(1.0/0.0, 1.0/0.0);
                    let wall=(-1.0/0.0, -1.0/0.0);

                    let touch_point=
                    if left_up.contains_point(this) {
                        if geometry::distance(center, left_up_corner)<=Ball::SIZE {left_down_corner}
                        else {not_touch}
                    }
                    else if right_up.contains_point(this) {
                        if geometry::distance(center, right_up_corner)<=Ball::SIZE {right_up_corner}
                        else {not_touch}
                    }
                    else if left_down.contains_point(this) {
                        if geometry::distance(center, left_down_corner)<=Ball::SIZE {left_down_corner}
                        else {not_touch}
                    }
                    else if right_down.contains_point(this) {
                        if geometry::distance(center, right_down_corner)<=Ball::SIZE {right_down_corner}
                        else {not_touch}
                    }
                    else {wall};
                    if touch_point==not_touch {
                        continue;
                    }
                    if touch_point!=wall {
                        self.direction=geometry::bounce(self.direction, (self.y+Ball::SIZE/2-touch_point.1).atan2(self.x+Ball::SIZE/2-touch_point.0));
                    }
                }
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

        self.scene.render(renderer);
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

    let scene=Scene{blocks: make_blocks(10, 0, 980, 600, 10, 10), width: 1000, height: 600};
    let mut app=App {
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
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}