use std::ops::Deref;

enum Brique {
    Entier(i64),
    Produit(Box<Self>, Box<Self>),
    Somme(Box<Self>, Box<Self>),
    Difference(Box<Self>, Box<Self>),
    Division(Box<Self>, Box<Self>),
    DivisionEuclidienne(Box<Self>, Box<Self>, Box<Self>),
}

impl Brique {
    fn developpe(self: &Self) -> Result<Self, String> {
        if let Brique::Produit(brique_1, brique_2) = self {
            if let Brique::Somme(entier_1, entier_2) = brique_1 {
                if let Brique::Somme(entier_3, entier_4) = brique_2 {
                    return Ok(
                        Brique::Produit(
                            Brique::Produit(
                                Brique::Somme(
                                    Brique::Entier(entier_1),
                                    Brique::Entier(entier_3),
                                ),
                                Brique::Somme(
                                    Brique::Entier(entier_1),
                                    Brique::Entier(entier_4),
                                )
                            )
                            Brique::Produit(
                                Brique::Somme(
                                    Brique::Entier(entier_2),
                                    Brique::Entier(entier_3),
                                ),
                                Brique::Somme(
                                    Brique::Entier(entier_2),
                                    Brique::Entier(entier_4),
                                )
                            )
                        )
                    );
                }
            }
        }
        Ok()
    }
}
