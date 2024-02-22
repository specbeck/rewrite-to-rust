fn main() {
    // Defining the temperatures
    let temperature_c: f32 = 1.0;
    let temperature_f: f32 = 1.0;
    println!("{}`C is {}F", temperature_c, celsius_to_fahrenheit(temperature_c));
    println!("{}F is {}`C", temperature_f, fahrenheit_to_celsius(temperature_f));
}

fn celsius_to_fahrenheit(temp_in_cel: f32) -> f32 {
    let temp_in_fahren: f32 = (temp_in_cel * 9.0/5.0) + 32.0;
    temp_in_fahren
}

fn fahrenheit_to_celsius(temp_in_fahren: f32) -> f32 {
    let temp_in_cel: f32 = 5.0/9.0 * (temp_in_fahren - 32.0);
    temp_in_cel
}
