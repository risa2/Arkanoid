extern crate sdl2;

use geometry;
use circle;

use sdl2::rect::Point;
use std::f32;
use std::result::Result;

fn horizontal(line: (Point, Point))->bool {
	line.0.x==line.1.x
}

pub fn triangle<T: sdl2::render::RenderTarget>(out: &mut sdl2::render::Canvas<T>, p: [Point; 3]) {
	let highest=p.iter().fold(Point::new(0x7fffffff, 0x7fffffff), |state, &p| {
		if p.y<state.y {p} else {state}
	});
	let left=(highest, p.iter().fold(Point::new(0x7fffffff, 0x7fffffff), |state, &p| {
		if p.x<state.x && p!=highest {p} else {state}
	}));
	let right=(highest, p.iter().fold(Point::new(0x7fffffff, 0x7fffffff), |state, &p| {
		if p!=left.1 && p!=highest {p} else {state}
	}));

	let l_k=(left.0.x-left.1.x) as f32/(left.0.y-left.1.y) as f32;
	let r_k=(right.0.x-right.1.x) as f32/(right.0.y-right.1.y) as f32;
	let l_p=left.0.x as f32-left.0.y as f32*l_k;
	let r_p=right.0.x as f32-right.0.y as f32*l_k;
	for y in highest.y..left.1.y.max(right.1.y)+1 {
		for x in (y as f32*l_k+l_p) as i32..(y as f32*r_k+r_p) as i32+1{
			out.draw_point(Point::new(x,y));
		}
	}
}

pub fn circle<T: sdl2::render::RenderTarget>(out: &mut sdl2::render::Canvas<T>, circle: circle::Circle, detail: f32)->Result<(), String>{
	let mut angle: f32=0.0;
	let mut res: Result<(), String> =Result::Ok(());
	while angle<2.0*geometry::PI {
		let (dx, dy)=geometry::to_cartesian(circle.radius, angle);
		let err=out.draw_point(Point::new((circle.x+dx) as i32, (circle.y+dy) as i32));
		res=res.and(err);
		angle+=detail;
	}
	res
}