use std::io;

fn convert_temperature(temperature: i32, temperature_type: &str) {
    let converted_temperature: i32;
    let converted_temperature_type: String;
    if temperature_type.eq("Celsius") {
        converted_temperature = (temperature * 9/5) + 32;
        converted_temperature_type = "Fahrenheit".to_string();
    } else {
        converted_temperature = (temperature - 32) * 5/9;
        converted_temperature_type = "Celsius".to_string();
    }

    println!("Converted temperature is: {converted_temperature} {converted_temperature_type}")
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
            "Celsius"  => break "Celsius",
            "Fahrenheit" => break "Fahrenheit",
            _ => println!("Please check your spelling.")
        }
    };

    convert_temperature(temperature, temperature_type)
}
