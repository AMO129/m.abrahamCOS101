use std::io::Read;


fn main(){
    let _file = std::fs::File::create("welcome_message.txt").expect("creation failed");
    let mut file = std::fs::File::open("welcome_message.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read");
    println!("{}",contents);
}