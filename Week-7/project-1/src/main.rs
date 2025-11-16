use std::io;

//RUST PROGRAM TO CARRY DIFFERENT MATHEMATICAL OPERATIONS

fn main(){
    println!("Welcome user \nPlease choose an operation you would like to perform");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("Enter your choice:");


    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid input");
    let choice:u32 = choice.trim().parse().expect("Please enter a valid choice");

    match choice{
        1 => {
            let base_1 = read_value("Enter first base value of trapezium:");
            let base_2 = read_value("Enter second base value of trapezium:");
            let height = read_value("Enter height of trapezium");
            let result = a_trapezium(base_1, base_2, height);
            println!("Area of the Trapezium = {}",result );
        }

         2 => {
            let d_1 = read_value("Enter first diagonal value of rhombus:");
            let d_2 = read_value("Enter second diagonal value of rhombus:");
            let result = a_rhombus(d_1, d_2);
            println!("Area of the Rhombus = {}",result );
        }


         3 => {
            let base = read_value("Enter base value of parallelogram:");
            let altitude = read_value("Enter altitude value of parallelogram:");
            let result = a_parallelogram(base, altitude);
            println!("Area of the Parallelogram = {}",result );
        }

         4 => {
            let side = read_value("Enter length of cube side:");
            let result = a_cube(side);
            println!("Area of the Cube = {}",result );
        }


         5 => {
            let radius = read_value("Enter radius value of cylinder:");
            let height = read_value("Enter height value of cylinder:");
            let result = v_cylinder(radius, height);
            println!("Area of the Cube = {}",result );
        }


        _ => println!("Your choice is out of scope"),

    }


}


//INPUT FUNCTION

fn read_value(prompt: &str) -> f64{
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    input.trim().parse().expect("Please enter a valid number")
}




//OPERATIONS

fn a_trapezium(base_1: f64, base_2: f64, height: f64) -> f64{
    (base_1 + base_2) * height/2.0
}


fn a_rhombus(d_1: f64, d_2: f64) -> f64 {
    0.5 * d_1 * d_2
}

fn a_parallelogram(base: f64, altitude: f64) -> f64{
    base * altitude
}


fn a_cube(side: f64) -> f64 {
    6.0 * side.powf(2.0)
}

fn v_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius.powf(2.0) * height
}

