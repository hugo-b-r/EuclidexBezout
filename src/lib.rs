struct Brique {
    briques: Vec<Brique>,
    nombres: Vec<f64>,
    type_operation: TypeOperation,
}

enum TypeOperation {
    Produit,
    Somme,
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
}