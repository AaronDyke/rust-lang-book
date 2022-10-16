use std::io;
use temperatures::temperature::Temperature;
use temperatures::temperature::TemperatureScale;

fn read_line(temp: &mut String) {
    // read in the temp input
    io::stdin()
        .read_line(temp)
        .expect("Failed to read line");
}

fn main() {
    
    let degree: i32 = loop {
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

    let scale = loop {
        let mut temp = String::new();
        println!("Is that Fahrenheit or Celsius?");

        read_line(&mut temp);
        
        match temp.to_lowercase().trim() {
            "celsius" | "c" => break TemperatureScale::Celsius,
            "fahrenheit" | "f" => break TemperatureScale::Fahrenheit,
            _ => println!("Please check your spelling.")
        }
    };

    let mut temperature = Temperature {
        degree,
        scale
    };

    temperature.convert();

    println!("Your converted temperature is {} degrees {:?}", temperature.degree, temperature.scale);
}
