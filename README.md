![img](img/rustacean-orig-noshadow.svg)

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

Lancer le binaire `src/bin/bulbizarre.rs`

    cargo run --bin bulbizarre

Lancer l'exemple `examples/custom_error.rs`

    cargo run --example custom_error

## Pense-bête

Lancer la génération de la présentation avec [marpit](https://marpit.marp.app/)

    marp -w presentation.md -o index.html
