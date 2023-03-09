#[cfg(test)]
mod tests {
    use euclide_x_bezout::{AlgoEuclide, TypeOperation};
    #[test]
    fn t_algo_euclide() { //teste la creation algorithem d'euclide
        let algorithme = AlgoEuclide::new(0, 0);
        let result = if algorithme.a == 0 && algorithme.b == 0 {
            true
        } else {
            false
        };
        assert_eq!(result, true);
    }

    #[test]
    fn t_algo_euclide_2() {
        let mut algorithme: AlgoEuclide = AlgoEuclide::new(2, 1);
        algorithme = algorithme.compute();
        assert_eq!(algorithme.print(), "2 = 1 x 2 + 0".to_string())
    }

    use euclide_x_bezout::Brique;
    #[test]
    fn t_brique_print() {
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
    }

    use euclide_x_bezout::pgcd;
    #[test]
    fn t_pgcd() {
        let pgcd = pgcd(585, 360);
        assert_eq!(pgcd, 45);
    }

    #[test]
    fn t_developpe_1() {
        let mut brique = Brique::new();
        brique.type_operation = TypeOperation::Produit;
        brique.briques.push( Brique {
            briques: Vec::new(),
            nombres: vec![25.0, 30.0],
            type_operation: TypeOperation::Somme,
        });
        brique.nombres.push(25.0);
        brique = brique.developpe().unwrap();

        let brique_texte = brique.print();

        assert_eq!(brique_texte, "(25 x 25) + (30 x 25)".to_string());
    }

    #[test]
    fn t_developpe_2() {
        let mut brique = Brique::new();
        brique.type_operation = TypeOperation::Produit;
        brique.briques.push( Brique {
            briques: Vec::new(),
            nombres: vec![25.0, 30.0],
            type_operation: TypeOperation::Somme,
        });
        brique.briques.push( Brique {
            briques: Vec::new(),
            nombres: vec![25.0, 30.0],
            type_operation: TypeOperation::Somme,
        });
        brique = brique.developpe().unwrap();

        let brique_texte = brique.print();

        assert_eq!(brique_texte, "(25 x 25) + (25 x 30) + (30 x 25) + (30 x 30)".to_string());
    }
}
