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
use sdl2::gfx::primitives::DrawRenderer;

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
	speed: i32
}

struct Palka {
	x: i32,
	y: i32,
	w: i32
}

struct App <'a> {
	font: sdl2::ttf::Font<'a, 'static>,
	ball: Ball,
	scene: Scene,
	score: i32
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
		let shift=5;
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
		renderer.filled_circle(self.circle.x as i16, self.circle.y as i16, self.circle.radius as i16, Color::RGB(0, 0, 255)).unwrap();
		renderer.aa_circle(self.circle.x as i16, self.circle.y as i16, self.circle.radius as i16, Color::RGB(0, 0, 255)).unwrap();
	}
	
	fn go(&mut self) {
		let (dx, dy)=geometry::to_cartesian(1.0, self.direction);
		self.circle.x+=dx;
		self.circle.y+=dy;
	}

	fn update(&mut self, scene: &mut Scene) {
		for i in 0..self.speed {
			self.go();

			if self.circle.x>scene.width as f32-self.circle.radius || self.circle.x<self.circle.radius {
				self.direction=geometry::horizontal_bounce(self.direction);
			}

			if self.circle.y>scene.height as f32-self.circle.radius || self.circle.y<self.circle.radius {
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
					let dx=self.circle.x-scene.palka.x as f32;
					self.direction+=dx as f32/scene.palka.w as f32;
					self.direction=self.direction.max(geometry::PI/6.0*7.0).min(geometry::PI/6.0*11.0);
				},
				circle::Collision::None => ()
			}
		}
	}
}

impl<'a> App<'a> {
	fn render(&mut self, renderer: &mut sdl2::render::WindowCanvas) {
		self.scene.render(renderer);
		self.ball.render(renderer);
		let text="Score: ".to_string()+&(self.score.to_string());
		let img=self.font.render(&text).blended(Color{r:0, g:0, b:0, a:255}).unwrap();
		let creator=renderer.texture_creator();
		let texture=creator.create_texture_from_surface(img).unwrap();
		let tinfo=texture.query();
		renderer.copy(&texture, Rect::new(0, 0,tinfo.width, tinfo.height), Rect::new(0, 0,tinfo.width, tinfo.height)).unwrap();

		renderer.present();
	}

	fn update(&mut self, evt: &sdl2::EventPump) {
		let blocks=self.scene.blocks.len();
		self.ball.update(&mut self.scene);
		if self.scene.blocks.len()!=blocks {
			self.score+=1;
		}
		self.scene.update(evt);
	}

    fn end(&self)->bool {
        self.ball.circle.y+self.ball.circle.radius+1.0>=self.scene.height as f32||self.scene.blocks.len()==0
    }
}

fn main() {
	let sdl=sdl2::init().unwrap();
	let video=sdl.video().unwrap();
	let ttf=sdl2::ttf::init().unwrap();

	let window=video.window("Arkanoid", 1000, 600).build().unwrap();
	let mut renderer=window.into_canvas().build().unwrap();

	let mut app=App {
		font: ttf.load_font("font.ttf", 17).unwrap(),
		ball: Ball{circle: circle::Circle{x: 350.0, y: 200.0, radius: 10.0}, direction: geometry::PI/2.0, speed: 5},
		scene: Scene{blocks: Blocks::new(10, 10, 990, 590, 5, 6), width: 1000, height: 600, palka: Palka{x: 300, y: 580, w: 80}},
		score: 0
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
	if app.end() {
		sdl2::messagebox::show_simple_message_box(sdl2::messagebox::MessageBoxFlag::empty(), "End of game", if app.scene.blocks.len()==0{"You win"}else {"You lose"}, Option::None).unwrap();
	}
}