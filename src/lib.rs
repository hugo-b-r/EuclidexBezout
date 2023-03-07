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

#[cfg(test)]
mod tests {
    #[test]
    fn test_algo_euclide() { //teste la creation algorithem d'euclide
        let algorithme = AlgoEuclide::new();
        if algorithme.a == 0 && algorithme.b == 0 && algorithme.lignes == Vec::default() {
            let result = true;
        } else {
            let result = false;
        }
        assert!(result, true);
    }
}