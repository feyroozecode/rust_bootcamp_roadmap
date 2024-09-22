use std::io;

pub fn start_function() { get_icm(); }


fn get_icm() {
    let mut weight = String::new();
    let mut size   = String::new();

    println!("Entree votre poids en Kg :");
    io::stdin().read_line(&mut weight).expect("Failed to readline");

    println!("Entree votre taille en M :");
    io::stdin().read_line(&mut size).expect("Failed to readline");

    println!("Calcul de l'ICM en cours..");

    let weight = weight.trim().to_string();
    let size = size.trim().to_string();

    let weight: f64 = match str_to_double(weight) {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let size: f64 = match str_to_double(size) {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            return ;
        }
    };

    let result = calculate_icm(weight, size);

    let final_result = result.unwrap_or(0.0);

    println!("votre ICM avec l'entree de {} kg et {} m = {:.2}", weight, size, final_result)

}
//  Entree votre poids en Kg : 57
//  Entree votre taille en M : 1.67
// Calcul de l'ICM en cours..
// votre ICM avec l'entree de 57 kg et 1.67 m = 20.44

fn calculate_icm(weight: f64, size: f64) -> Result<f64, String> {

    if size <= 0.0 {
        println!("Erreur veillez entree des valeur valide")
    }
    
    let icm = weight / ( size * size);

    Ok(icm)
    
}

fn str_to_double(input: String) -> Result<f64, String> {

    // conversion
   match input.parse::<f64>() {
       Ok(n) => Ok(n),
       Err(_e) => Err(format!("Failed to convert: '{}' to f64, error : {}", input, _e))
   }
}