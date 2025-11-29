<!--
## An Example Program Using Structs
-->

## Un exemple de programme qui utilise des structures

<!--
To understand when we might want to use structs, let’s write a program that
calculates the area of a rectangle. We’ll start by using single variables, and
then refactor the program until we’re using structs instead.
-->

Pour comprendre dans quels cas nous voudrions utiliser des structures, écrivons
un programme qui calcule l'aire d'un rectangle. Nous commencerons en utilisant
de simples variables, puis nous remanierons le code jusqu'à utiliser des structures
à la place.

<!--
Let’s make a new binary project with Cargo called *rectangles* that will take
the width and height of a rectangle specified in pixels and calculate the area
of the rectangle. Listing 5-8 shows a short program with one way of doing
exactly that in our project’s *src/main.rs*.
-->

Créons un nouveau projet binaire avec Cargo nommé *rectangles* qui prendra la
largeur et la hauteur en pixels d'un rectangle et qui calculera l'aire de ce
rectangle. L'encart 5-8 montre un petit programme qui effectue cette tâche d'une
certaine manière dans le *src/main.rs* de notre projet.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier: src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

<!--
<span class="caption">Listing 5-8: Calculating the area of a rectangle
specified by separate width and height variables</span>
-->

<span class="caption">Encart 5-8 : Calcul de l'aire d'un rectangle défini par
les variables distinctes `largeur` et `hauteur`</span>

<!--
Now, run this program using `cargo run`:
-->

Maintenant, lancez ce programme avec `cargo run` :

<!--
```console
{{#include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```
-->

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

<!--
This code succeeds in figuring out the area of the rectangle by calling the
`area` function with each dimension, but we can do more to make this code clear
and readable.
-->

Ce code arrive à déterminer l'aire du rectangle en appelant la fonction `aire`
avec chaque dimension, mais on peut faire mieux pour clarifier ce code et le
rendre plus lisible.

<!--
The issue with this code is evident in the signature of `area`:
-->

Le problème de ce code se voit dans la signature de `aire` :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

<!--
The `area` function is supposed to calculate the area of one rectangle, but the
function we wrote has two parameters, and it’s not clear anywhere in our
program that the parameters are related. It would be more readable and more
manageable to group width and height together. We’ve already discussed one way
we might do that in [“The Tuple Type”][the-tuple-type]<!-- ignore -- > section
of Chapter 3: by using tuples.
-->

La fonction `aire` est censée calculer l'aire d'un rectangle, mais la fonction
que nous avons écrite a deux paramètres, et il n'est précisé nulle part
dans notre programme à quoi sont liés les paramètres. Il serait plus lisible et
plus gérable de regrouper ensemble la largeur et la hauteur. Nous avons déjà vu
dans la section [“Le type *tuple*”][the-tuple-type]<!-- ignore --> du chapitre 3
une façon qui nous permettrait de le faire : en utilisant des tuples.

<!--
### Refactoring with Tuples
-->

### Remanier le code avec des tuples

<!--
Listing 5-9 shows another version of our program that uses tuples.
-->

L'encart 5-9 nous montre une autre version de notre programme qui utilise des
tuples.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

<!--
<span class="caption">Listing 5-9: Specifying the width and height of the
rectangle with a tuple</span>
-->

<span class="caption">Encart 5-9 : Passage de la largeur et la hauteur du
rectangle dans un tuple</span>

<!--
In one way, this program is better. Tuples let us add a bit of structure, and
we’re now passing just one argument. But in another way, this version is less
clear: tuples don’t name their elements, so we have to index into the parts of
the tuple, making our calculation less obvious.
-->

D'une certaine façon, ce programme est meilleur. Les tuples nous permettent de
structurer un peu plus et nous ne passons plus qu'un seul argument. Mais d'une autre
façon, cette version est moins claire : les tuples ne donnent pas de noms à
leurs éléments, donc il faut accéder aux éléments du tuple via leur indice, ce
qui rend plus compliqué notre calcul.

<!--
Mixing up the width and height wouldn’t matter for the area calculation, but if
we want to draw the rectangle on the screen, it would matter! We would have to
keep in mind that `width` is the tuple index `0` and `height` is the tuple
index `1`. This would be even harder for someone else to figure out and keep in
mind if they were to use our code. Because we haven’t conveyed the meaning of
our data in our code, it’s now easier to introduce errors.
-->

Le mélange de la largeur et de la hauteur n'est pas important pour calculer l'aire,
mais si on voulait afficher le rectangle à l'écran, cela serait problématique !
Il nous faut garder à l'esprit que la `largeur` est l'élément à l'indice `0` du
tuple et que la `hauteur` est l'élément à l'indice `1`. Cela serait encore plus 
complexe à comprendre et à retenir pour une personne tierce qui viendrait à 
réutiliser notre code. Du fait qu'on n'a pas exprimé la signification de nos données
dans notre code, il est alors plus facile de faire des erreurs.

<!--
### Refactoring with Structs: Adding More Meaning
-->

### Remanier avec des structures : donner plus de sens

<!--
We use structs to add meaning by labeling the data. We can transform the tuple
we’re using into a struct with a name for the whole as well as names for the
parts, as shown in Listing 5-10.
-->

On utilise des structures pour rendre les données plus expressives en leur
donnant des noms. On peut transformer le tuple que nous avons utilisé en une
structure nommée dont ses éléments sont aussi nommés, comme le montre l'encart
5-10.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

<!--
<span class="caption">Listing 5-10: Defining a `Rectangle` struct</span>
-->

<span class="caption">Encart 5-10 : Définition d'une structure
`Rectangle`</span>

<!--
Here we’ve defined a struct and named it `Rectangle`. Inside the curly
brackets, we defined the fields as `width` and `height`, both of which have
type `u32`. Then in `main`, we created a particular instance of `Rectangle`
that has a width of 30 and a height of 50.
-->

Ici, on a défini une structure et on l'a appelée `Rectangle`. Entre les
accolades, on a défini les champs `largeur` et `hauteur`, tous deux du type
`u32`. Puis dans `main`, on crée une instance de `Rectangle` de largeur 30 et de
hauteur 50.

<!--
Our `area` function is now defined with one parameter, which we’ve named
`rectangle`, whose type is an immutable borrow of a struct `Rectangle`
instance. As mentioned in Chapter 4, we want to borrow the struct rather than
take ownership of it. This way, `main` retains its ownership and can continue
using `rect1`, which is the reason we use the `&` in the function signature and
where we call the function.
-->

Notre fonction `aire` est désormais définie avec un unique paramètre, nommé
`rectangle`, et dont le type est une référence immuable vers une instance de la
structure `Rectangle`. Comme mentionné au chapitre 4, on préfère emprunter la
structure au lieu d'en prendre possession. Ainsi, elle reste en possession de
`main` qui peut continuer à utiliser `rect1` ; c'est pourquoi on utilise le `&`
dans la signature de la fonction ainsi que dans l'appel de fonction.

<!--
The `area` function accesses the `width` and `height` fields of the `Rectangle`
instance. Our function signature for `area` now says exactly what we mean:
calculate the area of `Rectangle`, using its `width` and `height` fields. This
conveys that the width and height are related to each other, and it gives
descriptive names to the values rather than using the tuple index values of `0`
and `1`. This is a win for clarity.
-->

La fonction `aire` accède aux champs `largeur` et `hauteur` de l'instance de
`Rectangle`. Notre signature de fonction pour `aire` est enfin explicite :
calculer l'aire d'un `Rectangle` en utilisant ses champs `largeur` et `hauteur`.
Cela explique que la largeur et la hauteur sont liées entre elles, et cela donne
des noms descriptifs aux valeurs plutôt que d'utiliser les valeurs du tuple avec
les indices `0` et `1`. On gagne en clarté.

<!--
### Adding Useful Functionality with Derived Traits
-->

### Ajouter des fonctionnalités utiles avec les traits dérivés

<!--
It’d be useful to be able to print an instance of `Rectangle` while we’re
debugging our program and see the values for all its fields. Listing 5-11 tries
using the [`println!` macro][println]<!-- ignore -- > as we have used in
previous chapters. This won’t work, however.
-->

Il serait pratique de pouvoir afficher une instance de `Rectangle` pendant
qu'on débogue notre programme et de voir la valeur de chacun de ses champs.
L'encart 5-11 essaye de le faire en utilisant [la macro
`println!`][println]<!-- ignore --> comme on l'a fait dans les chapitres
précédents. Cependant, cela ne fonctionne pas.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

<!--
<span class="caption">Listing 5-11: Attempting to print a `Rectangle`
instance</span>
-->

<span class="caption">Encart 5-11 : Tentative d'afficher une instance de
`Rectangle`</span>

<!--
When we compile this code, we get an error with this core message:
-->

Lorsqu'on compile ce code, on obtient ce message d'erreur qui nous informe que
`Rectangle` n'implémente pas le trait `std::fmt::Display` :

<!--
```text
{{#include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```
-->

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

<!--
The `println!` macro can do many kinds of formatting, and by default, the curly
brackets tell `println!` to use formatting known as `Display`: output intended
for direct end user consumption. The primitive types we’ve seen so far
implement `Display` by default, because there’s only one way you’d want to show
a `1` or any other primitive type to a user. But with structs, the way
`println!` should format the output is less clear because there are more
display possibilities: Do you want commas or not? Do you want to print the
curly brackets? Should all the fields be shown? Due to this ambiguity, Rust
doesn’t try to guess what we want, and structs don’t have a provided
implementation of `Display` to use with `println!` and the `{}` placeholder.
-->

La macro `println!` peut faire toutes sortes de formatages textuels, et par
défaut, les accolades demandent à `println!` d'utiliser le formatage appelé
`Display`, pour convertir en texte destiné à être vu par l'utilisateur final.
Les types primitifs qu'on a vus jusqu'ici implémentent `Display` par défaut
puisqu'il n'existe qu'une seule façon d'afficher un `1` ou tout autre type
primitif à l'utilisateur. Mais pour les structures, la façon dont `println!`
devrait formater son résultat est moins claire car il y a plus de possibilités
d'affichage : Voulez-vous des virgules ? Voulez-vous afficher les accolades ?
Est-ce que tous les champs devraient être affichés ? À cause de ces ambiguïtés,
Rust n'essaye pas de deviner ce qu'on veut, et les structures n'implémentent pas
`Display` par défaut pour l'utiliser avec `println!` et les espaces réservés
`{}`.

<!--
If we continue reading the errors, we’ll find this helpful note:
-->

Si nous continuons à lire les erreurs, nous trouvons cette remarque utile :

<!--
```text
{{#include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```
-->

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

Le compilateur nous informe que dans notre chaîne de formatage, on est peut-être
en mesure d'utiliser `{:?}` (ou `{:#?}` pour un affichage plus élégant).

<!--
Let’s try it! The `println!` macro call will now look like `println!("rect1 is
{:?}", rect1);`. Putting the specifier `:?` inside the curly brackets tells
`println!` we want to use an output format called `Debug`. The `Debug` trait
enables us to print our struct in a way that is useful for developers so we can
see its value while we’re debugging our code.
-->

Essayons cela ! L'appel de la macro `println!` ressemble maintenant à
`println!("rect1 est {:?}", rect1);`. Insérer le sélecteur `:?` entre les
accolades permet d'indiquer à `println!` que nous voulons utiliser le formatage
appelé `Debug`. Le trait `Debug` nous permet d'afficher notre structure d'une
manière utile aux développeurs pour qu'on puisse voir sa valeur pendant qu'on
débogue le code.

<!--
Compile the code with this change. Drat! We still get an error:
-->

Compilez le code avec ce changement. Zut ! On a encore une erreur, nous
informant cette fois-ci que `Rectangle` n'implémente pas `std::fmt::Debug` :

<!--
```text
{{#include ../listings-sources/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```
-->

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

<!--
But again, the compiler gives us a helpful note:
-->

Mais une nouvelle fois, le compilateur nous fait une remarque utile :

<!--
```text
{{#include ../listings-sources/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```
-->

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

Il nous conseille d'ajouter `#[derive(Debug)]` ou d'implémenter manuellement
`std::fmt::Debug`.

<!--
Rust *does* include functionality to print out debugging information, but we
have to explicitly opt in to make that functionality available for our struct.
To do that, we add the outer attribute `#[derive(Debug)]` just before the
struct definition, as shown in Listing 5-12.
-->

Rust *inclut* bel et bien une fonctionnalité pour afficher des informations de
débogage, mais nous devons l'activer explicitement pour la rendre disponible sur
notre structure. Pour ce faire, on ajoute l'attribut externe `#[derive(Debug)]`
juste avant la définition de la structure, comme le montre l'encart 5-12.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

<!--
<span class="caption">Listing 5-12: Adding the attribute to derive the `Debug`
trait and printing the `Rectangle` instance using debug formatting</span>
-->

<span class="caption">Encart 5-12 : Ajout de l'attribut pour dériver le
trait `Debug` et afficher l'instance de `Rectangle` en utilisant le formatage
de débogage</span>

<!--
Now when we run the program, we won’t get any errors, and we’ll see the
following output:
-->

Maintenant, quand on exécute le programme, nous n'avons plus d'erreurs et ce
texte s'affiche à l'écran :

<!--
```console
{{#include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```
-->

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

<!--
Nice! It’s not the prettiest output, but it shows the values of all the fields
for this instance, which would definitely help during debugging. When we have
larger structs, it’s useful to have output that’s a bit easier to read; in
those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string.
In this example, using the `{:#?}` style will output:
-->

Super ! Ce n'est pas le plus beau des affichages, mais cela montre les
valeurs de tous les champs de cette instance, ce qui serait assurément utile
lors du débogage. Quand on a des structures plus grandes, il serait bien d'avoir
un affichage un peu plus lisible ; dans ces cas-là, on pourra utiliser `{:#?}`
au lieu de `{:?}` dans la chaîne de formatage. Dans cette exemple,
l'utilisation du style `{:#?}` va afficher ceci :

<!--
```console
{{#include ../listings-sources/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```
-->

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

<!--
Another way to print out a value using the `Debug` format is to use the [`dbg!`
macro][dbg]<!-- ignore -- >, which takes ownership of an expression, prints the
file and line number of where that `dbg!` macro call occurs in your code along
with the resulting value of that expression, and returns ownership of the value.
-->

Une autre façon d'afficher une valeur en utilisant le format `Debug` est
d'utiliser la [macro `dbg!`][dbg]<!-- ignore -->, qui prend possession de
l'expression, affiche le nom du fichier et la ligne de votre code où se trouve
cet appel à la macro `dbg!` ainsi que le résultat de cette expression, puis
rend la possession de cette valeur.

<!--
> Note: Calling the `dbg!` macro prints to the standard error console stream
> (`stderr`), as opposed to `println!` which prints to the standard output
> console stream (`stdout`). We’ll talk more about `stderr` and `stdout` in the
> “[“Writing Error Messages to Standard Error Instead of Standard
> Output” section in Chapter 12][err]<!-- ignore -- >.
-->

> Remarque : l'appel à la macro `dbg!` écrit dans le flux d'erreur standard
> de la console (`stderr`), contrairement à `println!` qui écrit dans le flux
> de sortie standard de la console (`stdout`). Nous reparlerons de `stderr` et
> de `stdout` dans [une section du chapitre 12][err]<!-- ignore -->.

<!--
Here’s an example where we’re interested in the value that gets assigned to the
`width` field, as well as the value of the whole struct in `rect1`:
-->

Voici un exemple dans lequel nous nous intéressons à la valeur assignée au
champ `largeur`, ainsi que la valeur de toute la structure `rect1` :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

<!--
We can put `dbg!` around the expression `30 * scale` and, because `dbg!`
returns ownership of the expression’s value, the `width` field will get the
same value as if we didn’t have the `dbg!` call there. We don’t want `dbg!` to
take ownership of `rect1`, so we use a reference to `rect1` in the next call.
Here’s what the output of this example looks like:
-->

Nous pouvons placer le `dbg!` autour de l'expression `30 * echelle` et, comme
`dbg!` retourne la possession de la valeur issue de l'expression, le champ
`largeur` va avoir la même valeur que si nous n'avions pas appelé `dbg!` ici.
Nous ne voulons pas que `dbg!` prenne possession de `rect1`, donc nous donnons
une référence à `rect1` lors de son prochain appel. Voici à quoi ressemble la
sortie de cet exemple :

<!--
```console
{{#include ../listings-sources/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```
-->

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

<!--
We can see the first bit of output came from *src/main.rs* line 10, where we’re
debugging the expression `30 * scale`, and its resulting value is 60 (the
`Debug` formatting implemented for integers is to print only their value). The
`dbg!` call on line 14 of *src/main.rs* outputs the value of `&rect1`, which is
the `Rectangle` struct. This output uses the pretty `Debug` formatting of the
`Rectangle` type. The `dbg!` macro can be really helpful when you’re trying to
figure out what your code is doing!
-->

Nous pouvons constater que la première sortie provient de la ligne 10
de *src/main.rs*, où nous déboguons l'expression `30 * echelle`, et son résultat
est 60 (le formatage de `Debug` pour les entiers est d'afficher uniquement leur
valeur). L'appel à `dbg!` à la ligne 14 de *src/main.rs* affiche la valeur de
`&rect1`, qui est une structure `Rectangle`. La macro `dbg!` peut être très
utile lorsque vous essayez de comprendre ce que fait votre code !

<!--
In addition to the `Debug` trait, Rust has provided a number of traits for us
to use with the `derive` attribute that can add useful behavior to our custom
types. Those traits and their behaviors are listed in [Appendix C][app-c]<!--
ignore -- >. We’ll cover how to implement these traits with custom behavior as
well as how to create your own traits in Chapter 10. There are also many
attributes other than `derive`; for more information, see [the “Attributes”
section of the Rust Reference][attributes].
-->

En plus du trait `Debug`, Rust nous offre d'autres traits pour que nous
puissions les utiliser avec l'attribut `derive` pour ajouter des comportements
utiles à nos propres types. Ces traits et leurs comportements sont listés à
[l'annexe C][app-c]<!-- ignore -->. Nous expliquerons comment implémenter ces
traits avec des comportements personnalisés et comment créer vos propres traits
au chapitre 10. Il existe aussi de nombreux attributs autres que `derive` ; pour
en savoir plus, consultez [la section “Attributs” de la référence de
Rust][attributes]<!-- ignore -->.

<!--
Our `area` function is very specific: it only computes the area of rectangles.
It would be helpful to tie this behavior more closely to our `Rectangle`
struct, because it won’t work with any other type. Let’s look at how we can
continue to refactor this code by turning the `area` function into an `area`
*method* defined on our `Rectangle` type.
-->

Notre fonction `aire` est très spécifique : elle ne fait que calculer l'aire
d'un rectangle. Il serait utile de lier un peu plus ce comportement à notre
structure `Rectangle`, puisque cela ne fonctionnera pas avec un autre type.
Voyons comment on peut continuer de remanier ce code en transformant la fonction
`aire` en *méthode* `aire` définie sur notre type `Rectangle`.

<!--
[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
-->

[the-tuple-type]: ch03-02-data-types.html
[app-c]: appendix-03-derivable-traits.md
[println]: https://doc.rust-lang.org/std/macro.println.html
[dbg]: https://doc.rust-lang.org/std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: https://doc.rust-lang.org/reference/attributes.html
