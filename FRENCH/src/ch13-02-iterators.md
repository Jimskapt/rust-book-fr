<!--
## Processing a Series of Items with Iterators
-->

## Traiter une série d'éléments avec un itérateur

<!--
The iterator pattern allows you to perform some task on a sequence of items in
turn. An iterator is responsible for the logic of iterating over each item and
determining when the sequence has finished. When you use iterators, you don’t
have to reimplement that logic yourself.
-->

Les itérateurs vous permettent d'effectuer une tâche sur une séquence d'éléments
à tour de rôle. Un *itérateur* est responsable de la logique d'itération sur
chaque élément et de déterminer lorsque la séquence est terminée. Lorsque nous
utilisons des itérateurs, nous n'avons pas besoin de ré-implémenter cette
logique nous-mêmes.

<!--
In Rust, iterators are *lazy*, meaning they have no effect until you call
methods that consume the iterator to use it up. For example, the code in
Listing 13-13 creates an iterator over the items in the vector `v1` by calling
the `iter` method defined on `Vec<T>`. This code by itself doesn’t do anything
useful.
-->

En Rust, un itérateur est *une évaluation paresseuse*, ce qui signifie qu'il n'a
aucun effet jusqu'à ce que nous appelions des méthodes qui consomment
l'itérateur pour l'utiliser. Par exemple, le code dans l'encart 13-13 crée un
itérateur sur les éléments du vecteur `v1` en appelant la méthode `iter` définie
sur `Vec<T>`. Ce code en lui-même ne fait rien d'utile.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch13-functional-features/listing-13-13/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-13: Creating an iterator</span>
-->

<span class="caption">Encart 13-13 : création d'un itérateur</span>

<!--
Once we’ve created an iterator, we can use it in a variety of ways. In Listing
3-5 in Chapter 3, we used iterators with `for` loops to execute some code on
each item, although we glossed over what the call to `iter` did until now.
-->

Une fois que nous avons créé un itérateur, nous pouvons l'utiliser de diverses
manières. Dans l'encart 3-4 du chapitre 3, nous avions utilisé des itérateurs
avec des boucles `for` pour exécuter du code sur chaque élément, bien que nous
ayons laissé de côté ce que l'appel à `iter` faisait jusqu'à présent.

<!--
The example in Listing 13-14 separates the creation of the iterator from the
use of the iterator in the `for` loop. The iterator is stored in the `v1_iter`
variable, and no iteration takes place at that time. When the `for` loop is
called using the iterator in `v1_iter`, each element in the iterator is used in
one iteration of the loop, which prints out each value.
-->

L'exemple dans l'encart 13-14 sépare la création de l'itérateur de son
utilisation dans la boucle `for`. L'itérateur est stocké dans la variable
`v1_iter`, et aucune itération n'a lieu à ce moment-là. Lorsque la boucle `for`
est appelée en utilisant l'itérateur `v1_iter`, chaque élément de l'itérateur
est utilisé à chaque itération de la boucle, qui affiche chaque valeur.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch13-functional-features/listing-13-14/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-14: Using an iterator in a `for` loop</span>
-->

<span class="caption">Encart 13-14 : utilisation d'un itérateur dans une boucle
`for`</span>

<!--
In languages that don’t have iterators provided by their standard libraries,
you would likely write this same functionality by starting a variable at index
0, using that variable to index into the vector to get a value, and
incrementing the variable value in a loop until it reached the total number of
items in the vector.
-->

Dans les langages qui n'ont pas d'itérateurs fournis par leur bibliothèque
standard, nous écririons probablement cette même fonctionnalité en démarrant une
variable à l'indice 0, utiliser cette variable comme indice sur le vecteur afin
d'obtenir une valeur, et incrémenter la valeur de cette variable dans une boucle
jusqu'à ce qu'elle atteigne le nombre total d'éléments dans le vecteur.

<!--
Iterators handle all that logic for you, cutting down on repetitive code you
could potentially mess up. Iterators give you more flexibility to use the same
logic with many different kinds of sequences, not just data structures you can
index into, like vectors. Let’s examine how iterators do that.
-->

Les itérateurs s'occupent de toute cette logique pour nous, réduisant le code
redondant dans lequel nous pourrions potentiellement faire des erreurs. Les
itérateurs nous donnent plus de flexibilité pour utiliser la même logique avec
de nombreux types de séquences différentes, et non pas uniquement avec des
structures de données avec lesquelles nous pouvons utiliser des indices, comme
les vecteurs. Voyons comment les itérateurs font cela.

<!--
### The `Iterator` Trait and the `next` Method
-->

### Le trait `Iterator` et la méthode `next`

<!--
All iterators implement a trait named `Iterator` that is defined in the
standard library. The definition of the trait looks like this:
-->

Tous les itérateurs implémentent un trait appelé `Iterator` qui est défini dans
la bibliothèque standard. La définition du trait ressemble à ceci :

<!--
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```
-->

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // les méthodes avec des implémentations par défaut ont été exclues
}
```

<!--
Notice this definition uses some new syntax: `type Item` and `Self::Item`,
which are defining an *associated type* with this trait. We’ll talk about
associated types in depth in Chapter 19. For now, all you need to know is that
this code says implementing the `Iterator` trait requires that you also define
an `Item` type, and this `Item` type is used in the return type of the `next`
method. In other words, the `Item` type will be the type returned from the
iterator.
-->

Remarquez que cette définition utilise une nouvelle syntaxe : `type Item` et
`Self::Item`, qui définissent un *type associé* à ce trait. Nous verrons ce que
sont les types associés au chapitre 19. Pour l'instant, tout ce que vous devez
savoir est que ce code dit que l'implémentation du trait `Iterator` nécessite
que vous définissiez aussi un type `Item`, et ce type `Item` est utilisé dans le
type de retour de la méthode `next`. En d'autres termes, le type `Item` sera le
type retourné par l'itérateur.

<!--
The `Iterator` trait only requires implementors to define one method: the
`next` method, which returns one item of the iterator at a time wrapped in
`Some` and, when iteration is over, returns `None`.
-->

Le trait `Iterator` exige la définition d'une seule méthode par les
développeurs : la méthode `next`, qui retourne un élément de l'itérateur à la
fois intégré dans un `Some`, et lorsque l'itération est terminée, il retourne
`None`.

<!--
We can call the `next` method on iterators directly; Listing 13-15 demonstrates
what values are returned from repeated calls to `next` on the iterator created
from the vector.
-->

On peut appeler la méthode `next` directement sur les itérateurs ; l'encart
13-15 montre quelles valeurs sont retournées par des appels répétés à `next` sur
l'itérateur créé à partir du vecteur.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch13-functional-features/listing-13-15/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-15: Calling the `next` method on an
iterator</span>
-->

<span class="caption">Encart 13-15 : appel de la méthode `next` sur un itérateur
</span>

<!--
Note that we needed to make `v1_iter` mutable: calling the `next` method on an
iterator changes internal state that the iterator uses to keep track of where
it is in the sequence. In other words, this code *consumes*, or uses up, the
iterator. Each call to `next` eats up an item from the iterator. We didn’t need
to make `v1_iter` mutable when we used a `for` loop because the loop took
ownership of `v1_iter` and made it mutable behind the scenes.
-->

Remarquez que nous avons eu besoin de rendre mutable `v1_iter` : appeler la
méthode `next` sur un iterator change son état interne qui garde en mémoire où
il est dans la séquence. En d'autres termes, ce code *consomme*, ou utilise,
l'itérateur. Chaque appel à `next` consomme un élément de l'itérateur. Nous
n'avions pas eu besoin de rendre mutable `v1_iter` lorsque nous avions utilisé
une boucle `for` parce que la boucle avait pris possession de `v1_iter` et l'a
rendu mutable en coulisses.

<!--
Also note that the values we get from the calls to `next` are immutable
references to the values in the vector. The `iter` method produces an iterator
over immutable references. If we want to create an iterator that takes
ownership of `v1` and returns owned values, we can call `into_iter` instead of
`iter`. Similarly, if we want to iterate over mutable references, we can call
`iter_mut` instead of `iter`.
-->

Notez également que les valeurs que nous obtenons des appels à `next` sont des
références immuables aux valeurs dans le vecteur. La méthode `iter` produit un
itérateur pour des références immuables. Si nous voulons créer un itérateur qui
prend possession de `v1` et retourne les valeurs possédées, nous pouvons appeler
`into_iter` au lieu de `iter`. De même, si nous voulons itérer sur des
références mutables, nous pouvons appeler `iter_mut` au lieu de `iter`.

<!--
### Methods that Consume the Iterator
-->

### Les méthodes qui consomment un itérateur

<!--
The `Iterator` trait has a number of different methods with default
implementations provided by the standard library; you can find out about these
methods by looking in the standard library API documentation for the `Iterator`
trait. Some of these methods call the `next` method in their definition, which
is why you’re required to implement the `next` method when implementing the
`Iterator` trait.
-->

Le trait `Iterator` a un certain nombre de méthodes différentes avec des
implémentations par défaut que nous fournit la bibliothèque standard ; vous
pouvez découvrir ces méthodes en regardant dans la documentation de l'API de la
bibliothèque standard pour le trait `Iterator`. Certaines de ces méthodes
appellent la méthode `next` dans leur définition, c'est pourquoi nous devons
toujours implémenter la méthode `next` lors de l'implémentation du trait
`Iterator`.

<!--
Methods that call `next` are called *consuming adaptors*, because calling them
uses up the iterator. One example is the `sum` method, which takes ownership of
the iterator and iterates through the items by repeatedly calling `next`, thus
consuming the iterator. As it iterates through, it adds each item to a running
total and returns the total when iteration is complete. Listing 13-16 has a
test illustrating a use of the `sum` method:
-->

Les méthodes qui appellent `next` sont appelées des
*adaptateurs de consommation*, parce que les appeler consomme l'itérateur. Un
exemple est la méthode `sum`, qui prend possession de l'itérateur et itére sur
ses éléments en appelant plusieurs fois `next`, consommant ainsi l'itérateur. A
chaque étape de l'itération, il ajoute chaque élément à un total en cours et
retourne le total une fois l'itération terminée. L'encart 13-16 a un test
illustrant une utilisation de la méthode `sum` :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch13-functional-features/listing-13-16/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-16: Calling the `sum` method to get the total
of all items in the iterator</span>
-->

<span class="caption">Encart 13-16 : appel de la méthode `sum` pour obtenir la
somme de tous les éléments présents dans l'itérateur</span>

<!--
We aren’t allowed to use `v1_iter` after the call to `sum` because `sum` takes
ownership of the iterator we call it on.
-->

Nous ne sommes pas autorisés à utiliser `v1_iter` après l'appel à `sum` car
`sum` a pris possession de l'itérateur sur lequel nous l'appelons.

<!--
### Methods that Produce Other Iterators
-->

### Méthodes qui produisent d'autres itérateurs

<!--
Other methods defined on the `Iterator` trait, known as *iterator adaptors*,
allow you to change iterators into different kinds of iterators. You can chain
multiple calls to iterator adaptors to perform complex actions in a readable
way. But because all iterators are lazy, you have to call one of the consuming
adaptor methods to get results from calls to iterator adaptors.
-->

D'autres méthodes définies sur le trait `Iterator`, connues sous le nom
*d'adaptateurs d'itération*, nous permettent de transformer un itérateur en un
type d'itérateur différent. Nous pouvons enchaîner plusieurs appels à des
adaptateurs d'itération pour effectuer des actions complexes de manière
compréhensible. Mais comme les itérateurs sont *des évaluations paresseuses*,
nous devons faire appel à l'une des méthodes d'adaptation de consommation pour
obtenir les résultats des appels aux adaptateurs d'itération.

<!--
Listing 13-17 shows an example of calling the iterator adaptor method `map`,
which takes a closure to call on each item to produce a new iterator. The
closure here creates a new iterator in which each item from the vector has been
incremented by 1. However, this code produces a warning:
-->

L'encart 13-17 montre un exemple d'appel à la méthode d'adaptation d'itération
`map`, qui prend en paramètre une fermeture qui va s'exécuter sur chaque élément
pour produire un nouvel itérateur. La fermeture crée ici un nouvel itérateur
dans lequel chaque élément du vecteur a été incrémenté de 1. Cependant, ce code
déclenche un avertissement :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,not_desired_behavior
{{#rustdoc_include ../listings-sources/ch13-functional-features/listing-13-17/src/main.rs:here}}
```
-->

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-17/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-17: Calling the iterator adaptor `map` to
create a new iterator</span>
-->

<span class="caption">Encart 13-17 : appel de l'adaptateur d'itération `map`
pour créer un nouvel itérateur</span>

<!--
The warning we get is this:
-->

Voici l'avertissement que nous obtenons :

<!--
```console
{{#include ../listings-sources/ch13-functional-features/listing-13-17/output.txt}}
```
-->

```console
{{#include ../listings/ch13-functional-features/listing-13-17/output.txt}}
```

<!--
The code in Listing 13-17 doesn’t do anything; the closure we’ve specified
never gets called. The warning reminds us why: iterator adaptors are lazy, and
we need to consume the iterator here.
-->

Le code dans l'encart 13-17 ne fait rien ; la fermeture que nous avons renseigné
n'est jamais exécuté. L'avertissement nous rappelle pourquoi : les adaptateurs
d'itération sont des *évaluations paresseuses*, c'est pourquoi nous devons
consommer l'itérateur ici.

<!--
To fix this and consume the iterator, we’ll use the `collect` method, which we
used in Chapter 12 with `env::args` in Listing 12-1. This method consumes the
iterator and collects the resulting values into a collection data type.
-->

Pour corriger ceci et consommer l'itérateur, nous utiliserons la méthode
`collect`, que vous avez utilisé avec `env::args` dans l'encart 12-1 du
chapitre 12. Cette méthode consomme l'itérateur et collecte les valeurs
résultantes dans un type de collection de données.

<!--
In Listing 13-18, we collect the results of iterating over the iterator that’s
returned from the call to `map` into a vector. This vector will end up
containing each item from the original vector incremented by 1.
-->

Dans l'encart 13-18, nous recueillons les résultats de l'itération sur
l'itérateur qui sont retournés par l'appel à `map` sur un vecteur. Ce vecteur
finira par contenir chaque élément du vecteur original incrémenté de 1.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch13-functional-features/listing-13-18/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-18: Calling the `map` method to create a new
iterator and then calling the `collect` method to consume the new iterator and
create a vector</span>
-->

<span class="caption">Encart 13-18 : appel de la méthode `map` pour créer un
nouvel itérateur, puis appel de la méthode `collect` pour consommer le nouvel
itérateur afin de créer un vecteur</span>

<!--
Because `map` takes a closure, we can specify any operation we want to perform
on each item. This is a great example of how closures let you customize some
behavior while reusing the iteration behavior that the `Iterator` trait
provides.
-->

Comme `map` prend en paramètre une fermeture, nous pouvons renseigner n'importe
quelle opération que nous souhaitons exécuter sur chaque élément. C'est un bon
exemple de la façon dont les fermetures nous permettent de personnaliser
certains comportements tout en réutilisant le comportement d'itération fourni
par le trait `Iterator`.

<!--
### Using Closures that Capture Their Environment
-->

### Utilisation de fermetures capturant leur environnement

<!--
Now that we’ve introduced iterators, we can demonstrate a common use of
closures that capture their environment by using the `filter` iterator adaptor.
The `filter` method on an iterator takes a closure that takes each item from
the iterator and returns a Boolean. If the closure returns `true`, the value
will be included in the iterator produced by `filter`. If the closure returns
`false`, the value won’t be included in the resulting iterator.
-->

Maintenant que nous avons présenté les itérateurs, nous pouvons illustrer une
utilisation commune des fermetures qui capturent leur environnement en utilisant
l'adaptateur d'itération `filter`. La méthode `filter` appelée sur un itérateur
prend en paramètre une fermeture qui s'exécute sur chaque élément de l'itérateur
et retourne un booléen pour chacun. Si la fermeture retourne `true`, la valeur
sera incluse dans l'itérateur produit par `filter`. Si la fermeture retourne
`false`, la valeur ne sera pas incluse dans l'itérateur résultant.

<!--
In Listing 13-19, we use `filter` with a closure that captures the `shoe_size`
variable from its environment to iterate over a collection of `Shoe` struct
instances. It will return only shoes that are the specified size.
-->

Dans l'encart 13-19, nous utilisons `filter` avec une fermeture qui capture la
variable `pointure_chaussure` de son environnement pour itérer sur une
collection d'instances de la structure `Chaussure`. Il ne retournera que les
chaussures avec la pointure demandée.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch13-functional-features/listing-13-19/src/lib.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/lib.rs}}
```

<!--
<span class="caption">Listing 13-19: Using the `filter` method with a closure
that captures `shoe_size`</span>
-->

<span class="caption">Encart 13-19 : utilisation de la méthode `filter` avec une
fermeture capturant `pointure_chaussure`</span>

<!--
The `shoes_in_my_size` function takes ownership of a vector of shoes and a shoe
size as parameters. It returns a vector containing only shoes of the specified
size.
-->

La fonction `chaussures_a_la_pointure` prend possession d'un vecteur de
chaussures et d'une pointure comme paramètres. Il retourne un vecteur contenant
uniquement des chaussures de la pointure demandée.

<!--
In the body of `shoes_in_my_size`, we call `into_iter` to create an iterator
that takes ownership of the vector. Then we call `filter` to adapt that
iterator into a new iterator that only contains elements for which the closure
returns `true`.
-->

Dans le corps de `chaussures_a_la_pointure`, nous appelons `into_iter` pour
créer un itérateur qui prend possession du vecteur. Ensuite, nous appelons
`filter` pour adapter cet itérateur dans un nouvel itérateur qui ne contient que
les éléments pour lesquels la fermeture retourne `true`.

<!--
The closure captures the `shoe_size` parameter from the environment and
compares the value with each shoe’s size, keeping only shoes of the size
specified. Finally, calling `collect` gathers the values returned by the
adapted iterator into a vector that’s returned by the function.
-->

La fermeture capture le paramètre `pointure_chaussure` de l'environnement et
compare la valeur avec la pointure de chaque chaussure, en ne gardant que les
chaussures de la pointure spécifiée. Enfin, l'appel à `collect` retourne un
vecteur qui regroupe les valeurs renvoyées par l'itérateur.

<!--
The test shows that when we call `shoes_in_my_size`, we get back only shoes
that have the same size as the value we specified.
-->

Le test confirme que lorsque nous appelons `chaussures_a_la_pointure`, nous
n'obtenons que des chaussures qui ont la même pointure que la valeur que nous
avons demandé.

<!--
### Creating Our Own Iterators with the `Iterator` Trait
-->

### Créer nos propres itérateurs avec le trait `Iterator`

<!--
We’ve shown that you can create an iterator by calling `iter`, `into_iter`, or
`iter_mut` on a vector. You can create iterators from the other collection
types in the standard library, such as hash map. You can also create iterators
that do anything you want by implementing the `Iterator` trait on your own
types. As previously mentioned, the only method you’re required to provide a
definition for is the `next` method. Once you’ve done that, you can use all
other methods that have default implementations provided by the `Iterator`
trait!
-->

Nous avons vu que nous pouvons créer un itérateur en appelant `iter`,
`into_iter`, ou `iter_mut` sur un vecteur. Nous pouvons créer des itérateurs à
partir d'autres types de collections de la bibliothèque standard, comme les
tables de hachage. Nous pouvons aussi créer des itérateurs qui font tout ce que
nous voulons en implémentant le trait `Iterator` sur nos propres types. Comme
nous l'avons mentionné précédemment, la seule méthode pour laquelle nous devons
fournir une définition est la méthode `next`. Une fois que nous avons fait cela,
nous pouvons utiliser toutes les autres méthodes qui ont des implémentations par
défaut fournies par le trait `Iterator` !

<!--
To demonstrate, let’s create an iterator that will only ever count from 1 to 5.
First, we’ll create a struct to hold some values. Then we’ll make this struct
into an iterator by implementing the `Iterator` trait and using the values in
that implementation.
-->

Pour preuve, créons un itérateur qui ne comptera que de 1 à 5. D'abord, nous
allons créer une structure contenant quelques valeurs. Ensuite nous
transformerons cette structure en itérateur en implémentant le trait `Iterator`
et nous utiliserons les valeurs de cette implémentation.

<!--
Listing 13-20 has the definition of the `Counter` struct and an associated
`new` function to create instances of `Counter`:
-->

L'encart 13-20 montre la définition de la structure `Compteur` et une fonction
associée `new` pour créer des instances de `Compteur` :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch13-functional-features/listing-13-20/src/lib.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/lib.rs}}
```

<!--
<span class="caption">Listing 13-20: Defining the `Counter` struct and a `new`
function that creates instances of `Counter` with an initial value of 0 for
`count`</span>
-->

<span class="caption">Encart 13-20 : définition de la structure `Compteur` et
d'une fonction `new` qui crée des instances de `Compteur` avec une valeur
initiale de 0 pour le champ `compteur`.</span>

<!--
The `Counter` struct has one field named `count`. This field holds a `u32`
value that will keep track of where we are in the process of iterating from 1
to 5. The `count` field is private because we want the implementation of
`Counter` to manage its value. The `new` function enforces the behavior of
always starting new instances with a value of 0 in the `count` field.
-->

La structure `Compteur` a un champ `compteur`. Ce champ contient une valeur
`u32` qui gardera la trace de l'endroit où nous sommes dans le processus
d'itération de 1 à 5. Le champ `compteur` est privé car nous voulons que ce soit
l'implémentation de `Compteur` qui gère sa valeur. La fonction `new` impose le
de toujours démarrer de nouvelles instances avec une valeur de 0 pour le champ
`compteur`.

<!--
Next, we’ll implement the `Iterator` trait for our `Counter` type by defining
the body of the `next` method to specify what we want to happen when this
iterator is used, as shown in Listing 13-21:
-->

Ensuite, nous allons implémenter le trait `Iterator` sur notre type `Compteur`
en définissant le corps de la méthode `next` pour préciser ce que nous voulons
qu'il se passe quand cet itérateur est utilisé, comme dans l'encart 13-21 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch13-functional-features/listing-13-21/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-21/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-21: Implementing the `Iterator` trait on our
`Counter` struct</span>
-->

<span class="caption">Encart 13-21 : implémentation du trait `Iterator` sur
notre structure `Compteur`</span>

<!--
We set the associated `Item` type for our iterator to `u32`, meaning the
iterator will return `u32` values. Again, don’t worry about associated types
yet, we’ll cover them in Chapter 19.
-->

Nous avons défini le type associé `Item` pour notre itérateur à `u32`, ce qui
signifie que l'itérateur renverra des valeurs `u32`. Encore une fois, ne vous
préoccupez pas des types associés, nous les aborderons au chapitre 19.

<!--
We want our iterator to add 1 to the current state, so we initialized `count`
to 0 so it would return 1 first. If the value of `count` is less than 5, `next`
will increment `count` and return the current value wrapped in `Some`. Once
`count` is 5, our iterator will stop incrementing `count` and always return
`None`.
-->

Nous voulons que notre itérateur ajoute 1 à l'état courant, donc nous avons
initialisé `compteur` à 0 pour qu'il retourne 1 lors du premier appel à `next`.
Si la valeur de `compteur` est strictement inférieure à 5, `next` va incrémenter
`compteur` puis va retourner valeur courante intégrée dans un `Some`. Une fois
que `compteur` vaudra 5, notre itérateur va arrêter d'incrémenter `compteur` et
retournera toujours `None`.

<!--
#### Using Our `Counter` Iterator’s `next` Method
-->

### Utiliser la méthode `next` de notre Itérateur `Compteur`

<!--
Once we’ve implemented the `Iterator` trait, we have an iterator! Listing 13-22
shows a test demonstrating that we can use the iterator functionality of our
`Counter` struct by calling the `next` method on it directly, just as we did
with the iterator created from a vector in Listing 13-15.
-->

Une fois que nous avons implémenté le trait `Iterator`, nous avons un
itérateur ! L'encart 13-22 montre un test démontrant que nous pouvons utiliser
la fonctionnalité d'itération de notre structure `Compteur` en appelant
directement la méthode `next`, comme nous l'avons fait avec l'itérateur créé à
partir d'un vecteur dans l'encart 13-15.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-22: Testing the functionality of the `next`
method implementation</span>
-->

<span class="caption">Encart 13-22 : test de l'implémentation de la méthode
`next`</span>

<!--
This test creates a new `Counter` instance in the `counter` variable and then
calls `next` repeatedly, verifying that we have implemented the behavior we
want this iterator to have: returning the values from 1 to 5.
-->

Ce test créé une nouvelle instance de `Compteur` dans la variable `compteur` et
appelle ensuite `next` à plusieurs reprises, en vérifiant que nous avons
implémenté le comportement que nous voulions que cet itérateur suive : renvoyer
les valeurs de 1 à 5.

<!--
#### Using Other `Iterator` Trait Methods
-->

#### Utiliser d'autres méthodes du trait `Iterator`

<!--
We implemented the `Iterator` trait by defining the `next` method, so we
can now use any `Iterator` trait method’s default implementations as defined in
the standard library, because they all use the `next` method’s functionality.
-->

Maintenant que nous avons implémenté le trait `Iterator` en définissant la
méthode `next`, nous pouvons maintenant utiliser les implémentations par défaut
de n'importe quelle méthode du trait `Iterator` telles que définies dans la
bibliothèque standard, car elles utilisent toutes la méthode `next`.

<!--
For example, if for some reason we wanted to take the values produced by an
instance of `Counter`, pair them with values produced by another `Counter`
instance after skipping the first value, multiply each pair together, keep only
those results that are divisible by 3, and add all the resulting values
together, we could do so, as shown in the test in Listing 13-23:
-->

Par exemple, si pour une raison quelconque nous voulions prendre les valeurs
produites par une instance de `Compteur`, les coupler avec des valeurs produites
par une autre instance de `Compteur` après avoir sauté la première valeur,
multiplier chaque paire ensemble, ne garder que les résultats qui sont
divisibles par 3 et additionner toutes les valeurs résultantes ensemble, nous
pourrions le faire, comme le montre le test dans l'encart 13-23 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch13-functional-features/listing-13-23/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-23/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-23: Using a variety of `Iterator` trait
methods on our `Counter` iterator</span>
-->


<span class="caption">Encart 13-23 : utilisation d'une gamme de méthodes de
traits `Iterator` sur notre itérateur `Counter` </span>

<!--
Note that `zip` produces only four pairs; the theoretical fifth pair `(5,
None)` is never produced because `zip` returns `None` when either of its input
iterators return `None`.
-->

Notez que `zip` ne produit que quatre paires ; la cinquième paire théorique
`(5, None)` n'est jamais produite car `zip` retourne `None` lorsque l'un de
ses itérateurs d'entrée retourne `None`.

<!--
All of these method calls are possible because we specified how the `next`
method works, and the standard library provides default implementations for
other methods that call `next`.
-->

Tous ces appels de méthode sont possibles car nous avons renseigné comment
la méthode `next` fonctionne et la bibliothèque standard fournit des
implémentations par défaut pour les autres méthodes qui appellent `next`.
