
//Rust is a statically and strongly typed programming language

fn main(){
    //implicit type assignment or decision
    //at compile time, x is assigned as type integer
    //x=5; //error: cannot assign twice to immutable variable
    let x=4;
    println!("The value of x is : {}", x);

    //all variables in Rust are immutable and they cannot be updated
    let y = 5;
    println!("The value of y is : {}", y);

    //mutable variable
    let mut z = 6;
    println!("The value of z is : {}", z);

    z=z+12;
    println!("Updated value of z is : {}", z); 

    //creating your own private/interior scope
    {
        let x= 2;
        println!("The value of x inside the scope : {}", x);
    }   

    println!("The value of x outside scope is : {}", x);

    //Constants are created with const keyword and must have capital letters with a data type assigned
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("The value of SECONDS_IN_MINUTE is {}", SECONDS_IN_MINUTE);

}
