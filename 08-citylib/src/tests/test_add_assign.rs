use crate::city::City;

#[test]
fn test_add_assign_ok() {
    let mut city = City{
        name: String::from("Las Vegas"),
        population: 25000,
        average_speed_limit: 0
    };
    city += 5000;
    assert_eq!(30000, city.population)
}

#[test]
#[should_panic = "Population would be negative"]
fn test_add_assign_ko() {
    let mut city = City{
        name: String::from("Las Vegas"),
        population: 25000,
        average_speed_limit: 0
    };
    city += -25001;
}
