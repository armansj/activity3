enum Direction {
    Up,
    Down,
    Left,
    Right,
}
enum Color{
    Red,Green,Blue,White,Black,
}


struct GroceryItem{

    stock : i32,
    price : f64,
}

enum Flavors{

    Salty,
    Suger,
    Hot,


}

struct Drink{
    ounces: f64,
    flavors: Flavors,

}

fn main() {


    //activity 3

let message : bool = true;

if message == true {

    println!("hello");
    
}else if  message == false {
    println!("goodbye");
    
}



   // activity 4


   let num = 19;

   if num> 5{
    println!(" number is higher than 5");
   }else if num < 5{
    println!(" number is lower than 5");
   }else if num==5{
    println!("number is equal to 5");
   }

   // activity 5 match expression in rust

   let some_int = 4;
   match some_int {
    1=> println!("its 1"),
    2=> println!("its 2 "),
    3=> println!("its 3"),
    _=> println!("its something else"),
       
   }

   let decition = true;

   match decition {

    true => println!("its ture"),
    false => println!("its a false "),
    _ => println!(" its about text"),
       
   }

   // activity match expression b

   let num = 4;

   match  num {

    1 => println!(" its number 1"),
    2=> println!(" its number 2"),
    3=> println!(" its number 3"),
    _ => println!(" gattcha its your num!"),
       
   }

 
   // loop in rust
   let mut num = 1;

   loop {

    
    println!("{:?}",num);
    num = num + 1;

    if num>4{
        break;
    }

}

    // while loop
    let mut  i =1;
    while i < 3 {

        println!("{:?}",i);
        i=i+1;
        
    }


    // while activity 6 

    let mut num = 5;

    while num >= 1 {

        print!("{:?}",num);
        num = num-1;
       
        
    }
    println!("Done!");

    // enum in rust

    let go = Direction::Left;
    match go {
        Direction::Up => println!("go up"),
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
        Direction::Down => println!(" go down"),

        
    }

    // enum in activity 7 


    fn my_color(color: Color){
        match color {
            Color::Black => println!("Color is black"),
            Color::White => println!("Color is white"),
            Color::Blue => println!("Color is blue"),
            Color::Green => println!("Color is green"),
            Color::Red => println!("Color is red"),
        }
      

    }

    my_color(Color::Black);

    // struct in rust 

    let cereal = GroceryItem{

        stock: 10,
        price : 2.99,
    };

    println!(" stock {:?} and price {:?}",cereal.stock,cereal.price);


    // struck activity 8

    fn my_drink(drinks:Drink){
        match drinks.flavors {
            Flavors::Hot => println!("Hot"),
            Flavors::Salty => println!(" Salty"),
            Flavors::Suger => println!(" Suger"),
            
        }

        println!("{:?}",drinks.ounces);



    }

    let sweet = Drink{
        ounces : 10.0,
        flavors : Flavors::Suger,

    };
    my_drink(sweet);





 

 



    }










       




