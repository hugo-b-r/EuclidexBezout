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
}


enum TypeOperation {
    Produit,
    Somme,
}

impl Default for TypeOperation {
    fn default() -> Self{
        TypeOperation::Somme
    }
}

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
        let (mut quotient, mut reste) = division_euclidienne(self.a, self.b);
        let mut ligne = Brique::new();
        let mut output = self; 
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
        output

    }

    pub fn print(self: Self) -> String{
        let mut a = self.a;
        let mut b = self.b;
        let mut quotient = self.lignes[0].briques[0].nombres[1];
        let mut reste = self.lignes[0].nombres[0];
        let mut output = String::default();
        for ligne in self.lignes {
            output.push_str(format!("{} = {} x {} + {}", a, b, quotient, reste).as_str());
            a = b;
            b = reste as i32;
            quotient = ligne.briques[0].nombres[1];
            reste = ligne.nombres[0];
        }
        output
    }
}


fn division_euclidienne(mut a: i32, mut b: i32) -> (i32, i32) {
    if b > a {
        b += a;
        a = b - a;
        b = b - a;
    }
    let quotient = a/b;
    let reste = a-(b*quotient);
    (quotient, reste)
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

    #[test]
    fn test_algo_euclide_2() {
        let mut algorithme: AlgoEuclide = AlgoEuclide::new(1, 2);
        algorithme = algorithme.compute();
        assert_eq!(algorithme.print(), "2 = 1 x 2 + 0".to_string())
    }
}