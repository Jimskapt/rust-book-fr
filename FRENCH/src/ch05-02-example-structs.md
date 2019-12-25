<!--
## An Example Program Using Structs
-->

## Un exemple de programme qui utilise des structures

<!--
To understand when we might want to use structs, let’s write a program that
calculates the area of a rectangle. We’ll start with single variables, and then
refactor the program until we’re using structs instead.
-->

Pour comprendre dans quels cas nous voudrions utiliser des structures, écrivons
un programme qui calcule l'aire d'un rectangle. Nous commencerons avec de
simples variables, puis on remaniera le code jusqu'à utiliser des structures à
la place.

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
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```
-->

```rust
fn main() {
    let largeur1 = 30;
    let hauteur1 = 50;

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        aire(largeur1, hauteur1)
    );
}

fn aire(largeur: u32, hauteur: u32) -> u32 {
    largeur * hauteur
}
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
```text
The area of the rectangle is 1500 square pixels.
```
-->

```text
L'aire du rectangle est de 1500 pixels carrés.
```

<!--
Even though Listing 5-8 works and figures out the area of the rectangle by
calling the `area` function with each dimension, we can do better. The width
and the height are related to each other because together they describe one
rectangle.
-->

Bien que l'encart 5-8 fonctionne et détermine l'aire du rectangle en appelant
la fonction `aire` avec chaque dimension, on peut faire mieux. La largeur et la
hauteur sont couplées entre elles car elles décrivent toutes les deux un rectangle.

<!--
The issue with this code is evident in the signature of `area`:
-->

Le problème de ce code se voit dans la signature de `aire` :

<!--
```rust,ignore
fn area(width: u32, height: u32) -> u32 {
```
-->

```rust,ignore
fn aire(largeur: u32, hauteur: u32) -> u32 {
```

<!--
The `area` function is supposed to calculate the area of one rectangle, but the
function we wrote has two parameters. The parameters are related, but that’s
not expressed anywhere in our program. It would be more readable and more
manageable to group width and height together. We’ve already discussed one way
we might do that in [“The Tuple Type”][the-tuple-type]<!−− ignore −− > section
of Chapter 3: by using tuples.
-->

La fonction `aire` est censée calculer l'aire d'un rectangle, mais la fonction
que nous avons écrite a deux paramètres. Les paramètres sont liés, mais ce n'est
exprimé nulle part dans notre programme. Il serait plus lisible et plus gérable
de regrouper ensemble la largeur et la hauteur. Nous avons déjà vu dans la
section [“Le type *tuple*”][the-tuple-type]<!-- ignore --> du chapitre 3 une
façon qui nous permettrait de le faire : en utilisant des tuples.

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
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```
-->

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        aire(rect1)
    );
}

fn aire(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

<!--
<span class="caption">Listing 5-9: Specifying the width and height of the
rectangle with a tuple</span>
-->

<span class="caption">Encart 5-9 : Renseigner la largeur et la hauteur du
rectangle dans un tuple</span>

<!--
In one way, this program is better. Tuples let us add a bit of structure, and
we’re now passing just one argument. But in another way, this version is less
clear: tuples don’t name their elements, so our calculation has become more
confusing because we have to index into the parts of the tuple.
-->

D'une certaine façon, ce programme est meilleur. Les tuples nous permettent de
structurer un peu plus et nous ne passons plus qu'un argument. Mais d'une autre
façon, cette version est moins claire : les tuples ne donnent pas de noms à
leurs éléments, donc notre calcul est devenu plus déroutant puisqu'il faut
accéder aux éléments du tuple via leur indice.

<!--
It doesn’t matter if we mix up width and height for the area calculation, but
if we want to draw the rectangle on the screen, it would matter! We would have
to keep in mind that `width` is the tuple index `0` and `height` is the tuple
index `1`. If someone else worked on this code, they would have to figure this
out and keep it in mind as well. It would be easy to forget or mix up these
values and cause errors, because we haven’t conveyed the meaning of our data in
our code.
-->

Ce n'est pas grave de confondre la largeur et la hauteur pour calculer l'aire,
mais si on voulait afficher le rectangle à l'écran, cela serait problématique !
Il nous faut garder à l'esprit que la `largeur` est l'élément à l'indice 0 du
tuple et que la `hauteur` est l'élément à l'indice 1. Si quelqu'un d'autre
travaillait sur ce code, il devrait le déduire et s'en souvenir aussi. Il est
facile d'oublier ou de confondre ces valeurs et par conséquent provoquer des
erreurs, parce qu'on n'a pas exprimé la signification de nos données dans notre
code.

<!--
### Refactoring with Structs: Adding More Meaning
-->

### Remanier avec des structures : donner plus de sens

<!--
We use structs to add meaning by labeling the data. We can transform the tuple
we’re using into a data type with a name for the whole as well as names for the
parts, as shown in Listing 5-10.
-->

On utilise des structures pour rendre les données plus expressives en leur
donnant des noms. On peut transformer le tuple que nous avons utilisé en un type
de donnée nommé dont ses éléments sont aussi nommés, comme le montre l'encart
5-10.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```
-->

```rust
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

fn main() {
    let rect1 = Rectangle { largeur: 30, hauteur: 50 };

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        aire(&rect1)
    );
}

fn aire(rectangle: &Rectangle) -> u32 {
    rectangle.largeur * rectangle.hauteur
}
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
It’d be nice to be able to print an instance of `Rectangle` while we’re
debugging our program and see the values for all its fields. Listing 5-11 tries
using the `println!` macro as we have used in previous chapters. This won’t
work, however.
-->

Cela serait bien de pouvoir afficher une instance de `Rectangle` pendant qu'on
débogue notre programme et de voir la valeur de chacun de ses champs. L'encart
5-11 essaye de le faire en utilisant la macro `println!` comme on l'a fait
dans les chapitres précédents. Cependant, cela ne fonctionne pas.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {}", rect1);
}
```
-->

```rust,ignore,does_not_compile
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

fn main() {
    let rect1 = Rectangle { largeur: 30, hauteur: 50 };

    println!("rect1 est {}", rect1);
}
```

<!--
<span class="caption">Listing 5-11: Attempting to print a `Rectangle`
instance</span>
-->

<span class="caption">Encart 5-11 : Tentative d'afficher une instance de
`Rectangle`</span>

<!--
When we run this code, we get an error with this core message:
-->

Lorsqu'on exécute ce code, on obtient ce message d'erreur qui nous informe que
`Rectangle` n'implémente pas le trait `std::fmt::Display` :

```text
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
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
implementation of `Display`.
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
`Display` par défaut.

<!--
If we continue reading the errors, we’ll find this helpful note:
-->

Si nous continuons de lire les erreurs, nous trouvons cette remarque utile :

```text
= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
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
Run the code with this change. Drat! We still get an error:
-->

Exécutez le code avec ce changement. Zut ! On a encore une erreur, nous
informant cette fois-ci que `Rectangle` n'implémente pas `std::fmt::Debug` :

```text
error[E0277]: `Rectangle` doesn't implement `std::fmt::Debug`
```

<!--
But again, the compiler gives us a helpful note:
-->

Mais une nouvelle fois, le compilateur nous fait une remarque utile :

```text
= help: the trait `std::fmt::Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
```

Il nous conseille d'ajouter `#[derive(Debug)]` ou d'implémenter manuellement
`std::fmt::Debug`.

<!--
Rust *does* include functionality to print out debugging information, but we
have to explicitly opt in to make that functionality available for our struct.
To do that, we add the annotation `#[derive(Debug)]` just before the struct
definition, as shown in Listing 5-12.
-->

Rust *inclut* bel et bien une fonctionnalité pour afficher des informations de
débogage, mais nous devons l'activer explicitement pour la rendre disponible sur
notre structure. Pour ce faire, on ajoute l'annotation `#[derive(Debug)]` juste
avant la définition de la structure, comme le montre l'encart 5-12.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
```
-->

```rust
#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

fn main() {
    let rect1 = Rectangle { largeur: 30, hauteur: 50 };

    println!("rect1 est {:?}", rect1);
}
```

<!--
<span class="caption">Listing 5-12: Adding the annotation to derive the `Debug`
trait and printing the `Rectangle` instance using debug formatting</span>
-->

<span class="caption">Encart 5-12 : L'ajout de l'annotation pour dériver le
trait `Debug` et afficher l'instance de `Rectangle` en utilisant le formatage
de débogage</span>

<!--
Now when we run the program, we won’t get any errors, and we’ll see the
following output:
-->

Maintenant, quand on exécute le programme, nous n'avons plus d'erreurs et ce
texte s'affiche à l'écran :

<!--
```text
rect1 is Rectangle { width: 30, height: 50 }
```
-->

```text
rect1 est Rectangle { largeur: 30, hauteur: 50 }
```

<!--
Nice! It’s not the prettiest output, but it shows the values of all the fields
for this instance, which would definitely help during debugging. When we have
larger structs, it’s useful to have output that’s a bit easier to read; in
those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string.
When we use the `{:#?}` style in the example, the output will look like this:
-->

Super ! Ce n'est pas le plus beau des affichages, mais cela montre les
valeurs de tous les champs de cette instance, ce qui serait assurément utile
lors du débogage. Quand on a des structures plus grandes, il serait bien d'avoir
un affichage un peu plus lisible ; dans ces cas-là, on pourra utiliser `{:#?}`
au lieu de `{:?}` dans la chaîne de formatage. Quand on utilise `{:#?}` dans cet
exemple, l'affichage donnera plutôt ceci :

<!--
```text
rect1 is Rectangle {
    width: 30,
    height: 50
}
```
-->

```text
rect1 est Rectangle {
    largeur: 30,
    hauteur: 50,
}
```

<!--
Rust has provided a number of traits for us to use with the `derive` annotation
that can add useful behavior to our custom types. Those traits and their
behaviors are listed in Appendix C. We’ll cover how to implement these traits
with custom behavior as well as how to create your own traits in Chapter 10.
-->

Rust nous fournit un certain nombre de traits qu'on peut utiliser avec
l'annotation `derive` qui peuvent ajouter des comportements utiles à nos propres
types. Ces traits et leurs comportements sont listés à l'annexe C. Nous
expliquerons comment implémenter ces traits avec des comportements personnalisés
et comment créer vos propres traits au chapitre 10.

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

<!-- [the-tuple-type]: ch03-02-data-types.html#the-tuple-type -->

[the-tuple-type]: ch03-02-data-types.html#le-type-tuple
