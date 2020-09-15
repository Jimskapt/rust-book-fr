<!--
## Advanced Types
-->

## Les types avancés

<!--
The Rust type system has some features that we’ve mentioned in this book but
haven’t yet discussed. We’ll start by discussing newtypes in general as we
examine why newtypes are useful as types. Then we’ll move on to type aliases, a
feature similar to newtypes but with slightly different semantics. We’ll also
discuss the `!` type and dynamically sized types.
-->

Le système de type de Rust offre quelques fonctionnalités que nous avons
mentionné dans ce livre mais que nous n'avons pas encore étudié. Nous allons
commencer par voir les newtypes en général lorsque nous examinerons pourquoi
les newtypes sont des types utiles. Ensuite nous nous pencherons sur les alias
de type, une fonctionnalité qui ressemble aux newtypes mais avec quelques
différences sémantiques. Nous allons aussi voir le type `!` et les types à
taille dynamique.

<!--
> Note: The next section assumes you’ve read the earlier section [“Using the
> Newtype Pattern to Implement External Traits on External
> Types.”][using-the-newtype-pattern]<!-- ignore -- >
-->

> Remarque : cette section suppose que vous avez lu la
> [section précédente][using-the-newtype-pattern]<!-- ignore -->

<!--
### Using the Newtype Pattern for Type Safety and Abstraction
-->

### Utiliser le motif newtype pour la sécurité et l'abstraction des types

<!--
The newtype pattern is useful for tasks beyond those we’ve discussed so far,
including statically enforcing that values are never confused and indicating
the units of a value. You saw an example of using newtypes to indicate units in
Listing 19-15: recall that the `Millimeters` and `Meters` structs wrapped `u32`
values in a newtype. If we wrote a function with a parameter of type
`Millimeters`, we couldn’t compile a program that accidentally tried to call
that function with a value of type `Meters` or a plain `u32`.
-->

Le motif newtype est utile pour des tâches qui se prolongent en dehors de ce
que nous avons vu jusqu'à présent, notamment pour faire en sorte statiquement
que les valeurs ne soient jamais confondues et pour indiquer les unités d'une
valeur. Vous avez vu un exemple d'utilisation des newtypes pour indiquer des
unités dans l'encart 19-15 : souvenez-vous des structures `Milimetres` et
`Metres` qui englobaient des valeurs `u32` dans ces newtypes. Si nous avions
écrit une fonction avec un paramètre de type `Milimetres`, nous ne pourrions
pas compiler un programme qui fait accidentellement appel à cette fonction avec
une valeur du type `Metres` ou un `u32` pur.

<!--
Another use of the newtype pattern is in abstracting away some implementation
details of a type: the new type can expose a public API that is different from
the API of the private inner type if we used the new type directly to restrict
the available functionality, for example.
-->

Une autre utilisation du motif newtype est de rendre abstrait certains détails
d'implémentation d'un type : le newtype peut exposer une API publique qui est
différente de l'API du type interne privé si nous avons utilisé directement le
newtype pour restreindre les fonctionnalités disponibles, par exemple.

<!--
Newtypes can also hide internal implementation. For example, we could provide a
`People` type to wrap a `HashMap<i32, String>` that stores a person’s ID
associated with their name. Code using `People` would only interact with the
public API we provide, such as a method to add a name string to the `People`
collection; that code wouldn’t need to know that we assign an `i32` ID to names
internally. The newtype pattern is a lightweight way to achieve encapsulation
to hide implementation details, which we discussed in the [“Encapsulation that
Hides Implementation
Details”][encapsulation-that-hides-implementation-details]<!-- ignore -- >
section of Chapter 17.
-->

Les newtypes peuvent aussi masquer des implémentations internes. Par exemple,
nous pouvons fournir un type `Personnes` pour embarquer un
`HashMap<i32, String>` qui stocke un identifiant d'une personne associé à son
nom. Le code qui utilisera `Personnes` ne pourra utiliser que l'API publique
que nous fournissons, comme une méthode pour ajouter une chaîne de caractères
de caractère qui est un nom à la collection `Personnes` ; ce code n'aura pas
besoin de savoir que nous assignons en interne un identifiant `i32` aux noms.
Le motif newtype est une façon allégée de procéder à de l'encapsulation pour
masquer des détails d'implémentation, comme nous l'avons vu dans [une partie du
chapitre 17][encapsulation-that-hides-implementation-details]<!-- ignore -->.

<!--
### Creating Type Synonyms with Type Aliases
-->

### Créer des synonymes de noms avec les alias de type

<!--
Along with the newtype pattern, Rust provides the ability to declare a *type
alias* to give an existing type another name. For this we use the `type`
keyword. For example, we can create the alias `Kilometers` to `i32` like so:
-->

En plus du motif newtype, Rust fournit la possibilité de déclarer un *alias de
type* pour donner un autre nom à un type déjà existant. Pour faire cela, nous
utilisons le mot-clé `type`. Par exemple, nous pouvons créer l'alias
`Kilometres` pour un `i32`, comme ceci :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

<!--
Now, the alias `Kilometers` is a *synonym* for `i32`; unlike the `Millimeters`
and `Meters` types we created in Listing 19-15, `Kilometers` is not a separate,
new type. Values that have the type `Kilometers` will be treated the same as
values of type `i32`:
-->

Désormais, l'alias `Kilometres` est un *synonyme* de `i32` ; contrairement aux
types `Milimetres` et `Metres` que nous avons créé dans l'encart 19-15,
`Kilometres` n'est pas un newtype séparé. Les valeurs qui ont le type
`Kilometre` seront traités comme si elles étaient du type `i32` :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

```rust
type Kilometres = i32;

let x: i32 = 5;
let y: Kilometres = 5;

println!("x + y = {}", x + y);
```

<!--
Because `Kilometers` and `i32` are the same type, we can add values of both
types and we can pass `Kilometers` values to functions that take `i32`
parameters. However, using this method, we don’t get the type checking benefits
that we get from the newtype pattern discussed earlier.
-->

Comme `Kilometres` et `i32` sont du même type, nous pouvons additionner les
valeurs des deux types et nous pouvons envoyer des valeurs `Kilometres` aux
fonctions qui prennent des paramètres `i32`. Cependant, en utilisant cette
méthode, nous ne bénéficions pas des bienfaits de la vérification du type que
nous avions avec le motif newtype que nous avons vu précédemment.

<!--
The main use case for type synonyms is to reduce repetition. For example, we
might have a lengthy type like this:
-->

L'utilisation principale pour les synonymes de types est de réduire la
répétition. Par exemple, nous pourrions avoir un type un peu long comme
celui-ci :

<!--
```rust,ignore
Box<dyn Fn() + Send + 'static>
```
-->

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

<!--
Writing this lengthy type in function signatures and as type annotations all
over the code can be tiresome and error prone. Imagine having a project full of
code like that in Listing 19-24.
-->

L'écriture de ce type un peu long dans des signatures de fonctions et comme
annotations de types tout au long du code peut être fatigante et faciliter les
erreurs. Imaginez avoir un projet avec plein de code comme celui dans l'encart
19-24.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-24/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-24/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-24: Using a long type in many places</span>
-->

<span class="caption">Encart 19-24 : utilisation d'un long type dans de nombreux
endroits</span>

<!--
A type alias makes this code more manageable by reducing the repetition. In
Listing 19-25, we’ve introduced an alias named `Thunk` for the verbose type and
can replace all uses of the type with the shorter alias `Thunk`.
-->

Un alias de type simplifie ce code en réduisant la répétition. Dans l'encart
19-25, nous avons ajouté un alias `Thunk` pour ce type verbeux et qui peut
remplacer tous ses cas d'emploi du type avec l'alias `Thunk`, plus court.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-25/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-25/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-25: Introducing a type alias `Thunk` to reduce
repetition</span>
-->

<span class="caption">Encart 19-25 : ajout et utilisation d'un alias `Thunk`
pour réduire les répétitions</span>

<!--
This code is much easier to read and write! Choosing a meaningful name for a
type alias can help communicate your intent as well (*thunk* is a word for code
to be evaluated at a later time, so it’s an appropriate name for a closure that
gets stored).
-->

Ce code est plus facile à lire et écrire ! Choisir un nom plus explicite pour
un alias peut aussi vous aider à communiquer ce que vous voulez faire (*thunk*
est un terme désignant du code qui peut être évalué plus tard, donc c'est un nom
approprié pour une fermeture qui est stockée).

<!--
Type aliases are also commonly used with the `Result<T, E>` type for reducing
repetition. Consider the `std::io` module in the standard library. I/O
operations often return a `Result<T, E>` to handle situations when operations
fail to work. This library has a `std::io::Error` struct that represents all
possible I/O errors. Many of the functions in `std::io` will be returning
`Result<T, E>` where the `E` is `std::io::Error`, such as these functions in
the `Write` trait:
-->

Les alias de type sont couramment utilisés avec le type `Result<T, E>` pour
réduire la répétition. Regardez le module `std::io` de la bibliothèque standard.
Les opérations d'entrée/sortie retournent parfois un `Result<T, E>` pour gérer
les situations lorsque les opérations échouent. Cette bibliothèque a une
structure `std::io::Error` qui représente toutes les erreurs possibles
d'entrée/sortie. De nombreuses fonctions dans `std::io` vont retourner un
`Result<T, E>` avec `E` qui est `std::io::Error`, ces fonctions sont dans le
trait `Write` :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

<!--
The `Result<..., Error>` is repeated a lot. As such, `std::io` has this type of
alias declaration:
-->

Le `Result<..., Error>` est répété plein de fois. Ainsi, `std::io` a ce type de
déclaration d'alias :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

<!--
Because this declaration is in the `std::io` module, we can use the fully
qualified alias `std::io::Result<T>`—that is, a `Result<T, E>` with the `E`
filled in as `std::io::Error`. The `Write` trait function signatures end up
looking like this:
-->

Comme cette déclaration est dans le module `std::io`, nous pouvons utiliser
l'alias `std::io::Result<T>` — qui est un `Result<T, E>` avec le `E` qui est
déjà renseigné comme étant un `std::io::Error`. Les fonctions du trait `Write`
ressemblent finalement à ceci :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

<!--
The type alias helps in two ways: it makes code easier to write *and* it gives
us a consistent interface across all of `std::io`. Because it’s an alias, it’s
just another `Result<T, E>`, which means we can use any methods that work on
`Result<T, E>` with it, as well as special syntax like the `?` operator.
-->

L'alias de type nous aide sur deux domaines : il permet de faciliter l'écriture
du code *et* il nous donne une interface uniforme pour tout `std::io`. Comme
c'est un alias, c'est simplement un autre `Result<T, E>`, ce qui signifie que
nous pouvons utiliser n'importe quelle méthode qui fonctionne avec
`Result<T, E>`, ainsi que les syntaxes spéciales comme l'opérateur `?`.

<!--
### The Never Type that Never Returns
-->

### Le type "jamais", qui ne retourna pas de valeur

<!--
Rust has a special type named `!` that’s known in type theory lingo as the
*empty type* because it has no values. We prefer to call it the *never type*
because it stands in the place of the return type when a function will never
return. Here is an example:
-->

Rust a un type spécial qui s'appelle `!` qui est connu dans le vocabulaire de
la théorie des types comme étant le *type vide* car il n'a pas de valeur. Nous
préférons appeler cela le *type jamais* car il remplace le type de retour
lorsqu'une fonction ne va jamais retourner quelque chose. Voici un exemple :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

```rust,ignore
fn bar() -> ! {
    // -- partie masquée ici --
}
```

<!--
This code is read as “the function `bar` returns never.” Functions that return
never are called *diverging functions*. We can’t create values of the type `!`
so `bar` can never possibly return.
-->

Ce code peut être interprété comme “la fonction `bar` qui ne retourne pas de
valeur”. Les fonctions qui ne retournent pas de valeur s'appellent des
*fonctions divergentes*. Nous ne pouvons pas créer de valeurs de type `!` donc
`bar` afin que `bar` ne puisse jamais retourner de valeur.

<!--
But what use is a type you can never create values for? Recall the code from
Listing 2-5; we’ve reproduced part of it here in Listing 19-26.
-->

Mais à quoi sert un type dont on ne peut jamais créer de valeurs ?
Souvenez-vous du code de l'encart 2-5 ; nous avons reproduit une partie de
celui-ci dans l'encart 19-26.

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

<!--
<span class="caption">Listing 19-26: A `match` with an arm that ends in
`continue`</span>
-->

<span class="caption">Encart 19-26 : un `match` avec une branche qui finit par
un `continue`</span>

<!--
At the time, we skipped over some details in this code. In Chapter 6 in [“The
`match` Control Flow Operator”][the-match-control-flow-operator]<!-- ignore
-- > section, we discussed that `match` arms must all return the same type. So,
for example, the following code doesn’t work:
-->

A l'époque, nous avions sauté quelques détails dans ce code. Dans la section
[“La structure de contrôle
`match`”][the-match-control-flow-operator]<!-- ignore --> du chapitre 6, nous
avons vu que les branches d'un `match` doivent toutes retourner le même type.
Donc, par exemple, le code suivant ne fonctionne pas :

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

```rust,ignore,does_not_compile
let supposition = match supposition.trim().parse() {
    Ok(_) => 5,
    Err(_) => "salut",
}
```

<!--
The type of `guess` in this code would have to be an integer *and* a string,
and Rust requires that `guess` have only one type. So what does `continue`
return? How were we allowed to return a `u32` from one arm and have another arm
that ends with `continue` in Listing 19-26?
-->

Le type de `supposition` dans ce code devrait être un entier *et* une chaîne de
caractères, et Rust nécessite que `supposition` n'ait qu'un seul type possible.
Donc que retourne `continue` ? Pourquoi pouvons-nous retourner un `u32` dans
une branche et avoir une autre branche qui finit avec un `continue` dans
l'encart 19-26 ?

<!--
As you might have guessed, `continue` has a `!` value. That is, when Rust
computes the type of `guess`, it looks at both match arms, the former with a
value of `u32` and the latter with a `!` value. Because `!` can never have a
value, Rust decides that the type of `guess` is `u32`.
-->

Comme vous l'avez deviné, `continue` a une valeur `!`. Ainsi, lorsque Rust
calcule le type de `supposition`, il regarde les deux branches, la première
avec une valeur `u32` et la seconde avec une valeur `!`. Comme `!` ne peut
jamais retourner de valeur, Rust décide alors que le type de `supposition` est
`u32`.

<!--
The formal way of describing this behavior is that expressions of type `!` can
be coerced into any other type. We’re allowed to end this `match` arm with
`continue` because `continue` doesn’t return a value; instead, it moves control
back to the top of the loop, so in the `Err` case, we never assign a value to
`guess`.
-->

Une façon classique de décrire ce comportement est de dire que les expressions
du type `!` peuvent être transformées dans n'importe quel type. Nous pouvons
finir cette branche de `match` avec `continue` car `continue` ne retourne pas
de valeur ; à la place, il retourne le contrôle en haut de la boucle, donc dans
le cas d'un `Err`, nous n'assignons jamais de valeur à `supposition`.

<!--
The never type is useful with the `panic!` macro as well. Remember the `unwrap`
function that we call on `Option<T>` values to produce a value or panic? Here
is its definition:
-->

Ce type "jamais" est aussi utile avec la macro `panic!`. Vous souvenez-vous que
la fonction `unwrap` que nous appelons sur les valeurs `Option<T>` fournissent
une valeur, ou paniquent ? Voici sa définition :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

<!--
In this code, the same thing happens as in the `match` in Listing 19-26: Rust
sees that `val` has the type `T` and `panic!` has the type `!`, so the result
of the overall `match` expression is `T`. This code works because `panic!`
doesn’t produce a value; it ends the program. In the `None` case, we won’t be
returning a value from `unwrap`, so this code is valid.
-->

Dans ce code, il se passe la même chose que l'encart 19-26 : Rust constate que
`val` est du type `T` et que `panic!` est du type `!`, donc le résultat de
l'ensemble de l'expression `match` est `T`. Ce code fonctionne car `panic!` ne
produit pas de valeur ; il termine le programme. Dans le cas d'un `None`, nous
ne retournons pas une valeur de `unwrap`, donc ce code est valide.

<!--
One final expression that has the type `!` is a `loop`:
-->

Une des expressions qui sont du type `!` est le `loop` :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

<!--
Here, the loop never ends, so `!` is the value of the expression. However, this
wouldn’t be true if we included a `break`, because the loop would terminate
when it got to the `break`.
-->

Ici, la boucle ne se termine jamais, donc `!` est la valeur de cette
expression. En revanche, cela ne sera pas vrai si nous utilisons un `break`,
car la boucle va s'arrêter lorsqu'elle rencontrera le `break`.

<!--
### Dynamically Sized Types and the `Sized` Trait
-->

### Les types à taille dynamique et le trait `Sized`

<!--
Due to Rust’s need to know certain details, such as how much space to allocate
for a value of a particular type, there is a corner of its type system that can
be confusing: the concept of *dynamically sized types*. Sometimes referred to
as *DSTs* or *unsized types*, these types let us write code using values whose
size we can know only at runtime.
-->

Vu qu'il est nécessaire pour Rust de connaître certains détails, comme la
quantité d'espace à allouer à une valeur d'un type donné, il y a un aspect de
ce système de type qui peut être déroutant : le concept des *types à taille
dynamique*. Parfois appelés *DST* (Dynamically Sized Types) ou *types sans
taille*, ces types nous permettent d'écrire du code qui utilisent des valeurs
qui ne peuvent être connues uniquement à l'exécution.

<!--
Let’s dig into the details of a dynamically sized type called `str`, which
we’ve been using throughout the book. That’s right, not `&str`, but `str` on
its own, is a DST. We can’t know how long the string is until runtime, meaning
we can’t create a variable of type `str`, nor can we take an argument of type
`str`. Consider the following code, which does not work:
-->

Voyons les détails d'un type à taille dynamique qui s'appelle `str`, que nous
avons utilisé dans ce livre. Plus précisément `&str`, car `str` en lui-même est
un DST. Nous ne connaître la longueur de la chaîne de caractère qu'à
l'exécution, ce qui signifie que nous ne pouvons pas ni créer une variable de
type `str`, ni prendre prendre en argument un type `str`. Imaginons le code
suivant, qui ne devrait pas fonctionner :

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

<!--
Rust needs to know how much memory to allocate for any value of a particular
type, and all values of a type must use the same amount of memory. If Rust
allowed us to write this code, these two `str` values would need to take up the
same amount of space. But they have different lengths: `s1` needs 12 bytes of
storage and `s2` needs 15. This is why it’s not possible to create a variable
holding a dynamically sized type.
-->

Rust a besoin de savoir combien de mémoire allouer pour chaque valeur d'un type
donné, et toutes les valeurs de ce type doivent utiliser la même quantité de
mémoire. Si Rust nous aurait autorisé à écrire ce code, ces deux valeurs `str`
devraient occuper la même quantité de mémoire. Mais elles ont deux longueurs
différentes : `s1` prend 21 octets en mémoire alors que `s2` en a besoin de 15.
C'est pourquoi il est impossible de créer une variable qui stocke un type à
taille dynamique.

<!--
So what do we do? In this case, you already know the answer: we make the types
of `s1` and `s2` a `&str` rather than a `str`. Recall that in the [“String
Slices”][string-slices]<!-- ignore -- > section of Chapter 4, we said the slice
data structure stores the starting position and the length of the slice.
-->

Donc qu'est-ce qu'on peut faire ? Dans ce cas, vous connaissez déjà la réponse :
nous faisons en sorte que le type de `s1` et `s2` soit `&str` plutôt que `str`.
Souvenez-vous que dans la section
[“Les slices de chaînes de caractères”][string-slices]<!-- ignore -->
du chapitre 4, nous avions dit que la structure de données slice stockait
l'emplacement de départ et la longueur de la slice.

<!--
So although a `&T` is a single value that stores the memory address of where
the `T` is located, a `&str` is *two* values: the address of the `str` and its
length. As such, we can know the size of a `&str` value at compile time: it’s
twice the length of a `usize`. That is, we always know the size of a `&str`, no
matter how long the string it refers to is. In general, this is the way in
which dynamically sized types are used in Rust: they have an extra bit of
metadata that stores the size of the dynamic information. The golden rule of
dynamically sized types is that we must always put values of dynamically sized
types behind a pointer of some kind.
-->

Aussi, bien qu'un `&T` soit une seule valeur qui stocke l'adresse mémoire d'où
se trouve le `T`, un `&str` représente *deux* valeurs : l'adresse du `str` et sa
longueur. Ainsi, nous pouvons connaître la taille d'une valeur `&str` à la
compilation : elle vaut deux fois la taille d'un `usize`. Ce faisant, nous
connaissons toujours la taille d'un `&str`, peu importe la longueur de la chaîne
de caractères sur laquelle cela pointe. Généralement, c'est comme cela que les
types à taille dynamique sont utilisés en Rust : ils ont des métadonnées
supplémentaires qui stockent la taille des informations dynamiques. La règle
d'or des types à taille dynamique est que nous devons toujours placer les
valeurs à types à taille dynamique dans une sorte de pointeur.

<!--
We can combine `str` with all kinds of pointers: for example, `Box<str>` or
`Rc<str>`. In fact, you’ve seen this before but with a different dynamically
sized type: traits. Every trait is a dynamically sized type we can refer to by
using the name of the trait. In Chapter 17 in the [“Using Trait Objects That
Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore -- > section, we mentioned that to use traits as trait objects, we must
put them behind a pointer, such as `&dyn Trait` or `Box<dyn Trait>` (`Rc<dyn
Trait>` would work too).
-->

Nous pouvons combiner `str` avec n'importe quel type de pointeur : par exemple,
`Box<str>` ou `Rc<str>`. En fait, vous avez vu cela déjà auparavant mais avec un
type à taille dynamique : les traits. Chaque trait est un type à taille
dynamique auquel nous pouvons nous référer en utilisant le nom du trait. Dans
[une section][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore --> du chapitre 17, nous avions mentionné que pour utiliser les traits
comme des objets traits, nous devions les utiliser avec un pointeur, comme le
`&dyn Trait` ou `Box<dyn Trait>` (`Rc<dyn Trait>` devrait aussi fonctionner).

<!--
To work with DSTs, Rust has a particular trait called the `Sized` trait to
determine whether or not a type’s size is known at compile time. This trait is
automatically implemented for everything whose size is known at compile time.
In addition, Rust implicitly adds a bound on `Sized` to every generic function.
That is, a generic function definition like this:
-->

Pour pouvoir travailler avec les DST, Rust a un trait particulier `Sized` pour
déterminer si oui ou non la taille d'un type est connue à la compilation. Ce
trait est automatiquement implémenté sur tout ce qui a une taille connue à la
compilation. De plus, Rust ajoute implicitement le trait lié `Sized` sur chaque
fonction générique. Ainsi, la définition d'une fonction générique comme
celle-ci :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

<!--
is actually treated as though we had written this:
-->

... est en réalité traitée comme si nous avions écris ceci :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

<!--
By default, generic functions will work only on types that have a known size at
compile time. However, you can use the following special syntax to relax this
restriction:
-->

Par défaut, les fonctions génériques vont fonctionner uniquement sur les types
qui ont une taille connue à la compilation. Cependant, vous pouvez utiliser la
syntaxe spéciale suivante pour éviter cette restriction :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

<!--
A trait bound on `?Sized` is the opposite of a trait bound on `Sized`: we would
read this as “`T` may or may not be `Sized`.” This syntax is only available for
`Sized`, not any other traits.
-->

Le trait lié `?Sized` est l'opposé du trait lié `Sized` : nous pourrions lire
ceci comme étant “`T` peut être ou non un `Sized`”. Cette syntaxe est disponible
uniquement pour `Sized`, et non pas pour les autres traits.

<!--
Also note that we switched the type of the `t` parameter from `T` to `&T`.
Because the type might not be `Sized`, we need to use it behind some kind of
pointer. In this case, we’ve chosen a reference.
-->

Remarquez aussi que nous avons changé le type du paramètre `t` de `T` en `&T`.
Comme ce type pourrait ne pas être un `Sized`, nous devons l'utiliser avec
quelque chose qui sert de pointeur. Dans ce cas, nous avons choisi une
référence.

<!--
Next, we’ll talk about functions and closures!
-->

Dans la partie suivante, nous allons parler des fonctions et des fermetures !

<!--
[encapsulation-that-hides-implementation-details]:
ch17-01-what-is-oo.html#encapsulation-that-hides-implementation-details
[string-slices]: ch04-03-slices.html#string-slices
[the-match-control-flow-operator]:
ch06-02-match.html#the-match-control-flow-operator
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[using-the-newtype-pattern]: ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
-->

[encapsulation-that-hides-implementation-details]: ch17-01-what-is-oo.html
[string-slices]: ch04-03-slices.html#les-slices-de-chaînes-de-caractères
[the-match-control-flow-operator]:
ch06-02-match.html#la-structure-de-contrôle-match
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html
[using-the-newtype-pattern]: ch19-03-advanced-traits.html
