extern crate sdl2;
extern crate rand;

use geometry;
use circle;

use rand::Rng;
use std::time;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub trait GameObject {
	fn update(&mut self, scene: &mut Scene) {}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {}
	fn to_rect(&self)-> Rect {
		Rect::new(0,0,0,0)
	}
	fn exists(&self, scene: &Scene)->bool {
		true
	}
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

pub trait Bonus: GameObject {
	fn activate(&self, scene: &mut Scene);
}

pub struct Scene {
	pub width: i32,
	pub height: i32,
	pub objects: ObjectList,
	pub evt: sdl2::EventPump,
	pub rand: rand::ThreadRng
}

#[derive(Copy, Clone)]
pub struct Block {
	color: Color,
	pos: sdl2::rect::Rect
}

#[derive(Copy, Clone)]
pub struct Ball {
	circle: circle::Circle,
	direction: f32,
	speed: i32
}

#[derive(Copy, Clone)]
pub struct Palka {
	pos: sdl2::rect::Rect
}

#[derive(Copy, Clone)]
pub struct Watcher {
	start: time::SystemTime,
	destroy: bool
}

#[derive(Copy, Clone)]
pub struct NewBallBonus {
	pos: sdl2::rect::Rect
}

#[derive(Copy, Clone)]
pub struct GrowBonus {
	pos: sdl2::rect::Rect
}

impl Block {
	fn new(col: Color, pos: sdl2::rect::Point, size: sdl2::rect::Point)->Block {
		Block{color: col, pos: sdl2::rect::Rect::new(pos.x, pos.y, size.x as u32, size.y as u32)}
	}
}

impl Watcher {
	fn new()->Watcher {
		Watcher{start: time::SystemTime::now(), destroy: false}
	}
}


impl GameObject for Watcher {
	fn update(&mut self, scene: &mut Scene) {
		if time::SystemTime::now().duration_since(self.start).unwrap()>time::Duration::from_millis(10000) {
			let mut i=0;
			while i<scene.objects.len() {
				if scene.objects[i].is_palka().is_some() {
					let mut palka=*scene.objects.remove(i).is_palka().unwrap();
					palka.pos.w-=43;
					scene.objects.insert(i, Box::new(palka));
					self.destroy=true;
					break;
				}
				i+=1;
			}
		}
	}
	fn exists(&self, scene: &Scene)->bool {
		!self.destroy
	}
}

impl GameObject for Block {
	fn to_rect(&self)->Rect {
		self.pos
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(self.color);
		renderer.fill_rect(self.to_rect()).unwrap();
	}
	fn is_block(&self)->Option<&Block> {
		Option::Some(self)
	}
}

impl NewBallBonus {
	fn new(pos: sdl2::rect::Rect)->NewBallBonus {
		NewBallBonus{pos: pos}
	}
}

impl GameObject for NewBallBonus {
	fn to_rect(&self) -> Rect {
		self.pos
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(200, 100, 0));
		renderer.fill_rect(self.pos).unwrap();
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
		scene.objects.push(new!(Ball; scene.width/2, scene.height/2, 10, 7));
	}
}

impl GrowBonus {
	fn new(pos: sdl2::rect::Rect)->GrowBonus {
		GrowBonus{pos: pos}
	}
}

impl GameObject for GrowBonus {
	fn to_rect(&self) -> Rect {
		self.pos
	}
	fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(200, 0, 100));
		renderer.fill_rect(self.pos).unwrap();
	}
	fn update(&mut self, scene: &mut Scene) {
		self.pos.y+=1;
	}
	fn is_bonus(&self)->Option<&Bonus> {
		Option::Some(self)
	}
}

impl Bonus for GrowBonus {
	fn activate(&self, scene: &mut Scene) {
		let mut i=0;
		while i<scene.objects.len() {
			if scene.objects[i].is_palka().is_some() {
				let mut palka=*scene.objects.remove(i).is_palka().unwrap();
				palka.pos.w+=40;
				scene.objects.insert(i, Box::new(palka));
				scene.objects.push(new!(Watcher;));
				break;
			}
			i+=1;
		}
	}
}

pub fn make_blocks(pos: sdl2::rect::Rect, count: sdl2::rect::Point, size: sdl2::rect::Point)->Vec<Box<GameObject>> {
	let mut blocks: Vec<Box<GameObject>>=vec![];
	for y in 0..count.y {
		for x in 0..count.x {
			let dst=pos.top_left()+sdl2::rect::Point::new(geometry::split(pos.w, count.x, x), geometry::split(pos.h, count.y, y));
			blocks.push(new!(Block; Color::RGB((x%2*255) as u8, (y%2*255) as u8, 0), dst, size));
		}
	}
	blocks
}

impl Scene {
	pub fn render(&self, renderer: &mut sdl2::render::WindowCanvas) {
		renderer.set_draw_color(Color::RGB(0,200,0));
		renderer.clear();

		for object in &self.objects {
			object.render(renderer);
		}
	}
	pub fn update(&mut self) {
		let mut i=0;
		while i<self.objects.len() {
			let mut obj=self.objects.remove(i);
			obj.update(self);
			if obj.exists(self) {
				self.objects.insert(0, obj);
				i+=1;
			}
		}
	}
}

impl Palka {
	pub fn new(x: i32, y: i32, w: u32, h: u32)->Palka {
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
		with!(scene.evt.keyboard_state()=>kb; {
			if kb.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
				self.pos.x=(self.pos.x-8).max(0)
			}
			if kb.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
				self.pos.x=(self.pos.x+8).min(scene.width-self.pos.w)
			}
		});

		let bonuses=with!(ObjectList::new()=>mut tmp; {
			let objects=&mut scene.objects;
			let mut i=0;
			while i<objects.len() {
				if self.pos.has_intersection(objects[i].to_rect())&&objects[i].is_bonus().is_some() {
					let object=objects.remove(i);
					tmp.push(object);
				}
				else {
					i+=1;
				}
			}
			tmp
		});
		scene.objects.push(Box::new(*self));
		for bonus in bonuses {
			bonus.is_bonus().unwrap().activate(scene);
		}
		let i=scene.objects.iter().position(|ref x|x.is_palka().is_some()).unwrap();
		*self=*scene.objects.remove(i).is_palka().unwrap();
	}
}

impl Ball {
	pub fn new(x: i32, y: i32, radius: i32, speed: i32)->Ball {
		Ball{circle: circle::Circle{x: x as f32, y: y as f32, radius: radius as f32}, direction: geometry::to_rad(90.0), speed: speed}
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

	fn exists(&self, scene: &Scene)->bool {
		self.to_rect().bottom()<=scene.height-1
	}
	fn update(&mut self, scene: &mut Scene) {
		for _ in 0..self.speed {
			let (dx, dy)=geometry::to_cartesian(1.0, self.direction);
			self.circle.x+=dx;
			self.circle.y+=dy;

			if self.circle.x>scene.width as f32-self.circle.radius && (self.direction<geometry::to_rad(90.0)||self.direction>geometry::to_rad(270.0)) {
				self.direction=geometry::bounce(self.direction, geometry::to_rad(180.0));
			}
			if self.circle.x<self.circle.radius && self.direction>geometry::to_rad(90.0)&&self.direction<geometry::to_rad(270.0) {
				self.direction=geometry::bounce(self.direction, geometry::to_rad(0.0));
			}

			if self.circle.y<self.circle.radius && self.direction>geometry::to_rad(180.0)&&self.direction<geometry::to_rad(360.0) {
				self.direction=geometry::bounce(self.direction, geometry::to_rad(90.0));
			}

			let objects=&mut scene.objects;
			let mut i=0;
			while i<objects.len() {
				if let circle::Collision::At(x, y)=self.circle.collision(objects[i].to_rect()) {
					if objects[i].is_block().is_some() {
						self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
						let object=objects.remove(i);
						if scene.rand.next_f32()<0.2 {
							if scene.rand.next_f32()<0.5 {
								objects.push(new!(NewBallBonus; object.to_rect()));
							}
							else {
								objects.push(new!(GrowBonus; object.to_rect()));
							}
						}
						break;
					}
					else if objects[i].is_palka().is_some() {
						self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
						let dx=self.circle.x-objects[i].to_rect().center().x as f32;
						self.direction+=dx as f32/objects[i].to_rect().w as f32;
						if self.direction<geometry::to_rad(90.0) {
							self.direction=geometry::to_rad(190.0);
						}
						self.direction=self.direction.max(geometry::to_rad(190.0)).min(geometry::to_rad(350.0));
					}
					else if objects[i].is_ball().is_some() {
						let ball=*objects.remove(i).is_ball().unwrap();
						if let circle::Collision::At(x, y)=self.circle.circle_collision(ball.circle) {
							self.direction=geometry::bounce(self.direction, geometry::line_angle((x, y), self.circle.center()));
						}
						objects.insert(i, Box::new(ball));
					}
				}
				i+=1;
			}
		}
	}
}