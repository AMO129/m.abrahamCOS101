use std::fs::OpenOptions;
use std::io::Write;

fn main(){

    let mut file = std::fs::File::create("Moses.txt").expect("failed to create file");
    file.write_all("\nMY RUST PROGRAMMING LANGUAGE JOURNEY".as_bytes()).expect("failed to write");
    let mut file = OpenOptions::new().append(true).open("Moses.txt").expect(
        "could not open file");
    file.write_all("\nHello user".as_bytes()).expect("failed to write");
    file.write_all("\nI just learnt how to append in Rust programming language".as_bytes()).expect(
        "failed to write");
    println!("file updated successfully");
}