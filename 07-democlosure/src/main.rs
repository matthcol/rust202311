

// see also FnMut, FnOnce
fn play_with_f1<F>(f: F) 
where
    F: Fn(u32) -> u32
{
    let x = 4;
    let y = f(x);
    println!("In play 1: f({x}) = {y}");
}

fn play_with_f2(f: fn(u32) -> u32) 
{
    let x = 4;
    let y = f(x);
    println!("In play 2: f({x}) = {y}");
}

fn compute(x: u32) -> u32 {
    x*x+1
}

fn main() {
    let f0 = || 3;    
    let res = f0();
    println!("{res}");

    let f1 = |x| x + 1; // type of x computed by inference
    let res = f1(4000);
    println!("In main: {res}");
    play_with_f1(f1);
    play_with_f2(f1);
    // let res = f1(4.4);
    // let res = f1(1u8);

    play_with_f1(compute);
    play_with_f2(compute);

    let f1 = |x: u8| x + 1; 
    let res = f1(40);
    println!("{res}");

    let f2 = |x, y| x + y; // types of x, y computed by inference
    let res = f2(40, 50);
    println!("{res}");

}
