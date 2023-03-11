use crate::type_operation::TypeOperation;
use crate::brique::Brique;
use crate::generic::{DivisionEuclidienne, division_euclidienne, pgcd};



#[derive(Clone)]
pub struct AlgoEuclide {
    pub lignes: Vec<DivisionEuclidienne>, 
}

impl AlgoEuclide {
    pub fn new(a: i32, b:i32) -> Self {
        AlgoEuclide {
            lignes: Vec::new(),
        }
    }

    pub fn compute(mut self: Self) -> Self {
        
        if self.b > self.a {
            let c = self.a;
            self.a = self.b;
            self.b = c;
        }
        let (mut quotient, mut reste) = division_euclidienne(self.a, self.b);
        let mut ligne: Brique;
        let mut output = self.clone(); 
        while {
            ligne = Brique {
                briques: vec![Brique {
                    briques: Vec::new(),
                    nombres: vec![output.b as f64, quotient as f64],
                    type_operation: TypeOperation::Produit,
                }],
                nombres: vec![reste as f64],
                type_operation: TypeOperation::Somme,
            };
            output.lignes.push(ligne);

            output.a = output.b;
            output.b = reste;
            if reste != 0 {
                (quotient, reste) = division_euclidienne(output.a, output.b);
            }
            reste != 0
        } {}
        output.a = self.a;
        output

    }

    pub fn print(self: Self) -> String{
        let mut a = self.a;
        let mut b = self.lignes[0].briques[0].nombres[0] as i32;
        let mut quotient = self.lignes[0].briques[0].nombres[1] as i32;
        let mut reste = self.lignes[0].nombres[0] as i32;
        let mut output = String::default();
        for ligne in self.lignes {
            output.push_str(format!("{} = {} x {} + {}", a, b, quotient, reste).as_str());
            a = b;
            b = reste as i32;
            quotient = ligne.briques[0].nombres[1] as i32;
            reste = ligne.nombres[0] as i32;
        }
        output
    }

    pub fn etendu(self: Self) -> Result<Vec<Brique>, String> {
        if pgcd(self.a, self.b) != 1 {
            Err("Numbers not prime between them. ".to_string())
        } else {
            let mut output = Vec::new();
            let mut i = 0;
            while self.lignes[i].nombres[0] != 1. {
                i += 1; //on positionne i à la ligne interressant, celle à laquelle le reste = 1
            }

            let mut ligne = Brique {
                briques: vec![
                    self.lignes[i].briques[0].clone()
                ],
                nombres: vec![self.lignes[i-1].briques[0].nombres[0]],
                type_operation: TypeOperation::Somme,
            };
            ligne.briques[0].nombres[0] = -ligne.briques[0].nombres[0];

            while i > 0 {
                //3 etapes;
                
                //etape 1: on remplace par dividende - diviseur x quotient a i -1 
                
                ligne = remplacer_reste_dividende_moins_diviseur_fois_quotient(&mut ligne, &self, i);
                output.push(ligne.clone());


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