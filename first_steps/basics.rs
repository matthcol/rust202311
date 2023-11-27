// global variables
const THE_CITY_1: &'static str = "Pau";
static THE_CITY_2: &'static str = "Toulouse";

fn play_with_integers() {
    let _x = 13; // i32 ?
    let _y = 13u16;
    let _z: i32;
    _z = 128;
    let a: u8 = 0b10001111;
    let b: u8 = 0x8F;
    let c: u8 = 0o217;
    println!("same nb: {a} {b} {c}");
    // x = 15; // forbidden
    let mut cpt = 0u64;
    cpt += 1;
    println!("cpt = {}", cpt);
    println!("cpt = {cpt}");
    println!("cpt = {{{cpt}}}");
}

fn play_with_floats() {
    let a: f32 = 0.0;
    let b = 1.0 / a;
    let price: f64 = 0.1;
    let total_price = price * 3.0;
    println!("a = {a} ; b = {b}");
    println!("price = {price} ; total_price = {total_price:.18}");
}

fn play_with_booleans() {
    let _fact = true;
    let _ok: bool = false;
    let weather_good: bool;
    let temperature = 1;
    weather_good = temperature < 10;
    println!("weather ok to ski: {weather_good}");
}

fn play_with_text() {
    let a_char: char = '🦜';
    println!("parrot emoji: {a_char}");
    let name = "Mr Robot"; // &str by inference
    println!("My name is {name}");
    let name2: &str = "Luke";
    println!("My name is {name2}");
    let text: &'static str = "Toulouse, ville rose";
    let mut part_text: &str = &text[..];
    println!("Text part: {part_text}");
    part_text = &text[..8];
    println!("Text part: {part_text}");
    part_text = &text[12..];
    println!("Text part: {part_text}");
    part_text = &text[4..8];
    println!("Text part: {part_text}");
    let length = text.len();
    println!("Text length: {length}");
    println!("Text length: {}", text.len());
    println!("Famous cities: {THE_CITY_1}, {THE_CITY_2}");
}

// #[warn(dead_code)]
// fn dead_function() {

// }

fn play_with_tuples() -> (&'static str, i32, bool) {
    let temperature = 7;
    let is_sunny = false;
    let city = "Toulouse";
    let infos = (city, temperature, is_sunny);
    // println!("{}", infos); // by default: error
    println!("{} {} {}", infos.0, infos.1, infos.2); // by default: error
    infos // or return infos or return infos;
}

fn play_with_arrays(){
    let temperatures: [i8; 5] = [-1, 4, 12, 7, 0];
    let mut temperatures2 = [0i8; 365];
    println!("{:?}", temperatures);
    println!("{:?}", &temperatures2[..10]);
    let value = temperatures[0];
    println!("First value array 1: {}", value);
    temperatures2[0] = 25;
    println!("First value array 2: {}", temperatures2[0]);
    // out of bounds access:
    // println!("Out of bound  access: {}", temperatures2[365]); // error at compilation
    // temperatures2[366] = 25; // error at compilation
    let mut i = 366;
    i -= 2;
    temperatures2[i] = 25; // bound ok (try -=1 => error)
    for i in 0..temperatures.len() {
        println!("- temperature[{}] = {}", i, temperatures[i]);
    }
}

fn main() {
    // Statements here are executed when the compiled binary is called.
    play_with_integers();
    play_with_floats();
    play_with_booleans();
    play_with_text();
    let infos = play_with_tuples();
    println!("{} {} {}", infos.0, infos.1, infos.2);
    play_with_arrays();
}