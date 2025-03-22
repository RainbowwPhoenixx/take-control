#[allow(unused)]

#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
#[repr(C)]
pub struct SIMDTransform {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub e: f32,
    pub f: f32,
    pub g: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub k: f32,
}