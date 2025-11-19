//Method to print the get value

fn value(n:Option<&char>)
{
    println!("Element of vector {:?}",n);
}

fn main(){

    let h = vec!['O','K','I','O','G','H','E','N','E',];

    let mut input1 = String::new();
    println!("\nEnter an inbox value btw (0 - 8)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

    //index is the non negative value which is smaller than the size of the vector
    let index:usize = input1.trim().parse().expect("Invalid input");

    // getting value t given index value
    let ch: Option<&char> = h.get(index);
    value(ch);

}