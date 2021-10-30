use introduction_a_rust::pokemon::Pokemon;
use serde_json;

/*
À LIRE D'ABORD: result.rs

En Rust on est rarement aussi verbose que dans `result.rs`.
Dans cet exemple, je propose de faire en sorte que toutes les fonctions,
y compris main(), renvoient un Result qui a le même type d'erreur.
Cela permet d'utiliser du sucre syntaxique et d'alléger grandement le code dans main().
 */

// Créons une erreur custom qui sera un enum. Les variantes de l'enum ont chacune
// un message associé, de type String
#[derive(Debug)]
enum MonErreurAMoi {
    StdIoError(String),
    Utf8Error(String),
    SerdeJsonError(String),
}

fn lire_un_fichier_avec_la_bibliotheque_standard(path: &str) -> Result<Vec<u8>, MonErreurAMoi> {
    // la fonction read() renvoie une erreur de type std::io::Error
    // mais moi je veux absolument la convertir pour en faire une ErreurAMoi
    match std::fs::read(path) {
        Ok(vec) => Ok(vec),
        // La syntaxe: Err() qui signifie que read() a planté
        Err(error) => Err(
            // conformément à la signature de la fonction que j'écris,
            // je renvoie MonErreurAMoi, et dedans je choisis la variante StdIoError
            MonErreurAMoi::StdIoError(
                // Je pourrais créer une String et ça suffirait, mais je veux récupérer
                // l'erreur d'origine de la fonction read(), alors je l'inclus
                format!("Erreur lors de la lecture du fichier:\n    {}", error),
            ),
        ),
    }
}

// Même topo. Notez que toutes mes fonctions renvoient MonErreurAMoi
fn convertir_un_vecteur_d_octet_en_chaine_utf8(vec: Vec<u8>) -> Result<String, MonErreurAMoi> {
    match String::from_utf8(vec) {
        Ok(vec) => Ok(vec),
        Err(error) => Err(MonErreurAMoi::Utf8Error(format!(
            "Erreur lors de la conversion du fichier en utf8:\n    {}",
            error
        ))),
    }
}

fn deserialiser_une_liste_de_pokemon(
    chaine_de_caracteres: &str,
) -> Result<Vec<Pokemon>, MonErreurAMoi> {
    match serde_json::from_str::<Vec<Pokemon>>(chaine_de_caracteres) {
        Ok(vec) => Ok(vec),
        Err(error) => Err(MonErreurAMoi::SerdeJsonError(format!(
            "Erreur lors de la désérialisation du JSON:\n    {}",
            error
        ))),
    }
}


fn main() -> Result<(), MonErreurAMoi> {
    let path = String::from("pokemons.json");

    // Comme main() renvoie MonErreurAMoi, et que lire_un_fichier_avec_la_bibliotheque_standard
    // renvoie le même type d'erreur, je peux économiser la gestion d'erreur avec "?"
    // Le point d'interrogation dit en substance :
    //    - si la fonction lire_un_fichier_avec_la_bibliotheque_standard() réussit, déballe le Result
    //    - si elle plante, ON S'ARRÊTE ICI. On envoie l'erreur à main(), et main() s'en débrouille
    let vecteur_d_octet = lire_un_fichier_avec_la_bibliotheque_standard(&path)?;

    let contenu_en_ut8 = convertir_un_vecteur_d_octet_en_chaine_utf8(vecteur_d_octet)?;

    let liste_de_pokemon = deserialiser_une_liste_de_pokemon(&contenu_en_ut8)?;

    println!("{:#?}", liste_de_pokemon);
    println!("{:#?}", liste_de_pokemon[0]);
    Ok(())
}
