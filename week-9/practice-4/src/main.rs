use std::fs::OpenOptions;
use std::io::Write;
use std::io;

fn main(){

    let mut file = std::fs::File::create("Moses.txt").expect("failed to create file");
    file.write_all("\nMY RUST PROGRAMMING LANGUAGE JOURNEY".as_bytes()).expect("failed to write");
    let mut file = OpenOptions::new().append(true).open("Moses.txt").expect(
        "could not open file");
    file.write_all("\nHello user".as_bytes()).expect("failed to write");
    file.write_all("\nI just learnt how to append in Rust programming language\n".as_bytes()).expect(
        "failed to write");
    println!("file updated successfully");

    println!("Enter a text");
    let mut user_input =String::new();
    io::stdin().read_line(&mut user_input).expect("failed to input");
    file.write_all(user_input.as_bytes()).expect("failed to write");
}