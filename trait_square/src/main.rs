struct Point {
    x: usize,
    y: usize,
}

struct Square {
    lower_left: Point,
    upper_right: Point
}

struct Rectangle {
    lower_left: Point,
    upper_right: Point
}

trait Volume {
    fn get_size(&self) -> usize;
}

impl Volume for Square {
    fn get_size(&self) -> usize {
        return (self.upper_right.x - self.lower_left.x) * (self.upper_right.y - self.lower_left.y)
    }
}

impl Volume for Rectangle {
    fn get_size(&self) -> usize {
        return (self.upper_right.x - self.lower_left.x) * (self.upper_right.y - self.lower_left.y)
    }
}

fn main() {
    let square = Square{
        lower_left: Point{ x: 0, y: 0},
        upper_right: Point{ x: 1, y: 1},
    };

    let rect = Rectangle{
        lower_left: Point{ x: 2, y: 0},
        upper_right: Point{ x: 3, y: 22},
    };

    println!("The volume of square is {}", square.get_size());
    println!("The volume of rectangle is {}", rect.get_size());
}
