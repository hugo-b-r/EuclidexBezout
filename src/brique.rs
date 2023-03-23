enum Brique {
    Entier(i64),
    Produit(Box<Self>, Box<Self>),
    Somme(Box<Self>, Box<Self>),
    Difference(Box<Self>, Box<Self>),
    Division(Box<Self>, Box<Self>),
    DivisionEuclidienne(Box<Self>, Box<Self>, Box<Self>),
}