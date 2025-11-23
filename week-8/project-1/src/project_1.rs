use std::collections::HashMap;
use std::io;


//Read input

fn read_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_lowercase()
}


fn main(){
    println!("------------------------------------------------");
    println!("                  \nHello                       ");
    println!("      \nWelcome to the APS level checker         ");
    println!("        \nFederal Government of Nigeria          ");
    println!("------------------------------------------------\n");


    //FIELDS LIST
    println!("Select your occupational field");
    println!("1 - Office Administrator");
    println!("2 - Academic");
    println!("3 - Lawyer");
    println!("4 - Tearcher\n");


    let field_input = read_input("Enter your field's number");


    //STRUCTURE (HASHMAPS + VECTORS)
    //MAPS APS LEVEL to Vec OF ROLES WITHIN EACH FIELD

    let mut admin: HashMap<&str, Vec<&str>> = HashMap::new();
    admin.insert("aps 1-2", vec!["intern"]);
    admin.insert("aps 3-5", vec!["administrator"]);
    admin.insert("aps 5-8", vec!["senior administrator"]);
    admin.insert("aps 8-10", vec!["office manager"]);
    admin.insert("aps 10-13", vec!["director"]);
    admin.insert("ses", vec!["ceo"]);


    let mut academic: HashMap<&str, Vec<&str>> = HashMap::new();
    academic.insert("aps 3-5", vec!["research assistant"]);
    academic.insert("aps 5-8", vec!["phd candidate"]);
    academic.insert("aps 8-10", vec!["post-doc researcher"]);
    academic.insert("aps 10-13", vec!["senior lecturer"]);
    academic.insert("ses", vec!["dean"]);


    let mut lawyer: HashMap<&str, Vec<&str>> = HashMap::new();
    lawyer.insert("aps 1-2", vec!["paralegal"]);
    lawyer.insert("aps 3-5", vec!["junior associate"]);
    lawyer.insert("aps 5-8", vec!["associate"]);
    lawyer.insert("aps 8-10", vec!["senior associate 1-2"]);
    lawyer.insert("aps 10-13", vec!["senior associate 3-4"]);
    lawyer.insert("ses", vec!["partner"]);


    let mut teacher: HashMap<&str, Vec<&str>> = HashMap::new();
    teacher.insert("aps 1-2", vec!["placement"]);
    teacher.insert("aps 3-5", vec!["classroom teacher"]);
    teacher.insert("aps 5-8", vec!["snr teacher"]);
    teacher.insert("aps 8-10", vec!["leading teacher"]);
    teacher.insert("aps 10-13", vec!["deputy principal"]);
    teacher.insert("ses", vec!["principal"]);


    //SELECTED FIELD

    let field_map = match field_input.as_str(){
        "1" => {
            println!("\nYou selected: Office Administrator\n");
            admin
        }


            "2" => {
            println!("\nYou selected: Academic\n");
            academic
        }

        "3" => {
            println!("\nYou selected: Lawyer\n");
            lawyer
        }

        "4" => {
            println!("\nYou selected: Tearcher\n");
            teacher
        }

        _ => {
            println!("Invalid field selection. Exiting the program.");
            return;
        }
    
    };


    loop {
        let role = read_input("Enter your role (e.g director, placement, ceo, etc.)");

        //VALIDATION

        let mut found = false;

        for(aps_level,roles) in &field_map {
            for r in roles {
                if r.to_lowercase() == role {
                   
                    println!("Your role corresponds to APS level: {}", aps_level.to_uppercase());
                   
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }

        }

        if found {
        break; //exit loop if matched

        } else {
            println!("\nThe role you entered does not correspond with the occupational field.");
            println!("Please try again.\n");
        }

    }

}






