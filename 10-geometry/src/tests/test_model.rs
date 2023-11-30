use assert_float_eq::*;

use crate::geo::{model::{Point, Segment, Circle}, traits::{Mesurable1d, Mesurable2d}};

#[test]
fn test_segment_length(){
    let p1 = Point(2.5, 4.5);
    let p2 = Point(5.5, 0.5);
    let s = Segment(p1, p2);
    let len = s.length();
    assert_eq!(5.0, len)
}

#[test]
fn test_segment_length_big(){
    let p1 = Point(2.5E300, 4.5E300);
    let p2 = Point(5.5E300, 0.5E300);
    let s = Segment(p1, p2);
    let len = s.length();
    assert_eq!(5.0E300, len)
}

#[test]
fn test_circle_area(){
    let p1 = Point(2.5, 4.5);
    let c = Circle{
        center: p1,
        radius: 10.0
    };
    let a = c.area();
    assert_float_absolute_eq!(314.1593, a, 1E-4) 
}

#[test]
fn test_circle_perimeter(){
    let p1 = Point(2.5, 4.5);
    let c = Circle{
        center: p1,
        radius: 10.0
    };
    let p = c.perimeter();
    assert_float_absolute_eq!(62.8319, p, 1E-4) 
}




