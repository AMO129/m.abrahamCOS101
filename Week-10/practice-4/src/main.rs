fn main(){

    // a list of nos
    let v = vec![15, 34, 21, 55];
    //print_vector(v);
    print_vector(v.clone());
    println!("inside main {}",v[0]); //this line will gove an error because v has been transferred to x

}

fn print_vector(x: Vec<i32>){

    println!("Inside print_vector function {:?}",x);
}