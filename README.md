# Introduction à rust

[Rust](https://www.rust-lang.org/) c'est génial, on va voir pourquoi.

Installez rust en suivant [ces instructions](https://www.rust-lang.org/learn/get-started),
résumables ainsi :

## Installez rust

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

Vérifier que cargo fonctionne en lançant le programme contenu dans `src/main.rs` :

    cargo run

qui devrait afficher "Hello, world!"

