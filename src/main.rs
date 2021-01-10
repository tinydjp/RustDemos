mod lights;
mod sum;
mod area;

fn main() {
    println!("Hello, world!");
    // test lights
    lights::test_lights();
    // test sum
    let acceptable_list = [1,2,3,4];
    let exceptional_list = [1, u32::MAX];
    assert_eq!(sum::sum_list(&acceptable_list), Some(10));
    assert_eq!(sum::sum_list(&exceptional_list), None);

    // 
    let circle = area::Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 3.2f64,
        ..Default::default()
    };

    let square = area::Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 2.0f64,
        ..Default::default()
    };

    area::print_area(circle);
    area::print_area(square);
}
