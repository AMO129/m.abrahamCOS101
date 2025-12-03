// declaring a struct defining dimensions of a rectangle
struct Rectangle {
    width:f32, 
    height:f32,
    //base:f32,


}

//logic to calculate rectangle's area
impl Rectangle {
    fn area(&self) -> f32 {
        //use the '.' operator to fetch the value of a field via self keyword

        self.width * self.height

        //0.5*(self.base * self.height)

    }
    
}

fn main (){
    // initiate the structure
    let small = Rectangle{
        width: 10.0,
        height: 20.0
        //base:0.0
    };

    //let triangle = Shape{
     //   base: 10.0,
     ///   height: 5.0,
    //    width: 0.0
   // };

   // display(rectangle);
   // display(triangle);


    //print the rectangle's area
   println!(" width is {} \n height is {} \n area of Rectangle is {}", 
       small.width, small.height, small.area());
}

//fn display(area:Shape){
  //  println!("Rectangle's width is: {} Rectangle's height is: {} Rectangle's base is: {}
    //    Triangle's base is: {} Triangle's height is: {} Triangle's width is: {}",area.width, area.height, area.base, 
       // area.base, area.height, area.width);
//}