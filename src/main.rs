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






}
