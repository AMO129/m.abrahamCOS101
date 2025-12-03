use std::fs::File;
use std::io::{self, Write};

fn main() {


    //COLUMNS

    let s = vec![
    "S/N",
    "1",
    "2",
    "3",
    "4",
    "5",
    ];

    let names = vec![
    "NAMES OF MINISTERS",
    "Mirabel Ojo",
    "Umar Gusau",
    "Yakubu John",
    "Akinwunmi Kolade",
    "Ekenne Obi",

    ];

    let ministry = vec![
    "MINISTRY",
    "Fnternal Affairs",
    "Foreign Affairs",
    "Defense",
    "Health",
    "Finance",
    ];

    let geopolitacal_zone = vec![
    "GEOPOLITICAL ZONE",
    "South West",
    "North West",
    "North central",
    "South West",
    "South East",
    ];


    //CREATING FILE

    let mut file = File::create("EFCC.csv").expect("Failed to create file");


    //WRITING TABLE HEADERS

   // writeln!(file, "{},{},{},{}", 
                   // "S/N","NAMES OF MINISTERS","MINISTRY","GEOPOLITICAL ZONE").expect("Failed to write");


    //ALLOWS THE DATA TO BE ENTERED ROW BY ROW AND SIDE BY SIDE

    for i in 0..s.len(){
        writeln!(file,"{},{},{},{}",
            s[i],
            names[i],
            ministry[i],
            geopolitacal_zone[i],).expect("Failed to write data in rows")
    }


    println!("EFCC file created successfully");





}