// declaring a struct defining dimensions of a rectangle
struct Shape {
    name:String,
    width:f32, 
    height:f32,
    base:f32,


}

//logic to calculate rectangle's area
impl Shape {
    fn area(&self) -> f32 {
        //use the '.' operator to fetch the value of a field via self keyword
        if self.name == "Rectangle"{
            //formula for rectangle area
             self.width * self.height

        } else if self.name == "Triangle" {
            //formula for triangle area
            0.5 * (self.base * self.height)
            
        }else {
            //return for safety
            0.0
        }

       

        

    }
    
}

fn main (){
    // initiate the structure
    let rectangle = Shape{
        name:String::from("Rectangle"),
        width: 10.0,
        height: 20.0,
        base:0.0
    };

    let triangle = Shape{
        name:String::from("Triangle"),
        base: 10.0,
        height: 5.0,
        width: 0.0
    };

    display(&rectangle);
    display(&triangle);


    //print the rectangle's area
   // println!(" width is {} \n height is {} \n area of Rectangle is {}", 
     //   small.width, small.height, small.area());
}

fn display(area:&Shape){
    println!(" The shape is a {}\n Width is: {}\n Height is: {}\n Base is: {}\n
        Clculated area is: {}", area.name, area.width, area.height, area.base,area.area());
}