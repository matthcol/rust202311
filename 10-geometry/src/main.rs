use geo::traits::Mesurable2d;

use crate::geo::{model::{Point, Segment, Circle}, generic::GenPoint};

mod geo;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

fn play_with_forms() {
    println!("*** play with forms ***");
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
    println!()
}

fn play_with_generic_forms() {
    println!("*** play with forms ***");
    let p1 = GenPoint(12, 7);
    let p2 = GenPoint(12u8, 7u8);
    let p3: GenPoint<u32> = GenPoint(4_000_000_000, 2);
    let p4: GenPoint<_> = GenPoint(4_000_000_000u32, 2u32);
    println!("{p1:?} ; {p2:?} ; {p3:?}; {p4:?}");
    let x = p1.0 + p1.1;
    println!("x = {x}");
    let x = p1.sum();
    let y = p1.sub();
    println!("x = {x} ; y = {y}");
    println!()
}

fn do_something_with_mesurable2d(form: &dyn Mesurable2d) {
    let a = form.area();
    let p = form.perimeter();
    println!("area = {a} ; perimeter = {p}")
}

fn do_something_with_mesurable2d_alt<F>(form: &F) where 
    F: Mesurable2d
{
    let a = form.area();
    let p = form.perimeter();
    println!("area = {a} ; perimeter = {p}")
}

fn circles_total_area(circles: &Vec<Circle>) -> f64 {
    circles.iter()
        .map(Circle::area)
        .sum()
}

fn circles_total_area_it<'a, I>(mut it_circles: I) -> f64 where 
    I : Iterator<Item=&'a Circle>
{
    it_circles
        .map(Circle::area)
        .sum()
}

fn mesurables2d_total_area<T>(mesurables: &Vec<T>) -> f64 where 
    T: Mesurable2d
{
    mesurables.iter()
        .map(Mesurable2d::area)
        .sum()
}

fn play_with_mesurable2d() {
    println!("*** play with mesurable2d ***");
    let p1 = Point(2.5, 4.5);
    let c = Circle{
        center: p1, 
        radius: 10.0
    };
    do_something_with_mesurable2d(&c);
    do_something_with_mesurable2d_alt(&c);
    let circles: Vec<Circle> = (1..=10)
        //.skip(2)
        //.take(3)
        .map(|r| Circle{ center: p1, radius: r as f64})
        .collect();
    println!("{circles:#?}");
    let ta = circles_total_area(&circles);
    println!("total area = {ta}");
    let ta = circles_total_area_it(circles.iter());
    println!("total area = {ta}");
    let ta = mesurables2d_total_area(&circles);
    println!("total area = {ta}");
    println!()
}

fn play_with_macros() {
    let x = 3;
    let y = 5;
    let z = 14;
    let c = Circle{
        center: Point(1.0, 2.0),
        radius: 10.0
    };
    println_var!(x);
    // println_var!(c); // error: Display
    println_var_debug!(c);
    println_var_debug_pretty!(c);
    println_var_g!(c*);
    println_var_g!(c+);

    let v: Vec<i32> = (0..1_000_000).collect();
    println_sliceable!(v);
    println_sliceable!(&v[..10]);
    println_sliceable!(v, 20);

    println_vars!(x, y, z);
}

fn main() {
    play_with_forms();
    play_with_generic_forms();
    play_with_mesurable2d();
    play_with_macros();
}
