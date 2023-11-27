
fn inspect_slice(a_slice: &[i8]){
    println!("Slice: length = {} ; data = {:?}", a_slice.len(), a_slice);
}

fn inspect_optional_slice(an_opt_slice: Option<&[i8]>){
    if an_opt_slice.is_some() {
        let a_slice = an_opt_slice.unwrap();
        inspect_slice(a_slice);
    } else {
        println!("slice not valid");
    }
}

fn inspect_optional_slice_iflet(an_opt_slice: Option<&[i8]>){
    if let Some(a_slice) =  an_opt_slice {
        inspect_slice(a_slice);
    } else {
        println!("slice not valid");
    }
}

fn inspect_optional_slice_match(an_opt_slice: Option<&[i8]>){
    match an_opt_slice {
        Some(&[]) => println!("empty slice"),
        Some(a_slice) => inspect_slice(a_slice),
        None => println!("slice not valid"),
        // _ => println!("slice not valid")  // default case
    }
} 


fn fill_temperatures(temperatures: &mut [i8] ) {
    // for i
    for i in 0..temperatures.len() {
        temperatures[i] = ((i as i32 * 2 + 1) % 128) as i8;
    }
}

fn display_temperatures(temperatures: &[i8] ) {
    // foreach
    println!("******  temperatures *******");
    for temperature in temperatures { // call .iter()
        println!(" - {temperature} ");
    }
}

fn display_temperatures2(temperatures: &[i8] ) {
    // foreach
    println!("******  temperatures *******");
    for (i, temperature) in temperatures.iter().enumerate() {
        println!(" - [{i}] {temperature} ");
    }
}


fn record_temperatures(){
    let mut temperatures: [i8; 365] = [0i8; 365];
    fill_temperatures(&mut temperatures);

    inspect_slice(&temperatures);
    inspect_slice(&temperatures[..]); // idem preceding lines
    inspect_slice(&temperatures[..10]);
    inspect_slice(&temperatures[temperatures.len() - 10 ..]);
    // inspect_slice(&temperatures[temperatures.len() - 10 ..=temperatures.len()]); // dynamic error
    inspect_slice(&temperatures[10..10]);
    // inspect_slice(&temperatures[10..9]); // dynamic error

    println!("{:?}", temperatures.get(3..2));
    println!("{:?}", temperatures.get(temperatures.len() - 10 ..=temperatures.len()));
    println!("{:?}", temperatures.get(temperatures.len() - 10 ..temperatures.len()));

    inspect_optional_slice(temperatures.get(3..2));
    inspect_optional_slice(temperatures.get(temperatures.len() - 10 ..=temperatures.len()));
    inspect_optional_slice(temperatures.get(temperatures.len() - 10 ..temperatures.len()));

    inspect_optional_slice_iflet(temperatures.get(3..2));
    inspect_optional_slice_iflet(temperatures.get(temperatures.len() - 10 ..=temperatures.len()));
    inspect_optional_slice_iflet(temperatures.get(temperatures.len() - 10 ..temperatures.len()));

    inspect_optional_slice_match(temperatures.get(3..3));
    inspect_optional_slice_match(temperatures.get(3..2));
    inspect_optional_slice_match(temperatures.get(temperatures.len() - 10 ..=temperatures.len()));
    inspect_optional_slice_match(temperatures.get(temperatures.len() - 10 ..temperatures.len()));


    display_temperatures(&temperatures[..5]);
    display_temperatures2(&temperatures[..5]);

    // foreach
    for temperature in temperatures { // call .into_iter()
        println!(" ~ {temperature} ");
    }
    inspect_slice(&temperatures);
}


fn example_while()  {
    // todo!()
    let mut cpt: i32 = 10;
    while cpt >= 0 {
        println!("{cpt}");
        cpt -= 1;
    }
    println!("Boom !!!")
}

fn main() {
    record_temperatures();
    example_while();
}

