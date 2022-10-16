pub mod temperature {
    #[derive(Debug)]
    pub enum TemperatureScale {
        Celsius,
        Fahrenheit,
    }

    #[derive(Debug)]
    pub struct Temperature {
        pub degree: i32,
        pub scale: TemperatureScale,
    }

    impl Temperature {
        pub fn convert(&mut self) {
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
}
