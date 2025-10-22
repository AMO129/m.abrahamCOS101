// Rust program to calculate quadratic equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter coefficient of x^2:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a: f32 = input1.trim().parse().expect("Not a valid number");


    println!("Enter coefficient of x:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

     println!("Enter constant term:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    // Call the function
    solve_quadratic(a, b, c);


   }

   // function must be outside of main

    fn solve_quadratic(a: f32, b: f32, c: f32){
            let d: f32 = b.powi(2) - (4.0 * a * c);
            
            if d > 0.0{
                 let root1 = (-b + d.sqrt())/(2.0 * a);
                 let root2 = (-b - d.sqrt())/(2.0 * a);
                 println!("There are two distinct roots: x1 = {}, x2 = {}", root1, root2);

        } else if d == 0.0{

            let root = -b/(2.0 * a);
            println!("There is exactly one real root: x = {}", root);

        }else {
            println!("There are no real roots(discriminant(d) < 0)");
        }


           
        




}
