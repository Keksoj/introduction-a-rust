![img](img/rustacean-orig-noshadow.svg)

## [Voir la conf sur youtube](https://youtu.be/68Yw_n6SySk)

# Rust, ou le plaisir de coder

Retrouvez la présentation [en ligne](https://keksoj.github.io/introduction-a-rust).

[Rust](https://www.rust-lang.org/) c'est génial, on va voir pourquoi.

Installez rust en suivant [ces instructions](https://www.rust-lang.org/learn/get-started),
résumables ainsi :

## Installez rustup

C'est l'installeur et gestionnaire de version de rust.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Mettez rustup à jour si ça fait un moment que vous avez installé rust

    rustup update

Rustup vérifiera que vous avez la dernière version des outils nécessaire, surtout cargo, qui est le gestionnaire de paquets de rust, et wrappe `rustc`, le compilateur, entre autres choses.

    cargo --version

## Découvrez rust avec ce dépot

Dans ce dépôt, il y a :

-   Un hello world parce que c'est la vie
-   Une lib Pokémon, qui modélise des pokémons, peut les créer, les transformer, les sérialiser
-   Trois examples de désérialisation d'un `pokemons.json` pour illustrer `Result`

Clonez ce dépot:

    git clone https://github.com/keksoj/introduction_a_rust.git
    cd introduction_a_rust

Vérifier que cargo fonctionne en lançant :

    cargo run

qui devrait afficher "Hello, world!"

Pour afficher la documentation du code de ce projet, faites

    cargo doc --open

Lancer les tests

    cargo test

### Un exemple tout simple de création de Pokémon

Lisez un peu `src/bin/bulbizarre.rs`, bidouillez-le, puis lancez-le avec :

    cargo run --bin bulbizarre

### Trois exemples de désérialisation de JSON

Les trois exemples font exactement la même chose : désérialiser `pokemons.json` pour en faire un vecteur d'instances du struct `Pokemon`.
Pourquoi trois fois la même chose ? C'est pour montrer comment on utilise `Result` en Rust, du plus verbose et facile à comprendre, au plus efficace une fois qu'on a compris.

Un exemple se lance comme ça :

    cargo run --example result

Je vous suggère de lire les exemples dans cet ordre :

1. `result.rs` qui décortique `Result` dans une gestion d'erreur ultra-verbose mais même les collégiens qui écrivent en scratch pourront comprendre.
2. `custom_error.rs` montre comment on peut inventer son propre type d'erreur custom, et comment utiliser du sucre syntaxique avec `?` pour propager les erreurs.
3. `anyhow_error.rs` montre comment les feignants blasés dans mon genre gèrent des erreurs en rust.

## Pense-bête

Lancer la génération de la présentation avec [marpit](https://marpit.marp.app/)

    marp -w presentation.md -o index.html
