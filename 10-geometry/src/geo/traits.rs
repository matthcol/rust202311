pub trait Mesurable1d {
    fn length(&self) -> f64;
}

pub trait Mesurable2d {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
}