#[derive(Debug)]
pub enum TrafficLight {
    Green,
    Orange,
    Red
}

pub fn inspect_traffic_light(traffic_light: TrafficLight){
    println!("{traffic_light:?}");
    match traffic_light {
        TrafficLight::Green => println!("Let's go"),
        TrafficLight::Orange => println!("Should I go or should I stop"),
        TrafficLight::Red => println!("Stop")
    }
    println!()
}