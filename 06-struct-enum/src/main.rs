mod trafficlight;
mod sign;
mod city;

use city::*;
use sign::*;
// use trafficlight::TrafficLight;
// use trafficlight::{TrafficLight, inspect_traffic_light};
use trafficlight::*;

fn play_with_traffic_light(){
    let light = TrafficLight::Green;
    println!("{light:?}");
    println!();
    inspect_traffic_light(TrafficLight::Green);
    inspect_traffic_light(TrafficLight::Orange);
    inspect_traffic_light(TrafficLight::Red);
}

fn play_with_signs(){
    for i in 0..6 {
        inspect_sign(Sign::Stop(i));
    }
    inspect_sign(Sign::SpeedLimit(10, String::from("City")));
    inspect_sign(Sign::SpeedLimit(30, String::from("City")));
    inspect_sign(Sign::SpeedLimit(50, String::from("City")));
    inspect_sign(Sign::SpeedLimit(130, String::from("Highway")));
    inspect_sign(Sign::SpeedLimit(110, String::from("Highway with rain")));
    inspect_sign(Sign::Overtake(true));
    inspect_sign(Sign::Overtake(false));
    
}

fn play_with_cities() {
    let city = City{ 
        name: String::from("Toulouse"),
        population: 47000,
        average_speed_limit: 35
    };
    println!("My city: {city:?}");
    println!("My city: {city:#?}");

    let city2 = City {
        population: 480000,
        name: city.name.clone(),
        ..city
    };
    println!("My city 2: {city2:#?}");
    println!("My city is still here: {city:#?}");    
}


fn play_with_city_methods(){
    println!("**** play with City methods ****");
    let mut city1 = City::new(String::from("Toulouse"), 470000, 50);
    println!("{city1:#?}");
    {
        // shadow variable city1 from block above
        let city1 = City::new_from_name(String::from("Pau"));
        println!("{city1:#?}");
        println!("Dummy computation: {}", city1.compute_something())
    }
    println!("{city1:?}");
    let res = city1.change_population_delta(10000);
    println!("{city1:?} ; log: {res:?}");
    // shadow variable res from same block
    let res = city1.change_population_delta(-500000);
    println!("{city1:?} ; log: {res:?}");
    println!("Dummy computation: {}", city1.compute_something());
    println!()
}

fn play_with_city_traits(){
    println!("**** play with City traits ****");
    let mut city1 = City::new(String::from("Toulouse"), 470000, 50);
    println!("{city1}"); // trait Display
    city1 += 20000;
    println!("{city1}"); 
    // city1 += -500000; // panic: Population would be negative
    println!();
    let city2 = City::from(String::from("Pau"));
    println!("{city2}");
    let city3 = City::from(String::from("Luchon"));
    println!("{city3}");
    println!();

}

fn play_with_vec_cities(){
    println!("**** play with vector of cities ****");
    let mut cities = vec![
        City::from("Toulouse"),
        City::from("Pau"),
        City::new(String::from("Bagnères de Luchon"), 2300, 30)
    ];
    println!("{cities:?}");
    cities.push(City::from("Bordeaux"));
    println!("{cities:?}");
    println!()
}

fn main() {
    play_with_traffic_light();
    play_with_signs();
    play_with_cities();
    play_with_city_methods();
    play_with_city_traits();
    play_with_vec_cities();
}
