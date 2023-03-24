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

    fn valeur(self: &Self, rang: usize) -> Result<i64, String> {
        match self {
            Entier(valeur) => Ok( *valeur ),
            Produit(vecteur) => {
                vecteur[rang].valeur(0)
            },
            Somme(vecteur) => {
                vecteur[rang].valeur(0)
            },
            Difference(vecteur) => {
                vecteur[rang].valeur(0)
            },
            Division(vecteur) => {
                vecteur[rang].valeur(0)
            },
            DivisionEuclidienne(_, _, _) => Err(String::from("Valeur impossible pour une division euclidienne")),
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
        if let Brique::Produit(brique_1, brique_2) = self {
            if let Brique::Somme(entier_1, entier_2) = &**brique_1 {
                if let Brique::Somme(entier_3, entier_4) = &**brique_2 {
                    return Ok(
                        Brique::Produit(
                            Box::new(
                                Brique::Produit(
                                    Box::new(
                                        Brique::Somme(
                                            Box::new(Brique::Entier(entier_1.valeur(0).unwrap())),
                                            Box::new(Brique::Entier(entier_3.valeur(0).unwrap())),
                                        )
                                    ),
                                    Box::new(
                                        Brique::Somme(
                                            Box::new(Brique::Entier(entier_1.valeur(0).unwrap())),
                                            Box::new(Brique::Entier(entier_4.valeur(0).unwrap())),
                                        )
                                    )
                                )
                            ),
                            Box::new(
                                Brique::Produit(
                                    Box::new(
                                        Brique::Somme(
                                            Box::new(Brique::Entier(entier_2.valeur(0).unwrap())),
                                            Box::new(Brique::Entier(entier_3.valeur(0).unwrap())),
                                        )
                                    ),
                                    Box::new(
                                        Brique::Somme(
                                            Box::new(Brique::Entier(entier_2.valeur(0).unwrap())),
                                            Box::new(Brique::Entier(entier_4.valeur(0).unwrap())),
                                        )
                                    )
                                )
                            )
                        )
                    );
                } else {
                    Err(String::from("deuxieme facteur n'est pas un produit"))
                }
            } else {
                Err(String::from("premier facteur n'est pas un produit"))
            }
        } else {
            Err(String::from("n'est pas un produit"))
        }
    }
}
