use crate::algorithme_euclide::AlgoEuclide;

#[derive(Clone)]
pub struct DivisionEuclidienne {
    pub dividende: i32,
    pub diviseur: i32,
    pub quotient: i32,
    pub reste: i32,
}

impl DivisionEuclidienne {
    fn compute(self: Self) {
        self.quotient = self.dividende/self.diviseur;
        self.reste = self.dividende - (self.diviseur * self.quotient);
    }

    pub fn print(self: Self) -> String {
        let text: String = format!("{} = {} x {} + {}", self.dividende, self.diviseur, self.quotient, self.reste);
        text
    }
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
