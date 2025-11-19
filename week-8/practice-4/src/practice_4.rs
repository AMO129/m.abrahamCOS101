fn main(){

    //Name vactor
    let name = vec!["Mary","Ishaku","Ashlame","Iyigulu","Peaceful"];

    // Age vector
    let age = vec![16,20,35,67,38];

    print!("\nAge allocation:\n");

    //loop to iterate elements in vector
    for i in 0..age.len()
    {

        //iterating through i on the vector
        print!("{} is {} years old\n",name[i],age[i]);
    }
}