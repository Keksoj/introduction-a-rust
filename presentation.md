# Rust

## ou le plaisir de coder

![bg w:80% right:40%](img/rustacean-orig-noshadow.svg)

par Emmanuel Bosquet `@Keksoj`

emmanuelbosquet.com/introduction-a-rust

---

# Pourquoi rust ?

![bg vertical](<rgb(255,128,0)>)
![](<rgb(255,255,255)>)

Certaines personnes à Mozilla devaient aimer l'optimisation => pas de runtime (java, python)

Mais en avaient sans doute marre d'écrire des pointeurs => bye bye C/C++

Rust a été mis au point pour être _memory safe_.

---

# Comment ça, memory safe ?

![bg vertical](<rgb(255,128,0)>)
![](<rgb(255,255,255)>)

-   les pointeurs sont créés, mais sous le capot
-   les fonctions se passent les références en attendant leur tour
-   la mémoire est nettoyée quand la variable sort du scope
-   le compilateur interdit le code hérétique (pointeurs nuls et autres horreurs)

Si vous y tenez vous pouvez quand même écrire les pointeurs vous-mêmes.

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

Le type qui vous laisse choisir

```rust
enum PokemonType {
    Grass,
    Fire,
    Earth,
    Water,
    Flying,
    Poison,
}
```

---

# Les structs

![bg w:80% right:30%](https://c.tenor.com/cD7U--DX9okAAAAC/bulbasaur-pokemon.gif)

```rust
struct Pokemon {
    id: u16,
    name: String,
    level: u16,
    pokemon_type: Vec<PokemonType>,
}

let bulbizarre = Pokemon {
    id: 1,
    name: String::from("Bulbizarre"),
    level: 1,
    pokemon_type: vec![
        PokemonType::Grass,
        PokemonType::Poison
    ],
};
```

---

# Des méthodes sur les structs

```rust
impl Pokemon {
    fn next_level(&mut self) {
        println!("{} passe au niveau suivant !", self.name);
        self.level += 1;
    }
}
```

Ça ressemble à de l'orienté objet, mais : **pas d'héritage en rust** !

---

# Des traits
