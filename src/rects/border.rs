use crate::rects::area::Area;
use crate::rects::Point;

pub trait Border {
    fn on(&self, p: &Point) -> bool;
    fn translate(&self, p: Point) -> Point { p }
}

//--- Vertical borders (Wall)

pub trait VerticalBorder: Border {
    fn is_right(&self) -> bool;
    fn x(&self) -> i32;
    fn dx(&self) -> i32;
    fn height(&self) -> i32;
}

pub(crate) struct AreaVBorder<'a, T> where T: Area {
    area: &'a T,
    right: bool,
}

impl<'a, T> AreaVBorder<'a, T> where T: Area {
    pub(crate) fn left(a: &'a T) -> Self {
        Self { area: a, right: false }
    }
    pub(crate) fn right(a: &'a T) -> Self {
        Self { area: a, right: true }
    }
}


impl<'a, T> Border for AreaVBorder<'a, T> where T: Area {
    fn on(&self, p: &Point) -> bool {
        (self.x() == p.x) && ((self.area.top() <= p.y) && (p.y <= self.area.bottom()))
    }
}

impl<'a, T> VerticalBorder for AreaVBorder<'a, T> where T: Area {
    fn is_right(&self) -> bool { self.right }
    fn x(&self) -> i32 { if self.right { self.area.right() } else { self.area.left() } }
    fn dx(&self) -> i32 { if self.right { self.area.d_right() } else { self.area.d_left() } }
    fn height(&self) -> i32 { self.area.height() }
}

//--- Horizontal borders (FloorCeiling)

pub trait HorizontalBorder: Border {
    fn is_bottom(&self) -> bool;
    fn y(&self) -> i32;
    fn dy(&self) -> i32;
    fn width(&self) -> i32;
}

pub(crate) struct AreaHBorder<'a, T> where T: Area {
    area: &'a T,
    bottom: bool,
}

impl<'a, T> AreaHBorder<'a, T> where T: Area {
    pub(crate) fn top(a: &'a T) -> Self {
        Self { area: a, bottom: false }
    }
    pub(crate) fn bottom(a: &'a T) -> Self {
        Self { area: a, bottom: true }
    }
}

impl<'a, T> Border for AreaHBorder<'a, T> where T: Area {
    fn on(&self, p: &Point) -> bool {
        (self.y() == p.y) && ((self.area.left() <= p.x) && (p.x <= self.area.right()))
    }
}

impl<'a, T> HorizontalBorder for AreaHBorder<'a, T> where T: Area {
    fn is_bottom(&self) -> bool { self.bottom }
    fn y(&self) -> i32 { if self.bottom { self.area.bottom() } else { self.area.top() } }
    fn dy(&self) -> i32 { if self.bottom { self.area.d_bottom() } else { self.area.d_top() } }
    fn width(&self) -> i32 { self.area.width() }
}
