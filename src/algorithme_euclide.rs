use super::Brique;

struct AlgorithmeEuclide {
    lignes: Vec<Brique>,
    a: i64,
    b: i64,
}

fn division_euclidienne(a: i64, b: i64) -> Brique {
    let mut c;
    let mut d;
    if a < b {
        c = b;
        d = a;
    } else {
        c = a;
        d = b;
    }

    let quotient = c/d;
    let reste = c - d*quotient;
    Brique::Egalite(
        vec![
            Box::new(
                Brique::Entier(c)
            ),
            Box::new(
                Brique::Somme(
                    vec![
                        Box::new(
                            Brique::Produit(
                                vec![
                                    Box::new(Brique::Entier(d)),
                                    Box::new(Brique::Entier(quotient)),
                                ]
                            )
                        ),
                        Box::new(
                            Brique::Entier(reste)
                        )
                    ]
                )
            )
        ]
    )
}