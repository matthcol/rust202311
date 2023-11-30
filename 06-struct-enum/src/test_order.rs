use crate::city::City;

#[test]
fn test_city_order() {
    let city1 = City::new(String::from("Valence"), 62_000, 50);
    let city2 = City::new(String::from("Toulouse"), 470_000, 20);
    assert!(city1 < city2, "less than");
    assert!(city1 <= city2, "lesst than or equal");
    assert!(city2 > city1, "greater than");
    assert!(city2 >= city1, "greater than or equal");
}