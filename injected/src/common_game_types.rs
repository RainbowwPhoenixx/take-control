#![allow(unused)]

// Physics stuff

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

// STRING STUFF

#[repr(C)]
pub struct InplaceString<const N: usize> {
    base: ControlString,
    chars: [u8; N],
}

impl<const N: usize> InplaceString<N> {
    pub fn new() -> Self {
        let mut res = Self {
            base: ControlString::new(),
            chars: [0; N],
        };

        res.base.data = &mut res.chars as *mut u8;
        res.base.capacity = N as u32;
        res.base.len = N as u32;

        res
    }

    pub fn get(&self) -> String {
        (&self.base).into()
    }
}

#[repr(C)]
pub struct ControlString {
    unk: u64,
    data: *mut u8,
    unk2: u32,
    capacity: u32,
    len: u32,
}

impl ControlString {
    pub fn new() -> Self {
        Self {
            unk: 0,
            data: std::ptr::null_mut(),
            unk2: 0,
            capacity: 0,
            len: 0,
        }
    }
}

impl Into<String> for &ControlString {
    fn into(self) -> String {
        let slice = unsafe { std::slice::from_raw_parts(self.data, self.len as usize) };
        String::from_utf8_lossy(slice).into()
    }
}
