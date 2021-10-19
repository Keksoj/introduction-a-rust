use introduction_a_rust::pokemon::Pokemon;
use serde_json;

fn lire_un_fichier_avec_la_bibliotheque_standard(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let result = std::fs::read(path);
    return result;
}

fn convertir_un_vecteur_d_octet_en_chaine_utf8(
    vec: Vec<u8>,
) -> Result<String, std::string::FromUtf8Error> {
    return String::from_utf8(vec);
}

fn deserialiser_une_liste_de_pokemons(
    chaine_de_caracteres: &str,
) -> Result<Vec<Pokemon>, serde_json::Error> {
    serde_json::from_str::<Vec<Pokemon>>(chaine_de_caracteres)
}

fn main() {
    let path = String::from("pokemons.json");

    let mut vecteur_d_octet = Vec::<u8>::new();

    match lire_un_fichier_avec_la_bibliotheque_standard(&path) {
        Ok(vec) => vecteur_d_octet = vec,
        Err(error) => {
            println!("Erreur lors de la lecture du fichier:\n    {}", error);
            std::process::exit(1)
        }
    }

    let mut contenu_en_utf8 = String::new();

    match convertir_un_vecteur_d_octet_en_chaine_utf8(vecteur_d_octet) {
        Ok(utf8_string) => contenu_en_utf8 = utf8_string,
        Err(error) => {
            println!("Erreur lors de la conversion:\n    {}", error);
            std::process::exit(1)
        }
    }

    let mut liste_de_pokemon = Vec::<Pokemon>::new();

    match deserialiser_une_liste_de_pokemons(&contenu_en_utf8) {
        Ok(liste) => liste_de_pokemon = liste,
        Err(error) => {
            println!("Erreur lors de la désérialisation:\n    {}", error);
            std::process::exit(1)
        }
    }

    println!("{:#?}", liste_de_pokemon);
    println!("{:#?}", liste_de_pokemon[0])
}
