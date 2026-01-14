<!--
## Advanced Traits
-->

## Les traits avancés

<!--
We first covered traits in the [“Traits: Defining Shared
Behavior”][traits-defining-shared-behavior]<!-- ignore -- > section of Chapter
10, but we didn’t discuss the more advanced details. Now that you know more
about Rust, we can get into the nitty-gritty.
-->

Nous avons vu les traits dans une section du chapitre 10, mais nous n'avons pas
abordé certains détails plus avancés. Maintenant que vous en savez plus sur
Rust, nous pouvons attaquer les choses sérieuses.

<!--
### Specifying Placeholder Types in Trait Definitions with Associated Types
-->

### Placer des types à remplacer dans les définitions des traits grâce aux types associés

<!--
*Associated types* connect a type placeholder with a trait such that the trait
method definitions can use these placeholder types in their signatures. The
implementor of a trait will specify the concrete type to be used in this type’s
place for the particular implementation. That way, we can define a trait that
uses some types without needing to know exactly what those types are until the
trait is implemented.
-->

Les *types associés* connectent un type à remplacer avec un trait afin que la
définition des méthodes puisse utiliser ces types à remplacer dans leur
signature. Celui qui implémente un trait doit renseigner un type concret pour
être utilisé à la place du type à remplacer pour cette implémentation précise.
Ainsi, nous pouvons définir un trait qui utilise certains types sans avoir
besoin de savoir exactement quels sont ces types jusqu'à ce que ce trait soit
implémenté.

<!--
We’ve described most of the advanced features in this chapter as being rarely
needed. Associated types are somewhere in the middle: they’re used more rarely
than features explained in the rest of the book but more commonly than many of
the other features discussed in this chapter.
-->

Nous avions dit que vous auriez rarement besoin de la plupart des
fonctionnalités avancées de ce chapitre. Les types associés sont un entre-deux :
ils sont utilisés plus rarement que les fonctionnalités expliquées dans le reste
de ce livre, mais on les rencontre plus fréquemment que la plupart des autres
fonctionnalités présentées dans ce chapitre.

<!--
One example of a trait with an associated type is the `Iterator` trait that the
standard library provides. The associated type is named `Item` and stands in
for the type of the values the type implementing the `Iterator` trait is
iterating over. In [“The `Iterator` Trait and the `next`
Method”][the-iterator-trait-and-the-next-method]<!-- ignore -- > section of
Chapter 13, we mentioned that the definition of the `Iterator` trait is as
shown in Listing 19-12.
-->

Un exemple de trait avec un type associé est le trait `Iterator` que fournit la
bibliothèque standard. Le type associé `Item` permet de renseigner le type des
valeurs que le type qui implémente le trait `Iterator` parcourt. Dans une
section du chapitre 13, nous avions mentionné que la définition du trait
`Iterator` ressemblait à cet encart 19-12.

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-12/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-12/src/lib.rs}}
```

<!--
<span class="caption">Listing 19-12: The definition of the `Iterator` trait
that has an associated type `Item`</span>
-->

<span class="caption">Encart 19-12 : la définition du trait `Iterator` qui a un
type `Item` associé</span>

<!--
The type `Item` is a placeholder type, and the `next` method’s definition shows
that it will return values of type `Option<Self::Item>`. Implementors of the
`Iterator` trait will specify the concrete type for `Item`, and the `next`
method will return an `Option` containing a value of that concrete type.
-->

Le type `Item` est un type à remplacer, et la définition de la méthode `next`
informe qu'elle va retourner des valeurs du type `Option<Self::Item>`. Ceux qui
implémenterons le trait `Iterator` devront renseigner un type concret pour
`Item`, et la méthode `next` va retourner une `Option` qui contiendra une
valeur de ce type concret.

<!--
Associated types might seem like a similar concept to generics, in that the
latter allow us to define a function without specifying what types it can
handle. So why use associated types?
-->

Les types associés ressemblent au même concept que les génériques, car ces
derniers nous permettent de définir une fonction sans avoir à renseigner les
types avec lesquels elle travaille. Donc pourquoi utiliser les types associés ?

<!--
Let’s examine the difference between the two concepts with an example from
Chapter 13 that implements the `Iterator` trait on the `Counter` struct. In
Listing 13-21, we specified that the `Item` type was `u32`:
-->

Examinons les différences entre les deux concepts grâce à un exemple du
chapitre 13 qui implémente le trait `Iterator` sur la structure `Compteur`.
Dans l'encart 13-21, nous avions renseigné que le type `Item` était `u32` :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-13-21-reproduced/src/lib.rs:ch19}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/listing-13-21-reproduced/src/lib.rs:ch19}}
```

<!--
This syntax seems comparable to that of generics. So why not just define the
`Iterator` trait with generics, as shown in Listing 19-13?
-->

Cette syntaxe ressemble aux génériques. Donc pourquoi ne pas simplement définir le
trait `Iterator` avec les génériques, comme dans l'encart 19-13 ?

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-13/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-13/src/lib.rs}}
```

<!--
<span class="caption">Listing 19-13: A hypothetical definition of the
`Iterator` trait using generics</span>
-->

<span class="caption">Encart 19-13 : une définition hypothétique du trait
`Iterator` en utilisant des génériques</span>

<!--
The difference is that when using generics, as in Listing 19-13, we must
annotate the types in each implementation; because we can also implement
`Iterator<String> for Counter` or any other type, we could have multiple
implementations of `Iterator` for `Counter`. In other words, when a trait has a
generic parameter, it can be implemented for a type multiple times, changing
the concrete types of the generic type parameters each time. When we use the
`next` method on `Counter`, we would have to provide type annotations to
indicate which implementation of `Iterator` we want to use.
-->

La différence est que lorsque on utilise les génériques, comme dans l'encart
19-13, on doit annoter les types dans chaque implémentation ; et comme nous
pouvons aussi implémenter `Iterator<String> for Compteur` ou tout autre type,
nous pourrions alors avoir plusieurs implémentations de `Iterator` pour
`Compteur`. Autrement dit, lorsqu'un trait a un paramètre générique, il peut
être implémenté sur un type plusieurs fois, en changeant à chaque fois le type
concret du paramètre de type générique. Lorsque nous utilisons la méthode `next`
sur `Compteur`, nous devons appliquer une annotation de type pour indiquer
quelle implémentation de `Iterator` nous souhaitons utiliser.

<!--
With associated types, we don’t need to annotate types because we can’t
implement a trait on a type multiple times. In Listing 19-12 with the
definition that uses associated types, we can only choose what the type of
`Item` will be once, because there can only be one `impl Iterator for Counter`.
We don’t have to specify that we want an iterator of `u32` values everywhere
that we call `next` on `Counter`.
-->

Avec les types associés, nous n'avons pas besoin d'annoter les types car nous
ne pouvons pas implémenter un trait plusieurs fois sur un même type. Dans l'encart
19-12 qui contient la définition qui utilise les types associés, nous ne pouvons
 choisir quel sera le type de `Item` qu'une seule fois, car il ne peut
y avoir qu'un seul `impl Iterator for Compteur`. Nous n'avons pas à préciser
que nous souhaitons avoir un itérateur de valeurs `u32` à chaque fois que nous
faisons appel à `next` sur `Compteur`.

<!--
### Default Generic Type Parameters and Operator Overloading
-->

### Les paramètres de types génériques par défaut et la surcharge d'opérateur

<!--
When we use generic type parameters, we can specify a default concrete type for
the generic type. This eliminates the need for implementors of the trait to
specify a concrete type if the default type works. The syntax for specifying a
default type for a generic type is `<PlaceholderType=ConcreteType>` when
declaring the generic type.
-->

Lorsque nous utilisons les paramètres de types génériques, nous pouvons
renseigner un type concret par défaut pour le type générique. Cela évite de
contraindre ceux qui implémentent ce trait d'avoir à renseigner un type concret
si celui par défaut fonctionne bien. La syntaxe pour renseigner un type par
défaut pour un type générique est `<TypeARemplacer=TypeConcret>` lorsque nous
déclarons le type générique.

<!--
A great example of a situation where this technique is useful is with operator
overloading. *Operator overloading* is customizing the behavior of an operator
(such as `+`) in particular situations.
-->

Un bon exemple d'une situation pour laquelle cette technique est utile est avec
la surcharge d'opérateurs. *La surcharge d'opérateur* permet de personnaliser
le comportement d'un opérateur (comme `+`) dans des cas particuliers.

<!--
Rust doesn’t allow you to create your own operators or overload arbitrary
operators. But you can overload the operations and corresponding traits listed
in `std::ops` by implementing the traits associated with the operator. For
example, in Listing 19-14 we overload the `+` operator to add two `Point`
instances together. We do this by implementing the `Add` trait on a `Point`
struct:
-->

Rust ne vous permet pas de créer vos propres opérateurs ou de surcharger des
opérateurs. Mais vous pouvez surcharger les opérations et les traits listés
dans `std::ops` en implémentant les traits associés à l'opérateur. Par exemple,
dans l'encart 19-14 nous surchargeons l'opérateur `+` pour additionner ensemble
deux instances de `Point`. Nous pouvons faire cela en implémentant le trait
`Add` sur une structure `Point` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-14/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-14/src/main.rs}}
```

<!--
<span class="caption">Listing 19-14: Implementing the `Add` trait to overload
the `+` operator for `Point` instances</span>
-->

<span class="caption">Encart 19-14 : implémentation du trait `Add` pour
surcharger l'opérateur `+` pour les instances de `Point`</span>

<!--
The `add` method adds the `x` values of two `Point` instances and the `y`
values of two `Point` instances to create a new `Point`. The `Add` trait has an
associated type named `Output` that determines the type returned from the `add`
method.
-->

La méthode `add` ajoute les valeurs `x` de deux instances de `Point` ainsi que
les valeurs `y` de deux instances de `Point` pour créer un nouveau `Point`. Le
trait `Add` a un type associé `Output` qui détermine le type retourné pour la
méthode `add`.

<!--
The default generic type in this code is within the `Add` trait. Here is its
definition:
-->

Le type générique par défaut dans ce code est dans le trait `Add`. Voici sa
définition :

<!--
```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```
-->

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

<!--
This code should look generally familiar: a trait with one method and an
associated type. The new part is `Rhs=Self`: this syntax is called *default
type parameters*. The `Rhs` generic type parameter (short for “right hand
side”) defines the type of the `rhs` parameter in the `add` method. If we don’t
specify a concrete type for `Rhs` when we implement the `Add` trait, the type
of `Rhs` will default to `Self`, which will be the type we’re implementing
`Add` on.
-->

Ce code devrait vous être familier : un trait avec une méthode et un type
associé. La nouvelle partie concerne `Rhs=Self` : cette syntaxe s'appelle les
*paramètres de types par défaut*. Le paramètre de type générique `Rhs`
(c'est le raccourci de “Right Hand Side”) qui définit le type du paramètre
`rhs` dans la méthode `add`. Si nous ne renseignons pas de type concret pour
`Rhs` lorsque nous implémentons le trait `Add`, le type de `Rhs` sera par
défaut `Self`, qui sera le type sur lequel nous implémentons `Add`.

<!--
When we implemented `Add` for `Point`, we used the default for `Rhs` because we
wanted to add two `Point` instances. Let’s look at an example of implementing
the `Add` trait where we want to customize the `Rhs` type rather than using the
default.
-->

Lorsque nous avons implémenté `Add` sur `Point`, nous avons utilisé la valeur
par défaut de `Rhs` car nous voulions additionner deux instances de `Point`.
Voyons un exemple d'implémentation du trait `Add` dans lequel nous souhaitons
personnaliser le type `Rhs` plutôt que d'utiliser celui par défaut.

<!--
We have two structs, `Millimeters` and `Meters`, holding values in different
units. This thin wrapping of an existing type in another struct is known as the
*newtype pattern*, which we describe in more detail in the [“Using the Newtype
Pattern to Implement External Traits on External Types”][newtype]<!-- ignore
-- > section. We want to add values in millimeters to values in meters and have
the implementation of `Add` do the conversion correctly. We can implement `Add`
for `Millimeters` with `Meters` as the `Rhs`, as shown in Listing 19-15.
-->

Nous avons deux structures, `Millimetres` et `Metres`, qui stockent des valeurs
dans différentes unités. Ce léger enrobage d'un type existant dans une autre
structure s'appelle le *motif newtype*, que nous décrivons plus en détail dans
la section [Utiliser le motif newtype pour la sécurité et l'abstraction des
types][newtype]<!-- ignore -->. Nous voulons pouvoir additionner les valeurs en
millimètres avec les valeurs en mètres et appliquer l'implémentation de `Add`
pour pouvoir faire la conversion correctement. Nous pouvons implémenter `Add`
sur `Millimetres` avec `Metres` comme étant le `Rhs`, comme dans l'encart
19-15.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-15/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-15/src/lib.rs}}
```

<!--
<span class="caption">Listing 19-15: Implementing the `Add` trait on
`Millimeters` to add `Millimeters` to `Meters`</span>
-->

<span class="caption">Encart 19-15 : implémentation du trait `Add` sur
`Millimetres` pour pouvoir additionner `Millimetres` à `Metres`</span>

<!--
To add `Millimeters` and `Meters`, we specify `impl Add<Meters>` to set the
value of the `Rhs` type parameter instead of using the default of `Self`.
-->

Pour additionner `Millimetres` et `Metres`, nous renseignons
`impl Add<Metres>` pour régler la valeur du paramètre de type `Rhs` au lieu
d'utiliser la valeur par défaut `Self`.

<!--
You’ll use default type parameters in two main ways:
-->

Vous utiliserez les paramètres de types par défaut dans deux principaux cas :

<!--
* To extend a type without breaking existing code
* To allow customization in specific cases most users won’t need
-->

* Pour étendre un type sans casser le code existant
* Pour permettre la personnalisation dans des cas spécifiques que la plupart
  des utilisateurs n'auront pas

<!--
The standard library’s `Add` trait is an example of the second purpose:
usually, you’ll add two like types, but the `Add` trait provides the ability to
customize beyond that. Using a default type parameter in the `Add` trait
definition means you don’t have to specify the extra parameter most of the
time. In other words, a bit of implementation boilerplate isn’t needed, making
it easier to use the trait.
-->

Le trait `Add` de la bibliothèque standard est un exemple du second cas :
généralement, vous additionnez deux types similaires, mais le trait `Add` offre
la possibilité de personnaliser cela. L'utilisation d'un paramètre de type par
défaut dans la définition du trait `Add` signifie que vous n'aurez pas à
renseigner de paramètre en plus la plupart du temps. Autrement dit, il n'est
pas nécessaire d'avoir recours à des assemblages de code, ce qui facilite
l'utilisation du trait.

<!--
The first purpose is similar to the second but in reverse: if you want to add a
type parameter to an existing trait, you can give it a default to allow
extension of the functionality of the trait without breaking the existing
implementation code.
-->

Le premier cas est similaire au second mais dans le cas inverse : si vous
souhaitez ajouter un paramètre de type à un trait existant, vous pouvez lui en
donner un par défaut pour permettre l'ajout des fonctionnalités du trait sans
casser l'implémentation actuelle du code.

<!--
### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
-->

### La syntaxe totalement définie pour clarifier les appels à des méthodes qui ont le même nom

<!--
Nothing in Rust prevents a trait from having a method with the same name as
another trait’s method, nor does Rust prevent you from implementing both traits
on one type. It’s also possible to implement a method directly on the type with
the same name as methods from traits.
-->

Il n'y a rien en Rust qui empêche un trait d'avoir une méthode portant le même
nom qu'une autre méthode d'un autre trait, ni ne vous empêche d'implémenter
ces deux traits sur un même type. Il est aussi possible
d'implémenter directement une méthode avec le même nom que celle présente dans
les traits sur ce type.

<!--
When calling methods with the same name, you’ll need to tell Rust which one you
want to use. Consider the code in Listing 19-16 where we’ve defined two traits,
`Pilot` and `Wizard`, that both have a method called `fly`. We then implement
both traits on a type `Human` that already has a method named `fly` implemented
on it. Each `fly` method does something different.
-->

Lorsque nous faisons appel à des méthodes qui ont un conflit de nom, vous devez
préciser à Rust précisément celle que vous souhaitez utiliser. Imaginons le
code dans l'encart 19-16 dans lequel nous avons défini deux traits, `Pilote` et
`Magicien`, qui ont tous les deux une méthode `voler`. Nous
implémentons ensuite ces deux traits sur un type `Humain` qui a déjà lui-aussi
une méthode `voler` qui lui a été implémentée. Chaque méthode `voler` fait
quelque chose de différent.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-16/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-16/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-16: Two traits are defined to have a `fly`
method and are implemented on the `Human` type, and a `fly` method is
implemented on `Human` directly</span>
-->

<span class="caption">Encart 19-16 : deux traits qui ont une méthode `voler`
et qui sont implémentés sur le type `Humain`, et une méthode `voler` est aussi
implémentée directement sur `Humain`</span>

<!--
When we call `fly` on an instance of `Human`, the compiler defaults to calling
the method that is directly implemented on the type, as shown in Listing 19-17.
-->

Lorsque nous utilisons `voler` sur une instance de `Humain`, le compilateur
fait appel par défaut à la méthode qui est directement implémentée sur le type,
comme le montre l'encart 19-17.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-17/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-17/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-17: Calling `fly` on an instance of
`Human`</span>
-->

<span class="caption">Encart 19-17 : utilisation de `voler` sur une instance de
`Humain`</span>

<!--
Running this code will print `*waving arms furiously*`, showing that Rust
called the `fly` method implemented on `Human` directly.
-->

L'exécution de ce code va afficher `*agite frénétiquement ses bras*`, ce qui
démontre que Rust a appelé la méthode `voler` implémentée directement sur
`Humain`.

<!--
To call the `fly` methods from either the `Pilot` trait or the `Wizard` trait,
we need to use more explicit syntax to specify which `fly` method we mean.
Listing 19-18 demonstrates this syntax.
-->

Pour faire appel aux méthodes `voler` des traits `Pilote` ou `Magicien`, nous
devons utiliser une syntaxe plus explicite pour préciser quelle méthode `voler`
nous souhaitons utiliser. L'encart 19-18 montre cette syntaxe.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-18/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-18/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-18: Specifying which trait’s `fly` method we
want to call</span>
-->

<span class="caption">Encart 19-18 : préciser de quel trait nous souhaitons
utiliser la méthode `voler`</span>

<!--
Specifying the trait name before the method name clarifies to Rust which
implementation of `fly` we want to call. We could also write
`Human::fly(&person)`, which is equivalent to the `person.fly()` that we used
in Listing 19-18, but this is a bit longer to write if we don’t need to
disambiguate.
-->

Si on renseigne le nom du trait avant le nom de la méthode, cela indique à Rust
quelle implémentation de `voler` nous souhaitons utiliser. Nous pouvons aussi
écrire `Humain::voler(&une_personne)`, qui est équivalent à
`une_personne.voler()` que nous avons utilisé dans l'encart 19-18, mais c'est
un peu plus long à écrire si nous n'avons pas besoin de préciser les choses.

<!--
Running this code prints the following:
-->

L'exécution de ce code affiche ceci :

<!--
```console
{{#include ../listings-sources/ch19-advanced-features/listing-19-18/output.txt}}
```
-->

```console
{{#include ../listings/ch19-advanced-features/listing-19-18/output.txt}}
```

<!--
Because the `fly` method takes a `self` parameter, if we had two *types* that
both implement one *trait*, Rust could figure out which implementation of a
trait to use based on the type of `self`.
-->

Comme la méthode `voler` prend un paramètre `self`, si nous avions deux
*types* qui implémentaient chacun un des deux *traits*, Rust pourrait en
déduire quelle implémentation de quel trait utiliser en fonction du type
de `self`.

<!--
However, associated functions that are not methods don’t have a `self`
parameter. When there are multiple types or traits that define non-method
functions with the same function name, Rust doesn't always know which type you
mean unless you use *fully qualified syntax*. For example, the `Animal` trait
in Listing 19-19 has the associated non-method function `baby_name`, and the
`Animal` trait is implemented for the struct `Dog`. There’s also an associated
non-method function `baby_name` defined on `Dog` directly.
-->

Cependant, les fonctions associées qui ne sont pas des méthodes n'ont pas de
paramètre `self`. Lorsqu'il y a plusieurs types ou traits qui définissent des
fonctions qui ne sont pas des méthodes et qui ont le même nom de fonction, Rust
ne peut pas toujours savoir quel type vous sous-entendez jusqu'à ce que vous
utilisiez la *syntaxe totalement définie*. Par exemple, le trait `Animal` de
l'encart 19-19 a une fonction associée `nom_bebe` qui n'est pas une méthode, et
le trait `Animal` est implémenté pour la structure `Chien`.Il y a aussi une
fonction associée `nom_bebe` qui n'est pas une méthode et qui est définie
directement sur `Chien`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-19/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-19/src/main.rs}}
```

<!--
<span class="caption">Listing 19-19: A trait with an associated function and a
type with an associated function of the same name that also implements the
trait</span>
-->

<span class="caption">Encart 19-19 : un trait avec une fonction associée et un
type avec une autre fonction associée qui porte le même nom et qui implémente
aussi ce trait</span>

<!--
This code is for an animal shelter that wants to name all puppies Spot, which
is implemented in the `baby_name` associated function that is defined on `Dog`.
The `Dog` type also implements the trait `Animal`, which describes
characteristics that all animals have. Baby dogs are called puppies, and that
is expressed in the implementation of the `Animal` trait on `Dog` in the
`baby_name` function associated with the `Animal` trait.
-->

Ce code a été conçu pour un refuge pour animaux qui souhaite que tous leurs chiots
soient nommés Spot, ce qui est implémenté dans la fonction associée `nom_bebe`
de `Chien`. Le type `Chien` implémente lui aussi le trait `Animal`, qui décrit
les caractéristiques que tous les animaux doivent avoir. Les bébés chiens
doivent s'appeler des chiots, et ceci est exprimé dans l'implémentation du
trait `Animal` sur `Chien` dans la fonction `nom_bebe` associée au trait
`Animal`.

<!--
In `main`, we call the `Dog::baby_name` function, which calls the associated
function defined on `Dog` directly. This code prints the following:
-->

Dans le `main`, nous faisons appel à la fonction `Chien::nom_bebe`, qui fait
appel à la fonction associée directement définie sur `Chien`. Ce code affiche
ceci :

<!--
```console
{{#include ../listings-sources/ch19-advanced-features/listing-19-19/output.txt}}
```
-->

```console
{{#include ../listings/ch19-advanced-features/listing-19-19/output.txt}}
```

<!--
This output isn’t what we wanted. We want to call the `baby_name` function that
is part of the `Animal` trait that we implemented on `Dog` so the code prints
`A baby dog is called a puppy`. The technique of specifying the trait name that
we used in Listing 19-18 doesn’t help here; if we change `main` to the code in
Listing 19-20, we’ll get a compilation error.
-->

Ce résultat n'est pas celui que nous souhaitons. Nous voulons appeler la
fonction `nom_bebe` qui fait partie du trait `Animal` que nous avons implémenté
sur `Chien` afin que le code affiche `Un bébé chien s'appelle un chiot`. La
technique pour préciser le nom du trait que nous avons utilisée précédemment ne
va pas nous aider ici ; si nous changeons le `main` par le code de l'encart
19-20, nous allons avoir une erreur de compilation.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-20/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-20/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-20: Attempting to call the `baby_name`
function from the `Animal` trait, but Rust doesn’t know which implementation to
use</span>
-->

<span class="caption">Encart 19-20 : tentative d'appel à la fonction `nom_bebe`
du trait `Animal`, mais Rust ne sait pas quelle implémentation utiliser</span>

<!--
Because `Animal::baby_name` doesn’t have a `self` parameter, and there could be
other types that implement the `Animal` trait, Rust can’t figure out which
implementation of `Animal::baby_name` we want. We’ll get this compiler error:
-->

Comme `Animal::nom_bebe` n'a pas de paramètre `self`, et qu'il peut y avoir
d'autres types qui implémentent le trait `Animal`, Rust ne peut pas savoir
quelle implémentation de `Animal::nom_bebe` nous souhaitons utiliser. Nous
obtenons alors cette erreur de compilation :

<!--
```console
{{#include ../listings-sources/ch19-advanced-features/listing-19-20/output.txt}}
```
-->

```console
{{#include ../listings/ch19-advanced-features/listing-19-20/output.txt}}
```

<!--
To disambiguate and tell Rust that we want to use the implementation of
`Animal` for `Dog` as opposed to the implementation of `Animal` for some other
type, we need to use fully qualified syntax. Listing 19-21 demonstrates how to
use fully qualified syntax.
-->

Pour expliquer à Rust que nous souhaitons utiliser l'implémentation de `Animal`
pour `Chien` et non pas l'implémentation de `Animal` pour d'autres types, nous
devons utiliser la syntaxe totalement définie. L'encart 19-21 montre comment
utiliser la syntaxe totalement définie.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-21/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-21/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-21: Using fully qualified syntax to specify
that we want to call the `baby_name` function from the `Animal` trait as
implemented on `Dog`</span>
-->

<span class="caption">Encart 19-21 : utilisation de la syntaxe totalement
définie pour préciser que nous souhaitons appeler la fonction `nom_bebe` du
trait `Animal` tel qu'il est implémenté sur `Chien`</span>

<!--
We’re providing Rust with a type annotation within the angle brackets, which
indicates we want to call the `baby_name` method from the `Animal` trait as
implemented on `Dog` by saying that we want to treat the `Dog` type as an
`Animal` for this function call. This code will now print what we want:
-->

Nous avons donné à Rust une annotation de type entre des chevrons, ce qui
indique que nous souhaitons appeler la méthode `nom_bebe` du trait `Animal`
telle qu'elle est implémentée sur `Chien` en indiquant que nous souhaitons traiter
le type `Chien` comme étant un `Animal` pour cet appel de fonction. Ce code va
désormais afficher ce que nous souhaitons :

<!--
```console
{{#include ../listings-sources/ch19-advanced-features/listing-19-21/output.txt}}
```
-->

```console
{{#include ../listings/ch19-advanced-features/listing-19-21/output.txt}}
```

<!--
In general, fully qualified syntax is defined as follows:
-->

De manière générale, une syntaxe totalement définie est définie comme ceci :

<!--
```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```
-->

```rust,ignore
<Type as Trait>::function(destinataire_si_methode, argument_suivant, ...);
```

<!--
For associated functions that aren’t methods, there would not be a `receiver`:
there would only be the list of other arguments. You could use fully qualified
syntax everywhere that you call functions or methods. However, you’re allowed
to omit any part of this syntax that Rust can figure out from other information
in the program. You only need to use this more verbose syntax in cases where
there are multiple implementations that use the same name and Rust needs help
to identify which implementation you want to call.
-->

Pour les fonctions associées qui ne sont pas des méthodes, il n'y a pas de
`destinataire` : il n'y a qu'une liste d'arguments. Vous pouvez utiliser la
syntaxe totalement définie à n'importe quel endroit où vous faites appel à des
fonctions ou des méthodes. Cependant, vous avez la possibilité de ne pas
renseigner toute partie de cette syntaxe que Rust peut déduire à partir
d'autres informations présentes dans le code. Vous avez seulement besoin
d'utiliser cette syntaxe plus verbeuse dans les cas où il y a plusieurs
implémentations qui utilisent le même nom et que Rust doit être aidé pour
identifier quelle implémentation vous souhaitez appeler.

<!--
### Using Supertraits to Require One Trait’s Functionality Within Another Trait
-->

### Utiliser les supertraits pour utiliser la fonctionnalité d'un trait dans un autre trait

<!--
Sometimes, you might need one trait to use another trait’s functionality. In
this case, you need to rely on the dependent trait also being implemented.
The trait you rely on is a *supertrait* of the trait you’re implementing.
-->

Des fois, vous pourriez avoir besoin d'un trait pour utiliser la fonctionnalité
d'un autre trait. Dans ce cas, vous devez pouvoir compter sur le fait que le
trait dépendant soit bien implémenté. Le trait sur lequel vous comptez est
alors un *supertrait* du trait que vous implémentez.

<!--
For example, let’s say we want to make an `OutlinePrint` trait with an
`outline_print` method that will print a value framed in asterisks. That is,
given a `Point` struct that implements `Display` to result in `(x, y)`, when we
call `outline_print` on a `Point` instance that has `1` for `x` and `3` for
`y`, it should print the following:
-->

Par exemple, imaginons que nous souhaitons créer un trait `OutlinePrint` qui
offre une méthode `outline_print` affichant une valeur entourée d'astérisques.
Ainsi, pour une structure `Point` qui implémente `Display` pour afficher `(x, y)`,
lorsque nous faisons appel à `outline_print` sur une instance de `Point` qui a
`1` pour valeur de `x` et `3` pour `y`, cela devrait afficher ceci :

<!--
```text
**********
*        *
* (1, 3) *
*        *
**********
```
-->

```text
**********
*        *
* (1, 3) *
*        *
**********
```

<!--
In the implementation of `outline_print`, we want to use the `Display` trait’s
functionality. Therefore, we need to specify that the `OutlinePrint` trait will
work only for types that also implement `Display` and provide the functionality
that `OutlinePrint` needs. We can do that in the trait definition by specifying
`OutlinePrint: Display`. This technique is similar to adding a trait bound to
the trait. Listing 19-22 shows an implementation of the `OutlinePrint` trait.
-->

Dans l'implémentation de `outline_print`, nous souhaitons utiliser la
fonctionnalité du trait `Display`. De ce fait, nous devons indiquer que le
trait `OutlinePrint` fonctionnera uniquement pour les types qui auront également
implémenté `Display` et qui fourniront la fonctionnalité dont a besoin
`OutlinePrint`. Nous pouvons faire ceci dans la définition du trait en
renseignant `OutlinePrint: Display`. Cette technique ressemble à l'ajout d'un
trait lié au trait. L'encart 19-22 montre une implémentation du trait
`OutlinePrint`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-22/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-22/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-22: Implementing the `OutlinePrint` trait that
requires the functionality from `Display`</span>
-->

<span class="caption">Encart 19-22 : implémentation du trait `OutlinePrint` qui
nécessite la fonctionnalité offerte par `Display`</span>

<!--
Because we’ve specified that `OutlinePrint` requires the `Display` trait, we
can use the `to_string` function that is automatically implemented for any type
that implements `Display`. If we tried to use `to_string` without adding a
colon and specifying the `Display` trait after the trait name, we’d get an
error saying that no method named `to_string` was found for the type `&Self` in
the current scope.
-->

Comme nous avons précisé que `OutlinePrint` nécessite le trait `Display`, nous
pouvons utiliser la fonction `to_string` qui est automatiquement implémentée
pour n'importe quel type qui implémente `Display`. Si nous avions essayé
d'utiliser `to_string` sans ajouter un double-point et en renseignant le trait
`Display` après le nom du trait, nous aurions alors obtenu une erreur qui nous
informerait qu'il n'y a pas de méthode `to_string` pour le type `&Self` dans la
portée courante.

<!--
Let’s see what happens when we try to implement `OutlinePrint` on a type that
doesn’t implement `Display`, such as the `Point` struct:
-->

Voyons ce qui ce passe lorsque nous essayons d'implémenter `OutlinePrint` sur
un type qui n'implémente pas `Display`, comme c'est le cas de la structure
`Point` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

<!--
We get an error saying that `Display` is required but not implemented:
-->

Nous obtenons une erreur qui dit que `Display` est nécessaire mais n'est pas
implémenté :

<!--
```console
{{#include ../listings-sources/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```
-->

```console
{{#include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

<!--
To fix this, we implement `Display` on `Point` and satisfy the constraint that
`OutlinePrint` requires, like so:
-->

Pour régler cela, nous implémentons `Display` sur `Point` afin de répondre aux
besoins de `OutlinePrint`, comme ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

<!--
Then implementing the `OutlinePrint` trait on `Point` will compile
successfully, and we can call `outline_print` on a `Point` instance to display
it within an outline of asterisks.
-->

Ceci fait, l'implémentation du trait `OutlinePrint` sur `Point` va se
compiler avec succès, et nous pourrons appeler `outline_print` sur une instance
de `Point` pour l'afficher dans le cadre constitué d'astérisques.

<!--
### Using the Newtype Pattern to Implement External Traits on External Types
-->

### Utiliser le motif newtype pour implémenter des traits externes sur des types externes

<!--
In Chapter 10 in the [“Implementing a Trait on a
Type”][implementing-a-trait-on-a-type]<!-- ignore -- > section, we mentioned
the orphan rule that states we’re allowed to implement a trait on a type as
long as either the trait or the type are local to our crate. It’s possible to
get around this restriction using the *newtype pattern*, which involves
creating a new type in a tuple struct. (We covered tuple structs in the
[“Using Tuple Structs without Named Fields to Create Different
Types”][tuple-structs]<!-- ignore -- > section of Chapter 5.) The tuple struct
will have one field and be a thin wrapper around the type we want to implement
a trait for. Then the wrapper type is local to our crate, and we can implement
the trait on the wrapper. *Newtype* is a term that originates from the Haskell
programming language. There is no runtime performance penalty for using this
pattern, and the wrapper type is elided at compile time.
-->

Dans [une section][implementing-a-trait-on-a-type]<!-- ignore --> du chapitre
10, nous avions mentionné la règle de l'orphelin qui énonçait que nous pouvions
implémenter un trait sur un type à condition que le trait ou le type soit
local à notre crate. Il est possible de contourner cette restriction en
utilisant le *motif newtype*, ce qui implique de créer un nouveau type dans une
structure tuple (nous avons vu les structures tuple dans la section
[“Utilisation de structures tuples sans champ nommé pour créer des types différents”][tuple-structs]<!-- ignore -->
du chapitre 5). La structure tuple aura un champ et sera une petite enveloppe
pour le type sur lequel nous souhaitons implémenter le trait. Ensuite, le type
enveloppant est local à notre crate, et nous pouvons lui implémenter un trait.
*Newtype* est un terme qui provient du langage de programmation Haskell. Il n'y
a pas de conséquence sur les performance à l'exécution pour l'utilisation de ce
motif, ce qui signifie que le type enveloppant est résolu à la compilation.

<!--
As an example, let’s say we want to implement `Display` on `Vec<T>`, which the
orphan rule prevents us from doing directly because the `Display` trait and the
`Vec<T>` type are defined outside our crate. We can make a `Wrapper` struct
that holds an instance of `Vec<T>`; then we can implement `Display` on
`Wrapper` and use the `Vec<T>` value, as shown in Listing 19-23.
-->

Comme exemple, disons que nous souhaitons implémenter `Display` sur `Vec<T>`, ce
que la règle de l'orphelin nous empêche de faire directement car le trait
`Display` et le type `Vec<T>` sont définis en dehors de notre crate. Nous
pouvons construire une structure `Enveloppe` qui possède une instance de
`Vec<T>` ; et ensuite nous pouvons implémenter `Display` sur `Enveloppe` et
utiliser la valeur `Vec<T>`, comme dans l'encart 19-23.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-23/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-23/src/main.rs}}
```

<!--
<span class="caption">Listing 19-23: Creating a `Wrapper` type around
`Vec<String>` to implement `Display`</span>
-->

<span class="caption">Encart 19-23 : création d'un type `Enveloppe` autour de
`Vec<String>` pour implémenter `Display`</span>

<!--
The implementation of `Display` uses `self.0` to access the inner `Vec<T>`,
because `Wrapper` is a tuple struct and `Vec<T>` is the item at index 0 in the
tuple. Then we can use the functionality of the `Display` type on `Wrapper`.
-->

L'implémentation de `Display` utilise `self.0` pour accéder à la valeur de
`Vec<T>`, car `Enveloppe` est une structure tuple et `Vec<T>` est l'élément à
l'indice 0 du tuple. Ensuite, nous pouvons utiliser la fonctionnalité du type
`Display` sur `Enveloppe`.

<!--
The downside of using this technique is that `Wrapper` is a new type, so it
doesn’t have the methods of the value it’s holding. We would have to implement
all the methods of `Vec<T>` directly on `Wrapper` such that the methods
delegate to `self.0`, which would allow us to treat `Wrapper` exactly like a
`Vec<T>`. If we wanted the new type to have every method the inner type has,
implementing the `Deref` trait (discussed in Chapter 15 in the [“Treating Smart
Pointers Like Regular References with the `Deref`
Trait”][smart-pointer-deref]<!-- ignore -- > section) on the `Wrapper` to return
the inner type would be a solution. If we don’t want the `Wrapper` type to have
all the methods of the inner type—for example, to restrict the `Wrapper` type’s
behavior—we would have to implement just the methods we do want manually.
-->

Le désavantage d'utiliser cette technique est que `Enveloppe` est un nouveau
type, donc il n'implémente pas toutes les méthodes de la valeur qu'il possède.
Il faudrait implémenter toutes les méthodes de `Vec<T>` directement sur
`Enveloppe` de façon à ce qu'elles délèguent aux méthodes correspondantes de
`self.0`, ce qui nous permettrait d'utiliser `Enveloppe` exactement comme un
`Vec<T>`. Si nous voulions que le nouveau type ait toutes les méthodes du type
qu'il possède, l'implémentation du trait `Deref` (que nous avons vu dans
[une section du chapitre 15][smart-pointer-deref]<!-- ignore -->) sur
`Enveloppe` pour retourner le type interne pourrait être une solution. Si nous
ne souhaitons pas que le type `Enveloppe` ait toutes les méthodes du type qu'il
possède (par exemple, pour limiter les fonctionnalités du type `Enveloppe`),
nous n'avons qu'à implémenter manuellement que les méthodes que nous souhaitons.

<!--
Now you know how the newtype pattern is used in relation to traits; it’s also a
useful pattern even when traits are not involved. Let’s switch focus and look
at some advanced ways to interact with Rust’s type system.
-->

Maintenant vous savez comment le motif newtype est utilisé en lien avec les
traits ; c'est aussi un motif très utile même lorsque les traits ne sont pas
concernés. Changeons de sujet et découvrons d'autres techniques avancées pour
interagir avec le système de type de Rust.

<!-- markdownlint-disable -->
<!--
[newtype]: ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
[implementing-a-trait-on-a-type]:
ch10-02-traits.html#implementing-a-trait-on-a-type
[the-iterator-trait-and-the-next-method]:
ch13-02-iterators.html#the-iterator-trait-and-the-next-method
[traits-defining-shared-behavior]:
ch10-02-traits.html#traits-defining-shared-behavior
[smart-pointer-deref]: ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait
[tuple-structs]: ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
-->
<!-- markdownlint-restore -->

[newtype]: #utiliser-le-motif-newtype-pour-implémenter-des-traits-externes-sur-des-types-externes
[implementing-a-trait-on-a-type]: ch10-02-traits.html
[the-iterator-trait-and-the-next-method]: ch13-02-iterators.html
[traits-defining-shared-behavior]: ch10-02-traits.html
[smart-pointer-deref]: ch15-02-deref.html
[tuple-structs]:
ch05-01-defining-structs.html#utilisation-de-structures-tuples-sans-champ-nommé-pour-créer-des-types-différents
