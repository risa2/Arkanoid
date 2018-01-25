extern crate sdl2;

mod geometry;
mod circle;

use std::f32;

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

type ObjectList=Vec<Box<GameObject>>;

struct Scene {
	width: i32,
	height: i32,
	objects: ObjectList,
	evt: sdl2::EventPump
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

struct NewBallBonus {
	circle: circle::Circle
}

struct App <'a> {
	font: sdl2::ttf::Font<'a, 'static>,
	scene: Scene,
	score: i32
}

trait GameObject {
	fn update(&mut self, scene: &mut Scene) {}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas);
	fn to_rect(&self) -> Rect;
	fn is_block(&self)->Option<&Block> {
		Option::None
	}
	fn is_palka(&self)->Option<&Palka> {
		Option::None
	}
	fn is_ball(&self)->Option<&Ball> {
		Option::None
	}
}

impl Block {
	const WIDTH: i32 = 100;
	const HEIGHT: i32 = 40;
}
impl GameObject for Block {
	fn to_rect(&self)->Rect {
		Rect::new(self.x, self.y, Block::WIDTH as u32, Block::HEIGHT as u32)
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(self.color);
		renderer.fill_rect(self.to_rect()).unwrap();
	}
	fn is_block(&self)->Option<&Block> {
		Option::Some(self)
	}
}

impl GameObject for NewBallBonus {
	fn to_rect(&self) -> Rect {
		self.circle.to_rect()
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		self.circle.render(renderer, Color::RGB(255, 128, 0));
	}
	fn update(&mut self, scene: &mut Scene) {
		self.circle.y-=1;
		match self.circle.collision(scene.palka.to_rect()) {
			circle::Collision::At(x, y) => {

			},
			circle::Collision::None => ()
		}
	}
}

fn make_blocks(left: u32, top: u32, width: u32, height: u32, x_count: u32, y_count: u32)->ObjectList {
	let mut blocks: ObjectList=vec![];
	for y in 0..y_count {
		for x in 0..x_count {
			let (dst_x, dst_y)=(left+geometry::split(width, x_count, x), top+geometry::split(height, y_count, y));
			blocks.push(Box::new(Block{color: Color::RGB((x%2*255) as u8, ((x+1)%2*255) as u8, 0), x: dst_x as i32, y: dst_y as i32}));
		}
	}
	blocks
}

impl Scene {
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(0,200,0));
		renderer.clear();

		for object in self.objects {
			object.render(renderer);
		}
	}
	fn update(&mut self) {
		for object in self.objects {
			object.render(self);
		}
	}
}
impl Palka {
	const HEIGHT: i32 = 10;
}
impl GameObject for Palka {
	fn to_rect(&self) -> Rect {
		Rect::new(self.x - self.w / 2, self.y, self.w as u32, Palka::HEIGHT as u32)
	}
	fn is_palka(&self)->Option<&Palka> {
		Option::Some(self)
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(0,0,0));
		renderer.fill_rect(self.to_rect()).unwrap();
	}
	fn update(&mut self, scene: &mut Scene) {
		let kb=scene.evt.keyboard_state();
		let shift=5;
		if kb.is_scancode_pressed(keyboard::Scancode::Left) {
			self.x=if self.x-self.w/2<shift {self.w/2} else {self.x-shift}
		}
		if kb.is_scancode_pressed(keyboard::Scancode::Right) {
            self.x=if self.x>scene.width-shift-self.w/2 {scene.width-self.w/2} else {self.x+shift}
		}
	}
}

impl GameObject for Ball {
	fn to_rect(&self) -> Rect {
		self.circle.to_rect()
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		self.circle.render(renderer, Color::RGB(0, 0, 255));
	}
	fn is_ball(&self)->Option<&Ball> {
		Option::Some(self)
	}

	fn update(&mut self, scene: &mut Scene) {
		for i in 0..self.speed {
			let (dx, dy)=geometry::to_cartesian(1.0, self.direction);
			self.circle.x+=dx;
			self.circle.y+=dy;

			if self.circle.x>scene.width as f32-self.circle.radius || self.circle.x<self.circle.radius {
				self.direction=geometry::horizontal_bounce(self.direction);
			}

			if self.circle.y>scene.height as f32-self.circle.radius || self.circle.y<self.circle.radius {
				self.direction=geometry::vertical_bounce(self.direction);
			}


			let objects=&mut scene.objects;
			for i in 0..objects.len() {
				match self.circle.collision(objects[i].to_rect()) {
					circle::Collision::At(x, y) => {
						if let Some(block)=objects[i].is_block() {
							self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
							objects.remove(i);
						}
						if let Some(palka)=objects[i].is_palka() {
							self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
							let dx=self.circle.x-scene.palka.x as f32;
							self.direction+=dx as f32/scene.palka.w as f32;
							self.direction=self.direction.max(geometry::PI/6.0*7.0).min(geometry::PI/6.0*11.0);
						}
					},
					circle::Collision::None => ()
				};
			}
		}
	}
}

impl<'a> App<'a> {
	fn render_text(&self, renderer: &mut sdl2::render::WindowCanvas, text: &str) {
		let img=self.font.render(&text).blended(Color::RGB(0, 0, 0)).unwrap();
		let creator=renderer.texture_creator();
		let texture=creator.create_texture_from_surface(img).unwrap();
		let t_info=texture.query();
		renderer.copy(&texture, Rect::new(0, 0,t_info.width, t_info.height), Rect::new(0, 0,t_info.width, t_info.height)).unwrap();
	}
	fn render(&mut self, renderer: &mut sdl2::render::WindowCanvas) {
		self.scene.render(renderer);
		self.render_text(renderer, "Score: ".to_string()+&(self.score.to_string()));

		renderer.present();
	}

	fn update(&mut self) {
		let blocks=self.scene.blocks.len();
		self.ball.update(&mut self.scene);
		if self.scene.blocks.len()!=blocks {
			self.score+=1;
		}
		self.scene.update();
	}

	fn lose(&self) {
		self.scene.objects.find(|&x|x.is_ball().is_some()).to_rect().bottom()>=self.scene.height as f32-1.0
	}
	fn win(&self) {
		!self.scene.objects.any(|&x|x.is_block().is_some())
	}
    fn end(&self)->bool {
        self.lose()||self.win()
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
		scene: Scene{objects: make_blocks(10, 10, 990, 590, 5, 6)
			.extend([Palka{x: 300, y: 580, w: 80}, Ball{circle: circle::Circle{x: 350.0, y: 200.0, radius: 10.0}, direction: geometry::PI/2.0, speed: 5}]),
				width: 1000, height: 600, evt: sdl.event_pump().unwrap()},
		score: 0
	};

	'main: while !app.end() {
		for event in app.scene.evt.poll_iter() {
			match event {
				event::Event::Quit{timestamp: _t} => {break 'main},
				_ => ()
			}
		}
		app.update();
		app.render(&mut renderer);
		std::thread::sleep(std::time::Duration::from_millis(20));
	}
	if app.end() {
		sdl2::messagebox::show_simple_message_box(sdl2::messagebox::MessageBoxFlag::empty(), "End of game", if app.win() {"You win"}else {"You lose"}, Option::None).unwrap();
	}
}