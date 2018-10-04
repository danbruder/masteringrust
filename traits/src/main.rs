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

struct Drink {
    level: usize,
    temperature: Temp
}

impl Display for Drink {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.level, self.temperature)
    }
}

fn main() {
    let milk = Drink{
        level: 10,
        temperature: Temp::Cold,
    };

    let coffee = Drink{
        level: 2,
        temperature: Temp::Hot,
    };

    println!("{}", milk);
    println!("{}", coffee);
}
