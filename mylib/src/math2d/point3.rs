use std::fmt::{ Display, Formatter, Result };
use crate::render::*;

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    pub fn new(x: i32, y: i32, z: i32) -> Point3 {
        return Point3{x, y, z};
    }
    pub fn add(&mut self, x: i32, y: i32, z: i32) -> &mut Point3 {
        self.x += x;
        self.y += y;
        self.z += z;
        return self;
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // write to output stream
        return write!(f, "Point3({}, {}, {})", self.x, self.y, self.z);
    }
}

impl Print for Point3 {
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
        let mut a = Point3::new(1, 1, 1);
        a.add(1, 1, 1);

        assert_eq!(a.x, 2, "x wrong value");
        assert_eq!(a.y, 2, "y wrong value");
        assert_eq!(a.z, 2, "z wrong value");
    }
}
