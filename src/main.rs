use rustfft::{FftPlanner, num_complex::Complex};


// fonction test pour calculer la somme d'un vecteur
fn sum_vec(xs: &[i32]) -> i32 {
    xs.iter().sum()
}

// fonction test pour trouver max d'un gecteur
fn spot_max(xs:&[i32]) -> i32 {
    let max = *xs.iter().max().unwrap();
    max
}

// fonction test pour trouver min d'un gecteur
fn spot_min(xs:&[i32]) -> i32 {
    let min = *xs.iter().min().unwrap();
    min
}


// fonction test pour compute une deriver
fn derive(signal: &[i32]) -> Vec<f32> {

    // si pas assez de point on compute pas
    if signal.len() < 3 {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(signal.len());

    result.push(0.0);

    for i in 1..signal.len() - 1 {
        result.push((signal[i + 1] - signal[i - 1]) as f32 / 2.0);
    }

    result.push(0.0);

    result

}

// Run une FFT 
fn fft_1d(signal: &[f32]) -> Vec<Complex<f32>> {
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(signal.len());

    // Copie du signal réel dans un buffer complexe
    let mut buffer: Vec<Complex<f32>> = signal
        .iter()
        .map(|&x| Complex { re: x, im: 0.0 })
        .collect();

    // Calcul FFT
    fft.process(&mut buffer);

    buffer
}


fn main() {

    // tadam 
    let i:i32 = 22;
    let nombres: Vec<i32> = vec![10, 20, 30, 20, 10];
    let signal = vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0];
    
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
    let m:i32 = spot_max(&nombres);
    println!("Avec un max de {}", m);

    // Trouver le min
    let min:i32 = spot_min(&nombres);
    println!("Et un min de {}", min);

    // calcul de la dérivée (pas de temps = 1)
    let deriv = derive(&nombres);
    println!("On peut faire une dérivée {:?}", deriv);

    // FFT
    let spectrum = fft_1d(&signal);
    println!("calcul d'un FFT");
    for (i, c) in spectrum.iter().enumerate() {
        println!(
            "bin {:>2} | re = {:>6.3}, im = {:>6.3}, magnitude = {:>6.3}",
            i,
            c.re,
            c.im,
            c.norm()
        );
    }

    // Affichage du spectrogam

}
