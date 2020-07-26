<!--
## Improving Our I/O Project
-->

## Amélioration de notre projet d'entrée/sortie

<!--
With this new knowledge about iterators, we can improve the I/O project in
Chapter 12 by using iterators to make places in the code clearer and more
concise. Let’s look at how iterators can improve our implementation of the
`Config::new` function and the `search` function.
-->

Grâce à ces nouvelles connaissances sur les itérateurs, nous pouvons améliorer
le projet d'entrée/sortie du chapitre 12 en utilisant des itérateurs pour
rendre certains endroits du code plus clairs et plus concis. Voyons comment les
itérateurs peuvent améliorer notre implémentation de la fonction `Config::new`
et de la fonction `rechercher`.

<!--
### Removing a `clone` Using an Iterator
-->

### Supprimer l'appel à `clone` à l'aide d'un itérateur

<!--
In Listing 12-6, we added code that took a slice of `String` values and created
an instance of the `Config` struct by indexing into the slice and cloning the
values, allowing the `Config` struct to own those values. In Listing 13-24,
we’ve reproduced the implementation of the `Config::new` function as it was in
Listing 12-23:
-->

Dans l'encart 12-6, nous avions ajouté du code qui prenait une *slice* de
`String` et qui créait une instance de la structure `Config` en utilisant les
indices de la *slice* et en clonant les valeurs, permettant ainsi à la
structure `Config` de posséder ces valeurs. Dans l'encart 13-24, nous avions
reproduit l'implémentation de la fonction `Config::new` comme était dans
l'encart 12-23 à la fin du chapitre 12 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/lib.rs:ch13}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/lib.rs:ch13}}
```

<!--
<span class="caption">Listing 13-24: Reproduction of the `Config::new` function
from Listing 12-23</span>
-->

<span class="caption">Encart 13-24 : reproduction de la fonction `Config::new`
de la fin du chapitre 12</span>

<!--
At the time, we said not to worry about the inefficient `clone` calls because
we would remove them in the future. Well, that time is now!
-->

À ce moment-là, nous avions dit de ne pas s'inquiéter des appels inefficaces à
`clone` parce que nous les supprimerions à l'avenir. Et bien, ce moment est
venu !

<!--
We needed `clone` here because we have a slice with `String` elements in the
parameter `args`, but the `new` function doesn’t own `args`. To return
ownership of a `Config` instance, we had to clone the values from the `query`
and `filename` fields of `Config` so the `Config` instance can own its values.
-->

Nous avions besoin de `clone` ici parce que nous avons une slice d'éléments
`String` dans le paramètre `args`, mais la fonction `new` ne possède pas `args`.
Pour rendre la propriété d'une instance de `Config`, nous avons dû cloner les
valeurs des champs `recherche` et `nom_fichier` de `Config` pour qu'elle puisse
prendre possession de ses valeurs.

<!--
With our new knowledge about iterators, we can change the `new` function to
take ownership of an iterator as its argument instead of borrowing a slice.
We’ll use the iterator functionality instead of the code that checks the length
of the slice and indexes into specific locations. This will clarify what the
`Config::new` function is doing because the iterator will access the values.
-->

Avec nos nouvelles connaissances sur les itérateurs, nous pouvons changer la
fonction `new` pour prendre possession d'un itérateur passé en argument au lieu
d'emprunter une *slice*. Nous utiliserons les fonctionnalités des itérateurs à
la place du code qui vérifie la taille de la slice et qui utilise les indices
des éléments précis. Cela clarifiera ce que la fonction `Config::new` fait car
l'itérateur accédera aux valeurs.

<!--
Once `Config::new` takes ownership of the iterator and stops using indexing
operations that borrow, we can move the `String` values from the iterator into
`Config` rather than calling `clone` and making a new allocation.
-->

Une fois que `Config::new` prend possession de l'itérateur et cesse d'utiliser
les opérations avec les indices et d'emprunter les données, nous pouvons
déplacer les valeurs `String` de l'iterator dans `Config` plutôt que de faire
appel à `clone` et de créer par conséquent de nouvelles allocations.

<!--
#### Using the Returned Iterator Directly
-->

#### Utiliser directement l'itérateur retourné

<!--
Open your I/O project’s *src/main.rs* file, which should look like this:
-->

Ouvrez le fichier *src/main.rs* de votre projet d'entrée/sortie, qui devrait
ressembler à ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

<!--
We’ll change the start of the `main` function that we had in Listing 12-24 to
the code in Listing 13-25. This won’t compile until we update `Config::new` as
well.
-->

Nous allons changer le début de la fonction `main` que nous avions dans l'encart
12-24 pour le code dans l'encart 13-25. Ceci ne compilera pas encore jusqu'à ce
que nous mettions également à jour `Config::new`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-25/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-25/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-25: Passing the return value of `env::args` to
`Config::new`</span>
-->

<span class="caption">Encart 13-25 : on passe directement la valeur de retour de
`env::args` à `Config::new`.</span>

<!--
The `env::args` function returns an iterator! Rather than collecting the
iterator values into a vector and then passing a slice to `Config::new`, now
we’re passing ownership of the iterator returned from `env::args` to
`Config::new` directly.
-->

La fonction `env::args` retourne un itérateur ! Plutôt que de collecter les
valeurs de l'itérateur dans un vecteur et de passer ensuite une *slice* à
`Config::new`, nous passons maintenant la possession de l'itérateur de
`env::args` directement à `Config::new`.

<!--
Next, we need to update the definition of `Config::new`. In your I/O project’s
*src/lib.rs* file, let’s change the signature of `Config::new` to look like
Listing 13-26. This still won’t compile because we need to update the function
body.
-->

Ensuite, nous devons mettre à jour la définition de `Config::new`. Dans le
fichier *src/lib.rs* de votre projet d'entrée/sortie, modifions la signature de
`Config::new` pour qu'elle ressemble à l'encart 13-26. Ceci ne compilera pas
encore car nous devons mettre à jour le corps de la fonction.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-26/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-26/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-26: Updating the signature of `Config::new` to
expect an iterator</span>
-->

<span class="caption">Encart 13-26 : mise à jour de la signature de
`Config::new` pour recevoir un itérateur</span>

<!--
The standard library documentation for the `env::args` function shows that the
type of the iterator it returns is `std::env::Args`. We’ve updated the
signature of the `Config::new` function so the parameter `args` has the type
`std::env::Args` instead of `&[String]`. Because we’re taking ownership of
`args` and we’ll be mutating `args` by iterating over it, we can add the `mut`
keyword into the specification of the `args` parameter to make it mutable.
-->

La documentation de la bibliothèque standard de la fonction `env::args`
indique que le type de l'itérateur qu'elle renvoie est `std::env::Args`. Nous
avons mis à jour la signature de la fonction `Config::new` pour que le
paramètre `args` ait le type `std::env::Args` au lieu de `&[String]`. Etant
donné que nous prenons possession de `args` et que nous allons muter `args`
en itérant dessus, nous pouvons ajouter le mot-clé `mut` dans la spécification
du paramètre `args` pour le rendre mutable.

<!--
#### Using `Iterator` Trait Methods Instead of Indexing
-->

#### Utilisation des méthodes du trait `Iterator` au lieu des indices

<!--
Next, we’ll fix the body of `Config::new`. The standard library documentation
also mentions that `std::env::Args` implements the `Iterator` trait, so we know
we can call the `next` method on it! Listing 13-27 updates the code from
Listing 12-23 to use the `next` method:
-->

Ensuite corrigeons le corps de `Config::new`. La documentation de la bibliothèque
standard explique aussi que `std::env::Args` implémente le trait `Iterator`, donc
nous savons que nous pouvons appeler la méthode `next` dessus ! L'encart 13-27
met à jour le code de l'encart 12-23 afin d'utiliser la méthode `next` :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-27/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-27/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-27: Changing the body of `Config::new` to use
iterator methods</span>
-->

<span class="caption">Encart 13-27 : changement du corps de `Config::new` afin
d'utiliser les méthodes d'itération</span>

<!--
Remember that the first value in the return value of `env::args` is the name of
the program. We want to ignore that and get to the next value, so first we call
`next` and do nothing with the return value. Second, we call `next` to get the
value we want to put in the `query` field of `Config`. If `next` returns a
`Some`, we use a `match` to extract the value. If it returns `None`, it means
not enough arguments were given and we return early with an `Err` value. We do
the same thing for the `filename` value.
-->

Rappelez-vous que la première valeur de ce qui est retourné par `env::args` est
le nom du programme. Nous voulons ignorer cette valeur et passer à la suivante,
donc d'abord nous appelons une fois `next` et nous ne faisons rien avec sa
valeur de retour. Ensuite, nous appelons `next` pour obtenir la valeur que nous
voulons mettre dans le champ `recherche` de `Config`. Si `next` renvoie un
`Some`, nous utilisons un `match` pour extraire sa valeur. S'il retourne `None`,
cela signifie qu'il n'y a pas assez d'arguments donnés et nous revenons plus tôt
avec une valeur `Err`. De même pour la valeur `nom_fichier`.

<!--
### Making Code Clearer with Iterator Adaptors
-->

### Rendre le code plus clair avec des adaptateurs d'itération

<!--
We can also take advantage of iterators in the `search` function in our I/O
project, which is reproduced here in Listing 13-28 as it was in Listing 12-19:
-->

Nous pouvons également tirer parti des itérateurs dans la fonction
`rechercher` de notre projet d'entrée/sortie, qui est reproduite ici dans
l'encart 13-28, comme elle l'était dans l'encart 12-19 à la fin du chapitre 12 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

<!--
<span class="caption">Listing 13-28: The implementation of the `search`
function from Listing 12-19</span>
-->

<span class="caption">Encart 13-28 : La mise en oeuvre de la fonction
`rechercher` de l'encart 12-19</span>

<!--
We can write this code in a more concise way using iterator adaptor methods.
Doing so also lets us avoid having a mutable intermediate `results` vector. The
functional programming style prefers to minimize the amount of mutable state to
make code clearer. Removing the mutable state might enable a future enhancement
to make searching happen in parallel, because we wouldn’t have to manage
concurrent access to the `results` vector. Listing 13-29 shows this change:
-->

Nous pouvons écrire ce code de façon plus concise en utilisant des méthodes
des adaptateurs d'itération. Ce faisant, nous évitons ainsi d'avoir le vecteur
mutable `resultats`. Le style de programmation fonctionnelle préfère minimiser
la quantité d'états modifiables pour rendre le code plus clair. Supprimer l'état
mutable pourrait nous aider à faire une amélioration future afin que la recherche
se fasse en parallèle, car nous n'aurions pas à gérer l'accès concurrent au
vecteur `resultats`. L'encart 13-29 montre ce changement :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-29/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-29/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-29: Using iterator adaptor methods in the
implementation of the `search` function</span>
-->

<span class="caption">Encart 13-29 : utilisation des méthodes des adaptateurs
d'itération dans l'implémentation de la fonction `rechercher`</span>

<!--
Recall that the purpose of the `search` function is to return all lines in
`contents` that contain the `query`. Similar to the `filter` example in Listing
13-19, this code uses the `filter` adaptor to keep only the lines that
`line.contains(query)` returns `true` for. We then collect the matching lines
into another vector with `collect`. Much simpler! Feel free to make the same
change to use iterator methods in the `search_case_insensitive` function as
well.
-->

Souvenez-vous que le but de la fonction `rechercher` est de renvoyer toutes
les lignes dans `contenu` qui contiennent `recherche`. Comme dans l'exemple de
`filter` dans l'encart 13-19, nous pouvons utiliser l'adaptateur `filter`
pour garder uniquement les lignes pour lesquelles `ligne.contains(recherche)`
renvoie `true`. Nous collectons ensuite les lignes correspondantes dans un
autre vecteur avec `collect`. C'est bien plus simple ! N'hésitez pas à faire
le même changement pour utiliser les méthodes d'itération dans la fonction
`rechercher_insensible_casse`.

<!--
The next logical question is which style you should choose in your own code and
why: the original implementation in Listing 13-28 or the version using
iterators in Listing 13-29. Most Rust programmers prefer to use the iterator
style. It’s a bit tougher to get the hang of at first, but once you get a feel
for the various iterator adaptors and what they do, iterators can be easier to
understand. Instead of fiddling with the various bits of looping and building
new vectors, the code focuses on the high-level objective of the loop. This
abstracts away some of the commonplace code so it’s easier to see the concepts
that are unique to this code, such as the filtering condition each element in
the iterator must pass.
-->

Logiquement la question suivante est de savoir quel style utiliser dans votre
propre code et pourquoi : l'implémentation originale de l'encart 13-28 ou la
version utilisant l'itérateur dans l'encart 13-29. La plupart des développeurs
Rust préfèrent utiliser le style avec l'itérateur. C'est un peu plus difficile
à comprendre au début, mais une fois que vous avez compris les différents
adaptateurs d'itération et de ce qu'ils font, les itérateurs peuvent devenir
plus faciles à comprendre. Au lieu de jongler avec différentes boucles et de
construire de nouveaux vecteurs, ce code se concentre sur l'objectif de haut
niveau de la boucle. Cette abstraction permet d'éliminer une partie du code
trivial, de sorte qu'il soit plus facile de dégager les concepts propres à ce
code, comme le filtrage de chaque élément de l'itérateur qui est appliqué.

<!--
But are the two implementations truly equivalent? The intuitive assumption
might be that the more low-level loop will be faster. Let’s talk about
performance.
-->

Mais ces deux implémentations sont-elles réellement équivalentes ? L'hypothèse
intuitive pourrait être que la boucle de plus bas niveau sera plus rapide.
Intéressons nous maintenant à leurs performances.
