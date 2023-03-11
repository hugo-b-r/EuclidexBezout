use crate::algorithme_euclide::AlgoEuclide;

struct DivisionEuclidienne {
    dividende: i32,
    diviseur: i32,
    quotient: i32,
    reste: i32,
}

pub fn division_euclidienne(mut dividende: i32, mut diviseur: i32) -> DivisionEuclidienne {
    if diviseur > dividende {
        (dividende, diviseur) = (diviseur, dividende);
    }
    let quotient = dividende/diviseur;
    let reste = dividende-(diviseur*quotient);
    DivisionEuclidienne {
        dividende,
        diviseur,
        quotient,
        reste,
    }
}

pub fn pgcd(a: i32, b: i32) -> i32 {
    let mut algorithme = AlgoEuclide::new(a, b);
    algorithme = algorithme.compute();
    algorithme.lignes[(algorithme.lignes.len()-1)].nombres[0] as i32
}
