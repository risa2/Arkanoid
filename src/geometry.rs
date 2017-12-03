use std::f32;

pub const PI:f32=3.14159265358;
pub fn split(lenght: u32, width: u32, index: u32)->u32 {
    index*lenght/width
}
pub fn to_cartesian(lenght: f32, angle: f32)->(f32, f32) {
    (lenght*angle.cos(), lenght*angle.sin())
}

pub fn distance(a: (f32,f32), b:(f32,f32))->f32 {
    ((a.0-b.0)*(a.0-b.0)+(a.1-b.1)*(a.1-b.1)).sqrt()
}

pub fn horizontal_bounce(angle: f32)->f32 {
    if angle<=PI {PI-angle} else {PI-angle+2.0*PI}
}
pub fn vertical_bounce(angle: f32)->f32 {
    2.0*PI-angle
}
pub fn bounce(move_angle: f32, kolmice: f32)->f32 {
    if kolmice-(move_angle-kolmice)<=PI {kolmice-(move_angle-kolmice)+PI} else {kolmice-(move_angle-kolmice)-PI}
}

pub fn line_angle(begin: (f32, f32), end: (f32, f32))->f32 {
	if (end.1-begin.1).atan2(end.0-begin.0)<0.0 {2.0*PI+(end.1-begin.1).atan2(end.0-begin.0)} else {(end.1-begin.1).atan2(end.0-begin.0)}
}


fn eq_float(a: f32, b: f32)->bool {
	(a-b).abs()<0.001
}

#[test]
fn test_horizontal_bounce() {
	assert!(eq_float(horizontal_bounce(PI/4.0), PI*3.0/4.0));
	assert!(eq_float(horizontal_bounce(PI*7.0/4.0), PI*5.0/4.0));
	
	assert!(eq_float(horizontal_bounce(PI*3.0/4.0), PI/4.0));
	assert!(eq_float(horizontal_bounce(PI*5.0/4.0), PI*7.0/4.0));
}

#[test]
fn test_vertical_bounce() {
	assert!(eq_float(vertical_bounce(PI/4.0), PI*7.0/4.0));
	assert!(eq_float(vertical_bounce(PI*7.0/4.0), PI/4.0));
	
	assert!(eq_float(vertical_bounce(PI*3.0/4.0), PI*5.0/4.0));
	assert!(eq_float(vertical_bounce(PI*5.0/4.0), PI*3.0/4.0));
}

#[test]
fn test_line_angle() {
	assert!(eq_float(line_angle((1.0, 1.0), (0.0, 0.0)), PI*5.0/4.0));
	assert!(eq_float(line_angle((0.0, 0.0), (1.0, 1.0)), PI/4.0));
	assert!(eq_float(line_angle((0.0, 0.0), (0.0, 1.0)), PI/2.0));
	assert!(eq_float(line_angle((0.0, 0.0), (1.0, 0.0)), 0.0));
}

#[test]
fn test_bounce() {
	assert!(eq_float(bounce(PI, 0.0), 0.0));
	assert!(eq_float(bounce(0.0, PI*3.0/4.0), PI/2.0));
	assert!(eq_float(bounce(PI*3.0/2.0, PI/4.0), 0.0));
	assert!(eq_float(bounce(PI*3.0/4.0, 0.0), PI/4.0));
	assert!(eq_float(bounce(PI*5.0/4.0, PI/2.0), PI*3.0/4.0));
}