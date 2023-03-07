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
