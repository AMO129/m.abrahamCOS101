fn main(){

    let v = vec![101,201,450,3000];
    // vector v owns the objects in heap

    //only a single variable owns the heap memory at any given time 

    //let v2 = v; ->To correct the error make it 
    let v2 = v.clone();

    //here two variables own the heap value which is incorrect.
    //two ponters to the same content is not allowed in rust

    //Rust is very smart in terms of memory access, so it detects a race condition 
    // as two variables point to same heap

    println!("{:?}",v);
    //println!("{:?}",v2); just to prove that rust made a copy and not transfer
}