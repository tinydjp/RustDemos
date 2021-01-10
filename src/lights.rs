enum TrafficLight {
    Green,
    Red,
    Yellow
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Green => 10,
            TrafficLight::Red => 20,
            TrafficLight::Yellow => 30
        }
    }
}

pub fn test_lights() {
    let green = TrafficLight::Green;
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    assert!(green.time() == 10);
    assert!(red.time() == 20);
    assert!(yellow.time() == 30);
    println!("lights cases all passed!");
}