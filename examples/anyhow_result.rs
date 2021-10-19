use anyhow::{bail, Context, Error};
use introduction_a_rust::pokemon::Pokemon;
use serde::{Deserialize, Serialize};
use serde_json;

fn main() -> anyhow::Result<()> {
    let path = String::from("pokemons.json");

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
