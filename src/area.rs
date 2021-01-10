pub trait HasArea {
    fn name(&self) -> String;
    fn area(&self) -> f64;
}

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub name: String
}

impl HasArea for Circle {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
impl Default for Circle {
    fn default() -> Self { 
        Self {
            x: 0.0f64,
            y: 0.0f64,
            radius: 0.0f64,
            name: String::from("circle"),
        }
    }
}

pub struct Square {
    pub x: f64,
    pub y: f64,
    pub side: f64,
    pub name: String
}

impl HasArea for Square {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
impl Default for Square {
    fn default() -> Self {
        Self {
            x: 0.0f64,
            y: 0.0f64,
            side: 0.0f64,
            name: String::from("square"),
        }
    }
}

pub fn print_area<T: HasArea>(shape: T) {
    println!("This {} has an area of {}", shape.name(), shape.area());
}