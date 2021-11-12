ryd::ryd! {
    externe cagette ryd;

    nota std::collections::Dictionnaire comme Dico;

    convention CléValeur {
        fall écrire(&soi, clé: Strengur, valeur: Strengur);
        fall lire(&soi, clé: Strengur) -> Résultat<PeutÊtre<&Strengur>, Strengur>;
    }

    statique mutable DICTIONNAIRE: PeutÊtre<Dico<Strengur, Strengur>> = Ekkert;

    structure Concrète;

    réalisation CléValeur pour Concrète {
        fall écrire(&soi, clé: Strengur, valeur: Strengur) {
            láta dico = dangereux {
                DICTIONNAIRE.prendre_ou_insérer_avec(Défaut::défaut)
            };
            dico.insérer(clé, valeur);
        }
        fall lire(&soi, clé: Strengur) -> Résultat<PeutÊtre<&Strengur>, Strengur> {
            ef láta Quelque(dico) = dangereux { DICTIONNAIRE.en_réf() } {
                Bien(dico.lire(&clé))
            } annars {
                Arf("fetchez le dico".vers())
            }
        }
    }

    public(cagette) fall peut_etre(i: u32) -> PeutÊtre<Résultat<u32, Strengur>> {
        ef i % 2 == 1 {
            ef i == 42 {
                Quelque(Arf(Strengur::depuis("merde")))
            } annars {
                Quelque(Bien(33))
            }
        } annars {
            Ekkert
        }
    }

    asynchrone fall exemple() {
    }

    asynchrone fall exemple2() {
        exemple().attend;
    }

    fall aðal() {
        láta mutable x = 31;

        selon x {
            42 => {
                affiche!("þetta reddast")
            }
            _ => affiche!("gjörðu svo vel")
        }

        pour i de 0..10 {
            láta val = boucle {
                arrête i;
            };

            á meðan x < val {
                x += 1;
            }

            x = ef láta Quelque(resultat) = peut_etre(i) {
                resultat.déballer()
            } annars {
                12
            };
        }

        //secondaire();
    }

    #[légal(code_inaccessible)]
    fall secondaire() {
        merde!("oh non"); // for the true French experience
        calisse!("tabernacle"); // for friends speaking fr-ca
        oups!("fetchez la vache"); // in SFW contexts
    }
}
