use std::f64::consts::PI;

use super::traits::{Mesurable1d, Mesurable2d};

#[derive(Debug, Copy, Clone)]
pub struct Point(pub f64, pub f64);

#[derive(Debug)]
pub struct Segment(pub Point, pub Point);

#[derive(Debug)]
pub struct Circle{
    pub center: Point,
    pub radius: f64
}

#[derive(Debug)]
pub struct Polygon{
    pub summits: Vec<Point>
}


impl Mesurable1d for Segment {
    fn length(&self) -> f64 {
        let delta_x = self.0.0 - self.1.0;
        let delta_y = self.0.1 - self.1.1;
        delta_x.hypot(delta_y)
    }
}

impl Mesurable2d for Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}