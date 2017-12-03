extern crate sdl2;

use sdl2::rect::Point;
use std::f32;
use std::result::Result;

pub fn circle<T: sdl2::render::RenderTarget>(out: &mut sdl2::render::Canvas<T>, center: Point, radius: f32, detail: f32){
	let mut angle: f32=0.0;
	let mut res: Result<(), String> =Result::Ok(());
	while angle<2.0*3.14159265358 {
		let err=out.draw_point(Point::new(center.x+(radius*angle.cos()) as i32, center.y+(radius*angle.sin()) as i32));
		res=res.and(err);
		angle+=detail;
	}
}