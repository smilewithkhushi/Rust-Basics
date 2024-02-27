//DATA TYPES IN RUST

//1. Scaler types : a single value. Eg: integers
//2.  Compound types : multiple values. Eg: Tuple,

fn main() {

    println!("__PRINTING DATA TYPES IN RUST___");

    //SIGNED INTEGERS (i)
    //i32 is the default value of integers in rust
    //i8 - i16-  i32 - i64 - i128 are all signed integers
    let x: i32 = -2;
    println!("The value of signed x is : {}", x);

  
    //UNSIGNED INTEGERS (u)
    //u8 - u16- u32
    let y: u32 = 45;
    println!("The value of unsinged y is : {}", y);

    //FLOATING NUMBERS (f)
    //f32 - f64
    let floating_point = 10.92;
    println!("The value of floating_point is : {}", floating_point);

    //BOOLEAN (true/1 or false/0)
    let true_or_false = false;
    let true_or_false:bool = true;
    println!("The value of true_or_false is : {}", true_or_false);

    //CHARACTER 
    let letter: char = 'A';
    println!("The value of letter is : {}", letter);


    //TUPLE : fixed length sequence of elements
    //tuple is immutable
    //accessing tuple: tuple.idx
    let tup = (1,2,3,4,5);
    let tup2 = (1,2.3 , "Hello", 'A');
    //the following tuples have different data type
    let tup3 : (i8, bool, char) = (3, true, 'A');
    let tup4 : (i32, bool, char) = (8, true, 'A');
    println!("The first value of tuple named tup is {}", tup.0);
    println!("The second value of tuple named tup2 is {}", tup.1);
    
    //ARRAY
    //accessing arrays : arr[idx]
    let arr = [1,2,3,4,5];
    //creates an array with data type and size
    let mut arr1 : [i32; 5] = [22,33,44,55,66];
    println!("The value of array : {}", arr[0]);
    println!("The value of array1 : {}", arr1[0]);


    //VARIABLES ASSIGNED TO EACH OTHER
    //When one variable is assigned to other, all its properties, including the value & the data type of parent variable is copied as well
    let var1: u8 = 23;
    let var2 = var1;
    println!("The value of var1 is {} and var2 is {}", var1,var2);


}
