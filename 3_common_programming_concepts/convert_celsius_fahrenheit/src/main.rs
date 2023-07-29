use std::io;
fn main() {

    run_celsius_to_fahrenheit();


    run_fahrenheit_to_celsius();

}

fn run_celsius_to_fahrenheit() {
    println!("Please input your celsius");

    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius: f32 = celsius.trim().parse().expect("Please input decimal");

    let fahrenheit_value = convert_to_fahrenheit(celsius);

    println!("{celsius} celsius is {fahrenheit_value} fahrenheit");
}


fn run_fahrenheit_to_celsius() {
    println!("Please input your fahrenheit");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f32 = fahrenheit.trim().parse().expect("Please input decimal");

    let celsius_value = convert_to_celsius(fahrenheit);

    println!("{fahrenheit} fahrenheit is {celsius_value} celsius");
}


fn convert_to_fahrenheit(celsius: f32) -> f32{
    (celsius * 9.0/5.0) + 32.0
}

fn convert_to_celsius(fahrenheit: f32) -> f32{
    (fahrenheit - 32.0) * 5.0 / 9.0
}