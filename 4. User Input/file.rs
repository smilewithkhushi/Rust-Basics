//PRELUDE is a list of functionalities added to rust code automatically without us having to import it from STL.

//to take user input, the library package has to be imported for user input
//CRATE (is a package or library) contains module (specific piece of functionality)

use std::io;
//:: is a package separator operator

fn main(){
    println!("__TYPE CASTING__");

    println!("Enter any string here :: ");
    //variable to store string input
    let mut input = String::new();

    //collects input from the user
    //& is the reference of location/variable which is immutable by default
    //io::stdin() helps the user to give input
    //read_line() helps to read stream of characters and store it in input variable
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    //expect() function is catching error that occur
    println!("\n Your input is -> {}", input);



}