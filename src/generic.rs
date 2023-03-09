use crate::algorithme_euclide::AlgoEuclide;



pub fn division_euclidienne(mut a: i32, mut b: i32) -> (i32, i32) {
    if b > a {
        (a, b) = (b, a);
    }
    let quotient = a/b;
    let reste = a-(b*quotient);
    (quotient, reste)
}

pub fn pgcd(a: i32, b: i32) -> i32 {
    let mut algorithme = AlgoEuclide::new(a, b);
    algorithme = algorithme.compute();
    algorithme.lignes[(algorithme.lignes.len()-1)].nombres[0] as i32
}
