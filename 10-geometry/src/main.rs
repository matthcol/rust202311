use crate::geo::model::{Point, Segment, Circle};

mod geo;

#[cfg(test)]
mod tests;

fn play_with_forms() {
    let p1 = Point(2.5, 4.5);
    let p2 = Point(5.5, 0.5);
    let s = Segment(p1, p2);
    let c = Circle{
        center: p1, 
        radius: 10.0
    };
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:#?}", s);
    println!("{:#?}", c);
}

fn main() {
    play_with_forms();
}
