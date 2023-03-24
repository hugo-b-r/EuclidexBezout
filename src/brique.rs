/*
    2023 Hugo Berthet-Rambaud
    Licencié sous la licence MIT. La districution de ce code sans cette licence
    ne peut se faire.

    --- Pour l'humanité, son bonheur et sa gloire ---
 */

use crate::brique::Brique::*;

#[derive(Clone)]
pub enum Brique {
    Entier(i64),
    Produit(Vec<Box<Self>>),
    Somme(Vec<Box<Self>>),
    Difference(Vec<Box<Self>>),
    Division(Vec<Box<Self>>),
    DivisionEuclidienne(Box<Self>, Box<Self>, Box<Self>),
    Egalite(Vec<Box<Self>>),
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
            let rang = self.pos -1;

            self.inner.get(rang as usize)
        }
    }
}

impl Brique {
    fn len(&self) -> usize {
        match self {
            Entier(_)                      => 1 as usize,
            Produit(vector)     => vector.len(),
            Somme(vector)       => vector.len(),
            Difference(vector)  => vector.len(),
            Division(vector)    => vector.len(),
            DivisionEuclidienne(_, _, _)           => 3 as usize,
            Egalite(vecteur)    => vecteur.len(),
        } 
    }

    pub fn get(&self, rang: usize) -> Option<&Self> {
        let rang_ici = &rang;
        match self {
            Entier(_)                             =>     Some(&self),
            Produit(vecteur)   =>     Some(&*vecteur[rang]),
            Somme(vector)      =>     Some(&*vector[*rang_ici]),
            Difference(vector) =>     Some(&*vector[*rang_ici]),
            Division(vector)   =>     Some(&*vector[*rang_ici]),
            DivisionEuclidienne(un, deux, trois) => {
                match rang {
                    1 => Some(&(**un)),
                    2 => Some(&(**deux)),
                    3 => Some(&(**trois)),
                    _ => Some(&(**trois)), 
                }
            },
            Egalite(vecteur)   => Some(vecteur.get(rang).unwrap()),
        }
    }

    pub fn valeur_entiere(&self) -> Result<i64, String> {
        if let Brique::Entier(entier) = self {
            Ok( *entier )
        } else {
            Err(String::from("pas un entier"))
        }
    }

    fn longueur(self: &Self) -> usize {
        match self {
            Entier(_)                           => 1,
            Produit(_)                          => 2,
            Somme(_)                            => 2,
            Difference(_)                       => 2,
            Division(_)                         => 2,
            DivisionEuclidienne(_, _, _)        => 3,
            Egalite(vecteur) => vecteur.len(),
        }
    }

    fn iter<'a>(&'a self) -> IterBrique<'a> {
        IterBrique {
            inner:  self,
            pos: 0,
        }
    }

    //use with care - utiliser avec attention,
    //pas de sous somme de produit de rien du tout
    fn developpe(self: &Self) -> Result<Self, String> {
        let mut vecteur_pre_sortie = Vec::new();

        if let Brique::Produit(vecteur_sommes) = self {
            if let Brique::Somme(vecteur_membres_1) = &*vecteur_sommes[0] {
                if let Brique::Somme(vecteur_membres_2) = &*vecteur_sommes[1] {
                    for a in (*vecteur_membres_1).iter() {
                        for b in (*vecteur_membres_2).iter() {
                            vecteur_pre_sortie.push(Box::new(Brique::Produit(vec![a.clone(), b.clone()])));
                        }
                    }
                } 
            }
        } else {
            return Err(String::from("n'est pas un produit"))
        }
        let sortie = Brique::Somme(vecteur_pre_sortie);
        Ok( sortie )
    }

    pub fn entier(&self, rang: usize) -> Option<i64> {
        let mut i = 0;
        for brique in self.iter() {
            if let Brique::Entier(nombre) = *brique {
                if i == rang as i32 {
                    return Some(nombre);
                } else {
                    i += 1;
                }
            }
        }
        //if no return
        None
    }

    pub fn somme(&self, rang: usize) -> Option<Self> {
        let mut i = 0;
        for brique in self.iter() {
            if let Brique::Somme(vecteur) = *brique {
                if i == rang as i32 {
                    return Some(*brique);
                } else {
                    i += 1;
                }
            }
        }
        //if no return
        None
    }

    pub fn produit(&self, rang: usize) -> Option<Self> {
        let mut i = 0;
        for brique in self.iter() {
            if let Brique::Produit(vecteur) = *brique {
                if i == rang as i32 {
                    return Some(*brique);
                } else {
                    i += 1;
                }
            }
        }
        //if no return
        None
    }

    pub fn difference(&self, rang: usize) -> Option<Self> {
        let mut i = 0;
        for brique in self.iter() {
            if let Brique::Difference(vecteur) = *brique {
                if i == rang as i32 {
                    return Some(*brique);
                } else {
                    i += 1;
                }
            }
        }
        //if no return
        None
    }

    pub fn division(&self, rang: usize) -> Option<Self> {
        let mut i = 0;
        for brique in self.iter() {
            if let Brique::Division(vecteur) = *brique {
                if i == rang as i32 {
                    return Some(*brique);
                } else {
                    i += 1;
                }
            }
        }
        //if no return
        None
    }
}
