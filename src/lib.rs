#[derive(Clone)]
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

    fn print(self: Self) -> String{
        let mut output: String = "".to_string();
        let symbol = match self.type_operation {
            TypeOperation::Somme => " + ",
            TypeOperation::Produit => " x ", 
        };
        for brique in self.briques {
            output.push_str("(");
            output.push_str(brique.print().as_str());
            output.push_str(")");
            output.push_str(format!("{}", symbol).as_str());
        }
        for nombre in self.nombres {
            output.push_str(format!("{}", nombre).as_str());
            output.push_str(format!("{}", symbol).as_str());
        }
        output = output[0..(output.len()-3)].to_string();
        output
    }
}

#[derive(Clone)]
enum TypeOperation {
    Produit,
    Somme,
}

impl Default for TypeOperation {
    fn default() -> Self{
        TypeOperation::Somme
    }
}

#[derive(Clone)]
pub struct AlgoEuclide {
    a: i32,
    b: i32,
    lignes: Vec<Brique>, 
}

impl AlgoEuclide {
    fn new(a: i32, b:i32) -> Self {
        AlgoEuclide {
            a: a,
            b: b,
            lignes: Vec::new(),
        }
    }

    pub fn compute(mut self: Self) -> Self {
        if false {
            if self.b > self.a {
                (self.a, self.b) = (self.b, self.a);
                println!("{}|{}", self.a, self.b);
            }
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


fn division_euclidienne(mut a: i32, mut b: i32) -> (i32, i32) {
    if b > a {
        (a, b) = (b, a);
    }
    let quotient = a/b;
    let reste = a-(b*quotient);
    (quotient, reste)
}


#[cfg(test)]
mod tests {
    use crate::{AlgoEuclide, TypeOperation};
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
        let mut algorithme: AlgoEuclide = AlgoEuclide::new(2, 1);
        algorithme = algorithme.compute();
        assert_eq!(algorithme.print(), "2 = 1 x 2 + 0".to_string())
    }

    use crate::Brique;
    #[test]
    fn brique_print() {
        let brique = Brique {
            briques: vec![ Brique {
                briques: Vec::new(),
                nombres: vec![2.0, 5.0],
                type_operation: TypeOperation::Produit,
            }],
            nombres: vec![5.0, 2.0],
            type_operation: TypeOperation::Produit,
        };
        assert_eq!(brique.print(), "(2 x 5) x 5 x 2");
    }}