// Rust program to read the height of a person
// and then print if person is tall, dwarf,
// or average height person

use std::io;

fn main() {

    let mut input = String::new();

    println!("\nEnter Your Height (in centimentres):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid numbe");

    if height >= 150.0 && height <= 170.00
    {
        println!("You are of average height person");
    }
    else if height >170.00 && height <= 195.00
    {
        println!("You are tall");

    }

    else if height < 150.00 && height > 100.0
    {
        println!("You are dwarf");
    }

    else {
        println!("Abnormal height");
    }
}
