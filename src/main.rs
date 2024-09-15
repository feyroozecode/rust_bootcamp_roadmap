fn main() {
    
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

    // Arrays 
    let numbers: [i32; 8] = [1,2,3,4,5,6,7,8];

    println!("Numer Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Mango"];

    println!("Frutis Array {:?}", fruits );
    

    // Tuples
    let human: (String, i32, bool) = ("Moussa".to_string(), 24, false);
    
    println!("Himan tuple {:?}", human );

    let my_mix_tuples = ("Asma", 25, true, [2, 3, 4, 5]);

    println!("My Mix tuples {:?}", my_mix_tuples);

    // Slices
    let mumbers_slices: &[i32] = &[1,2,3,4,5];

    println!("Number Slice: {:?}", mumbers_slices);

    let animals: &[&str] = &["Cat", "Dog", "Elephan"];

    println!("Animals {:?}", animals)

}
