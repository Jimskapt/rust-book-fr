<!--
## Storing Lists of Values with Vectors
-->

## Stocker des listes de valeurs avec des vecteurs

<!--
The first collection type we’ll look at is `Vec<T>`, also known as a *vector*.
Vectors allow you to store more than one value in a single data structure that
puts all the values next to each other in memory. Vectors can only store values
of the same type. They are useful when you have a list of items, such as the
lines of text in a file or the prices of items in a shopping cart.
-->

Le premier type de collection que nous allons voir est `Vec<T>`, aussi appelé
*vecteur*. Les vecteurs vous permettent de stocker plus d'une valeur dans une
seule structure de données qui stocke les valeurs les unes à côté des autres
dans la mémoire. Les vecteurs peuvent stocker uniquement des valeurs du même
type. Ils sont utiles lorsque vous avez une liste d'éléments, tels que les
lignes de texte provenant d'un fichier ou les prix des articles d'un panier
d'achat.

<!--
### Creating a New Vector
-->

### Créer un nouveau vecteur

<!--
To create a new empty vector, we call the `Vec::new` function, as shown in
Listing 8-1.
-->

Pour créer un nouveau vecteur vide, nous appelons la fonction `Vec::new`, comme
dans l'encart 8-1.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-01/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-1: Creating a new, empty vector to hold values
of type `i32`</span>
-->

<span class="caption">Encart 8-1 : création d'un nouveau vecteur vide pour y
stocker des valeurs de type `i32`</span>

<!--
Note that we added a type annotation here. Because we aren’t inserting any
values into this vector, Rust doesn’t know what kind of elements we intend to
store. This is an important point. Vectors are implemented using generics;
we’ll cover how to use generics with your own types in Chapter 10. For now,
know that the `Vec<T>` type provided by the standard library can hold any type.
When we create a vector to hold a specific type, we can specify the type within
angle brackets. In Listing 8-1, we’ve told Rust that the `Vec<T>` in `v` will
hold elements of the `i32` type.
-->

Remarquez que nous avons ajouté ici une annotation de type. Comme nous
n'ajoutons pas de valeurs dans ce vecteur, Rust ne sait pas quel type d'éléments
nous souhaitons stocker. C'est une information importante. Les vecteurs sont
implémentés avec la généricité ; nous verrons comment utiliser la généricité sur
vos propres types au chapitre 10. Pour l'instant, sachez que le type `Vec<T>`
qui est fourni par la bibliothèque standard peut stocker n'importe quel type.
Lorsque nous créons un vecteur pour stocker un type précis, nous pouvons
renseigner ce type entre des chevrons. Dans l'encart 8-1, nous précisons à Rust
que le `Vec<T>` dans `v` va stocker des éléments de type `i32`.

<!--
More often, you’ll create a `Vec<T>` with initial values and Rust will infer
the type of value you want to store, so you rarely need to do this type
annotation. Rust conveniently provides the `vec!` macro, which will create a
new vector that holds the values you give it. Listing 8-2 creates a new
`Vec<i32>` that holds the values `1`, `2`, and `3`. The integer type is `i32`
because that’s the default integer type, as we discussed in the [“Data
Types”][data-types]<!-- ignore -- > section of Chapter 3.
-->

Le plus souvent, vous allez créer un `Vec<T>` avec des valeurs initiales et
Rust va deviner le type de la valeur que vous souhaitez stocker, donc vous
n'aurez pas souvent besoin de faire cette annotation de type. Rust propose la
macro très pratique `vec!`, qui va créer un nouveau vecteur qui stockera les
valeurs que vous lui donnerez. L'encart 8-2 crée un nouveau `Vec<i32>` qui
stocke les valeurs `1`, `2` et `3`. Le type d'entier est `i32` car c'est le
type d'entier par défaut, comme nous l'avons évoqué dans la section [“Les types
de données”][data-types]<!-- ignore --> du chapitre 3.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-02/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-2: Creating a new vector containing
values</span>
-->

<span class="caption">Encart 8-2 : création d'un nouveau vecteur qui contient
des valeurs</span>

<!--
Because we’ve given initial `i32` values, Rust can infer that the type of `v`
is `Vec<i32>`, and the type annotation isn’t necessary. Next, we’ll look at how
to modify a vector.
-->

Comme nous avons donné des valeurs initiales `i32`, Rust peut en déduire que le
type de `v` est `Vec<i32>`, et l'annotation de type n'est plus nécessaire.
Maintenant, nous allons voir comment modifier un vecteur.

<!--
### Updating a Vector
-->

### Modifier un vecteur

<!--
To create a vector and then add elements to it, we can use the `push` method,
as shown in Listing 8-3.
-->

Pour créer un vecteur et ensuite lui ajouter des éléments, nous pouvons utiliser
la méthode `push`, comme dans l'encart 8-3.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-03/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-3: Using the `push` method to add values to a
vector</span>
-->

<span class="caption">Encart 8-3 : utilisation de la méthode `push` pour ajouter
des valeurs à un vecteur</span>

<!--
As with any variable, if we want to be able to change its value, we need to
make it mutable using the `mut` keyword, as discussed in Chapter 3. The numbers
we place inside are all of type `i32`, and Rust infers this from the data, so
we don’t need the `Vec<i32>` annotation.
-->

Comme pour toute variable, si nous voulons pouvoir modifier sa valeur, nous
devons la rendre mutable en utilisant le mot-clé `mut`, comme nous l'avons vu
au chapitre 3. Les nombres que nous ajoutons dedans sont tous du type `i32`, et
Rust le devine à partir des données, donc nous n'avons pas besoin de
l'annotation `Vec<i32>`.

<!--
### Dropping a Vector Drops Its Elements
-->

### Libérer un vecteur libère aussi ses éléments

<!--
Like any other `struct`, a vector is freed when it goes out of scope, as
annotated in Listing 8-4.
-->

Comme toutes les autres structures, un vecteur est libéré quand il sort de la
portée, comme précisé dans l'encart 8-4.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-04/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-4: Showing where the vector and its elements
are dropped</span>
-->

<span class="caption">Encart 8-4 : mise en évidence de là où le vecteur et ses
éléments sont libérés</span>

<!--
When the vector gets dropped, all of its contents are also dropped, meaning
those integers it holds will be cleaned up. This may seem like a
straightforward point but it can get complicated when you start to introduce
references to the elements of the vector. Let’s tackle that next!
-->

Lorsque le vecteur est libéré, tout son contenu est aussi libéré, ce qui veut
dire que les nombres entiers qu'il stocke vont être effacés de la mémoire. Cela
semble très simple mais cela peut devenir plus compliqué quand vous commencez à
utiliser des références vers les éléments du vecteur. Voyons ceci dès à
présent !

<!--
### Reading Elements of Vectors
-->

### Lire les éléments des vecteurs

<!--
There are two ways to reference a value stored in a vector: via indexing or
using the `get` method. In the following examples, we’ve annotated the types of
the values that are returned from these functions for extra clarity.
-->

Il existe deux façons de désigner une valeur enregistrée dans un vecteur : via
les indices ou en utilisant la méthode `get`. Dans les exemples suivants, nous
avons précisé les types des valeurs qui sont retournées par ces fonctions pour
plus de clarté.

<!--
Listing 8-5 shows both methods of accessing a value in a vector, with indexing
syntax and the `get` method.
-->

L'encart 8-5 nous montre les deux façons d'accéder à une valeur d'un vecteur,
via la syntaxe d'indexation et avec la méthode `get`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-05/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-5: Using indexing syntax or the `get` method to
access an item in a vector</span>
-->

<span class="caption">Encart 8-5 : utilisation de la syntaxe d'indexation ainsi
que la méthode `get` pour accéder à un élément d'un vecteur</span>

<!--
Note two details here. First, we use the index value of `2` to get the third
element because vectors are indexed by number, starting at zero. Second, we get
the third element by either using `&` and `[]`, which gives us a reference, or
using the `get` method with the index passed as an argument, which gives us an
`Option<&T>`.
-->

Il y a deux détails à remarquer ici. Premièrement, nous avons utilisé l'indice
`2` pour obtenir le troisième élément car les vecteurs sont indexés par des
nombres, qui commencent à partir de zéro. Deuxièmement, nous obtenons le
troisième élément soit en utilisant `&` et `[]`, ce qui nous donne une
référence, soit en utilisant la méthode `get` avec l'indice en argument, ce qui
nous fournit une `Option<&T>`.

<!--
The reason Rust provides these two ways to reference an element is so you can
choose how the program behaves when you try to use an index value outside the
range of existing elements. As an example, let’s see what happens when we have
a vector of five elements and then we try to access an element at index 100
with each technique, as shown in Listing 8-6.
-->

La raison pour laquelle Rust offre ces deux manières d'obtenir une référence
vers un élément est de vous permettre de choisir le comportement du programme
lorsque vous essayez d'utiliser une valeur dont l'indice est à l'extérieur de
la plage des éléments existants. Par exemple, voyons dans l'encart 8-6 ce qui
se passe lorsque nous avons un vecteur de cinq éléments et qu'ensuite nous
essayons d'accéder à un élément à l'indice 100 avec chaque technique.

<!--
```rust,should_panic,panics
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-06/src/main.rs:here}}
```
-->

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-6: Attempting to access the element at index
100 in a vector containing five elements</span>
-->

<span class="caption">Encart 8-6 : tentative d'accès à l'élément à l'indice 100
dans un vecteur qui contient cinq éléments</span>

<!--
When we run this code, the first `[]` method will cause the program to panic
because it references a nonexistent element. This method is best used when you
want your program to crash if there’s an attempt to access an element past the
end of the vector.
-->

Lorsque nous exécutons ce code, la première méthode `[]` va faire paniquer le
programme car il demande un élément non existant. Cette méthode doit être
favorisée lorsque vous souhaitez que votre programme plante s'il y a une
tentative d'accéder à un élément après la fin du vecteur.

<!--
When the `get` method is passed an index that is outside the vector, it returns
`None` without panicking. You would use this method if accessing an element
beyond the range of the vector may happen occasionally under normal
circumstances. Your code will then have logic to handle having either
`Some(&element)` or `None`, as discussed in Chapter 6. For example, the index
could be coming from a person entering a number. If they accidentally enter a
number that’s too large and the program gets a `None` value, you could tell the
user how many items are in the current vector and give them another chance to
enter a valid value. That would be more user-friendly than crashing the program
due to a typo!
-->

Lorsque nous passons un indice en dehors de l'intervalle du vecteur à la
méthode `get`, elle retourne `None` sans paniquer. Vous devriez utiliser cette
méthode s'il peut arriver occasionnellement de vouloir accéder à un élément en
dehors de l'intervalle du vecteur en temps normal. Votre code va ensuite devoir
gérer les deux valeurs `Some(&element)` ou `None`, comme nous l'avons vu au
chapitre 6. Par exemple, l'indice peut provenir d'une saisie utilisateur. Si
par accident il saisit un nombre qui est trop grand et que le programme obtient
une valeur `None`, vous pouvez alors dire à l'utilisateur combien il y a
d'éléments dans le vecteur courant et lui donner une nouvelle chance de saisir
une valeur valide. Cela sera plus convivial que de faire planter le programme à
cause d'une faute de frappe !

<!--
When the program has a valid reference, the borrow checker enforces the
ownership and borrowing rules (covered in Chapter 4) to ensure this reference
and any other references to the contents of the vector remain valid. Recall the
rule that states you can’t have mutable and immutable references in the same
scope. That rule applies in Listing 8-7, where we hold an immutable reference
to the first element in a vector and try to add an element to the end. This
program won’t work if we also try to refer to that element later in the
function:
-->

Lorsque le programme obtient une référence valide, le vérificateur d'emprunt va
faire appliquer les règles de possession et d'emprunt (que nous avons vues au
chapitre 4) pour s'assurer que cette référence ainsi que toutes les autres
références au contenu de ce vecteur restent valides. Souvenez-vous de la règle
qui dit que vous ne pouvez pas avoir des références mutables et immuables dans
la même portée. Cette règle s'applique à l'encart 8-7, où nous obtenons une
référence immuable vers le premier élément d'un vecteur et nous essayons
d'ajouter un élément à la fin. Ce programme ne fonctionnera pas si nous
essayons aussi d'utiliser cet élément plus tard dans la fonction :

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-07/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-7: Attempting to add an element to a vector
while holding a reference to an item</span>
-->

<span class="caption">Encart 8-7 : tentative d'ajout d'un élément à un vecteur
alors que nous utilisons une référence à un élément</span>

<!--
Compiling this code will result in this error:
-->

Compiler ce code va nous mener à cette erreur :

<!--
```console
{{#include ../listings-sources/ch08-common-collections/listing-08-07/output.txt}}
```
-->

```console
{{#include ../listings/ch08-common-collections/listing-08-07/output.txt}}
```

<!--
The code in Listing 8-7 might look like it should work: why should a reference
to the first element care about changes at the end of the vector? This error is
due to the way vectors work: because vectors put the values next to each other
in memory, adding a new element onto the end of the vector might require
allocating new memory and copying the old elements to the new space, if there
isn’t enough room to put all the elements next to each other where the vector
is currently stored. In that case, the reference to the first element would be
pointing to deallocated memory. The borrowing rules prevent programs from
ending up in that situation.
-->

Le code dans l'encart 8-7 semble pourtant marcher : pourquoi une référence au
premier élément devrait se soucier de ce qui se passe à la fin du vecteur ?
Cette erreur s'explique par la façon dont les vecteurs fonctionnent : comme les
vecteurs ajoutent les valeurs les unes à côté des autres dans la mémoire,
l'ajout d'un nouvel élément à la fin du vecteur peut nécessiter d'allouer un
nouvel espace mémoire et copier tous les anciens éléments dans ce nouvel
espace, s'il n'y a pas assez de place pour placer tous les éléments les uns à
côté des autres dans la mémoire là où est actuellement stocké le vecteur. Dans
ce cas, la référence au premier élément pointerait vers de la mémoire
désallouée. Les règles d'emprunt évitent aux programmes de se retrouver dans
cette situation.

<!--
> Note: For more on the implementation details of the `Vec<T>` type, see [“The
> Rustonomicon”][nomicon].
-->

> Remarque : pour plus de détails sur l'implémentation du type `Vec<T>`,
> consultez [le “Rustonomicon”][nomicon].

<!--
### Iterating over the Values in a Vector
-->

### Itérer sur les valeurs d'un vecteur

<!--
To access each element in a vector in turn, we would iterate through all of the
elements rather than use indices to access one at a time. Listing 8-8 shows how
to use a `for` loop to get immutable references to each element in a vector of
`i32` values and print them.
-->

Pour accéder à chaque élément d'un vecteur chacun son tour, nous devrions
itérer sur tous les éléments plutôt que d'utiliser individuellement les
indices. L'encart 8-8 nous montre comment utiliser une boucle `for` pour
obtenir des références immuables pour chacun des éléments dans un vecteur de
`i32`, et les afficher.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-08/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-8: Printing each element in a vector by
iterating over the elements using a `for` loop</span>
-->

<span class="caption">Encart 8-8 : affichage de chaque élément d'un vecteur en
itérant sur les éléments en utilisant une boucle `for`</span>

<!--
We can also iterate over mutable references to each element in a mutable vector
in order to make changes to all the elements. The `for` loop in Listing 8-9
will add `50` to each element.
-->

Nous pouvons aussi itérer avec des références mutables pour chacun des éléments
d'un vecteur mutable afin de modifier tous les éléments. La boucle `for` de
l'encart 8-9 va ajouter `50` à chacun des éléments.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-09/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-9: Iterating over mutable references to
elements in a vector</span>
-->

<span class="caption">Encart 8-9 : itérations sur des références mutables vers
des éléments d'un vecteur</span>

<!--
To change the value that the mutable reference refers to, we have to use the
`*` dereference operator to get to the value in `i` before we can use the
`+=` operator. We’ll talk more about the dereference operator in the
[“Following the Pointer to the Value with the Dereference Operator”][deref]<!-- ignore -- >
section of Chapter 15.
-->

Afin de changer la valeur vers laquelle pointe la référence mutable, nous devons
utiliser l'opérateur de déréférencement `*` pour obtenir la valeur dans `i`
avant que nous puissions utiliser l'opérateur `+=`. Nous verrons plus en détail
l'opérateur de déréférencement dans une section du
[chapitre 15][deref]<!-- ignore -->.

<!--
### Using an Enum to Store Multiple Types
-->

### Utiliser une énumération pour stocker différents types

<!--
Vectors can only store values that are the same type. This can be inconvenient;
there are definitely use cases for needing to store a list of items of
different types. Fortunately, the variants of an enum are defined under the
same enum type, so when we need one type to represent elements of different
types, we can define and use an enum!
-->

Les vecteurs ne peuvent stocker que des valeurs du même type. Cela peut être un
problème ; il y a forcément des cas où on a besoin de stocker une liste
d'éléments de types différents. Heureusement, les variantes d'une énumération
sont définies sous le même type d'énumération, donc lorsque nous avons besoin
d'un type pour représenter les éléments de types différents, nous pouvons
définir et utiliser une énumération !

<!--
For example, say we want to get values from a row in a spreadsheet in which
some of the columns in the row contain integers, some floating-point numbers,
and some strings. We can define an enum whose variants will hold the different
value types, and all the enum variants will be considered the same type: that
of the enum. Then we can create a vector to hold that enum and so, ultimately,
holds different types. We’ve demonstrated this in Listing 8-10.
-->

Par exemple, imaginons que nous voulions obtenir les valeurs d'une ligne d'une
feuille de calcul dans laquelle quelques colonnes sont des entiers, d'autres
des nombres à virgule flottante, et quelques chaînes de caractères. Nous
pouvons définir une énumération dont les variantes vont avoir les différents
types, et toutes les variantes de l'énumération seront du même type : celui de
l'énumération. Ensuite, nous pouvons créer un vecteur pour stocker cette
énumération et ainsi, au final, qui stocke différents types. La démonstration
de cette technique est dans l'encart 8-10.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-10/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-10: Defining an `enum` to store values of
different types in one vector</span>
-->

<span class="caption">Encart 8-10 : définition d'une `enum` pour stocker des
valeurs de différents types dans un seul vecteur</span>

<!--
Rust needs to know what types will be in the vector at compile time so it knows
exactly how much memory on the heap will be needed to store each element. We
must also be explicit about what types are allowed in this vector. If Rust
allowed a vector to hold any type, there would be a chance that one or more of
the types would cause errors with the operations performed on the elements of
the vector. Using an enum plus a `match` expression means that Rust will ensure
at compile time that every possible case is handled, as discussed in Chapter 6.
-->

Rust a besoin de savoir quel type de donnée sera stocké dans le vecteur au
moment de la compilation afin de connaître la quantité de mémoire nécessaire
pour stocker chaque élément sur le tas. Nous devons être précis sur les types
autorisés dans ce vecteur. Si Rust avait permis qu'un vecteur stocke n'importe
quel type, il y aurait pu avoir un risque qu'un ou plusieurs des types
provoquent une erreur avec les manipulations effectuées sur les éléments du
vecteur. L'utilisation d'une énumération ainsi qu'une expression `match` permet
à Rust de garantir au moment de la compilation que tous les cas possibles sont
traités, comme nous l'avons appris au chapitre 6.

<!--
If you don’t know the exhaustive set of types a program will get at runtime to
store in a vector, the enum technique won’t work. Instead, you can use a trait
object, which we’ll cover in Chapter 17.
-->

Si vous n'avez pas une liste exhaustive des types que votre programme va
stocker dans un vecteur, la technique de l'énumération ne va pas fonctionner. À
la place, vous pouvez utiliser un objet trait, que nous verrons au chapitre 17.

<!--
Now that we’ve discussed some of the most common ways to use vectors, be sure
to review [the API documentation][vec-api]<!-- ignore -- > for all the many
useful methods defined on `Vec<T>` by the standard library. For example, in
addition to `push`, a `pop` method removes and returns the last element. Let’s
move on to the next collection type: `String`!
-->

Maintenant que nous avons vu les manières les plus courantes d'utiliser les
vecteurs, prenez le temps de consulter [la documentation de
l'API][vec-api]<!-- ignore --> pour découvrir toutes les méthodes très utiles
définies dans la bibliothèque standard pour `Vec<T>`. Par exemple, en plus de
`push`, nous avons une méthode `pop` qui retire et retourne le dernier élément.
Intéressons-nous maintenant au prochain type de collection : la `String` !

<!--
[data-types]: ch03-02-data-types.html#data-types
[nomicon]: ../nomicon/vec/vec.html
[vec-api]: ../std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
-->

[data-types]: ch03-02-data-types.html
[nomicon]: https://doc.rust-lang.org/nomicon/vec/vec.html
[vec-api]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[deref]: ch15-02-deref.html
