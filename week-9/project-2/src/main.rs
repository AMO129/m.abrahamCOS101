use std::fs::File;
use std::io::{self, Write};


fn main() -> io::Result<()>{

    //Table columns
    let student_name = vec![
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Moses Abraham", 
    ];

    let matric_number = vec![
        "ACC10211111",
        "ECO10110101",
        "CSC10328828",
        "EEE11020202",                     //Empty rows alignment
        "SEN10202001",
    ];

    let department = vec![
        "Accounting",
        "Economics",
        "Computer Sci",
        "Electrical Eng",
        "Software Eng",                 
    ];

     let level = vec![
        "300",
        "100",
        "200",
        "200",
        "100",                 
    ];


    // Create file
    let mut file = File::create("PAU SMIS.csv").expect("Failed to create file");


    //Write table header
    writeln!(file, "PAU SMIS");
    writeln!(
        file, "{},{},{},{}",
         "Student Name", "Matric. Number", "Department", "Level"
    ).expect("failed to write");


    // write rows side by side 
    for i in 0..student_name.len() {
        writeln!(
            file, "{},{},{},{}",
            student_name[i],
            matric_number[i],
            department[i],
            level[i]

        ).expect("Failed to write");
    }

    println!("file created successfully");

    Ok(())


}