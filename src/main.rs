extern crate sdl2;
extern crate rand;

mod geometry;
mod circle;

use std::f32;
use rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event;

trait GameObject {
	fn update(&mut self, scene: &mut Scene) {}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas);
	fn to_rect(&self)-> Rect;
	fn is_block(&self)->Option<&Block> {
		Option::None
	}
	fn is_ball(&self)->Option<&Ball> {
		Option::None
	}
	fn is_palka(&self)->Option<&Palka> {
		Option::None
	}
	fn is_bonus(&self)->Option<&Bonus>{
		Option::None
	}
}

type ObjectList=Vec<Box<GameObject>>;

trait Bonus: GameObject {
	fn activate(&self, scene: &mut Scene);
}

struct Scene {
	width: i32,
	height: i32,
	objects: ObjectList,
	evt: sdl2::EventPump,
	rand: rand::ThreadRng
}

#[derive(Copy, Clone)]
struct Block {
	color: Color,
	x: i32,
	y: i32
}

#[derive(Copy, Clone)]
struct Ball {
	circle: circle::Circle,
	direction: f32,
	speed: i32
}

#[derive(Copy, Clone)]
struct Palka {
	pos: sdl2::rect::Rect
}

#[derive(Copy, Clone)]
struct NewBallBonus {
	pos: sdl2::rect::Rect
}

struct App <'a> {
	font: sdl2::ttf::Font<'a, 'static>,
	scene: Scene,
	score: i32
}

impl Block {
	const WIDTH: i32 = 50;
	const HEIGHT: i32 = 20;
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
		self.pos
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(200, 100, 0));
		renderer.fill_rect(self.pos);
	}
	fn update(&mut self, scene: &mut Scene) {
		self.pos.y+=1;
	}
	fn is_bonus(&self)->Option<&Bonus> {
		Option::Some(self)
	}
}

impl Bonus for NewBallBonus {
	fn activate(&self, scene: &mut Scene) {
		scene.objects.push(Box::new(Ball::new(scene.width/2, scene.height/2, 10, 7)));
	}
}

fn make_blocks(left: u32, top: u32, width: u32, height: u32, x_count: u32, y_count: u32)->Vec<Box<GameObject>> {
	let mut blocks: Vec<Box<GameObject>>=vec![];
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

		for object in &self.objects {
			object.render(renderer);
		}
	}
	fn update(&mut self) {
		let mut i=0;
		while i<self.objects.len() {
			let mut obj=self.objects.remove(i);
			obj.update(self);
			self.objects.insert(0, obj);
			i+=1;
		}
	}
}

impl Palka {
	fn new(x: i32, y: i32, w: u32, h: u32)->Palka {
		Palka{pos: Rect::new(x, y, w, h)}
	}
}

impl GameObject for Palka {
	fn to_rect(&self) -> Rect {
		self.pos
	}
	fn is_palka(&self)->Option<&Palka> {
		Option::Some(self)
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(0,0,0));
		renderer.fill_rect(self.to_rect()).unwrap();
	}
	fn update(&mut self, scene: &mut Scene) {
		{
			let kb=scene.evt.keyboard_state();
			if kb.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
				self.pos.x=(self.pos.x-10).max(0)
			}
			if kb.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
				self.pos.x=(self.pos.x+10).min(scene.width-self.pos.w)
			}
		}

		let mut bonuses: ObjectList=vec![];
		{
			let objects=&mut scene.objects;
			let mut i=0;
			while i<objects.len() {
				if self.pos.has_intersection(objects[i].to_rect())&&objects[i].is_bonus().is_some() {
					let object=objects.remove(i);
					bonuses.push(object);
					i-=1;
				}
				i+=1;
			}
		}
		for bonus in bonuses {
			bonus.is_bonus().unwrap().activate(scene);
		}
	}
}

impl Ball {
	fn new(x: i32, y: i32, radius: i32, speed: i32)->Ball {
		Ball{circle: circle::Circle{x: x as f32, y: y as f32, radius: radius as f32}, direction: geometry::PI/2.0, speed: speed}
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
		for _ in 0..self.speed {
			let (dx, dy)=geometry::to_cartesian(1.0, self.direction);
			self.circle.x+=dx;
			self.circle.y+=dy;

			if self.circle.x>scene.width as f32-self.circle.radius || self.circle.x<self.circle.radius {
				self.direction=geometry::horizontal_bounce(self.direction);
			}

			if self.circle.y<self.circle.radius {
				self.direction=geometry::vertical_bounce(self.direction);
			}


			let objects=&mut scene.objects;
			let mut i=0;
			while i<objects.len() {
				match self.circle.collision(objects[i].to_rect()) {
					circle::Collision::At(x, y) => {
						if objects[i].is_block().is_some() {
							self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
							let object=objects.remove(i);
							if scene.rand.next_f32()<0.1 {
								objects.push(Box::new(NewBallBonus{pos: object.to_rect()}));
							}
							break;
						}
						else if objects[i].is_palka().is_some() {
							self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
							let dx=self.circle.x-objects[i].to_rect().center().x as f32;
							self.direction+=dx as f32/objects[i].to_rect().w as f32;
							self.direction=self.direction.max(geometry::PI/8.0*9.0).min(geometry::PI/8.0*15.0);
						}
					},
					circle::Collision::None => ()
				};
				if objects[i].is_ball().is_some() {
					let ball=*objects.remove(i).is_ball().unwrap();
					match self.circle.circle_collision(ball.circle) {
						circle::Collision::At(x, y) => {
							self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
						},
						circle::Collision::None => ()
					}
					objects.insert(i, Box::new(ball));
				}
				i+=1;
			}
		}
	}
}

impl<'a> App<'a> {
	fn render_text(&self, renderer: &mut sdl2::render::WindowCanvas, text: String) {
		let img=self.font.render(&text).blended(Color::RGB(0, 0, 0)).unwrap();
		let creator=renderer.texture_creator();
		let texture=creator.create_texture_from_surface(img).unwrap();
		let t_info=texture.query();
		renderer.copy(&texture, Rect::new(0,0, t_info.width, t_info.height), Rect::new(0, 0,t_info.width, t_info.height)).unwrap();
	}
	fn render(&mut self, renderer: &mut sdl2::render::WindowCanvas) {
		self.scene.render(renderer);
		self.render_text(renderer, format!("Score: {}", self.score));

		renderer.present();
	}

	fn update(&mut self) {
		let blocks=self.scene.objects.iter().filter(|&x|x.is_block().is_some()).count() as i32;
		self.scene.update();
		self.score+=blocks-self.scene.objects.iter().filter(|&x|x.is_block().is_some()).count() as i32;
	}

	fn lose(&self)->bool {
		self.scene.objects.iter().any(|ref x|x.is_ball().is_some()&&x.to_rect().bottom()>=self.scene.height-1)
	}
	fn win(&self)->bool {
		!self.scene.objects.iter().any(|ref x|x.is_block().is_some())
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

	let mut tmp_vec=make_blocks(10, 10, 990, 390, 15, 10);
	tmp_vec.push(Box::new(Palka::new(300, 590, 80, 10)) as Box<GameObject>);
	tmp_vec.push(Box::new(Ball::new(350, 500, 10, 6)) as Box<GameObject>);
	let mut app=App {
		font: ttf.load_font("font.ttf", 17).unwrap(),
		scene: Scene{objects: tmp_vec, width: 1000, height: 600, evt: sdl.event_pump().unwrap(), rand: rand::thread_rng()},
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