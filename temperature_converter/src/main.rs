fn celsius_to_fahrenheit(c_temp: f32) -> f32 {
    (c_temp * 1.8) + 32.0
}

fn celsius_to_kelvin(c_temp: f32) -> f32 {
    c_temp + 273.0
}

fn fahrenheit_to_celsius(f_temp: f32) -> f32 {
    (f_temp - 32.0)/1.8
}

fn fahrenheit_to_kelvin(f_temp: f32) -> f32 {
    ((f_temp - 32.0) * 0.55) + 273.0
}

fn kelvin_to_celsius(k_temp: f32) -> f32 {
    k_temp - 273.0
}

fn kelvin_to_fahrenheit(k_temp: f32) -> f32 {
    ((k_temp - 273.0) * 1.8) + 32.0
}

fn main() {
    let test_temp = 25.0;

    println!("Test temperature: {}", test_temp);

    println!("Considering the temperature is in Celsius...");
    println!("Celsius to Fahrenheit: {}", celsius_to_fahrenheit(test_temp));
    println!("Celsius to Kelvin: {}", celsius_to_kelvin(test_temp));

    println!("Considering the temperature is in Fahrenheit...");
    println!("Fahrenheit to Celsius: {}", fahrenheit_to_celsius(test_temp));
    println!("Fahrenheit to Kelvin: {}", fahrenheit_to_kelvin(test_temp));

    println!("Considering the temperature is in Kelvin...");
    println!("Kelvin to Celsius: {}", kelvin_to_celsius(test_temp));
    println!("Kelvin to Fahrenheit: {}", kelvin_to_fahrenheit(test_temp));
}
