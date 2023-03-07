struct Brique {
    briques: Vec<Brique>,
    nombres: Vec<f64>,
    type_operation: TypeOperation,
}

impl Brique {
    fn new() -> Self {
        Brique {
            briques: Vec::new(),
            nombres: Vec::new(),
            type_operation: TypeOperation::default(),
        }
    }
}


enum TypeOperation {
    Produit,
    Somme,
}

impl Default for TypeOperation {
    fn default() -> Self{
        TypeOperation::Somme
    }
}

struct AlgoEuclide {
    a: i32,
    b: i32,
    lignes: Vec<Brique>, 
}

impl AlgoEuclide {
    fn new(a: i32, b:i32) -> Self {
        AlgoEuclide {
            a,
            b,
            lignes: Vec::new(),
        }
    }
}

fn division_euclidienne(mut a: i32, mut b: i32) -> (i32, i32) {
    if b > a {
        b += a;
        a = b - a;
        b = b - a;
    }
    let quotient = a/b;
    let reste = a-(b*quotient);
    (quotient, reste)
}


#[cfg(test)]
mod tests {
    use crate::AlgoEuclide;
    #[test]
    fn test_algo_euclide() { //teste la creation algorithem d'euclide
        let algorithme = AlgoEuclide::new(0, 0);
        let result = if algorithme.a == 0 && algorithme.b == 0 {
            true
        } else {
            false
        };
        assert_eq!(result, true);
    }

    #[test]
    fn test_algo_euclide_2() {
        let algorithme = AlgoEuclide::new(1, 2).compute();
        assert_eq!(algorithme.print(), "2 = 1 x 2 + 0".to_string())
    }
}