use std::io;

//user input
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}


//name must contain only letters & spaces
fn valid_name(name: &str) -> bool {
    name.chars().all(|c| c.is_alphabetic() || c.is_whitespace())
}

fn main() {
    println!("--------------------------------------------");
    println!("      EY Nigeria - Experience Scanner");
    println!("--------------------------------------------\n");


    //number of people to be interviewed
    let num_people: u32 = loop {
        let input = read_input("Enter number of candidates:");
        match input.trim().parse() {
            Ok(n) if n > 0 => break n,
            _ => println!("Please enter a valid positive number.\n"),
        }
    };



    // Vector to store (name, experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();


    // Collecting each person's data
    for i in 1..=num_people {
        println!("\n--- Candidate {} ---", i);

        // NAME VALIDATION LOOP
        let name = loop {
            let n = read_input("Enter candidate's name (letters only):");
            if valid_name(&n) && !n.is_empty() {
                break n;
            } else {
                println!("Invalid name! Use letters only (A-Z). Try again.\n");
            }
        };

        // EXPERIENCE VALIDATION LOOP
        let experience: u32 = loop {
            let exp_input = read_input("Enter years of programming experience (numbers only):");

            //Experience years must be numbers
            if exp_input.chars().all(|c| c.is_digit(10)) {
                match exp_input.parse() {
                    Ok(e) => break e,
                    Err(_) => println!("Enter a valid number."),
                }
            } else {
                println!("Invalid input! Enter digits only (0-9).");
            }
        };

        candidates.push((name, experience));
    }

    // Find the person with the highest experience
    let mut highest = &candidates[0];

    for candidate in &candidates {
        if candidate.1 > highest.1 {
            highest = candidate;
        }
    }

    println!("\n--------------------------------------------");
    println!(
        "The candidate with the highest programming experience is: {}",
        highest.0
    );
    println!(
        "Years of experience: {} years",
        highest.1
    );
    println!("--------------------------------------------");
}
