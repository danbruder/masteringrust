use std::fmt::{Formatter, Display, Result};

enum Temp {
    Hot,
    Cold
}

impl Display for Temp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Temp::Hot =>  write!(f, "Hot"),
            Temp::Cold =>  write!(f, "Cold")
        }
    }
}

struct Drink<T> {
    level: T,
    temperature: Temp,
}

impl <T: Display> Display for Drink<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.level, self.temperature)
    }
}

fn main() {
    let milk = Drink{
        level: &"Cat",
        temperature: Temp::Cold,
    };

    let coffee = Drink{
        level: 23.93,
        temperature: Temp::Hot,
    };

    println!("{}", milk);
    println!("{}", coffee);
}
