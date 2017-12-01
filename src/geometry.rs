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