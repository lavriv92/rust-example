extern crate std;
use std::f64::consts::PI;
use super::traits::HasArea;

pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {  
            x: x,
            y: y,
            radius: radius
        }
    }
}

pub struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl Square {
    pub fn new(x: f64, y: f64, side: f64) -> Square {
        Square {
            x: x,
            y: y,
            side: side
        }
    }
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}


pub struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}


impl CircleBuilder {
    pub fn new() -> CircleBuilder {
        CircleBuilder{x: 0.0, y: 0.0, radius: 1.0}
    }

    pub fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    pub fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }

    pub fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    pub fn finalize(&self) -> Circle {
        Circle::new(self.x, self.y, self.radius)
    }
}
