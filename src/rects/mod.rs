mod border;
mod area;

pub use crate::rects::border::{Border, HorizontalBorder, VerticalBorder};
pub use crate::rects::area::{Area, MovingArea};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rectangle {
    pub fn zero() -> Self {
        Self { x: 0, y: 0, w:0, h:0 }
    }
}




