use std::{fmt::Display, ops::AddAssign};


#[derive(Debug)]
pub struct City {
    pub name: String,
    pub population: u64,
    pub average_speed_limit: u8
}

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!("{} (pop = {} ; speed limit = {})", self.name, self.population, self.average_speed_limit)
        )
    }
}

impl AddAssign<i64> for City {
    fn add_assign(&mut self, rhs: i64) {
        if let Err(message) = self.change_population_delta(rhs) {
            panic!("{message}")
        }
    }
}

impl From<String> for City{
    fn from(value: String) -> Self {
        Self::new_from_name(value)       
    }   
}

impl From<&str> for City{
    fn from(value: &str) -> Self {
        Self::new_from_name(String::from(value))       
    }   
}

impl City {
    pub fn new(name: String, population: u64, average_speed_limit: u8) -> City {
        // here check something
        City {
            name: name,
            population: population,
            average_speed_limit: average_speed_limit
        }
    }

    pub fn new_from_name(name: String) -> City {
        Self::new(name, 0, 0)
    }

    pub fn change_population_delta(&mut self, delta: i64) -> Result<u64, String> {
        let new_population = self.population as i64 + delta;
        if new_population < 0 {
            return Err(String::from("Population would be negative"))
        }
        self.population = new_population as u64;
        Ok(self.population)
    }

    pub fn compute_something(&self) -> u64 {
        self.population * self.average_speed_limit as u64
    }
}

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
