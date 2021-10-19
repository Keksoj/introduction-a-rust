use serde::{Deserialize, Serialize};
use serde_json;
use introduction_a_rust::pokemon::Pokemon;

#[derive(Debug)]
enum MonErreurAMoi {
    StdIoError(String),
    Utf8Error(String),
    SerdeJsonError(String),
}

fn lire_un_fichier_avec_la_bibliotheque_standard(path: &str) -> Result<Vec<u8>, MonErreurAMoi> {
    match std::fs::read(path) {
        Ok(vec) => Ok(vec),
        Err(error) => Err(MonErreurAMoi::StdIoError(format!(
            "Erreur lors de la lecture du fichier:\n    {}",
            error
        ))),
    }
}

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

    let vecteur_d_octet = lire_un_fichier_avec_la_bibliotheque_standard(&path)?;

    let contenu_en_ut8 = convertir_un_vecteur_d_octet_en_chaine_utf8(vecteur_d_octet)?;

    let liste_de_pokemon = deserialiser_une_liste_de_pokemon(&contenu_en_ut8)?;

    println!("{:#?}", liste_de_pokemon);
    println!("{:#?}", liste_de_pokemon[0]);
    Ok(())
}
