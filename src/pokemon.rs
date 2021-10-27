use serde::{Deserialize, Serialize};

/// Cette énumération liste les différents types de pokémon
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TypeDePokemon {
    Herbe,
    Feu,
    Terre,
    Eau,
    Vol,
    Poison,
    Normal,
    Electrique,
}

/// les perspectives d'évolution d'un pokemon.
/// - `Some(tel_pokemon)`. Peut évoluer en tel pokemon.
/// - `None`. Pas d'évolution possible.
pub type EvolueEn = Option<String>;

/// La description d'un Pokémon, avec ses méthodes.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Pokemon {
    /// le numéro du pokémon dans le pokédex
    pub id: u16,
    /// le nom du pokémon en français
    pub nom: String,
    /// le niveau du pokémon, appelé à changer avec le temps
    pub niveau: u16,
    /// Type custom qui wrappe `Option<String>`
    pub evolue_en: EvolueEn,
    /// Un pokémon peut avoir plusieurs types
    pub pokemon_type: Vec<TypeDePokemon>,
}

impl Pokemon {
    /// Cette méthode:
    /// - prend l'ownership de la référence mutable (`&mut`)
    /// - transforme la donnée
    /// - rend l'ownership
    pub fn next_level(&mut self) {
        println!("{} passe au niveau suivant !", self.nom);
        self.niveau += 1;
    }
}

/// Un constructeur de Pokémons.  
/// Le builder pattern est très pratique quand on fait des libs en rust.
/// C'est verbose mais ça rend le code lisse côté binaire.
///
/// Exemple:
/// ```
/// use introduction_a_rust::pokemon::{PokemonBuilder, TypeDePokemon};
///
/// let mut carapuce = PokemonBuilder::nouveau()
///    .avec_comme_numero(7)
///    .avec_comme_nom("Carapuce")
///    .avec_comme_niveau(4)
///    .peut_evoluer_en("Carabaffe")
///    .avec_comme_type(TypeDePokemon::Eau)
///    .build();
/// ```
#[derive(Default)]
pub struct PokemonBuilder {
    pub id: Option<u16>,
    pub nom: Option<String>,
    /// Niveau par défaut: 1
    pub niveau: Option<u16>,
    /// Valeur par défaut: None
    pub evolue_en: Option<String>,
    pub pokemon_type: Vec<TypeDePokemon>,
}

impl PokemonBuilder {
    /// Crée un builder de pokémon
    pub fn nouveau() -> Self {
        // default() instancie PokemonBuilder avec des nombres à 0,
        // des Strings vides, des options en None, des vecteurs vides
        Default::default()
    }

    /// cette fonction :
    /// - prend l'ownership de self (du builder),
    /// - transforme self
    /// - renvoie self
    pub fn avec_comme_numero(mut self, id: u16) -> Self {
        self.id = Some(id);
        self
    }

    /// Cette syntaxe permet de dire:
    /// - « La fonction prend en argument un variable de type `T` »
    /// - « Ce type `T` doit satisfaire le trait (= l'interface) `ToString` »
    ///
    /// la fonction pourra donc accepter `String`, `&str`,
    /// ou même `std::net::SocketAddr` si ça nous chante
    pub fn avec_comme_nom<T>(mut self, nom: T) -> Self
    where
        T: ToString,
    {
        self.nom = Some(nom.to_string());
        self
    }

    /// Attribue un niveau au pokémon (défaut: 1)
    pub fn avec_comme_niveau(mut self, niveau: u16) -> Self {
        self.niveau = Some(niveau);
        self
    }

    /// Attribue un possibilité d'évolution
    pub fn peut_evoluer_en<T>(mut self, autre_pokemon: T) -> Self
    where
        T: ToString,
    {
        self.evolue_en = Some(autre_pokemon.to_string());
        self
    }

    pub fn avec_comme_type(mut self, type_de_pokemon: TypeDePokemon) -> Self {
        self.pokemon_type.push(type_de_pokemon);
        self
    }

    /// Cette méthode consomme le builder et renvoie une instance de Pokemon.
    /// Si `id`, `nom`, `niveau`, ou `pokemon_type` ne sont pas renseignés,
    /// un message d'erreur s'affiche et le process est stoppé.
    pub fn build(self) -> Pokemon {
        if self.id.is_none() {
            println!("Veuillez renseigner une id pour votre pokemon");
            std::process::exit(1);
        }

        if self.nom.is_none() {
            println!("Veuillez renseigner un nom pour votre pokemon");
            std::process::exit(1);
        }

        if self.pokemon_type.is_empty() {
            println!("Veuillez renseigner au moins un type pour votre pokemon");
            std::process::exit(1);
        }

        Pokemon {
            // unwrap() extrait la valeur des options. À utiliser avec modération !
            // Si l'option est None, le thread panique!
            id: self.id.unwrap(),
            nom: self.nom.unwrap(),
            niveau: self.niveau.unwrap_or(1),
            evolue_en: self.evolue_en,
            pokemon_type: self.pokemon_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_pokemon_builder_fonctionne() {
        let pikachu_cree_avec_le_struct = Pokemon {
            id: 25,
            nom: "Pikachu".to_string(),
            niveau: 1,
            evolue_en: Some("Raichu".to_string()),
            pokemon_type: vec![TypeDePokemon::Electrique],
        };

        let pikachu_cree_avec_le_builder = PokemonBuilder::nouveau()
            .avec_comme_numero(25)
            .avec_comme_nom("Pikachu")
            .peut_evoluer_en("Raichu")
            .avec_comme_type(TypeDePokemon::Electrique)
            .build();

        assert_eq!(pikachu_cree_avec_le_struct, pikachu_cree_avec_le_builder);
    }

    #[test]
    fn la_fonction_next_level_fonctionne() {
        let mut pikachu = PokemonBuilder::nouveau()
            .avec_comme_numero(25)
            .avec_comme_nom("Pikachu")
            .avec_comme_type(TypeDePokemon::Electrique)
            .avec_comme_niveau(5)
            .build();

        pikachu.next_level();
        assert_eq!(pikachu.niveau, 6);
    }
}
