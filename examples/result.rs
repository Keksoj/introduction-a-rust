use introduction_a_rust::pokemon::Pokemon;
use serde_json;

/*
On a vu dans la présentation que pour gérer les erreurs, on a un type custom, Result.
Une fonction qui renvoie Result peut soit réussir et renvoyer un résultat (ouiiiii),
soit elle peut planter (oh non!).

Par exemple, la fonction std::fs::read(), qui lit un fichier, va soit :
- Réussir et renvoyer une suite d'octet, de type Vec<u8>
- planter (si le fichier n'existe pas par exemple), et renvoyer une erreur de type std::io::Error

std = bibliothèque standard de rust
fs = file system = système de fichier
io = in/out = entrée/sortie
*/
fn lire_un_fichier_avec_la_bibliotheque_standard(path: &str) -> Result<Vec<u8>, std::io::Error> {
    // appeler la fonction read() renvoie un Result, qu'on stocke dans Result…
    let result = std::fs::read(path);
    // …et qu'on renvoie. La syntaxe est overkill.
    return result;
}

fn convertir_un_vecteur_d_octet_en_chaine_utf8(
    vec: Vec<u8>,
) -> Result<String, std::string::FromUtf8Error> {
    // Même chose, la fonction from_utf8() peut soit:
    // - réussir et donner une chaîne de caractère (String)
    // - planter et envoyer une erreur de type string::FromUtf8Error (c'est très spécifique)
    // syntaxe plus directe
    return String::from_utf8(vec);
}

fn deserialiser_une_liste_de_pokemons(
    chaine_de_caracteres: &str,
) -> Result<Vec<Pokemon>, serde_json::Error> {
    // Attention c'est un peu plus perché. Décortiquons mot par mot.
    //
    // - serde_json est la bibliothèque qui convertit du Rust en JSON et inversement
    // - from_str prend une chaîne de caractère et la convertit en Rust
    // - ::<Vec<Pokemon>> est là pour dire à from_str() :
    //   « Le JSON à convertir ça doit être une liste de Pokémons »
    //
    // Pareil, soit ça marche => vecteur de Pokémons
    //         soit ça plante => Erreur spécifique de type serde_json::Error
    //
    // Notez qu'on peut juste écrire la fonction sans return et sans point-virgule
    serde_json::from_str::<Vec<Pokemon>>(chaine_de_caracteres)
}

fn main() {
    // amusez-vous à changer "pokemons.json" en autre chose
    // et lancez 
    //           cargo run --example result
    let path = String::from("pokemons.json");

    // Créer un vecteur d'octets vides dans lequel on lira le fichier
    let mut vecteur_d_octet = Vec::<u8>::new();

    // Ici j'écris le type de result en entier mais c'est juste pour bien comprendre
    let result: Result<Vec<u8>, std::io::Error> =
        lire_un_fichier_avec_la_bibliotheque_standard(&path);

    // On peut faire du pattern matching sur le type Result,
    // donc lancer des actions différentes selon qu'il est Ok() ou Err()
    match result {
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
