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
	circle: geometry::Circle,
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
	const WIDTH:i32=100;
	const HEIGHT:i32=40;
	fn to_rect(&self)->Rect {
		Rect::new(self.x, self.y, Block::WIDTH as u32, Block::HEIGHT as u32)
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

enum Collision {
	None,
	Side,
	Corner((i32, i32))
}

impl<'a> Ball<'a> {
	fn to_rect(&self)->Rect {
		Rect::new(self.circle.corner().0, self.circle.corner().1, self.circle.radius as u32*2, self.circle.radius as u32*2)
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(0,0,255));
		draw::circle(renderer, self.circle, 0.1).unwrap();
	}
	
	fn collision(&self, block: &Block)->Collision {
		if self.to_rect().has_intersection(block.to_rect()) {
			let left_up_corner=(block.x, block.y);
			let right_up_corner=(block.x+Block::WIDTH as i32, block.y);
			let left_down_corner=(block.x, block.y+Block::HEIGHT as i32);
			let right_down_corner=(block.x+Block::WIDTH as i32, block.y+Block::HEIGHT as i32);

			let left_up=Rect::new(left_up_corner.0, left_up_corner.1, self.circle.radius as u32*2, self.circle.radius as u32*2);
			let right_up=Rect::new(right_up_corner.0, right_up_corner.1, self.circle.radius as u32*2, self.circle.radius as u32*2);
			let left_down=Rect::new(left_down_corner.0, left_down_corner.1, self.circle.radius as u32*2, self.circle.radius as u32*2);
			let right_down=Rect::new(right_down_corner.0, right_down_corner.1, self.circle.radius as u32*2, self.circle.radius as u32*2);

			let center=(self.circle.x, self.circle.y);
			let this=Point::new(self.circle.corner().0, self.circle.corner().1);

			if left_up.contains_point(this) {
				if geometry::distance(center, left_up_corner)<=self.circle.radius {Collision::Corner(left_down_corner)}
				else {Collision::None}
			}
			else if right_up.contains_point(this) {
				if geometry::distance(center, right_up_corner)<=self.circle.radius {Collision::Corner(right_up_corner)}
				else {Collision::None}
			}
			else if left_down.contains_point(this) {
				if geometry::distance(center, left_down_corner)<=self.circle.radius {Collision::Corner(left_down_corner)}
				else {Collision::None}
			}
			else if right_down.contains_point(this) {
				if geometry::distance(center, right_down_corner)<=self.circle.radius {Collision::Corner(right_down_corner)}
				else {Collision::None}
			}
			else {Collision::Side}
		}
		else {Collision::None}
	}
		
	fn go(&mut self) {
		let (dx, dy)=geometry::to_cartesian(self.speed, self.direction);
		self.circle.x+=dx;
		self.circle.y+=dy;
	}

	fn update(&mut self) {
		self.go();
	
		if self.circle.x>self.scene.width-self.circle.radius as i32 || self.circle.x<self.circle.radius as i32 {
			self.direction=geometry::horizontal_bounce(self.direction);
		}

		if self.circle.y>self.scene.height-self.circle.radius as i32 || self.circle.y<self.circle.radius as i32 {
			self.direction=geometry::vertical_bounce(self.direction);
		}

		let blocks=&self.scene.blocks;
		for block in blocks {
			match self.collision(block) {
				Collision::Corner(touch_point) => {
					self.direction=geometry::bounce(self.direction, geometry::line_angle(touch_point, self.circle.center()));
				},
				Collision::Side => {
					if block.x-self.circle.x<=self.circle.radius as i32 || self.circle.x-block.x-Block::WIDTH<=self.circle.radius as i32 {
						self.direction=geometry::horizontal_bounce(self.direction);
					}
					if block.y-self.circle.y<=self.circle.radius as i32 || self.circle.y-block.y-Block::HEIGHT<=self.circle.radius as i32 {
						self.direction=geometry::vertical_bounce(self.direction);
					}
				},
				Collision::None => ()
			};
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
		ball: Ball{circle: geometry::Circle{x: 350, y: 500, radius: 10.0}, direction: geometry::PI/4.0, speed: 2.0, scene: &scene},
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
	let ball_1=Ball{circle: geometry::Circle{x: 410, y: 400-10+1, radius: 10.0}, direction: 0.0, speed: 1.0, scene: &scene};
	let ball_2=Ball{circle: geometry::Circle{x: 410, y: 400+Block::HEIGHT+10-1, radius: 10.0}, direction: 0.0, speed: 1.0, scene: &scene};
	let ball_3=Ball{circle: geometry::Circle{x: 400-10+1, y: 410, radius: 10.0}, direction: 0.0, speed: 1.0, scene: &scene};
	let ball_4=Ball{circle: geometry::Circle{x: 400+Block::WIDTH+10-1, y: 410, radius: 10.0}, direction: 0.0, speed: 1.0, scene: &scene};
	let ball_5=Ball{circle: geometry::Circle{x: 400+Block::WIDTH/2, y: 400+Block::HEIGHT+10-1, radius: 10.0}, direction: 0.0, speed: 1.0, scene: &scene};
	let ball_6=Ball{circle: geometry::Circle{x: 400-10, y: 400-10, radius: 10.0}, direction: 0.0, speed: 1.0, scene: &scene};
	assert!(!ball_1.collision(&block).unwrap().is_some());
	assert!(!ball_2.collision(&block).unwrap().is_some());
	assert!(!ball_3.collision(&block).unwrap().is_some());
	assert!(!ball_4.collision(&block).unwrap().is_some());
	assert!(!ball_5.collision(&block).unwrap().is_some());
	assert!(!ball_6.collision(&block).is_some());
}