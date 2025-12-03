fn main() {

    // a list of nos
    let x = vec![100, 200, 300];
    borrow_vector(&x); // passing reference

    println!("Values from main() x[0]={}",x[0]);
    println!("**************************");

}


fn borrow_vector(z:&Vec<i32>){

    println!("**********************");
    println!("Inside borrow_vector{:?} \n",z);
    println!("-----------------------");
}
