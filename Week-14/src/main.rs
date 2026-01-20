use postgres::{Client, NoTls};
use std::fs::File;
use std::io::{self, Write};

fn main() {
    // 1. Establish the connection
    let mut client = Client::connect("host=localhost user=postgres password=cos101 dbname=globacom_dbase", NoTls)
        .expect("Failed to connect to database. Ensure PostgreSQL is running and password is correct.");

    println!("Welcome to Globacom Database Portal");
    println!("Please enter your role (admin, project manager, employee, customer, vendor):");

    // 2. Capture user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let role_input = input.trim().to_lowercase();

    // 3. Logic based on Globacom criteria
    match role_input.as_str() {
        "admin" => {
            process_request(&mut client, "ALL", "SELECT table_name, column_name, data_type FROM information_schema.columns WHERE table_schema = 'public'");
        },
        "project manager" => {
            process_request(&mut client, "project", "SELECT * FROM project"); // Access project table
        },
        "employee" => {
            process_request(&mut client, "staff", "SELECT * FROM staff"); // Access staff table
        },
        "customer" => {
            process_request(&mut client, "customer", "SELECT * FROM customer"); // Access customer table
        },
        "vendor" => {
            process_request(&mut client, "data_plan", "SELECT * FROM data_plan"); // Access data-plan table
        },
        _ => println!("Access Denied: Invalid Role"),
    }
}

fn process_request(client: &mut Client, table_name: &str, data_query: &str) {
    // --- PART A: Display Structure on CMD ---
    println!("\n[CMD DISPLAY] Structure for Table: {}", table_name);
    println!("{:-<45}", "");
    
    // Querying the schema information
    let schema_query = if table_name == "ALL" {
        "SELECT column_name, data_type FROM information_schema.columns WHERE table_schema = 'public'".to_string()
    } else {
        format!("SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '{}'", table_name)
    };

    let schema_rows = client.query(&schema_query, &[]).expect("Failed to fetch schema");
    println!("{:<20} | {:<20}", "Column Name", "Data Type");
    for row in schema_rows {
        let col: String = row.get(0);
        let dtype: String = row.get(1);
        println!("{:<20} | {:<20}", col, dtype);
    }

    // --- PART B: Export Actual Data to File ---
    let file_path = format!("{}_data.txt", table_name);
    let mut file = File::create(&file_path).expect("Could not create output file");
    
    let data_rows = client.query(data_query, &[]).expect("Failed to fetch table data");
    
    writeln!(file, "Globacom Database Export: {}\n{}", table_name, "=".repeat(40)).unwrap();
    
    for row in data_rows {
        let mut row_output = String::new();
        for i in 0..row.len() {
            // Using Debug formatting {:?} to capture various types safely into the file
            let value: String = format!("{:?}", row.columns()[i].name()); 
            row_output.push_str(&format!("| {} ", value));
        }
        writeln!(file, "{}", row_output).unwrap();
    }

    println!("\n[FILE EXPORT] Actual rows saved to: {}", file_path);
}