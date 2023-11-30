fn main() {
    let f0 = || 3;    
    let res = f0();
    println!("{res}");

    let f1 = |x| x + 1; // type of x computed by inference
    let res = f1(4000);
    println!("In main: {res}");

    // let res = f1(4.4);
    // let res = f1(1u8);

    let f1 = |x: u8| x + 1; 
    let res = f1(40);
    println!("{res}");

    let f2 = |x, y| x + y; // types of x, y computed by inference
    let res = f2(40, 50);
    println!("{res}");

}
