use introduction_a_rust::pokemon::{Pokemon, PokemonBuilder, PokemonType};

fn main() {
    let bulbizarre = Pokemon {
        id: 1,
        nom: String::from("Bulbizarre"),
        niveau: 1,
        evolue_en: Some(String::from("Herbizarre")),
        pokemon_type: vec![PokemonType::Grass, PokemonType::Poison],
    };

    println!("{:?}", bulbizarre);

    let mut carapuce = PokemonBuilder::nouveau()
        .avec_comme_numero(7)
        .avec_comme_nom("Carapuce")
        .avec_comme_niveau(4)
        .peut_evoluer_en("Carabaffe".to_string())
        .avec_comme_type(PokemonType::Water)
        .build();

    println!("{:?}", carapuce);
    carapuce.next_level();
}
