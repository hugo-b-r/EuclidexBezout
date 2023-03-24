/*
    2023 Hugo Berthet-Rambaud
    Licencié sous la licence MIT. La districution de ce code sans cette licence
    ne peut se faire.

    --- Pour l'humanité, son bonheur et sa gloire ---
 */


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
            ligne.get(1).unwrap().get(1).unwrap().valeur_entiere().unwrap() != 0
        } {}
             
    }

    fn etendu(&self) -> Result<AlgoEuclideEtendu, String> {
        if pgcd(self.dividende, self.diviseur) == 1 {
            //on positionne i sur l'avant derniere ligne:
            let i: usize;
            while i != self.lignes.len()-2 as usize {
                i += 1;
            }
            let dividende = self.lignes[i].entier(0).unwrap();
            let produit = self.lignes[i].produit(0).unwrap();
            let ligne = Brique::Egalite(vec![
                Box::new(Brique::Entier(1)),
                Box::new(
                    Brique::Difference(vec![
                        Box::new(Brique::Entier(dividende)),
                        Box::new(produit)
                    ])
                )
            ])

            //3 etapes: 
            //1: on remplace
            //2: on developpe
            //3 on rassemble
            Ok( ligne)

        } else {
            Err(String::from("nombres non premiers entre eux"))
        }
    }
}

fn pgcd(dividende: i64, diviseur: i64) -> i64 {
    let mut algorithme = AlgorithmeEuclide {
        lignes: Vec::new(),
        dividende: dividende,
        diviseur: diviseur,
    };
    algorithme.calcule();

    //on veut le reste de l'avant derniere ligne
    let ligne = algorithme.lignes[algorithme.lignes.len() - 1];
    let pgcd = ligne.get(1).unwrap().get(1).unwrap().valeur_entiere().unwrap();
    pgcd
}
