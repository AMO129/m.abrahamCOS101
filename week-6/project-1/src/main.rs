//RUST PROGRAM FOR CAFETERIA 

use std::io;

fn main(){
    println!("========================================");
    println!("Code\tMenu           \t\tPrice");
    println!("P\tPoundo Yam/Edinkaiko\t-N3,200");
    println!("F\tFried Rice & Chicken\t-N3,000");
    println!("A\tAmala & Ewedu Soup\t-N2,500");
    println!("E\tEba & Egusi Soup\t-N2,000");
    println!("W\tWhite Rice & Stew\t-N2,500");


    let mut code =String::new();

    println!("Enter a code");
    io::stdin().read_line(&mut code).expect("Invalid code");
    let code = code.trim().to_uppercase();  // CONVERTING LOWERCASE CODE TO UPPERCASE


    // CONVERTING PRICES TO CODES


    let price:f32 = if code == "P"{
        3_200.0
    } else if code =="F" {
        3_000.0
    } else if code =="A"{
        2_500.0
    } else if code =="E"{
        2_000.0
    } else if code =="W"{
        2_500.0
    } else {
        println!("Not a valid code, please try again");
        return;
    };

    let mut quantity = String::new();
    println!("How many do you want");
    io::stdin().read_line(&mut quantity).expect("Invalid input");
    let quantity:u32 = match quantity.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid, please enter a number");   // CHECKING IN QUANTITY IS A NUMBER
            return;
        }
    }; 


    let mut total_order = price * quantity as f32;
    if total_order > 10_000.0{
        total_order = 95.0/100.0 * (total_order);   // GIVING 5% DISCOUNT
        println!("\n You got 5% discount");
        println!("Total cost: {:.2}",total_order );
    } else {
        println!("Total cost: {:.2}",total_order );
    }


}