extern crate sdl2;

mod geometry;
mod draw;

use std::f32;

use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::event;
use sdl2::keyboard;

#[derive(Copy, Clone)]
struct Block {
	color: Color,
	x: i32,
	y: i32
}

type Blocks=Vec<Block>;

struct Scene {
	width: i32,
	height: i32,
	blocks: Blocks,
	palka: Palka
}

struct Ball {
	circle: geometry::Circle,
	direction: f32,
	speed: f32
}

struct Palka {
	x: i32,
	y: i32,
	w: i32
}

enum Collision {
	None,
	At(i32, i32)
}

struct App {
	ball: Ball,
	scene: Scene
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
		self.palka.render(renderer);
	}
	fn update(&mut self, evt: &sdl2::EventPump) {
        self.palka.update(evt, self.width);
	}
}
impl Palka {
	const HEIGHT: i32=10;
	fn to_rect(&self)->Rect {
		Rect::new(self.x-self.w/2, self.y, self.w as u32, Palka::HEIGHT as u32)
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(0,0,0));
		renderer.fill_rect(self.to_rect()).unwrap();
	}
	fn update(&mut self, evt: &sdl2::EventPump, width: i32) {
		let kb=evt.keyboard_state();
		if kb.is_scancode_pressed(keyboard::Scancode::Left) {
			self.x=if self.x-self.w/2<3 {self.w/2} else {self.x-3}
		}
		if kb.is_scancode_pressed(keyboard::Scancode::Right) {
            self.x=if self.x>width-3-self.w/2 {width-self.w/2} else {self.x+3}
		}
	}
}

impl Ball {
	fn to_rect(&self)->Rect {
		Rect::new(self.circle.corner().0, self.circle.corner().1, self.circle.radius as u32*2, self.circle.radius as u32*2)
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(0,255,255));
		draw::circle(renderer, self.circle, 0.1).unwrap();
	}

	fn collision(&self, rect: Rect)->Collision {
		if self.to_rect().has_intersection(rect) {
			let (left, up)=(rect.x, rect.y);
			let (right, down)=(rect.x+Block::WIDTH as i32, rect.y+Block::HEIGHT as i32);
			let size=self.circle.radius as i32*2;

			let left_up=Rect::new(left-size, up-size, size as u32, size as u32);
			let right_up=Rect::new(right, up-size, size as u32, size as u32);
			let left_down=Rect::new(left-size, down, size as u32, size as u32);
			let right_down=Rect::new(right, down, size as u32, size as u32);

			let left_r=Rect::new(left-size, up, size as u32, Block::HEIGHT as u32);
			let up_r=Rect::new(left, up-size, Block::WIDTH as u32, size as u32);
			let right_r=Rect::new(right, up, size as u32, Block::HEIGHT as u32);
			let down_r=Rect::new(left, down, Block::WIDTH as u32, size as u32);

			let this=Point::new(self.circle.x, self.circle.y);

			if left_up.contains_point(this) {
				if geometry::distance(self.circle.center(), (left, up))<=self.circle.radius {Collision::At(left, up)}
				else {Collision::None}
			}
			else if right_up.contains_point(this) {
				if geometry::distance(self.circle.center(), (right, up))<=self.circle.radius {Collision::At(right, up)}
				else {Collision::None}
			}
			else if left_down.contains_point(this) {
				if geometry::distance(self.circle.center(), (left, down))<=self.circle.radius {Collision::At(left, down)}
				else {Collision::None}
			}
			else if right_down.contains_point(this) {
				if geometry::distance(self.circle.center(), (right, down))<=self.circle.radius {Collision::At(right, down)}
				else {Collision::None}
			}
			else if left_r.contains_point(this) {Collision::At(left, self.circle.y)}
			else if right_r.contains_point(this) {Collision::At(right, self.circle.y)}
			else if up_r.contains_point(this) {Collision::At(self.circle.x, up)}
			else if down_r.contains_point(this) {Collision::At(self.circle.x, down)}
			else {Collision::None}
		}
		else {Collision::None}
	}
		
	fn go(&mut self) {
		let (dx, dy)=geometry::to_cartesian(self.speed, self.direction);
		self.circle.x+=dx;
		self.circle.y+=dy;
	}

	fn update(&mut self, scene: &mut Scene) {
		self.go();
	
		if self.circle.x>scene.width-self.circle.radius as i32 || self.circle.x<self.circle.radius as i32 {
			self.direction=geometry::horizontal_bounce(self.direction);
		}

		if self.circle.y>scene.height-self.circle.radius as i32 || self.circle.y<self.circle.radius as i32 {
			self.direction=geometry::vertical_bounce(self.direction);
		}


		let blocks=&mut scene.blocks;
		for i in 0..blocks.len() {
			match self.collision(blocks[i].to_rect()) {
				Collision::At(x, y) => {
					self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
					blocks.remove(i);
					break;
				},
				Collision::None => ()
			};
		}
		match self.collision(scene.palka.to_rect()) {
			Collision::At(x, y) => {
				self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
				let dx=self.circle.x-scene.palka.x;
				self.direction+=dx as f32/scene.palka.w as f32;
                self.direction=self.direction.max(geometry::PI/6.0*7.0).min(geometry::PI/6.0*11.0);
			},
			Collision::None => ()
		}
	}
}

impl App {
	fn render(&mut self, renderer: &mut sdl2::render::WindowCanvas) {

		self.scene.render(renderer);
		self.ball.render(renderer);
		renderer.present();
	}

	fn update(&mut self, evt: &sdl2::EventPump) {
		self.ball.update(&mut self.scene);
		self.scene.update(evt);
	}

    fn end(&self)->bool {
        self.ball.circle.y>=self.scene.height-self.ball.circle.radius as i32
    }
}

fn main() {
	let sdl=sdl2::init().unwrap();
	let video=sdl.video().unwrap();
	let window=video.window("Arkanoid", 1000, 600).build().unwrap();
	let mut renderer=window.into_canvas().build().unwrap();

	let mut app=App {
		ball: Ball{circle: geometry::Circle{x: 350, y: 200, radius: 10.0}, direction: geometry::PI/2.0, speed: 3.0},
		scene: Scene{blocks: make_blocks(10, 10, 990, 590, 5, 6), width: 1000, height: 600, palka: Palka{x: 300, y: 580, w: 80}}
	};

	let mut events=sdl.event_pump().unwrap();

	'main: while !app.end() {
		for event in events.poll_iter() {
			match event {
				event::Event::Quit{timestamp: _t} => {break 'main},
				_ => ()
			}
		}
		app.update(&events);
		app.render(&mut renderer);
		std::thread::sleep(std::time::Duration::from_millis(20));
	}
}

#[test]
fn test_collision() {
	let blocks=vec![];
	let scene=Scene{width: 800, height:600, blocks: blocks, palka: Palka{x: 300, y: 580, w: 80}};

	let block=Block{x: 400, y: 400, color: Color::RGB(0,0,0)};
	let ball_1=Ball{circle: geometry::Circle{x: 410, y: 400-10+1, radius: 10.0}, direction: 0.0, speed: 1.0};
	let ball_2=Ball{circle: geometry::Circle{x: 410, y: 400+Block::HEIGHT+10-1, radius: 10.0}, direction: 0.0, speed: 1.0};
	let ball_3=Ball{circle: geometry::Circle{x: 400-10+1, y: 410, radius: 10.0}, direction: 0.0, speed: 1.0};
	let ball_4=Ball{circle: geometry::Circle{x: 400+Block::WIDTH+10-1, y: 410, radius: 10.0}, direction: 0.0, speed: 1.0};
	let ball_5=Ball{circle: geometry::Circle{x: 400+Block::WIDTH/2, y: 400+Block::HEIGHT+10-1, radius: 10.0}, direction: 0.0, speed: 1.0};
	let ball_6=Ball{circle: geometry::Circle{x: 400-10, y: 400-10, radius: 10.0}, direction: 0.0, speed: 1.0};
	assert!(!ball_1.collision(&block).unwrap().is_some());
	assert!(!ball_2.collision(&block).unwrap().is_some());
	assert!(!ball_3.collision(&block).unwrap().is_some());
	assert!(!ball_4.collision(&block).unwrap().is_some());
	assert!(!ball_5.collision(&block).unwrap().is_some());
	assert!(!ball_6.collision(&block).is_some());
}