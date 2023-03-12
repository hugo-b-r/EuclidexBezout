use crate::type_operation::TypeOperation;
use crate::brique::Brique;
use crate::generic::{DivisionEuclidienne, division_euclidienne, pgcd};



#[derive(Clone)]
pub struct AlgoEuclide {
    pub a: i32,
    pub b: i32,
    pub lignes: Vec<DivisionEuclidienne>, 
}

struct LigneAlgoEtendu {
    produit_1: Vec<i32>,
    produit_2: Vec<i32>,
}

impl AlgoEuclide {
    pub fn new(a: i32, b:i32) -> Self {
        AlgoEuclide {
            a: a,
            b: b,
            lignes: Vec::new(),
        }
    }

    pub fn compute(self: &mut Self) {
        
        if self.b > self.a {
            let c = self.a;
            self.a = self.b;
            self.b = c;
        }
        let mut diviseur_actuel: i32 = self.a.clone();
        let mut dividende_actuel: i32 = self.b.clone();
        self.lignes = Vec::new();
        self.lignes.push(division_euclidienne(dividende_actuel, diviseur_actuel));
        while self.lignes[self.lignes.len()].reste != 0 {
            dividende_actuel = diviseur_actuel;
            diviseur_actuel = self.lignes[self.lignes.len()].quotient;
            self.lignes.push(division_euclidienne(dividende_actuel, diviseur_actuel));
        }
    }

    pub fn print(self: Self) -> String{
        let mut text: String;
        for ligne in self.lignes {
            text.push_str(ligne.print().as_str());
        }
        text
    }

    pub fn etendu(self: Self) -> Result<Vec<Brique>, String> {
        if pgcd(self.a, self.b) != 1 {
            Err("Numbers not prime between them. ".to_string())
        } else {
            let mut output = Vec::new();
            let mut i = 0;
            while self.lignes[i].reste != 1 {
                i += 1; //on positionne i à la ligne interressant, celle à laquelle le reste = 1
            }
        
    /*        let mut ligne = Brique {
                briques: vec![
                    self.lignes[i].briques[0].clone()
                ],
                nombres: vec![self.lignes[i-1].briques[0].nombres[0]],
                type_operation: TypeOperation::Somme,
            };
                ligne.briques[0].nombres[0] = -ligne.briques[0].nombres[0];
    */
            
            while i > 0 {
                //3 etapes;
                
                //etape 1: on remplace par dividende - diviseur x quotient a i -1 
                
                //ligne = remplacer_reste_dividende_moins_diviseur_fois_quotient(&mut ligne, &self, i);
                //output.push(ligne.clone());


                //etape 2: on developpe
                //etape 3: on rassemble

            }
            
            Ok( output )
        }

    }
}

pub fn remplacer_reste_dividende_moins_diviseur_fois_quotient(
    brique: &mut Brique,
    algorithme: &AlgoEuclide,
    rang: usize
) -> Brique {
    
    let mut produit: Brique = algorithme.lignes[rang].clone();
    let mut resultat = brique.clone();
    resultat.nombres.remove(0);

    produit.nombres[0] = -produit.nombres[0];

    let replacing = Brique {
        briques: vec![
            produit
        ],
        nombres: vec![algorithme.lignes[rang-1].briques[0].nombres[0]],
        type_operation: TypeOperation::Somme,
    };

    resultat.briques.push(replacing);
    
    brique.clone()
}