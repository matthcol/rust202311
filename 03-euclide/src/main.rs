fn gcd(a: u64, b: u64) -> u64 {
    if (a == 0) || ( b == 0) {
        panic!("Gcd expects 2 numbers > 0")
    }
    let mut a = a;
    let mut b = b;
    while a != b  {
        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }
    a
}

#[test]
fn test_gcd_when_a_greater_b() {
    let a = 21;
    let b = 14;
    let g = gcd(a, b);
    assert_eq!(7, g)
}

#[test]
fn test_gcd_when_b_greater_a() {
    let a = 14;
    let b = 21;
    let g = gcd(a, b);
    assert_eq!(7, g)
}

#[test]
#[should_panic(expected = "Gcd expects 2 numbers > 0")]
fn test_gcd_when_b_is_zero() {
    let a = 14;
    let b = 0;
    gcd(a, b);
}

#[test]
#[should_panic(expected = "Gcd expects 2 numbers > 0")]
fn test_gcd_when_a_is_zero() {
    let a = 0;
    let b = 14;
    gcd(a, b);
}


fn main() {
    let a = 21;
    let b = 14;
    let g = gcd(a, b);
    println!("gcd({a}, {b}) = {g}");
}
