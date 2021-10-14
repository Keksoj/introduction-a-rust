use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Pokemon {
    id: u16,
    name: String,
    pokemon_type: Vec<String>,
}

fn lire_un_fichier_avec_la_bibliotheque_standard(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let vecteur_d_octet = std::fs::read(path);
    return vecteur_d_octet;
}

fn convertir_un_vecteur_d_octet_en_chaine_utf8(
    vec: Vec<u8>,
) -> Result<String, std::string::FromUtf8Error> {
    return String::from_utf8(vec);
}

fn deserialiser_une_liste_de_pokemon(
    chaine_de_caracteres: &str,
) -> Result<Vec<Pokemon>, serde_json::Error> {
    serde_json::from_str::<Vec<Pokemon>>(chaine_de_caracteres)
}

fn main() {
    let path = String::from("pokemons.json");

    let mut vecteur_d_octet = Vec::<u8>::new();
    match lire_un_fichier_avec_la_bibliotheque_standard(&path) {
        Ok(vec) => vecteur_d_octet = vec,
        Err(error) => println!("Erreur lors de la lecture du fichier:\n    {}", error),
    }

    let mut contenu_en_ut8 = String::new();
    match convertir_un_vecteur_d_octet_en_chaine_utf8(vecteur_d_octet) {
        Ok(utf8_string) => contenu_en_ut8 = utf8_string,
        Err(error) => println!("Erreur lors de la conversion:\n    {}", error),
    }

    let mut liste_de_pokemon = Vec::<Pokemon>::new();
    match deserialiser_une_liste_de_pokemon(&contenu_en_ut8) {
        Ok(liste) => liste_de_pokemon = liste,
        Err(error) => println!("Erreur lors de la désérialisation:\n    {}", error),
    }

    println!("{:#?}", liste_de_pokemon);
    println!("{:#?}", liste_de_pokemon[0])
}
