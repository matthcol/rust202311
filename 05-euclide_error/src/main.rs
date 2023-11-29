fn gcd(a: u64, b: u64) -> Result<u64,String> {
    if (a == 0) || ( b == 0) {
        return Err(String::from("Gcd expects 2 numbers > 0"))
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
    Ok(a)
}

#[test]
fn test_gcd_when_a_greater_b() {
    let a = 21;
    let b = 14;
    let g = gcd(a, b);
    assert_eq!(Ok(7), g)
}

#[test]
fn test_gcd_when_b_greater_a() {
    let a = 14;
    let b = 21;
    let g = gcd(a, b);
    assert_eq!(Ok(7), g)
}

#[test]
fn test_gcd_when_b_is_zero() {
    let a = 14;
    let b = 0;
    let res = gcd(a, b);
    assert_eq!(Err(String::from("Gcd expects 2 numbers > 0")), res);
}

#[test]
fn test_gcd_when_a_is_zero() {
    let a = 0;
    let b = 14;
    let res = gcd(a, b);
    assert_eq!(Err(String::from("Gcd expects 2 numbers > 0")), res);
}


fn main() {
    let a = 21;
    let b = 14;
    let res = gcd(a, b);
    if let Ok(g) = res {
        println!("gcd({a}, {b}) = {g}");
    } else {
        println!("no result !");
    }
}
