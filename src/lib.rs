//! Ce crate est un exemple de bibliothèque en rust. Cette documentation est
//! générée avec des commentaires dans `src/lib.rs` et dans le reste du code.
//!
//! La documentation peut contenir des exemples de code:
//!
//! ```
//! use introduction_a_rust::pokemon::{Pokemon, PokemonBuilder, PokemonType};
//! 
//! fn main() {
//!     let bulbizarre = Pokemon {
//!         id: 1,
//!         nom: String::from("Bulbizarre"),
//!         niveau: 1,
//!         evolue_en: Some(String::from("Herbizarre")),
//!         pokemon_type: vec![PokemonType::Grass, PokemonType::Poison],
//!     };
//!     println!("{:?}", bulbizarre);
//! }
//! ```
//!



/// Ce module contient les structs nécessaires pour créer et modifier des pokémons
pub mod pokemon;
