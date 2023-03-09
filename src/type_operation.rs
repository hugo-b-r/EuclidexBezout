


#[derive(Clone, PartialEq)]
pub enum TypeOperation {
    Produit,
    Somme,
}

impl Default for TypeOperation {
    fn default() -> Self{
        TypeOperation::Somme
    }
}
