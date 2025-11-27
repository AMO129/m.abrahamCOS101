use std::fs::File;
use std::io::{self, Write};


fn main() -> io::Result<()>{

    //Table columns
    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star", 
    ];

    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
        "",                     //Empty rows alignment
        "",
        "",
    ];

    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
        "",                 //Empty rows alignment
        "",
    ];


    // Create file
    let mut file = File::create("drinks.txt").expect("Failed to create file");


    //Write table header
    writeln!(
        file,
        "{:<20} | {:<20}  | {:<20}",
        "Lager", "Stout", "Non-Alcoholic" 
    )?;

    writeln!(file, "{}", "-".repeat(66)).expect("Failed to write");

    // write rows side by side 
    for i in 0..lager.len() {
        writeln!(
            file,
            "{:<20} | {:<20} | {:<20}",
            lager[i],
            stout[i],
            non_alcoholic[i]

        ).expect("Failed to write");
    }

    println!("file created successfully");

    Ok(())


}