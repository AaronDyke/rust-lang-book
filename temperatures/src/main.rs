use std::io;

enum TemperatureType {
    Celsius,
    Fahrenheit
}

fn fahrenheit_to_celsius(temperature: i32) -> i32 {
    (temperature - 32) * 5/9
}

fn celsius_to_fahrenheit(temperature: i32) -> i32 {
    (temperature * 9/5) + 32
}

fn convert_temperature(temperature: i32, temperature_type: TemperatureType) {
    let (converted_temperature, converted_temperature_type) = match temperature_type {
        TemperatureType::Celsius => (celsius_to_fahrenheit(temperature), String::from("Fahrenheit")),
        TemperatureType::Fahrenheit => (fahrenheit_to_celsius(temperature), String::from("Celsius"))
    };

    println!("Converted temperature is: {converted_temperature} {converted_temperature_type}");
}

fn read_line(temp: &mut String) {
    // read in the temp input
    io::stdin()
        .read_line(temp)
        .expect("Failed to read line");
}

fn main() {
    
    let temperature: i32 = loop {
        let mut temp = String::new();
        println!("Enter a temperature: ");
        
        // read in the temp input
        read_line(&mut temp);

        // check if temperature is a number
        match temp.trim().parse(){
            Ok(num) => break num,
            Err(_) => println!("Enter a number please!"),
        };
    };

    let temperature_type = loop {
        let mut temp = String::new();
        println!("Is that Fahrenheit or Celsius?");

        read_line(&mut temp);
        
        match temp.to_lowercase().trim() {
            "celsius"  => break TemperatureType::Celsius,
            "c"  => break TemperatureType::Celsius,
            "fahrenheit" => break TemperatureType::Fahrenheit,
            "f" => break TemperatureType::Fahrenheit,
            _ => println!("Please check your spelling.")
        }
    };

    convert_temperature(temperature, temperature_type)
}
