use crate::city::City;

#[test]
fn test_compute_something(){
    let city = City::from("Las Vegas");
    let res = city.compute_something();
    assert_eq!(0, res);
}