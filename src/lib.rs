//! Ce crate est un exemple de bibliothèque en rust. Cette documentation est
//! générée avec des commentaires dans `src/lib.rs` et dans le reste du code.
//!
//! La documentation peut contenir des exemples de code:
//!
//! ```
//! use introduction_a_rust::pokemon::{Pokemon, PokemonBuilder, TypeDePokemon};
//! 
//! fn main() {
//!     let bulbizarre = Pokemon {
//!         id: 1,
//!         nom: String::from("Bulbizarre"),
//!         niveau: 1,
//!         evolue_en: Some(String::from("Herbizarre")),
//!         pokemon_type: vec![TypeDePokemon::Herbe, TypeDePokemon::Poison],
//!     };
//!     println!("{:?}", bulbizarre);
//! }
//! ```
//! Le compilateur teste même les exemples de code dans la doc !



/// Ce module contient les structs nécessaires pour créer et modifier des pokémons
pub mod pokemon;
