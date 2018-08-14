#[derive(Debug)]
struct Money<T> {
    amount: T,
    currency: String,
}

fn select_first<T>(p1: T, _: T) -> T {
    p1
}

fn main() {
    let x = 1;
    let y = 2;
    let a = "meep";
    let b = "moop";

    println!("slected first: {}", select_first(x, y));
    println!("slected first: {}", select_first(a, b));
}
