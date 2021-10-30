# Les examples pour illustrer la gestion d'erreur

(Oui, je mets des readmes partout !)

La gestion d'erreur passe par un type super cool qu'on appelle Result et qui ressemble à ça:

```rust
enum Result {
    Ok(valeur),
    Err(erreur),
}
```

Result c'est soit :

-   Ok, voici la valeur que tu veux
-   Désolé, j'ai planté, voici l'erreur

Et avec ça on peut gérer les erreurs super facilement dans le code,
et même les transmettre le long d'une chaîne d'appels de fonctions,
ce qui fait qu'on peut créer un programme qui ne plante pas, ou alors
il plante en donnant une explication super claire.

### La méthode super verbose pour comprendre

Lisez le code de `result.rs`, les documentaires, faites joujou avec :

    cargo run --example result

### Le sucre syntaxique avec une erreur custom

Lisez le code et les commentaires de `custom_error.rs`, faites joujou avec:

    cargo run --example custom_error

### Se faciliter la vie avec `anyhow`

C'est le genre de code qu'on écrit à Clever Cloud. Propagation d'erreur plus plus.

    cargo run --example anyhow_result
