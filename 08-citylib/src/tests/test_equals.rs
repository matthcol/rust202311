use crate::city::City;

#[test]
fn test_eq_ne_city_ok(){
    let city1 = City::from("Toulouse");
    let city2 = City::from("Toulouse");
    let res_eq = city1 == city2;
    let res_ne = city1 != city2;
    assert!(res_eq, "equals");
    assert!(!res_ne, "not equals");
}

#[test]
fn test_eq_ne_city_ko(){
    let city1 = City::from("Toulouse");
    let city2 = City::new(String::from("Toulouse"), 10, 20);
    let res_eq = city1 == city2;
    let res_ne = city1 != city2;
    assert!(!res_eq, "equals");
    assert!(res_ne, "not equals");
}