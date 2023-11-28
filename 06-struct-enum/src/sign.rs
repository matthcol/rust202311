#[derive(Debug)]
pub enum Sign {
    Stop(u8),
    SpeedLimit(u8, String),
    Overtake(bool)
}

pub fn inspect_sign(sign: Sign){
    println!("sign: {sign:?}");
    match sign {
        Sign::Stop(0) => println!("Not really a stop"),
        Sign::Stop(duration @ 1..=2) => println!("Short stop: {duration}"),
        Sign::Stop(duration) => println!("Stop during {duration} seconds"),
        Sign::SpeedLimit(speed, description) if description == String::from("City") => println!("City speed limit: {speed}"),
        Sign::SpeedLimit(speed @ ..=89, description) => println!("Low cruise speed: {speed} ({description})"),
        Sign::SpeedLimit(speed @ 90.., description) => println!("Cruise speed: {speed} ({description})"),
        Sign::Overtake(ok) => println!("Ok to overtake: {ok}"),
        // _ => println!("not handled ...")
    }
    println!()
}