use std::io;

#[derive(Debug)]
enum TemperatureScale {
    Celsius,
    Fahrenheit
}

#[derive(Debug)]
struct Temperature {
    degree: i32,
    scale: TemperatureScale
}

impl Temperature {
    fn convert(&mut self) {
        match self.scale {
            TemperatureScale::Celsius => {
                self.scale = TemperatureScale::Fahrenheit;
                self.degree = (self.degree * 9/5) + 32
            }
            TemperatureScale::Fahrenheit => {
                self.scale = TemperatureScale::Celsius;
                self.degree = (self.degree - 32) * 5/9
            }
        }
    }
}

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
            "celsius"  => break TemperatureScale::Celsius,
            "c"  => break TemperatureScale::Celsius,
            "fahrenheit" => break TemperatureScale::Fahrenheit,
            "f" => break TemperatureScale::Fahrenheit,
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
