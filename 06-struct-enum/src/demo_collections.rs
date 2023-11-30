use std::collections::{LinkedList, BTreeSet, BTreeMap};

use crate::city::City;

pub fn play_with_linked_list(){
    println!("**** Demo LinkedList ****");
    let mut data: LinkedList<i32> = LinkedList::from([3, 7, 9]);
    let mut data2: LinkedList<i32> = LinkedList::from([8, 11, 14]);
    data.append(&mut data2);
    println!("{data:?}");
    println!("{data2:?}"); // empty:
    println!()
}


pub fn play_with_btreeset() {
    println!("**** Demo BTreeSet ****");
    let mut data = BTreeSet::from([3,7,9]);
    println!("{data:?}");
    let mut data2 = BTreeSet::new();
    println!("{data2:?}");
    data2.insert(15);
    data2.insert(13);
    data2.insert(21);
    data.append(&mut data2);
    println!("{data:?}");
    println!("{data2:?}");
    let data3 = vec![5, 17, 19];
    // TODO: move or copy all elements of data3 into data
    data.extend(data3.into_iter());
    println!("{data:?}");
    // println!("{data3:?}"); // borrowed

    // iter according to order
    for d in data.iter() {
        println!("\t- {d}")
    }
    for d in &data {
        println!("\t- {d}")
    }

    // extract range
    let data_extract: BTreeSet<_> = data.range(8..=20)
        .collect();
    println!("Data from 8 to 20: {data_extract:?}");
    
    println!("{data:?}");
    println!()
}


pub fn play_with_cities_index(){
    let cities: Vec<City> = vec![
        City::new(String::from("Toulouse"), 470000, 50),
        City::new(String::from("Pau"), 77000, 50),
        City::new(String::from("Bordes"), 2_800, 50),
        City::new(String::from("Bagnères de Luchon"), 2300, 30),
        City::new(String::from("Bordes"), 752, 50)
    ];
    let index_city_name_uniq: BTreeMap<String, &City> = BTreeMap::from_iter(
        cities.iter()
        .map(|city| (city.name.clone(), city))
    );
    println!("{index_city_name_uniq:?}"); // Bordes is only present once

    let mut index_city_name: BTreeMap<String, Vec<&City>> = BTreeMap::new();
    for city in cities.iter() {
        index_city_name.entry(city.name.clone())
            .and_modify(|v| v.push(city))
            .or_insert(vec![city]);
        // if let Some(v) = index_city_name.get_mut(&city.name) {
        //     v.push(city);
        // } else {
        //     index_city_name.insert(city.name.clone(), vec![city]);
        // }
    }
    println!("{index_city_name:#?}"); // Bordes is present twice
    
}