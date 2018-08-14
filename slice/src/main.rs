use std::fmt::Debug;

fn main() {
    println!("Hello, world!");

    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];

    for x in arr.iter() {
        for s in slice {
            println!("The sum is {}", x + s);
        }
    }
}
