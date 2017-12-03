extern crate sdl2;

mod geometry;
mod draw;

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
	const WIDTH:u32=100;
	const HEIGHT:u32=40;
	fn to_rect(&self)->Rect {
		Rect::new(self.x, self.y, Block::WIDTH, Block::HEIGHT)
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(self.color);
		renderer.fill_rect(self.to_rect()).unwrap();
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
	const SIDE: (f32, f32)=(-1.0/0.0, -1.0/0.0);
	const NOT_TOUCH: (f32, f32)=(1.0/0.0, 1.0/0.0);
	fn to_rect(&self)->Rect {
		Rect::new(self.x as i32, self.y as i32, Ball::SIZE as u32, Ball::SIZE as u32)
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(0,0,255));
		draw::circle(renderer, Point::new((self.x+Ball::SIZE/2.0) as i32, (self.y+Ball::SIZE/2.0) as i32), Ball::SIZE/2.0, 0.1);

	}

	fn collision(&self, block: &Block)->(f32, f32) {
		if self.to_rect().has_intersection(block.to_rect()) {

			let left_up_corner=(block.x as f32, block.y as f32);
			let right_up_corner=(block.x as f32+Block::WIDTH as f32, block.y as f32);
			let left_down_corner=(block.x as f32, block.y as f32+Block::HEIGHT as f32);
			let right_down_corner=(block.x as f32+Block::WIDTH as f32, block.y as f32+Block::HEIGHT as f32);

			let left_up=Rect::new(left_up_corner.0 as i32, left_up_corner.1 as i32, Ball::SIZE as u32, Ball::SIZE as u32);
			let right_up=Rect::new(right_up_corner.0 as i32, right_up_corner.1 as i32, Ball::SIZE as u32, Ball::SIZE as u32);
			let left_down=Rect::new(left_down_corner.0 as i32, left_down_corner.1 as i32, Ball::SIZE as u32, Ball::SIZE as u32);
			let right_down=Rect::new(right_down_corner.0 as i32, right_down_corner.1 as i32, Ball::SIZE as u32, Ball::SIZE as u32);

			let center=(self.x + Ball::SIZE / 2.0, self.y + Ball::SIZE / 2.0);
			let this=Point::new(self.x as i32, self.y as i32);

			if left_up.contains_point(this) {
				if geometry::distance(center, left_up_corner)<=Ball::SIZE {left_down_corner}
					else {Ball::NOT_TOUCH}
			}
			else if right_up.contains_point(this) {
				if geometry::distance(center, right_up_corner)<=Ball::SIZE {right_up_corner}
					else {Ball::NOT_TOUCH}
			}
			else if left_down.contains_point(this) {
				if geometry::distance(center, left_down_corner)<=Ball::SIZE {left_down_corner}
					else {Ball::NOT_TOUCH}
			}
			else if right_down.contains_point(this) {
				if geometry::distance(center, right_down_corner)<=Ball::SIZE {right_down_corner}
					else {Ball::NOT_TOUCH}
			}
			else {Ball::SIDE}
		}
		else {Ball::NOT_TOUCH}
	}
		
	fn go(&mut self) {
		let (dx, dy)=geometry::to_cartesian(self.speed, self.direction);
		self.x+=dx;
		self.y+=dy;
	}

	fn update(&mut self) {
		self.go();
	
		if self.x>self.scene.width as f32-Ball::SIZE || self.x<0.0 {
			self.direction=geometry::horizontal_bounce(self.direction);
		}

		if self.y>self.scene.height as f32-Ball::SIZE || self.y<0.0 {
			self.direction=geometry::vertical_bounce(self.direction);
		}

		let blocks=&self.scene.blocks;
		for block in blocks {
			let touch_point=self.collision(block);

			if touch_point==Ball::SIDE {
				if (block.x as f32-self.x).abs()<=Ball::SIZE {
					self.direction=geometry::horizontal_bounce(self.direction);
				}
				if (block.y as f32-self.y).abs()<=Ball::SIZE {
					self.direction=geometry::vertical_bounce(self.direction);
				}
			}
			else if touch_point!=Ball::NOT_TOUCH{
				self.direction=geometry::bounce(self.direction, geometry::line_angle(touch_point, (self.x+Ball::SIZE/2.0, self.y+Ball::SIZE/2.0)));
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

	let scene=Scene{blocks: make_blocks(10, 10, 990, 590, 5, 6), width: 1000, height: 600};

	let mut app=App {
		ball: Ball{x: 350.0, y: 500.0, direction: geometry::PI*3.0/4.0, speed: 2.0, scene: &scene},
		palka: Palka{x: 300},
		scene: &scene
	};

	let mut events=sdl.event_pump().unwrap();
	'main: loop {
		for event in events.poll_iter() {
			match event {
				event::Event::Quit{timestamp: _t} => {break 'main}
				_ => ()
			}
		}
		app.render(&mut renderer);
		app.update();
		std::thread::sleep(std::time::Duration::from_millis(20));
	}
}

#[test]
fn test_collision() {
	let blocks=vec![];
	let scene=Scene{width: 800, height:600, blocks: blocks};

	let block=Block{x: 400, y: 400, color: Color::RGB(0,0,0)};
	let ball_1=Ball{x: 405.0, y: 400.0-Ball::SIZE+1.0, direction: 0.0, speed: 1.0, scene: &scene};
	let ball_2=Ball{x: 405.0, y: 400.0+Block::HEIGHT as f32-1.0, direction: 0.0, speed: 1.0, scene: &scene};
	let ball_3=Ball{x: 400.0-Ball::SIZE+1.0, y: 405.0, direction: 0.0, speed: 1.0, scene: &scene};
	let ball_4=Ball{x: 400.0+Block::WIDTH as f32-1.0, y: 405.0, direction: 0.0, speed: 1.0, scene: &scene};
	let ball_5=Ball{x: 400.0+Block::WIDTH as f32/2.0, y: 400.0+Block::HEIGHT as f32-1.0, direction: 0.0, speed: 1.0, scene: &scene};
	let ball_6=Ball{x: 400.0-Ball::SIZE, y: 400.0-Ball::SIZE, direction: 0.0, speed: 1.0, scene: &scene};
	assert_eq!(ball_1.collision(&block), Ball::SIDE);
	assert_eq!(ball_2.collision(&block), Ball::SIDE);
	assert_eq!(ball_3.collision(&block), Ball::SIDE);
	assert_eq!(ball_4.collision(&block), Ball::SIDE);
	assert_eq!(ball_5.collision(&block), Ball::SIDE);
	assert_eq!(ball_6.collision(&block), Ball::NOT_TOUCH);
}