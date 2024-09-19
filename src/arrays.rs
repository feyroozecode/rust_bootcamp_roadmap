
pub fn run_arrays(){
     // Arrays 
     let numbers: [i32; 8] = [1,2,3,4,5,6,7,8];
     println!("Numer Array: {:?}", numbers);
 
     let fruits: [&str; 3] = ["Apple", "Banana", "Mango"];
     println!("Frutis Array {:?}", fruits );
     
     // Tuples
     let human: (String, i32, bool) = ("Moussa".to_string(), 24, false);
 
     println!("Himan tuple {:?}", human );
 
     let my_mix_tuples: (&str, i32, bool, [i32; 4]) = ("Asma", 25, true, [2, 3, 4, 5]);
     println!("My Mix tuples {:?}", my_mix_tuples);
 
     // Slices
     let mumbers_slices: &[i32] = &[1,2,3,4,5];
     println!("Number Slice: {:?}", mumbers_slices);
 
     let animals: &[&str] = &["Cat", "Dog", "Elephan"];
     println!("Animals {:?}", animals);
 
     let books = &[&"IT".to_string(), &"Sahih AlBukhari".to_string(), &"Muslim".to_string()];
     println!("Books : {:?}", books);
 
     // Strings Vs String Slices (&str)
     // Strings [ growable, mutable, owned string type ]
     let mut stone_cold:  String = String::from("Salam Aleykoum");    
     println!("Original Message Says: {}", stone_cold);
     stone_cold.push_str(" Ahlan");
     println!("After Push Message Says: {}", stone_cold);
 
     // B- &str (String Slices)
     let string: String = String::from("Ahlan wa Sahlan");
     let slice: &str = &string[0..8];
     println!("Slices values: {}", slice);
 
}