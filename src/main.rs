
// fonction test pour calculer la somme d'un vecteur
fn sum_vec(xs: &[i32]) -> i32 {
    xs.iter().sum()
}


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

    // calcul la somme
    let s:i32 = sum_vec(&nombres);
    println!("Ce qui nous donne une somme : {}", s);

    // Trouver le max

    // Trouver le min

    // calcul de la dérivée (pas de temps = 1)

    // calcul de la dérivée (pas de temps != 1)
}
