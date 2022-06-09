use std::fmt::{ Display, Formatter, Result };
use crate::render::*;

#[derive(Debug, Copy, Clone)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Point2 {
    pub fn new(x: i32, y: i32) -> Point2 {
        return Point2{x, y};
    }
    pub fn add(&mut self, x: i32, y: i32) -> &mut Point2 {
        self.x += x;
        self.y += y;
        return self;
    }
}

impl Display for Point2 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // write to output stream
        return write!(f, "Point2({}, {})", self.x, self.y);
    }
}

impl Print for Point2 {
    fn print(&self) {
        // relies on Display trait
        println!("{}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut a = Point2::new(1, 1);
        a.add(1, 1);

        assert_eq!(a.x, 2, "x wrong value");
        assert_eq!(a.y, 2, "y wrong value");
    }
}
