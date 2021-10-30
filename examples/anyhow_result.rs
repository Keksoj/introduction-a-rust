use anyhow::Context;
use introduction_a_rust::pokemon::Pokemon;
use serde_json;

/*
Quand on a compris comment fonctionne Result, et pas avant, on peut utiliser anyhow.
C'est une lib qui facilite la gestion d'erreur (il y en a d'autres).

La syntaxe que j'utilise c'est
let valeur = fonction_qui_renvoie_result.with_context(|| "erreur locale")?;

Si la fonction marche, son Result est déballé.
Si la fonction plante,
    - son erreur est renvoyée à la fonction d'au-dessus,
    - PLUS l'erreur locale que j'ai précisée.
 */

fn main() -> anyhow::Result<()> {
    // Essayez de changer "pokemons.js" en "truc.json" pour voir, et lancez
    // cargo run --example anyhow_result
    let path = String::from("truc.json");

    let vecteur_d_octet =
        std::fs::read(&path).with_context(|| "Erreur lors de la lecture du fichier")?;

    let contenu_en_ut8 =
        String::from_utf8(vecteur_d_octet).with_context(|| "Erreur lors de la conversion")?;

    let liste_de_pokemon = serde_json::from_str::<Vec<Pokemon>>(&contenu_en_ut8)
        .with_context(|| "Erreur lors de la désérialisation")?;

    println!("{:#?}", liste_de_pokemon);
    println!("{:#?}", liste_de_pokemon[0]);
    Ok(())
}
