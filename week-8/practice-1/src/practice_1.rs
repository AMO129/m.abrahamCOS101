fn main(){

    // USING Vec::new()
    let v : Vec<i64> = Vec::new();

    // Printing the size of vector
    println!("\nThe length of Vec::new is: {}",v.len());

    // Using macro
    let v = vec!["Okioghene","Mahfuz","Kennedy","Miracle","Moses","Elijah"];

    // printing the size of vector
    println!("\nThe length of vec macro is: { }",v.len());
}