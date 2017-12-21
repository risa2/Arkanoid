extern crate sdl2;

mod geometry;
mod circle;
mod draw;

use std::f32;
use std::ops::Index;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event;
use sdl2::keyboard;

#[derive(Copy, Clone)]
struct Block {
	color: Color,
	x: i32,
	y: i32
}

struct Blocks {
	data: Vec<Block>
}

struct Scene {
	width: i32,
	height: i32,
	blocks: Blocks,
	palka: Palka
}

struct Ball {
	circle: circle::Circle,
	direction: f32,
	speed: f32
}

struct Palka {
	x: i32,
	y: i32,
	w: i32
}

struct App {
	ball: Ball,
	scene: Scene
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

impl Blocks {
	fn new(left: u32, top: u32, width: u32, height: u32, x_count: u32, y_count: u32)->Blocks {
		let mut blocks: Vec<Block> =vec![];
		for y in 0..y_count {
			for x in 0..x_count {
				let (dst_x, dst_y)=(left+geometry::split(width, x_count, x), top+geometry::split(height, y_count, y));
				blocks.push(Block{color: Color::RGB((x%2*255) as u8, ((x+1)%2*255) as u8, 0), x: dst_x as i32, y: dst_y as i32});
			}
		}
		Blocks{data: blocks}
	}
	fn len(&self)->usize {
		self.data.len()
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		for block in &self.data {
			block.render(renderer);
		}
	}
	fn remove(&mut self, i: usize) {
		self.data.remove(i);
	}
}

impl Index<usize> for Blocks {
	type Output=Block;
	fn index(&self, i: usize)-> &Block {
		&self.data[i]
	}
}

impl Scene {
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(0,200,0));
		renderer.clear();

		let blocks=&self.blocks;
		blocks.render(renderer);
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
		let shift=if kb.is_scancode_pressed(keyboard::Scancode::Space) {5} else {2};
		if kb.is_scancode_pressed(keyboard::Scancode::Left) {
			self.x=if self.x-self.w/2<shift {self.w/2} else {self.x-shift}
		}
		if kb.is_scancode_pressed(keyboard::Scancode::Right) {
            self.x=if self.x>width-shift-self.w/2 {width-self.w/2} else {self.x+shift}
		}
	}
}

impl Ball {
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(0,255,255));
		draw::circle(renderer, self.circle, 0.1).unwrap();
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
			match self.circle.collision(blocks[i].to_rect()) {
				circle::Collision::At(x, y) => {
					self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
					blocks.remove(i);
					break;
				},
				circle::Collision::None => ()
			};
		}
		match self.circle.collision(scene.palka.to_rect()) {
			circle::Collision::At(x, y) => {
				self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
				let dx=self.circle.x-scene.palka.x;
				self.direction+=dx as f32/scene.palka.w as f32;
                self.direction=self.direction.max(geometry::PI/6.0*7.0).min(geometry::PI/6.0*11.0);
			},
			circle::Collision::None => ()
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
        self.ball.circle.y>=self.scene.height-self.ball.circle.radius as i32 self.scene.blocks.len()==0
    }
}

fn main() {
	let sdl=sdl2::init().unwrap();
	let video=sdl.video().unwrap();
	let window=video.window("Arkanoid", 1000, 600).build().unwrap();
	let mut renderer=window.into_canvas().build().unwrap();

	let mut app=App {
		ball: Ball{circle: circle::Circle{x: 350, y: 200, radius: 10.0}, direction: geometry::PI/2.0, speed: 3.0},
		scene: Scene{blocks: Blocks::new(10, 10, 990, 590, 5, 6), width: 1000, height: 600, palka: Palka{x: 300, y: 580, w: 80}}
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
	let ball_1=Ball{circle: circle::Circle{x: 410, y: 400-10+1, radius: 10.0}, direction: 0.0, speed: 1.0};
	let ball_2=Ball{circle: circle::Circle{x: 410, y: 400+Block::HEIGHT+10-1, radius: 10.0}, direction: 0.0, speed: 1.0};
	let ball_3=Ball{circle: circle::Circle{x: 400-10+1, y: 410, radius: 10.0}, direction: 0.0, speed: 1.0};
	let ball_4=Ball{circle: circle::Circle{x: 400+Block::WIDTH+10-1, y: 410, radius: 10.0}, direction: 0.0, speed: 1.0};
	let ball_5=Ball{circle: circle::Circle{x: 400+Block::WIDTH/2, y: 400+Block::HEIGHT+10-1, radius: 10.0}, direction: 0.0, speed: 1.0};
	let ball_6=Ball{circle: circle::Circle{x: 400-10, y: 400-10, radius: 10.0}, direction: 0.0, speed: 1.0};
	assert!(!ball_1.collision(&block).unwrap().is_some());
	assert!(!ball_2.collision(&block).unwrap().is_some());
	assert!(!ball_3.collision(&block).unwrap().is_some());
	assert!(!ball_4.collision(&block).unwrap().is_some());
	assert!(!ball_5.collision(&block).unwrap().is_some());
	assert!(!ball_6.collision(&block).is_some());
}