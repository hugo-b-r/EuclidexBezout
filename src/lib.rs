#[derive(Clone)]
pub struct Brique {
    pub briques: Vec<Brique>,
    pub nombres: Vec<f64>,
    pub type_operation: TypeOperation,
}

impl Brique {
    pub fn new() -> Self {
        Brique {
            briques: Vec::new(),
            nombres: Vec::new(),
            type_operation: TypeOperation::default(),
        }
    }

    pub fn print(self: Self) -> String{
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

    fn developpe(self: Self) -> Self {
        if self.type_operation == TypeOperation::Produit {
            if self.briques.len() >= 2 {
                
            } else if self.briques.len() == 1 {

            } else {
                self
            }
        } else {
            eprintln!("not a product !!");
            self
        }

    }
}

#[derive(Clone)]
pub enum TypeOperation {
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


fn division_euclidienne(mut a: i32, mut b: i32) -> (i32, i32) {
    if b > a {
        (a, b) = (b, a);
    }
    let quotient = a/b;
    let reste = a-(b*quotient);
    (quotient, reste)
}

pub fn pgcd(a: i32, b: i32) -> i32 {
    let mut algorithme = AlgoEuclide::new(a, b);
    algorithme = algorithme.compute();
    algorithme.lignes[(algorithme.lignes.len()-1)].nombres[0] as i32
}


