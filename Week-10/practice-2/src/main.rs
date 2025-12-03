fn main(){

    let v = vec![10,40,29];
    // vector v owns the object in heap

    let v2 = v;

    //display (v2);
    display (v2.clone());
    // v2 is moved to display and v2 is invalidated

    println!("In main {:?}",v2 );
    // v2 is no longer usable here
}

fn display(v:Vec<i32>){
    println!("Inside display{:?}",v);
}

// FOR THAT CODE TO RUN PROPERLY THE v2 MUST BE CLONED AS v2.clone()