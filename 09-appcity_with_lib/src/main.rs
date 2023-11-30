use citylib::city::*;

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

fn filter_city_dummy(city: &City) -> bool {
    match city { // dereferencement *city
        City{
            population: ..=50000,
            ..
        } =>  true,
        City{
            name,
            population: 77010,
            average_speed_limit: 40..
        } if name.starts_with("P") =>  true,
        _ => false
    }
}

fn filter_city_dummy2(city: &&City) -> bool {
    filter_city_dummy(city) // dereferenced auto: i.e. filter_city_dummy(*city)
}


fn play_with_vec_cities(){
    println!("**** play with vector of cities ****");
    let mut cities: Vec<City> = vec![
        City::new(String::from("Toulouse"), 470000, 50),
        City::new(String::from("Pau"), 77000, 50),
        City::new(String::from("Bagnères de Luchon"), 2300, 30)
    ];
    println!("{cities:?}");
    cities.push(City::from("Bordeaux"));
    println!("{cities:?}");
    for city in cities.iter() {
        println!("\t- {city} ; {}", city.name)
    }
    for city in cities.iter_mut() {
        // city.change_population_delta(10); // legal call
        *city += 10; // explicit dereferencement
    }
    println!("{cities:?}");
    let cities_filtered: Vec<&City> = cities.iter()
        .filter(|city| city.population > 10)
        .collect();
    println!("{cities_filtered:?}");
    cities.iter()
        .filter(|city| city.population > 10)
        .map(|city| city.population)
        .for_each(|pop| println!("\t- population = {pop}"));

    let total_population: u64 = cities.iter()
        .filter(|city| city.population > 10)
        .map(|city| city.population)
        .sum();
    println!("Total population: {total_population}");
    println!();
    println!("Cities filtered with dummy filter");
    cities.iter()
        .filter(filter_city_dummy2)
        .for_each(|city| println!("\t# {city}"));
    println!();
    println!("Cities filtered with threshold");
    let population_threshold = 100_000u64;
    cities.iter()
        .filter(|city| city.population >= population_threshold)
        .for_each(|city| println!("\t# {city}"));
    println!("population threshold after filter: {population_threshold}");
    println!();
    println!("Cities filtered with mutable threshold");
    let mut population_threshold = 1_000u64;
    cities.iter()
        .filter(|city| {
            let ok = city.population >= population_threshold;
            population_threshold *= 10;
            ok
        })
        .for_each(|city| println!("\t# {city}"));
    println!("population threshold after filter: {population_threshold}");
    println!()
}

#[allow(dead_code)]
fn play_with_vec_big() {
    let mut big_vec: Vec<f64> = vec![0.0; 500_000_000];
    big_vec.iter_mut()
        .for_each(|f| *f += 3.0);
    println!("{:?}", &big_vec[..10]);
    println!("{:?}", &big_vec[big_vec.len()-10..])
}

// borrow without returning
fn borrow_city(city: City){
    println!("I borrow city: {city}")
}

// borrow and return
fn borrow_city_give_back(city: City) -> City{
    println!("I borrow city temporarily: {city}");
    city
}

fn borrow_mut_city_give_back(mut city: City) -> City{
    println!("I borrow city temporarily: {city}");
    city += 10_000;
    city
}


fn play_with_borrow(){
    let city = City::from("Toulouse");
    borrow_city(city);
    // borrow_city(city); city is not mine anymore

    let city = City::from("Pau");
    let city = borrow_city_give_back(city);
    let city = borrow_city_give_back(city);
    let city = borrow_city_give_back(city);
    println!("{city}");

    let city = City::from("Bayonne");
    let city = borrow_mut_city_give_back(city);
    println!("{city}");
}

fn main() {
    play_with_cities();
    play_with_city_methods();
    play_with_city_traits();
    play_with_vec_cities();
    // play_with_vec_big()
    play_with_borrow();
}
