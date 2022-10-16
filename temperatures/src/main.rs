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
    // let converted_temperature: i32;
    // let converted_temperature_type: String;
    let (converted_temperature, converted_temperature_type) = match temperature_type {
        TemperatureType::Celsius => (celsius_to_fahrenheit(temperature), "Fahrenheit".to_string()),
        TemperatureType::Fahrenheit => (fahrenheit_to_celsius(temperature), "Celsius".to_string())
    };

    println!("Converted temperature is: {converted_temperature} {converted_temperature_type}");
}

fn main() {
    
    let temperature: i32 = loop {
        let mut temp = String::new();
        println!("Enter a temperature: ");
        
        // read in the temp input
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        // check if temperature is a number
        match temp.trim().parse(){
            Ok(num) => break num,
            Err(_) => println!("Enter a number please!"),
        };
    };

    let temperature_type = loop {
        let mut temp = String::new();
        println!("Is that Fahrenheit or Celsius?");

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        
        match temp.trim() {
            "Celsius"  => break TemperatureType::Celsius,
            "Fahrenheit" => break TemperatureType::Fahrenheit,
            _ => println!("Please check your spelling.")
        }
    };

    convert_temperature(temperature, temperature_type)
}
