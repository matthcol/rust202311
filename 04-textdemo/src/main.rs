
fn str_to_string1(){
    let city: &str = "Toulouse";
    let mut city2: String = city.replace("ou", "ah");
    println!("{city} -> {city2}");
    city2.push_str(", ville rose");
    println!("{city2}");
}

fn convert_str_string(){
    // str -> String
    let city_str: &str = "Toulouse";
    let city_string: String = city_str.to_string();
    let city_string2: String = String::from("Pau");
    println!("s:{city_str} S:{city_string} S:{city_string2}");
    // String -> str
    let part: &str = city_string.as_str(); // or as_mut_str
    let part2: &str = &city_string2; 
    print!("s:{part} s:{part2}");
}


fn inspect_text(text: &str){
    let size_chars = text.chars().count();
    println!("city = {} ; size bytes = {} ; size chars = {}", 
        text, text.len(), size_chars
    ) ;
    for c in text.chars() {
        println!("\t- {c}")
    }
    let opt_first_char = text.chars().nth(0);
    let opt_after_last_char = text.chars().nth(size_chars);
    println!("first: {opt_first_char:?} ; after last: {opt_after_last_char:?}");
}


fn play_with_str_chars(){
    let city1 = "Toulouse";
    let city2 = "Åre";
    let city3 = "L'Haÿ-les-Roses";
    let city4 = "東京";
    inspect_text(city1);
    inspect_text(city2);
    inspect_text(city3); 
    inspect_text(city4);

    // Non decodable UTF-8 sequence
    // let part = &city4[..2];
    // let part = &city4[2..5];
    // println!("{part}")
}

fn main() {
    str_to_string1();
    convert_str_string();
    play_with_str_chars();
}
