// declaring structure
struct Laptops {
    HP: u32,
    IBM: u32,
    Toshiba:u32,
    dell:u32
}

// logic for calculating total cost

impl Laptops {
    fn cost(&self)->u32{
        //using the '.' operator to fetch value via self keyword
        3*(self.HP + self.IBM + self.Toshiba + self.dell)
    }

}

fn main(){
        // initiate struct

        let buyer = Laptops{
            HP: 650_000,
            IBM:755_000,
            Toshiba:550_000,
            dell:850_000
        };

        //print the total cost
        println!("Total cost is {}\n",buyer.cost());
    }