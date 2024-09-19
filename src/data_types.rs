/**
 * A method about rust data types 
 */
pub fn run_data_types(){

    // Primitive data types or scaller data types

    // INTEGER
    let x: i32 = -45;
    let y: u64 = 245;

    println!("Signed Integer {}", x);
    print!("Unsigned Integer {} ", y);

    let e: i32 = 214483647;
    let i: i64 = 9223372036854775807;    

    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    // Double
    let height:f64 = 34.5;
    let weight: f64 = 19.5;

    println!("The height is {}", height);
    println!("The weight is {}", weight);
    
    let is_snowing: bool = true;
    println!("It is snowing ? {}", is_snowing);

    let letter: char = 'a';
    println!("Letters: {}", letter);

   
}