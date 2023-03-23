use crate::type_operation::TypeOperation;

#[derive(Clone)]
enum MembreBrique {
    Int(i32),
    Brique(Box<Brique>),
}

#[derive(Clone)]
pub struct Brique {
    pub expression: Vec<MembreBrique>,
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

    //utilisation de cette fonction avec grande attention, ne prend que deux
    // briques, une brique et un nombre, ou deux nombres
    pub fn developpe(self: Self) -> Result<Self, String> {
        if self.type_operation == TypeOperation::Produit {
            
            if self.briques.len() >= 2 && self.nombres.len() == 0 {
                //on fait le produit des deux premieres briques

                let mut output = Brique::new();
                output.type_operation = TypeOperation::Somme;

                for nombre in self.briques[0].nombres.iter() {
                    for nombre_bis in self.briques[1].nombres.iter() {
                        output.briques.push( Brique { 
                            briques: Vec::new(),
                            nombres: vec![*nombre, *nombre_bis],
                            type_operation: TypeOperation::Produit,
                        });
                    }
                }

                Ok( output )
                
            } else if self.briques.len() == 1 && self.nombres.len() == 1{
                //on fait le produit du premier nombre et de la premiere brique
                
                let mut output = Brique::new();
                output.type_operation = TypeOperation::Somme;
                for nombre in self.briques[0].nombres.iter() {
                    output.briques.push( Brique {
                        briques: Vec::new(),
                        nombres: vec![*nombre, self.nombres[0]],
                        type_operation: TypeOperation::Produit,
                    })
                }
                
                Ok( output )
            } else if (self.nombres.len() + self.briques.len()) > 2 {
                Err( "too much briques or numbers".to_string() )
            } else {
                Ok ( self )
            }
        } else {
            Err("not a product !!".to_string())
        }

    }
}
