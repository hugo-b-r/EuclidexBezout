use crate::type_operation::TypeOperation;
use crate::brique::Brique;
use crate::generic::division_euclidienne;



#[derive(Clone)]
pub struct AlgoEuclide {
    pub a: i32,
    pub b: i32,
    pub lignes: Vec<Brique>, 
}

impl AlgoEuclide {
    pub fn new(a: i32, b:i32) -> Self {
        AlgoEuclide {
            a: a,
            b: b,
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
}
