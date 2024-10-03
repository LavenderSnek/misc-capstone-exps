use crate::rects::border::{AreaHBorder, AreaVBorder, HorizontalBorder, VerticalBorder};
use crate::rects::Rectangle;

pub trait Area {
    fn left(&self) -> i32;
    fn right(&self) -> i32;
    fn top(&self) -> i32;
    fn bottom(&self) -> i32;

    fn left_border(&self) -> impl VerticalBorder;
    fn right_border(&self) -> impl VerticalBorder;
    fn top_border(&self) -> impl HorizontalBorder;
    fn bottom_border(&self) -> impl HorizontalBorder;

    fn d_left(&self) -> i32 { 0 }
    fn d_right(&self) -> i32 { 0 }
    fn d_top(&self) -> i32 { 0 }
    fn d_bottom(&self) -> i32 { 0 }

    fn contains(&self, x: i32, y: i32) -> bool {
        (self.left() <= x) && (x <= self.right())
            && (self.top() <= y) && (y <= self.bottom())
    }
    fn width(&self) -> i32 {
        self.right() - self.left()
    }
    fn height(&self) -> i32 {
        self.bottom() - self.top()
    }
}


// --- rect as a static area

impl Area for Rectangle {
    fn left(&self) -> i32 { self.x }
    fn right(&self) -> i32 { self.x + self.w }
    fn top(&self) -> i32 { self.y }
    fn bottom(&self) -> i32 { self.y + self.h }

    fn left_border(&self) -> impl VerticalBorder { AreaVBorder::left(self) }
    fn right_border(&self) -> impl VerticalBorder { AreaVBorder::right(self) }
    fn top_border(&self) -> impl HorizontalBorder { AreaHBorder::top(self) }
    fn bottom_border(&self) -> impl HorizontalBorder { AreaHBorder::bottom(self) }
}

// --- Moving Area with deltas

pub struct MovingArea {
    cur: Rectangle,
    prev: Rectangle
}

impl MovingArea {
    fn update(&mut self, r: Rectangle) {
        self.prev = self.cur;
        self.cur = r;
    }
}

impl Area for MovingArea {
    fn left(&self) -> i32 { self.cur.left() }
    fn right(&self) -> i32 { self.cur.right() }

    fn top(&self) -> i32 { self.cur.top() }
    fn bottom(&self) -> i32 { self.cur.bottom() }

    fn left_border(&self) -> impl VerticalBorder { AreaVBorder::left(self) }
    fn right_border(&self) -> impl VerticalBorder { AreaVBorder::right(self) }
    fn top_border(&self) -> impl HorizontalBorder { AreaHBorder::top(self) }
    fn bottom_border(&self) -> impl HorizontalBorder { AreaHBorder::bottom(self) }

    fn d_left(&self) -> i32 { self.left() - self.prev.left() }
    fn d_right(&self) -> i32 { self.right()- self.prev.right() }

    fn d_top(&self) -> i32 { self.top() - self.prev.top() }
    fn d_bottom(&self) -> i32 { self.bottom() - self.prev.bottom() }
}
