fn main() {

    // tadam 
    let i:i32 = 22;
    let nombres: Vec<i32> = vec![10, 20, 30];
    
    // intro
    println!("Hello Jimmy du {}, ça va faire un bail qu'on a pas fait de language compilé hein ? ", i);

    println!("Bon c'est pas compliqué, on a un vecteur avec les valeurs:");
    for valeur in nombres.iter() {
        println!("- {}", valeur);
    }
}
