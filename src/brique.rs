use crate::brique::Brique::*;

pub enum Brique {
    Entier(i64),
    Produit(Vec<Box<Self>>),
    Somme(Vec<Box<Self>>),
    Difference(Vec<Box<Self>>),
    Division(Vec<Box<Self>>),
    DivisionEuclidienne(Box<Self>, Box<Self>, Box<Self>),
}

struct IterBrique<'a> {
    inner: &'a Brique,
    pos: usize,
}

impl <'a> Iterator for IterBrique<'a> {
    type Item = &'a Brique;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos>= self.inner.len() {
            None
        } else {
            self.pos += 1;

            Some(&self.inner.get(self.pos -1))
        }
    }
}

impl Brique {
    fn len(&self) -> usize {
        match self {
            Entier(i64) => 1 as usize,
            Produit(vector) => vector.len(),
            Somme(vector) => vector.len(),
            Difference(vector) => vector.len(),
            Division(vector) => vector.len(),
            DivisionEuclidienne(_, _, _) => 3 as usize,
        } 
    }

    fn get(&self, rang: usize) -> Self {
        match self {
            Entier(entier) => Brique::Entier(*entier),
            Produit(vector) => *vector[rang],
            Somme(vector) => *vector[rang],
            Difference(vector) => *vector[rang],
            Division(vector) => *vector[rang],
            DivisionEuclidienne(un, deux, trois) => {
                match rang {
                    1 => **un,
                    2 => **deux,
                    3 => **trois,
                    _ => **trois, 
                }
            }
        }
    }

    fn longueur(self: &Self) -> usize {
        match self {
            Entier(_) => 1,
            Produit(_) => 2,
            Somme(_) => 2,
            Difference(_) => 2,
            Division(_) => 2,
            DivisionEuclidienne(_, _, _) => 3,
        }
    }

    fn developpe(self: &Self) -> Result<Self, String> {
        if let Brique::Produit(vecteur_sommes) = self {

        }
    }
}
