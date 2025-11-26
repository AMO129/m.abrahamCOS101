use std::fs;


fn main(){
    let file = std::fs::File::create("Moses.txt").expect("failed to create file");
    fs::remove_file("Moses.txt").expect("file removal failed");
    println!("file removed successfully");


}