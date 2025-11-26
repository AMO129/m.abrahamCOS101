use std::io::Write;

fn main() {

    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Software Engineering";

    let mut file = std::fs::File::create("data.txt").expect("creation failed");
    file.write_all("Welcome to Rust Programming\n".as_bytes())
    .expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("Write_all");
    println!("\nData written to file successfully");
}