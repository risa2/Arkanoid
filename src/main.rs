extern crate sdl2;
extern crate rand;

mod geometry;
mod circle;
#[macro_use]
mod objects;

use objects::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event;

struct App <'a> {
	font: sdl2::ttf::Font<'a, 'static>,
	scene: Scene,
	score: i32
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
		self.scene.objects.iter().all(|ref x|x.is_ball().is_none())
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
	
	let mut app=App {
		font: ttf.load_font("font.ttf", 17).unwrap(),
		scene: Scene{width: 1000, height: 600, evt: sdl.event_pump().unwrap(), rand: rand::thread_rng(),
			objects: append!(make_blocks(Rect::new(10, 10, 990, 390), sdl2::rect::Point::new(15, 10), sdl2::rect::Point::new(50, 20));
				new!(Palka; 350, 590, 80, 10) as Box<GameObject>, new!(Ball; 350, 500, 10, 6) as Box<GameObject>),
		},
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