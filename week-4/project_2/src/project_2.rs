// Rust program to determine 

use std::io;


fn main() {

    // Variables
    

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Experienced: true or false");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let exp:bool = input1.trim().parse().expect("Not a valid number");


    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:u32 = input2.trim().parse().expect("Not a valid input");


    // conditional statements

    if exp == true && age >= 40 {
        println!("Your incentive is 1,560,000 per annum");
    } else if exp == true && age >=30 {
        println!("Your incentive is 1,480,000 per annum");
    } else if exp == true && age < 28 {
        println!("Your incentive is 1,300,000 per annum");
    } else if exp == false && age >0 {
        println!("Your incentive is 100,000 per annum");
    }

    else {
        println!("Oops, out of range");
    }
}
