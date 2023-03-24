use super::Brique;

struct AlgorithmeEuclide {
    lignes: Vec<Brique>,
    dividende: i64,
    diviseur: i64,
}

struct AlgoEuclideEtendu {
    lignes: Vec<Brique>,
    algorithme_original: AlgorithmeEuclide,
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

impl AlgorithmeEuclide {
    fn calcule(&mut self) {
        let mut ligne: Brique;
        let mut dividende = self.dividende;
        let mut diviseur = self.diviseur;
        while {
            ligne = division_euclidienne(dividende, diviseur);
            dividende = ligne.get(1).unwrap().get(0).unwrap().get(0).unwrap().valeur_entiere().unwrap();
            diviseur = ligne.get(1).unwrap().get(1).unwrap().valeur_entiere().unwrap();
            self.lignes.push((ligne).clone());
            ligne.get(1).unwrap().get(1).unwrap().valeur_entiere().unwrap() != 1
        } {}
             
    }
}