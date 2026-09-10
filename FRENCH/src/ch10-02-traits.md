<!--
## Traits: Defining Shared Behavior
-->

## Définir des comportements partagés avec les traits

<!--
A *trait* tells the Rust compiler about functionality a particular type has and
can share with other types. We can use traits to define shared behavior in an
abstract way. We can use trait bounds to specify that a generic type can be any
type that has certain behavior.
-->

Un *trait* décrit une fonctionnalité qu'a un type particulier et qu'il peut
partager avec d'autres types, à destination du compilateur Rust. Nous pouvons
utiliser les traits pour définir un comportement partagé de manière abstraite.
Nous pouvons lier ces traits à un type générique pour exprimer le fait qu'il
puisse être de n'importe quel type à condition qu'il ait un comportement donné.

<!--
> Note: Traits are similar to a feature often called *interfaces* in other
> languages, although with some differences.
-->

> Remarque : les traits sont similaires à ce qu'on appelle parfois les
> *interfaces* dans d'autres langages, malgré quelques différences.

<!--
### Defining a Trait
-->

### Définir un trait

<!--
A type’s behavior consists of the methods we can call on that type. Different
types share the same behavior if we can call the same methods on all of those
types. Trait definitions are a way to group method signatures together to
define a set of behaviors necessary to accomplish some purpose.
-->

Le comportement d'un type s'exprime via les méthodes que nous pouvons appeler
sur ce type. Différents types peuvent partager le même comportement si nous
pouvons appeler les mêmes méthodes sur tous ces types. Définir un trait est une
manière de regrouper des signatures de méthodes pour définir un comportement
nécessaire pour accomplir un objectif.

<!--
For example, let’s say we have multiple structs that hold various kinds and
amounts of text: a `NewsArticle` struct that holds a news story filed in a
particular location and a `Tweet` that can have at most 280 characters along
with metadata that indicates whether it was a new tweet, a retweet, or a reply
to another tweet.
-->

Par exemple, imaginons que nous avons plusieurs structures qui stockent
différents types et quantités de texte : une structure `ArticleDePresse`, qui
contient un reportage dans un endroit donné et un `Tweet` qui peut avoir jusqu'à
280 caractères maximum et des métadonnées qui indiquent si cela est un nouveau
tweet, un retweet, ou une réponse à un autre tweet.

<!--
We want to make a media aggregator library crate named `aggregator` that can
display summaries of data that might be stored in a `NewsArticle` or `Tweet`
instance. To do this, we need a summary from each type, and we’ll request
that summary by calling a `summarize` method on an instance. Listing 10-12
shows the definition of a public `Summary` trait that expresses this behavior.
-->

Nous voulons construire une crate de bibliothèque `agregateur` pour des
agrégateurs de médias qui peut afficher le résumé des données stockées dans une
instance de `ArticleDePresse` ou de `Tweet`. Pour cela, il nous faut un résumé
pour chaque type, et nous allons demander ce résumé en appelant la méthode
`resumer` sur une instance. L'encart 10-12 nous montre la définition d'un trait
public `Resumable` qui décrit ce comportement.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

<!--
<span class="caption">Listing 10-12: A `Summary` trait that consists of the
behavior provided by a `summarize` method</span>
-->

<span class="caption">Encart 10-12 : un trait `Resumable` qui représente le
comportement fourni par une méthode `resumer`</span>

<!--
Here, we declare a trait using the `trait` keyword and then the trait’s name,
which is `Summary` in this case. We’ve also declared the trait as `pub` so that
crates depending on this crate can make use of this trait too, as we’ll see in
a few examples. Inside the curly brackets, we declare the method signatures
that describe the behaviors of the types that implement this trait, which in
this case is `fn summarize(&self) -> String`.
-->

Ici, nous déclarons un trait en utilisant le mot-clé `trait` et ensuite le nom
du trait, qui est `Resumable` dans notre cas. Nous avons aussi déclaré le trait
comme `pub` afin que les crates qui dépendent de cette crate puissent
aussi utiliser ce trait, comme nous allons le voir dans quelques exemples.
Entre les accolades, nous déclarons la signature de la méthode qui décrit le
comportement des types qui implémentent ce trait, qui est dans notre cas
`fn resumer(&self) -> String`.

<!--
After the method signature, instead of providing an implementation within curly
brackets, we use a semicolon. Each type implementing this trait must provide
its own custom behavior for the body of the method. The compiler will enforce
that any type that has the `Summary` trait will have the method `summarize`
defined with this signature exactly.
-->

A la fin de la signature de la méthode, au lieu de renseigner une implémentation
entre des accolades, nous utilisons un point-virgule. Chaque type qui implémente
ce trait doit renseigner son propre comportement dans le corps de la méthode. Le
compilateur va s'assurer que tous les types qui ont le trait `Resumable` auront
la méthode `resumer` définie avec cette signature précise.

<!--
A trait can have multiple methods in its body: the method signatures are listed
one per line and each line ends in a semicolon.
-->

Un trait peut avoir plusieurs méthodes dans son corps : les signatures des
méthodes sont ajoutées ligne par ligne et chaque ligne se termine avec un
point-virgule.

<!--
### Implementing a Trait on a Type
-->

### Implémenter un trait sur un type

<!--
Now that we’ve defined the desired signatures of the `Summary` trait’s methods,
we can implement it on the types in our media aggregator. Listing 10-13 shows
an implementation of the `Summary` trait on the `NewsArticle` struct that uses
the headline, the author, and the location to create the return value of
`summarize`. For the `Tweet` struct, we define `summarize` as the username
followed by the entire text of the tweet, assuming that tweet content is
already limited to 280 characters.
-->

Maintenant que nous avons défini les signatures souhaitées des méthodes du
trait `Resumable`, nous pouvons maintenant l'implémenter sur les types de notre
agrégateur de médias. L'encart 10-13 montre une implémentation du trait
`Resumable` sur la structure `ArticleDePresse` qui utilise le titre, le nom de
l'auteur et le lieu pour créer la valeur de retour de `resumer`. Pour la
structure `Tweet`, nous définissons `resumer` avec le nom d'utilisateur suivi
par le texte entier du tweet, en supposant que le contenu du tweet est déjà
limité à 280 caractères.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 10-13: Implementing the `Summary` trait on the
`NewsArticle` and `Tweet` types</span>
-->

<span class="caption">Encart 10-13 : implémentation du trait `Resumable` sur les
types `ArticleDePresse` et `Tweet`</span>

<!--
Implementing a trait on a type is similar to implementing regular methods. The
difference is that after `impl`, we put the trait name that we want to
implement, then use the `for` keyword, and then specify the name of the type we
want to implement the trait for. Within the `impl` block, we put the method
signatures that the trait definition has defined. Instead of adding a semicolon
after each signature, we use curly brackets and fill in the method body with
the specific behavior that we want the methods of the trait to have for the
particular type.
-->

L'implémentation d'un trait sur un type est similaire à l'implémentation d'une
méthode classique. La différence est que nous ajoutons le nom du trait que nous
voulons implémenter après le `impl`, et que nous utilisons ensuite le mot-clé
`for` suivi du nom du type sur lequel nous souhaitons implémenter le trait. À
l'intérieur du bloc `impl`, nous ajoutons les signatures des méthodes présentes
dans la définition du trait. Au lieu d'ajouter un point-virgule après chaque
signature, nous plaçons les accolades et on remplit le corps de la méthode avec
le comportement spécifique que nous voulons que les méthodes du trait suivent
pour le type en question.

<!--
Now that the library has implemented the `Summary` trait on `NewsArticle` and
`Tweet`, users of the crate can call the trait methods on instances of
`NewsArticle` and `Tweet` in the same way we call regular methods. The only
difference is that the trait has to be brought into scope as well as the types
to get the additional trait methods. Here’s an example of how a binary crate
could use our `aggregator` library crate:
-->

Maintenant que la bibliothèque a implémenté le trait `Resumable` sur
`ArticleDePresse` et `Tweet`, les utilisateurs de cette crate peuvent appeler
les méthodes de l'instance de `ArticleDePresse` et `Tweet` comme si elles
étaient des méthodes classiques. La seule différence est que le trait ainsi que
les types doivent être introduits dans la portée pour obtenir les méthodes de
trait additionnelles. Voici un exemple de comment la crate binaire pourra
utiliser notre crate de bibliothèque `agregateur` :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

<!--
This code prints `1 new tweet: horse_ebooks: of course, as you probably already
know, people`.
-->

Ce code affichera `1 nouveau tweet : jean : Bien sûr, les amis, comme vous le
savez probablement déjà`.

<!--
Other crates that depend on the `aggregator` crate can also bring the `Summary`
trait into scope to implement the trait on their own types. One restriction to
note with trait implementations is that we can implement a trait on a type only
if at least one of the trait or the type is local to our crate. For example, we
can implement standard library traits like `Display` on a custom type like
`Tweet` as part of our `aggregator` crate functionality, because the type
`Tweet` is local to our `aggregator` crate. We can also implement `Summary` on
`Vec<T>` in our `aggregator` crate, because the trait `Summary` is local to our
`aggregator` crate.
-->

Les autres crates qui dépendent de la crate `agregateur` peuvent aussi importer
dans la portée le trait `Resumable` afin d'implémenter le trait sur leurs
propres types. Il y a une limitation à souligner avec l'implémentation des
traits, c'est que nous ne pouvons implémenter un trait sur un type qu'à
condition qu'au moins le trait ou le type soit défini localement dans notre
crate. Par exemple, nous pouvons implémenter des traits de la bibliothèque
standard comme `Display` sur un type personnalisé comme `Tweet` comme une
fonctionnalité de notre crate `agregateur`, car le type `Tweet` est défini
localement dans notre crate `agregateur`. Nous pouvons aussi implémenter
`Resumable` sur `Vec<T>` dans notre crate `agregateur`, car le trait
`Resumable` est défini localement dans notre crate `agregateur`.

<!--
But we can’t implement external traits on external types. For example, we can’t
implement the `Display` trait on `Vec<T>` within our `aggregator` crate,
because `Display` and `Vec<T>` are defined in the standard library and aren’t
local to our `aggregator` crate. This restriction is part of a property of
programs called *coherence*, and more specifically the *orphan rule*, so named
because the parent type is not present. This rule ensures that other people’s
code can’t break your code and vice versa. Without the rule, two crates could
implement the same trait for the same type, and Rust wouldn’t know which
implementation to use.
-->

Mais nous ne pouvons pas implémenter des traits externes sur des types externes.
Par exemple, nous ne pouvons pas implémenter le trait `Display` sur `Vec<T>` à
l'intérieur de notre crate `agregateur`, car `Display` et `Vec<T>` sont définis
dans la bibliothèque standard et ne sont donc pas définis localement dans notre
crate `agregateur`. Cette limitation fait partie d'une propriété des programmes
que l'on appelle la *cohérence*, et plus précisément la *règle de l'orphelin*,
qui s'appelle ainsi car le type parent n'est pas présent. Cette règle s'assure
que le code des autres personnes ne casse pas votre code et réciproquement.
Sans cette règle, deux crates pourraient implémenter le même trait sur le même
type, et Rust ne saurait pas quelle implémentation utiliser.

<!--
### Default Implementations
-->

### Implémentations par défaut

<!--
Sometimes it’s useful to have default behavior for some or all of the methods
in a trait instead of requiring implementations for all methods on every type.
Then, as we implement the trait on a particular type, we can keep or override
each method’s default behavior.
-->

Il est parfois utile d'avoir un comportement par défaut pour toutes ou une
partie des méthodes d'un trait plutôt que de demander l'implémentation de toutes
les méthodes sur chaque type. Ainsi, si nous implémentons le trait sur un type
particulier, nous pouvons garder ou réécrire le comportement par défaut de
chaque méthode.

<!--
Listing 10-14 shows how to specify a default string for the `summarize` method
of the `Summary` trait instead of only defining the method signature, as we did
in Listing 10-12.
-->

L'encart 10-14 nous montre comment préciser une String par défaut pour la
méthode `resumer` du trait `Resumable` plutôt que de définir uniquement la
signature de la méthode, comme nous l'avons fait dans l'encart 10-12.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 10-14: Definition of a `Summary` trait with a
default implementation of the `summarize` method</span>
-->

<span class="caption">Encart 10-14 : définition du trait `Resumable` avec une
implémentation par défaut de la méthode `resumer`</span>

<!--
To use a default implementation to summarize instances of `NewsArticle` instead
of defining a custom implementation, we specify an empty `impl` block with
`impl Summary for NewsArticle {}`.
-->

Pour utiliser l'implémentation par défaut pour résumer des instances de
`ArticleDePresse` au lieu de préciser une implémentation personnalisée, nous
précisons un bloc `impl` vide avec `impl Resumable for ArticleDePresse {}`.

<!--
Even though we’re no longer defining the `summarize` method on `NewsArticle`
directly, we’ve provided a default implementation and specified that
`NewsArticle` implements the `Summary` trait. As a result, we can still call
the `summarize` method on an instance of `NewsArticle`, like this:
-->

Même si nous ne définissons plus directement la méthode `resumer` sur
`ArticleDePresse`, nous avons fourni une implémentation par défaut et précisé
que `ArticleDePresse` implémente le trait `Resumable`. Par conséquent, nous
pouvons toujours appeler la méthode `resumer` sur une instance de
`ArticleDePresse`, comme ceci :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

<!--
This code prints `New article available! (Read more...)`.
-->

Ce code va afficher `Nouvel article disponible ! (En savoir plus ...)`.

<!--
Creating a default implementation for `summarize` doesn’t require us to change
anything about the implementation of `Summary` on `Tweet` in Listing 10-13. The
reason is that the syntax for overriding a default implementation is the same
as the syntax for implementing a trait method that doesn’t have a default
implementation.
-->

La création d'une implémentation par défaut pour `resumer` n'a pas besoin que
nous modifiions quelque chose dans l'implémentation de `Resumable` sur `Tweet`
dans l'encart 10-13. C'est parce que la syntaxe pour réécrire l'implémentation
par défaut est la même que la syntaxe pour implémenter une méthode qui n'a pas
d'implémentation par défaut.

<!--
Default implementations can call other methods in the same trait, even if those
other methods don’t have a default implementation. In this way, a trait can
provide a lot of useful functionality and only require implementors to specify
a small part of it. For example, we could define the `Summary` trait to have a
`summarize_author` method whose implementation is required, and then define a
`summarize` method that has a default implementation that calls the
`summarize_author` method:
-->

Les implémentations par défaut peuvent appeler d'autres méthodes du même trait,
même si ces autres méthodes n'ont pas d'implémentation par défaut. Ainsi, un
trait peut fournir de nombreuses fonctionnalités utiles et n'exiger du
développeur qui l'utilise que d'en implémenter une petite partie. Par exemple,
nous pouvons définir le trait `Resumable` comme ayant une méthode
`resumer_auteur` dont l'implémentation est nécessaire, et ensuite définir une
méthode `resumer` qui a une implémentation par défaut qui appelle la méthode
`resumer_auteur` :

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

<!--
To use this version of `Summary`, we only need to define `summarize_author`
when we implement the trait on a type:
-->

Pour pouvoir utiliser cette version de `Resumable`, nous avons seulement besoin
de définir `resumer_auteur` lorsqu'on implémente le trait sur le type :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

<!--
After we define `summarize_author`, we can call `summarize` on instances of the
`Tweet` struct, and the default implementation of `summarize` will call the
definition of `summarize_author` that we’ve provided. Because we’ve implemented
`summarize_author`, the `Summary` trait has given us the behavior of the
`summarize` method without requiring us to write any more code.
-->

Après avoir défini `resumer_auteur`, nous pouvons appeler `resumer` sur des
instances de la structure `Tweet`, et l'implémentation par défaut de `resumer`
va appeler `resumer_auteur`, que nous avons défini. Comme nous avons implémenté
`resumer_auteur`, le trait `Resumable` nous a donné le comportement de la
méthode `resumer` sans nous obliger à écrire une ligne de code supplémentaire.

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

<!--
This code prints `1 new tweet: (Read more from @horse_ebooks...)`.
-->

Ce code affichera `1 nouveau tweet : (Lire plus d'éléments de @jean ...)`.

<!--
Note that it isn’t possible to call the default implementation from an
overriding implementation of that same method.
-->

Notez qu'il n'est pas possible d'appeler l'implémentation par défaut à partir
d'une réécriture de cette même méthode.

<!--
### Traits as Parameters
-->

### Des traits en paramètres

<!--
Now that you know how to define and implement traits, we can explore how to use
traits to define functions that accept many different types.
-->

Maintenant que vous savez comment définir et implémenter les traits, nous
pouvons regarder comment utiliser les traits pour définir des fonctions qui
acceptent plusieurs types différents.

<!--
For example, in Listing 10-13, we implemented the `Summary` trait on the
`NewsArticle` and `Tweet` types. We can define a `notify` function that calls
the `summarize` method on its `item` parameter, which is of some type that
implements the `Summary` trait. To do this, we can use the `impl Trait`
syntax, like this:
-->

Par exemple, dans l'encart 10-13, nous avons implémenté le trait `Resumable`
sur les types `ArticleDePresse` et `Tweet`. Nous pouvons définir une fonction
`notifier` qui va appeler la méthode `resumer` sur son paramètre `element`, qui
est d'un type qui implémente le trait `Resumable`. Pour faire ceci, nous
pouvons utiliser la syntaxe `impl Trait`, comme ceci :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

<!--
Instead of a concrete type for the `item` parameter, we specify the `impl`
keyword and the trait name. This parameter accepts any type that implements the
specified trait. In the body of `notify`, we can call any methods on `item`
that come from the `Summary` trait, such as `summarize`. We can call `notify`
and pass in any instance of `NewsArticle` or `Tweet`. Code that calls the
function with any other type, such as a `String` or an `i32`, won’t compile
because those types don’t implement `Summary`.
-->

Au lieu d'un type concret pour le paramètre `element`, nous précisons le mot-clé
`impl` et le nom du trait. Ce paramètre accepte n'importe quel type qui
implémente le trait spécifié. Dans le corps de `notifier`, nous pouvons appeler
toutes les méthodes sur `element` qui proviennent du trait `Resumable`, comme
`resumer`. Nous pouvons appeler `notifier` et passer une instance de
`ArticleDePresse` ou de `Tweet`. Le code qui appellera la fonction avec un autre
type, comme une `String` ou un `i32`, ne va pas se compiler car ces types
n'implémentent pas `Resumable`.

<!--
#### Trait Bound Syntax
-->

#### La syntaxe du trait lié

<!--
The `impl Trait` syntax works for straightforward cases but is actually
syntax sugar for a longer form, which is called a *trait bound*; it looks like
this:
-->

La syntaxe `impl Trait` fonctionne bien pour des cas simples, mais est en
réalité du sucre syntaxique pour une forme plus longue, qui s'appelle le
*trait lié*, qui ressemble à ceci :

<!--
```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
-->

```rust,ignore
pub fn notifier<T: Resumable>(element: &T) {
    println!("Flash info ! {}", element.resumer());
}
```

<!--
This longer form is equivalent to the example in the previous section but is
more verbose. We place trait bounds with the declaration of the generic type
parameter after a colon and inside angle brackets.
-->

Cette forme plus longue est équivalente à l'exemple dans la section précédente,
mais est plus verbeuse. Nous plaçons les traits liés dans la déclaration des
paramètres de type génériques après un deux-point entre des chevrons.

<!--
The `impl Trait` syntax is convenient and makes for more concise code in simple
cases. The trait bound syntax can express more complexity in other cases. For
example, we can have two parameters that implement `Summary`. Using the `impl
Trait` syntax looks like this:
-->

La syntaxe `impl Trait` est pratique pour rendre du code plus concis dans des
cas simples. La syntaxe du trait lié exprime plus de complexité dans certains
cas. Par exemple, nous pouvons avoir deux paramètres qui implémentent
`Resumable`. En utilisant la syntaxe `impl Trait`, nous aurons ceci :

<!--
```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```
-->

```rust,ignore
pub fn notifier(element1: &impl Resumable, element2: &impl Resumable) {
```

<!--
If we wanted this function to allow `item1` and `item2` to have different
types, using `impl Trait` would be appropriate (as long as both types implement
`Summary`). If we wanted to force both parameters to have the same type, that’s
only possible to express using a trait bound, like this:
-->

Si nous souhaitons permettre à `element1` et `element2` d'avoir des types
différents, l'utilisation de `impl Trait` est appropriée (du moment que chacun
de ces types implémentent `Resumable`). Mais si nous souhaitons forcer les deux
paramètres à être du même type, cela n'est possible à exprimer qu'avec un trait
lié, comme ceci :

<!--
```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```
-->

```rust,ignore
pub fn notifier<T: Resumable>(element1: &T, element2: &T) {
```

<!--
The generic type `T` specified as the type of the `item1` and `item2`
parameters constrains the function such that the concrete type of the value
passed as an argument for `item1` and `item2` must be the same.
-->

Le type générique `T` renseigné comme type des paramètres `element1` et
`element2` contraint la fonction de manière à ce que les types concrets des
valeurs passées en arguments pour `element1` et `element2` soient identiques.

<!--
#### Specifying Multiple Trait Bounds with the `+` Syntax
-->

#### Renseigner plusieurs traits liés avec la syntaxe `+`

<!--
We can also specify more than one trait bound. Say we wanted `notify` to use
display formatting on `item` as well as the `summarize` method: we specify in
the `notify` definition that `item` must implement both `Display` and
`Summary`. We can do so using the `+` syntax:
-->

Nous pouvons aussi préciser que nous attendons plus d'un trait lié. Imaginons
que nous souhaitons que `notifier` utilise le formatage d'affichage sur
`element` ainsi que la méthode `resumer` : nous indiquons dans la définition de
`notify` que `element` doit implémenter à la fois `Display` et `Resumable`.
Nous pouvons faire ceci avec la syntaxe `+` :

<!--
```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```
-->

```rust,ignore
pub fn notifier(element: &(impl Resumable + Display)) {
```

<!--
The `+` syntax is also valid with trait bounds on generic types:
-->

La syntaxe `+` fonctionne aussi avec les traits liés sur des types génériques :

<!--
```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```
-->

```rust,ignore
pub fn notifier<T: Resumable + Display>(element: &T) {
```

<!--
With the two trait bounds specified, the body of `notify` can call `summarize`
and use `{}` to format `item`.
-->

Avec les deux traits liés renseignés, le corps de `notifier` va appeler
`resumer` et utiliser `{}` pour formater `element`.

<!--
#### Clearer Trait Bounds with `where` Clauses
-->

#### Des traits liés plus clairs avec la clause `where`

<!--
Using too many trait bounds has its downsides. Each generic has its own trait
bounds, so functions with multiple generic type parameters can contain lots of
trait bound information between the function’s name and its parameter list,
making the function signature hard to read. For this reason, Rust has alternate
syntax for specifying trait bounds inside a `where` clause after the function
signature. So instead of writing this:
-->

L'utilisation de trop nombreux traits liés a aussi ses désavantages. Chaque
type générique a ses propres traits liés, donc les fonctions avec plusieurs
paramètres de type génériques peuvent aussi avoir de nombreuses informations de
traits liés entre le nom de la fonction et la liste de ses paramètres, ce qui
rend la signature de la fonction difficile à lire. Pour cette raison, Rust a une
syntaxe alternative pour renseigner les traits liés, dans une clause `where`
après la signature de la fonction. Donc, au lieu d'écrire ceci ...

<!--
```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```
-->

```rust,ignore
fn une_fonction<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

<!--
we can use a `where` clause, like this:
-->

... nous pouvons utiliser la clause `where`, comme ceci :

<!--
```rust,ignore
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```
-->

```rust,ignore
fn une_fonction<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

<!--
This function’s signature is less cluttered: the function name, parameter list,
and return type are close together, similar to a function without lots of trait
bounds.
-->

La signature de cette fonction est moins encombrée : le nom de la fonction, la
liste des paramètres et le type de retour sont plus proches les uns des autres,
comme une fonction sans traits liés.

<!--
### Returning Types that Implement Traits
-->

### Retourner des types qui implémentent des traits

<!--
We can also use the `impl Trait` syntax in the return position to return a
value of some type that implements a trait, as shown here:
-->

Nous pouvons aussi utiliser la syntaxe `impl Trait` à la place du type de
retour afin de retourner une valeur d'un type qui implémente un trait, comme
ci-dessous :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

<!--
By using `impl Summary` for the return type, we specify that the
`returns_summarizable` function returns some type that implements the `Summary`
trait without naming the concrete type. In this case, `returns_summarizable`
returns a `Tweet`, but the code calling this function doesn’t know that.
-->

En utilisant `impl Resumable` pour le type de retour, nous indiquons que la
fonction `retourne_resumable` retourne un type qui implémente le trait
`Resumable` sans avoir à écrire le nom du type concret. Dans notre cas,
`retourne_resumable` retourne un `Tweet`, mais le code qui appellera cette
fonction ne le saura pas.

<!--
The ability to return a type that is only specified by the trait it implements
is especially useful in the context of closures and iterators, which we cover
in Chapter 13. Closures and iterators create types that only the compiler knows
or types that are very long to specify. The `impl Trait` syntax lets you
concisely specify that a function returns some type that implements the
`Iterator` trait without needing to write out a very long type.
-->

La capacité de retourner un type qui est uniquement caractérisé par le trait
qu'il implémente est tout particulièrement utile dans le cas des fermetures et
des itérateurs, que nous verrons au chapitre 13. Les fermetures et les
itérateurs créent des types que seul le compilateur est en mesure de comprendre
ou alors des types qui sont très longs à définir. La syntaxe `impl Trait` vous
permet de renseigner de manière concise qu'une fonction retourne un type
particulier qui implémente le trait `Iterator` sans avoir à écrire un très long
type.

<!--
However, you can only use `impl Trait` if you’re returning a single type. For
example, this code that returns either a `NewsArticle` or a `Tweet` with the
return type specified as `impl Summary` wouldn’t work:
-->

Cependant, vous pouvez seulement utiliser `impl Trait` si vous retournez un
seul type possible. Par exemple, ce code va retourner soit un
`ArticleDePresse`, soit un `Tweet`, alors que le type de retour avec
`impl Resumable` ne va pas fonctionner :

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

<!--
Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions
around how the `impl Trait` syntax is implemented in the compiler. We’ll cover
how to write a function with this behavior in the [“Using Trait Objects That
Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore -- > section of Chapter 17.
-->

Retourner soit un `ArticleDePresse`, soit un `Tweet` n'est pas autorisé à cause
des restrictions sur la façon dont la syntaxe `impl Trait` est implémentée dans
le compilateur. Nous verrons comment écrire une fonction avec ce comportement
dans une section du
[chapitre 17][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore -->.

<!--
### Fixing the `largest` Function with Trait Bounds
-->

### Corriger la fonction `le_plus_grand` avec les traits liés

<!--
Now that you know how to specify the behavior you want to use using the generic
type parameter’s bounds, let’s return to Listing 10-5 to fix the definition of
the `largest` function that uses a generic type parameter! Last time we tried
to run that code, we received this error:
-->

Maintenant que vous savez comment renseigner le comportement que vous souhaitez
utiliser en utilisant les traits liés des paramètres de type génériques,
retournons à l'encart 10-5 pour corriger la définition de la fonction
`le_plus_grand` qui utilise un paramètre de type générique ! La dernière fois
que nous avons essayé de lancer ce code, nous avions l'erreur suivante :

<!--
```console
{{#include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```
-->

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

<!--
In the body of `largest` we wanted to compare two values of type `T` using the
greater than (`>`) operator. Because that operator is defined as a default
method on the standard library trait `std::cmp::PartialOrd`, we need to specify
`PartialOrd` in the trait bounds for `T` so the `largest` function can work on
slices of any type that we can compare. We don’t need to bring `PartialOrd`
into scope because it’s in the prelude. Change the signature of `largest` to
look like this:
-->

Dans le corps de `le_plus_grand`, nous voulions comparer les deux valeurs du
type `T` en utilisant l'opérateur *plus grand que* (`>`). Comme cet opérateur
est défini comme une méthode par défaut dans le trait de la bibliothèque
standard `std::cmp::PartialOrd`, nous devons préciser `PartialOrd` dans les
traits liés pour `T` afin que la fonction `le_plus_grand` puisse fonctionner
sur les slices de n'importe quel type que nous pouvons comparer. Nous n'avons
pas besoin d'importer `PartialOrd` dans la portée car il est importé dans
l'étape préliminaire. Changez la signature de `le_plus_grand` par quelque chose
comme ceci :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-07-fixing-listing-10-05/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-fixing-listing-10-05/src/main.rs:here}}
```

<!--
This time when we compile the code, we get a different set of errors:
-->

Cette fois, lorsque nous allons compiler le code, nous aurons un ensemble
d'erreurs différent :

<!--
```console
{{#include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-07-fixing-listing-10-05/output.txt}}
```
-->

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-fixing-listing-10-05/output.txt}}
```

<!--
The key line in this error is `cannot move out of type [T], a non-copy slice`.
With our non-generic versions of the `largest` function, we were only trying to
find the largest `i32` or `char`. As discussed in the [“Stack-Only Data:
Copy”][stack-only-data-copy]<!-- ignore -- > section in Chapter 4, types like
`i32` and `char` that have a known size can be stored on the stack, so they
implement the `Copy` trait. But when we made the `largest` function generic,
it became possible for the `list` parameter to have types in it that don’t
implement the `Copy` trait. Consequently, we wouldn’t be able to move the
value out of `list[0]` and into the `largest` variable, resulting in this
error.
-->

L'élément-clé dans ces erreurs est `cannot move out of type [T], a non-copy
slice` (*impossible de déplacer une valeur hors du type `[T]`, slice non
`Copy`*). Avec notre version non générique de la fonction `le_plus_grand`, nous
avions essayé de trouver le plus grand `i32` ou `char`. Comme nous l'avons vu
dans la section [“Données uniquement sur la pile : la
copie”][stack-only-data-copy]<!-- ignore --> du chapitre 4, les types comme
`i32` et `char` ont une taille connue et peuvent être stockés sur la pile, donc
ils implémentent le trait `Copy`. Mais quand nous avons rendu générique la
fonction `le_plus_grand`, il est devenu possible que le paramètre `liste`
contienne des types qui n'implémentent pas le trait `Copy`. Par conséquent,
nous ne pouvons pas forcément déplacer la valeur de `list[0]` dans notre
variable `le_plus_grand`, ce qui engendre cette erreur.

<!--
To call this code with only those types that implement the `Copy` trait, we can
add `Copy` to the trait bounds of `T`! Listing 10-15 shows the complete code of
a generic `largest` function that will compile as long as the types of the
values in the slice that we pass into the function implement the `PartialOrd`
*and* `Copy` traits, like `i32` and `char` do.
-->

Pour pouvoir appeler ce code avec seulement les types qui implémentent le trait
`Copy`, nous pouvons ajouter `Copy` aux traits liés de `T` ! L'encart 10-15 nous
montre le code complet d'une fonction générique `le_plus_grand` qui va se
compiler tant que le type des valeurs dans la slice que nous passons dans la
fonction implémente les traits `PartialOrd` *et* `Copy`, comme le font `i32` et
`char`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/main.rs}}
```

<!--
<span class="caption">Listing 10-15: A working definition of the `largest`
function that works on any generic type that implements the `PartialOrd` and
`Copy` traits</span>
-->

<span class="caption">Encart 10-15 : une définition de la fonction
`le_plus_grand` qui fonctionne et s'applique sur n'importe quel type générique
qui implémente les traits `PartialOrd` et `Copy`</span>

<!--
If we don’t want to restrict the `largest` function to the types that implement
the `Copy` trait, we could specify that `T` has the trait bound `Clone` instead
of `Copy`. Then we could clone each value in the slice when we want the
`largest` function to have ownership. Using the `clone` function means we’re
potentially making more heap allocations in the case of types that own heap
data like `String`, and heap allocations can be slow if we’re working with
large amounts of data.
-->

Si nous ne souhaitons pas restreindre la fonction `le_plus_grand` aux types qui
implémentent le trait `Copy`, nous pouvons préciser que `T` a le trait lié
`Clone` plutôt que `Copy`. Ainsi, nous pouvons cloner chaque valeur dans la
slice lorsque nous souhaitons que la fonction `le_plus_grand` en prenne
possession. L'utilisation de la fonction `clone` signifie que nous allons
potentiellement allouer plus d'espace sur le tas dans le cas des types qui
possèdent des données sur le tas, comme `String`, et les allocations sur le tas
peuvent être lentes si nous travaillons avec des grandes quantités de données.

<!--
Another way we could implement `largest` is for the function to return a
reference to a `T` value in the slice. If we change the return type to `&T`
instead of `T`, thereby changing the body of the function to return a
reference, we wouldn’t need the `Clone` or `Copy` trait bounds and we could
avoid heap allocations. Try implementing these alternate solutions on your own!
If you get stuck with errors having to do with lifetimes, keep reading: the
“Validating References with Lifetimes” section coming up will explain, but
lifetimes aren’t required to solve these challenges.
-->

Une autre façon d'implémenter `le_plus_grand` est de faire en sorte que la
fonction retourne une référence à une valeur `T` de la slice. Si nous changeons
le type de retour en `&T` à la place de `T` et que nous adaptons le corps de la
fonction afin de retourner une référence, nous n'aurions alors plus besoin des
traits liés `Clone` ou `Copy` et nous pourrions ainsi éviter l'allocation sur
le tas. Essayez d'implémenter ces solutions alternatives par vous-même !
Si vous bloquez sur des erreurs à propos des durées de vie *(lifetimes)*, lisez
la suite : la section suivante, “La conformité des références avec les durées
de vies” vous expliquera cela, mais les durées de vie ne sont pas nécessaires
pour résoudre ces exercices.

<!--
### Using Trait Bounds to Conditionally Implement Methods
-->

### Utiliser les traits liés pour conditionner l'implémentation des méthodes

<!--
By using a trait bound with an `impl` block that uses generic type parameters,
we can implement methods conditionally for types that implement the specified
traits. For example, the type `Pair<T>` in Listing 10-16 always implements the
`new` function to return a new instance of `Pair<T>` (recall from the
[”Defining Methods”][methods]<!-- ignore -- > section of Chapter 5 that `Self`
is a type alias for the type of the `impl` block, which in this case is
`Pair<T>`). But in the next `impl` block, `Pair<T>` only implements the
`cmp_display` method if its inner type `T` implements the `PartialOrd` trait
that enables comparison *and* the `Display` trait that enables printing.
-->

En utilisant un trait lié avec un bloc `impl` qui utilise les paramètres de type
génériques, nous pouvons implémenter des méthodes en fonction des types qui
implémentent des traits particuliers. Par exemple, le type `Paire<T>` de
l'encart 10-16 implémente toujours la fonction `new` pour retourner une
nouvelle instance de `Paire<T>` (pour rappel dans la section
[”Définir des méthodes”][methods]<!-- ignore --> du chapitre 5 que `Self` est
un alias de type pour le type du bloc `impl`, qui est dans ce cas le
`Paire<T>`). Mais dans le bloc `impl` suivant, `Paire<T>` implémente la
méthode `afficher_comparaison` uniquement si son type interne `T` implémente le
trait `PartialOrd` qui active la comparaison *et* le trait `Display` qui permet
l'affichage.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/lib.rs}}
```

<!--
<span class="caption">Listing 10-16: Conditionally implement methods on a
generic type depending on trait bounds</span>
-->

<span class="caption">Encart 10-16 : implémentation de méthodes sur un type
générique en fonction des traits liés</span>

<!--
We can also conditionally implement a trait for any type that implements
another trait. Implementations of a trait on any type that satisfies the trait
bounds are called *blanket implementations* and are extensively used in the
Rust standard library. For example, the standard library implements the
`ToString` trait on any type that implements the `Display` trait. The `impl`
block in the standard library looks similar to this code:
-->

Nous pouvons également implémenter un trait sur tout type qui implémente un
autre trait en particulier. L'implémentation d'un trait sur n'importe quel type
qui a un trait lié est appelée *implémentation générale* et est largement
utilisée dans la bibliothèque standard Rust. Par exemple, la bibliothèque
standard implémente le trait `ToString` sur tous les types qui implémentent le
trait `Display`. Le bloc `impl` de la bibliothèque standard ressemble au code
suivant :

<!--
```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```
-->

```rust,ignore
impl<T: Display> ToString for T {
    // -- partie masquée ici --
}
```

<!--
Because the standard library has this blanket implementation, we can call the
`to_string` method defined by the `ToString` trait on any type that implements
the `Display` trait. For example, we can turn integers into their corresponding
`String` values like this because integers implement `Display`:
-->

Comme la bibliothèque standard a cette implémentation générale, nous pouvons
appeler la méthode `to_string` définie par le trait `ToString` sur n'importe
quel type qui implémente le trait `Display`. Par exemple, nous pouvons
transformer les nombres entiers en leur équivalent dans une `String` comme
ci-dessous car les entiers implémentent `Display` :

<!--
```rust
let s = 3.to_string();
```
-->

```rust
let s = 3.to_string();
```

<!--
Blanket implementations appear in the documentation for the trait in the
“Implementors” section.
-->

Les implémentations générales sont décrites dans la documentation du trait, dans
la section “Implementors”.

<!--
Traits and trait bounds let us write code that uses generic type parameters to
reduce duplication but also specify to the compiler that we want the generic
type to have particular behavior. The compiler can then use the trait bound
information to check that all the concrete types used with our code provide the
correct behavior. In dynamically typed languages, we would get an error at
runtime if we called a method on a type which didn’t define the method. But Rust
moves these errors to compile time so we’re forced to fix the problems before
our code is even able to run. Additionally, we don’t have to write code that
checks for behavior at runtime because we’ve already checked at compile time.
Doing so improves performance without having to give up the flexibility of
generics.
-->

Les traits et les traits liés nous permettent d'écrire du code qui utilise des
paramètres de type génériques pour réduire la duplication de code, mais aussi
pour indiquer au compilateur que nous voulons que le type générique ait un
comportement particulier. Le compilateur peut ensuite utiliser les informations
liées aux traits pour vérifier que tous les types concrets utilisés dans notre
code suivent le comportement souhaité. Dans les langages typés dynamiquement,
nous aurions une erreur à l'exécution si nous appelions une méthode sur un type
qui n'implémenterait pas la méthode. Mais Rust décale l'apparition de ces erreurs
au moment de la compilation afin de nous forcer à résoudre les problèmes avant
même que notre code soit capable de s'exécuter. De plus, nous n'avons pas besoin
d'écrire un code qui vérifie le comportement lors de l'exécution car nous
l'avons déjà vérifié au moment de la compilation. Cela permet d'améliorer les
performances sans avoir à sacrifier la flexibilité des types génériques.

<!--
Another kind of generic that we’ve already been using is called *lifetimes*.
Rather than ensuring that a type has the behavior we want, lifetimes ensure
that references are valid as long as we need them to be. Let’s look at how
lifetimes do that.
-->

Une autre sorte de générique que nous avons déjà utilisée est la *durée de
vie*. Plutôt que de s'assurer qu'un type a le comportement que nous voulons, la
durée de vie s'assure que les références sont en vigueur aussi longtemps que
nous avons besoin qu'elles le soient. Nous allons voir à la page suivante
comment la durée de vie fait cela.

<!-- markdownlint-disable -->
<!--
[stack-only-data-copy]:
ch04-01-what-is-ownership.html#stack-only-data-copy
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[methods]: ch05-03-method-syntax.html#defining-methods
-->
<!-- markdownlint-restore -->

[stack-only-data-copy]:
ch04-01-what-is-ownership.html#données-uniquement-sur-la-pile--la-copie
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html
[methods]: ch05-03-method-syntax.html#définir-des-méthodes
