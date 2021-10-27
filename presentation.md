# Rust

## ou le plaisir de coder

![bg w:80% right:40%](img/rustacean-orig-noshadow.svg)

par Emmanuel Bosquet `@Keksoj`

emmanuelbosquet.com/introduction-a-rust

https://github.com/keksoj/introduction-a-rust.git

---

# Disclaimer

![bg left](https://c.tenor.com/Ob4g9Zm7j-YAAAAC/pok%C3%A9mon-ash.gif)

-   Rust est mon premier language
-   J'ai été déçu par tout le reste
-   En plus d'être biaisé il m'arrive d'être de mauvaise foi

---

# Pourquoi rust ?

![bg vertical](<rgb(255,128,0)>)
![](<rgb(255,255,255)>)

Certaines personnes à Mozilla devaient en avoir marre :

-   des runtimes et autres garbage collectors
-   des fuites de mémoire en C et C++

Rust a été mis au point pour être performant et _memory safe_.

---

# Comment ça, memory safe ?

![bg vertical](<rgb(255,128,0)>)
![](<rgb(255,255,255)>)

-   la mémoire est allouée à la création d'une variable
-   les fonctions se passent les références en attendant leur tour
-   la mémoire est nettoyée quand la variable sort du scope
-   le compilateur interdit le code hérétique (pointeurs nuls et autres horreurs)

Si ça compile ça plantera pas !

Le compilateur est votre meilleur ami (et votre pire ennemi).

---

# Facile à installer

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

Pas besoin de sudo.

---

# Une syntaxe lisible

```rust
fn hello_world() {
    // vec! est une macro qui crée un vecteur
    let words = vec!["hello", "world", "!"];

    for word in words {
        println!("{} ", word); // encore une macro
    }
}
```

Mais on ne trouve pas ce genre de Rust dans la nature. Ce qui manque dans cet exemple, c'est…

---

# Un typage fort

![bg vertical](#dcecd7)

Tous un tas de nombres, signés, non signés, flottants : `i32`, `u8`, `f64`.

Des `String`s en UTF8, il va falloir revoir vos habitudes !

Collections:

-   vecteurs (taille variable)
-   tableaux (taille fixe)
-   tuples (éléments hétéroclites)

Et vos propres types customs :

```rust
type mon_type_de_tableau_perso = [u16; 5];
```

---

# Des enums !

![bg left:40%](https://c.tenor.com/GNoM45eC-t4AAAAd/mr-bean-rowan-atkinson.gif)

Le type qui vous laisse choisir.

```rust
pub enum TypeDePokemon {
    Herbe,
    Feu,
    Eau,
    Vol,
    Normal,
    Poison,
    Electrique,
}
```

---

# `Option<type>`

Le type qui :

-   soit contient une valeur => `Some(valeur)`
-   soit ne contient rien => `None`

```rust
type EvolueEn = Option<String>;
```

Un Pokemon peut soit :

-   évoluer dans un autre pokemon => `Some(String::from("Dracofeu"))`
-   ne pas évoluer du tout => `None`

---

# Les structs

![bg w:80% right:30%](https://c.tenor.com/cD7U--DX9okAAAAC/bulbasaur-pokemon.gif)

```rust
pub struct Pokemon {
    id: u16,
    nom: String,
    niveau: u16,
    evolue_en: EvolueEn,
    pokemon_type: Vec<TypeDePokemon>,
}

let bulbizarre = Pokemon {
    id: 1,
    nom: String::from("Bulbizarre"),
    niveau: 1,
    evolue_en: Some(String::from("Herbizarre")),
    pokemon_type: vec![
        TypeDePokemon::Herbe,
        TypeDePokemon::Poison
    ],
};
```

---

# Des méthodes sur les structs

```rust
impl Pokemon {
    fn next_level(&mut self) {
        println!("{} passe au niveau suivant !", self.nom);
        self.niveau += 1;
    }
}
```

Ça ressemble à de l'orienté objet, mais : **pas d'héritage en rust** !

---

![bg left:30%](https://vignette.wikia.nocookie.net/parody/images/1/16/Ash_Ketchum_Surprised.jpg)

# Comment ça, pas d'héritage !?

| En             | le modèle est |    On écrit d'abord…    |     et ensuite…     |
| :------------- | ------------- | :---------------------: | :-----------------: |
| Orienté Objet, | top-down.     |   les classes mères,    | les classes filles. |
| Rust,          | bottom-up.    | le format de la donnée, |    les structs.     |

Par contre, on a plein d'interfaces en Rust (on appelle ça des traits).

---

# Gestion d'erreur => Result

Try/catch, throw… en Rust on a mieux que ça.

```rust
enum Result {
    Ok(valeur),
    Err(erreur),
}
```

Inclus dans l'offre :

-   du pattern matching,
-   des erreurs customs,
-   du sucre syntaxique,
-   des libs qui vous facilitent la vie

---

# Ownership & borrow checker

3 règles absolues :

1. Une fonction qui prend en argument une variable `T` en a l'_ownership_ — plus personne d'autre ne peut y toucher.
2. Une référence immutable `&T` peut être utilisée par plein de fonctions à la fois
3. Une référence mutable `&mut T` ne peut être utilisée que par **une seule fonction** à la fois

Big up à Benjamin Coenen qui a fait [une super conf à Devoxx 2021](https://youtu.be/YuHoujk8SUc) à ce sujet.

---

![bg right:40%](https://c.tenor.com/Z4adaEQmUqkAAAAd/pokemon-fighting.gif)

# J'ai pas compris, qu'est-ce que je dois faire ?

## Bats-toi contre le compilateur – et perds !

À force de cogner sur le clavier ça va passer.

---

## Des programmes cools en rust: la ligne de commande

-   `ripgrep` : remplace `grep` (plus convivial)
-   `bat`: remplace `cat` (coloration)
-   `sd` : remplace `sed`
-   `fd` : remplace `find`
-   `dust` : remplace `du` (c'est beaucoup plus joli)

En Rust ça va plus vite.

---

## Des programmes plus vénères

![img w:200](https://upload.wikimedia.org/wikipedia/commons/thumb/8/84/Deno.svg/440px-Deno.svg.png) Deno, runtime JavaScript et Typescript.

![img](https://avatars.githubusercontent.com/u/25197590?s=200&v=4) Sōzu, reverse proxy fait sur mesure pour Clever Cloud.
Écrit par [Geoffroy Couprie, aka Geal](https://github.com/Geal).

---

# Comment apprendre Rust ?

Le [Rust book](https://doc.rust-lang.org/book/) a été [largement traduit en français](https://upload.wikimedia.org/wikipedia/commons/thumb/8/84/Deno.svg/440px-Deno.svg.png)!

Udemy, Exercism, etc.

Les questions de noobs sont tolérées sur [le forum de la communauté](https://users.rust-lang.org/) (en anglais)

Stack Overflow pour les puristes.

Écrivez-moi ! <emmanuel.bosquet@clever-cloud.com>