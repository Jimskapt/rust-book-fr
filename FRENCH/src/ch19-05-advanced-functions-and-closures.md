<!--
## Advanced Functions and Closures
-->

## Les fonctions et fermetures avancées

<!--
Next, we’ll explore some advanced features related to functions and
closures, which include function pointers and returning closures.
-->

Maintenant, nous allons explorer quelques fonctionnalités avancées liées aux
fonctions et aux fermetures, comme les pointeurs de fonctions et retourner des
fermetures.

<!--
### Function Pointers
-->

### Pointeurs de fonctions

<!--
We’ve talked about how to pass closures to functions; you can also pass regular
functions to functions! This technique is useful when you want to pass a
function you’ve already defined rather than defining a new closure. Doing this
with function pointers will allow you to use functions as arguments to other
functions. Functions coerce to the type `fn` (with a lowercase f), not to be
confused with the `Fn` closure trait. The `fn` type is called a *function
pointer*. The syntax for specifying that a parameter is a function pointer is
similar to that of closures, as shown in Listing 19-27.
-->

Nous avons déjà vu comment envoyer des fermetures dans des fonctions ; mais vous
pouvez aussi envoyer des fonctions classiques dans d'autres fonctions ! Cette
technique est utile lorsque vous souhaitez envoyer une fonction que vous avez
déjà définie plutôt que de définir une nouvelle fermeture. Vous pouvez faire
ceci avec des pointeurs de fonctions, qui vous permettent d'utiliser des
fonctions en argument d'autres fonctions. Les fonctions nécessitent le type `fn`
(avec un f minuscule), à ne pas confondre avec le trait de fermeture `Fn`. Le
type `fn` s'appelle un *pointeur de fonction*. La syntaxe pour renseigner qu'un
paramètre est un pointeur de fonction ressemble à celle des fermetures, comme
vous pouvez le voir dans l'encart 19-27.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-27/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-27/src/main.rs}}
```

<!--
<span class="caption">Listing 19-27: Using the `fn` type to accept a function
pointer as an argument</span>
-->

<span class="caption">Encart 19-27 : utiliser le type `fn` pour accepter un
pointeur de fonction en argument</span>

<!--
This code prints `The answer is: 12`. We specify that the parameter `f` in
`do_twice` is an `fn` that takes one parameter of type `i32` and returns an
`i32`. We can then call `f` in the body of `do_twice`. In `main`, we can pass
the function name `add_one` as the first argument to `do_twice`.
-->

Ce code affiche `La réponse est : 12`. Nous avons précisé que le paramètre `f`
dans `le_faire_deux_fois` est une `fn` qui prend en argument un paramètre du
type `i32` et retourne un `i32`. Nous pouvons ensuite appeler `f` dans le corps
de `le_faire_deux_fois`. Dans `main`, nous pouvons envoyer le nom de la fonction
`add_one` dans le premier argument de `le_faire_deux_fois`.

<!--
Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the
parameter type directly rather than declaring a generic type parameter with one
of the `Fn` traits as a trait bound.
-->

Contrairement aux fermetures, `fn` est un type plutôt qu'un trait, donc nous
renseignons `fn` directement comme type de paramètre plutôt que de déclarer un
paramètre de type générique avec un des traits `Fn` liés.

<!--
Function pointers implement all three of the closure traits (`Fn`, `FnMut`, and
`FnOnce`), so you can always pass a function pointer as an argument for a
function that expects a closure. It’s best to write functions using a generic
type and one of the closure traits so your functions can accept either
functions or closures.
-->

Les pointeurs de fonctions implémentent tous les trois traits de fermeture
(`Fn`, `FnMut`, et `FnOnce`), afin que vous puissiez toujours envoyer un
pointeur de fonction en argument d'une fonction qui attendait une fermeture. Il
vaut mieux écrire des fonctions qui utilisent un type générique et un des traits
de fermetures afin que vos fonctions puissent accepter soit des fonctions, soit
des fermetures.

<!--
An example of where you would want to only accept `fn` and not closures is when
interfacing with external code that doesn’t have closures: C functions can
accept functions as arguments, but C doesn’t have closures.
-->

Un exemple dans lequel vous voudriez accepter uniquement des `fn`, et non pas
des fermetures, est lorsque vous vous interfacez avec du code externe qui n'a
pas de fermetures : les fonctions C peuvent accepter des fonctions en argument,
mais le C n'a pas fermetures.

<!--
As an example of where you could use either a closure defined inline or a named
function, let’s look at a use of `map`. To use the `map` function to turn a
vector of numbers into a vector of strings, we could use a closure, like this:
-->

Un autre exemple dans lequel vous pouvez utiliser soit une fermeture définie en
une seule ligne ou le nom d'une fonction, est lorsque vous utilisez `map`. Pour
utiliser la fonction `map` pour transformer un vecteur de nombres en vecteur de
chaînes de caractères, nous pouvons utiliser une fermeture, comme ceci :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-15-map-closure/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-15-map-closure/src/main.rs:here}}
```

<!--
Or we could name a function as the argument to `map` instead of the closure,
like this:
-->

Ou alors nous pouvons utiliser le nom d'une fonction en argument de `map` plutôt
qu'une fermeture, comme ceci :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-16-map-function/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-16-map-function/src/main.rs:here}}
```

<!--
Note that we must use the fully qualified syntax that we talked about earlier
in the [“Advanced Traits”][advanced-traits]<!-- ignore -- > section because
there are multiple functions available named `to_string`. Here, we’re using the
`to_string` function defined in the `ToString` trait, which the standard
library has implemented for any type that implements `Display`.
-->

Notez que nous devons utiliser la syntaxe complète que nous avons vu
précédemment dans [la section précédente][advanced-traits]<!-- ignore --> car il
y a plusieurs fonctions de disponibles qui s'appellent `to_string`. Ici, nous
utilisons la fonction `to_string` définie dans le trait `ToString`, que la
bibliothèque standard a implémenté sur chaque type qui implémente `Display`.

<!--
We have another useful pattern that exploits an implementation detail of tuple
structs and tuple-struct enum variants. These types use `()` as initializer
syntax, which looks like a function call. The initializers are actually
implemented as functions returning an instance that’s constructed from their
arguments. We can use these initializer functions as function pointers that
implement the closure traits, which means we can specify the initializer
functions as arguments for methods that take closures, like so:
-->

Nous avons un autre motif utile qui exploite un détail de l'implémentation des
structures tuple et des variantes d'énumérations de structures de tuples. Ces
types utilisent `()` comme syntaxe d'initialisation, qui ressemble à un appel de
fonction. Les initialisateurs sont actuellement implémentés comme étant des
fonctions qui retournent une instance qui est construite avec leurs propres
arguments. Nous pouvons utiliser ces fonctions d'initialisation comme étant des
pointeurs de fonctions qui implémentent les traits de fermetures, ce qui
signifie que nous pouvons renseigner les fonctions d'initialisation en argument
de méthodes qui prennent en argument des fermetures, comme ceci :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-17-map-initializer/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-17-map-initializer/src/main.rs:here}}
```

<!--
Here we create `Status::Value` instances using each `u32` value in the range
that `map` is called on by using the initializer function of `Status::Value`.
Some people prefer this style, and some people prefer to use closures. They
compile to the same code, so use whichever style is clearer to you.
-->

Nous avons ici créé des instances de `Statut::Valeur` en utilisant chacune des
valeurs `u32` présentes dans l'intervalle sur laquelle nous appelons `map` en
utilisant la fonction d'initialisation de `Statut::Valeur`. Certaines personnes
préfèrent ce style, et d'autres préfèrent utiliser des fermetures. Ces deux cas
se compilent pour faire la même chose, donc vous pouvez utiliser le style qui
vous conviens le plus.

<!--
### Returning Closures
-->

### Retourner des fermetures

<!--
Closures are represented by traits, which means you can’t return closures
directly. In most cases where you might want to return a trait, you can instead
use the concrete type that implements the trait as the return value of the
function. But you can’t do that with closures because they don’t have a
concrete type that is returnable; you’re not allowed to use the function
pointer `fn` as a return type, for example.
-->

Les fermetures sont représentées par des traits, ce qui signifie que vous ne
pouvez pas retourner directement des fermetures. Dans la plupart des cas où vous
voudrez retourner un trait, vous pouvez utiliser à la place le type concret qui
implémente le trait comme valeur de retour de la fonction. Mais vous ne pouvez
pas faire ceci avec les fermetures car elles n'ont pas de type concret qu'elles
peuvent retourner ; vous n'êtes pas autorisé à utiliser le pointeur de fonction
`fn` comme type de retour, par exemple.

<!--
The following code tries to return a closure directly, but it won’t compile:
-->

Le code suivant essaye de retourner directement une fermeture, mais ne peut pas
se compiler :

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-18-returns-closure/src/lib.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-18-returns-closure/src/lib.rs}}
```

<!--
The compiler error is as follows:
-->

Voici l'erreur de compilation :

<!--
```console
{{#include ../listings-sources/ch19-advanced-features/no-listing-18-returns-closure/output.txt}}
```
-->

```console
{{#include ../listings/ch19-advanced-features/no-listing-18-returns-closure/output.txt}}
```

<!--
The error references the `Sized` trait again! Rust doesn’t know how much space
it will need to store the closure. We saw a solution to this problem earlier.
We can use a trait object:
-->

Encore l'erreur du trait `Sized` ! Rust ne sait pas combien de mémoire sera
nécessaire pour stocker la fermeture. Nous avons vu une solution à ce problème
précédemment. Nous pouvons utiliser un objet trait :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-19-returns-closure-trait-object/src/lib.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-19-returns-closure-trait-object/src/lib.rs}}
```

<!--
This code will compile just fine. For more about trait objects, refer to the
section [“Using Trait Objects That Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore -- > in Chapter 17.
-->

Ce code va se compiler à merveille. Pour en savoir plus sur les objets trait,
rendez-vous
[à la section][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore --> du chapitre 17.

<!--
Next, let’s look at macros!
-->

Maintenant, penchons-nous sur les macros !

<!--
[advanced-traits]:
ch19-03-advanced-traits.html#advanced-traits
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
-->

[advanced-traits]: ch19-03-advanced-traits.html
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html
