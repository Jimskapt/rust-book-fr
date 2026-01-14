<!--
## Using Trait Objects That Allow for Values of Different Types
-->

## Utiliser les objets traits qui permettent des valeurs de types différents

<!--
In Chapter 8, we mentioned that one limitation of vectors is that they can
store elements of only one type. We created a workaround in Listing 8-10 where
we defined a `SpreadsheetCell` enum that had variants to hold integers, floats,
and text. This meant we could store different types of data in each cell and
still have a vector that represented a row of cells. This is a perfectly good
solution when our interchangeable items are a fixed set of types that we know
when our code is compiled.
-->

Au chapitre 8, nous avions mentionné qu'une limite des vecteurs est qu'ils ne
peuvent stocker des éléments que d'un seul type. Nous avions contourné le
problème dans l'encart 8-10 en définissant une énumération `Cellule` avec des
variantes pouvant contenir des entiers, des flottants et du texte. Ainsi, on
pouvait stocker différents types de données dans chaque cellule et quand même
avoir un vecteur qui représentait une rangée de cellules. C'est une très bonne
solution quand nos éléments interchangeables ne possèdent qu'un ensemble bien
déterminé de types que nous connaissons lors de la compilation de notre code.

<!--
However, sometimes we want our library user to be able to extend the set of
types that are valid in a particular situation. To show how we might achieve
this, we’ll create an example graphical user interface (GUI) tool that iterates
through a list of items, calling a `draw` method on each one to draw it to the
screen—a common technique for GUI tools. We’ll create a library crate called
`gui` that contains the structure of a GUI library. This crate might include
some types for people to use, such as `Button` or `TextField`. In addition,
`gui` users will want to create their own types that can be drawn: for
instance, one programmer might add an `Image` and another might add a
`SelectBox`.
-->

Cependant, nous avons parfois envie que l'utilisateur de notre bibliothèque
puisse étendre l'ensemble des types valides dans une situation donnée. Pour
montrer comment nous pourrions y parvenir, créons un exemple d'outil d'interface
graphique (GUI) qui itère sur une liste d'éléments et appelle une méthode
`afficher` sur chacun d'entre eux pour l'afficher à l'écran — une technique
courante pour les outils d'interface graphique. Créons une *crate* de
bibliothèque appelée `gui` qui contient la structure d'une bibliothèque
d'interface graphique. Cette *crate* pourrait inclure des types que les usagers
pourront utiliser, tels que `Bouton` ou `ChampDeTexte`. De plus, les
utilisateurs de `gui` voudront créer leurs propres types qui pourront être
affichés : par exemple, un développeur pourrait ajouter une `Image` et un autre
pourrait ajouter une `ListeDeroulante`.

<!--
We won’t implement a fully fledged GUI library for this example but will show
how the pieces would fit together. At the time of writing the library, we can’t
know and define all the types other programmers might want to create. But we do
know that `gui` needs to keep track of many values of different types, and it
needs to call a `draw` method on each of these differently typed values. It
doesn’t need to know exactly what will happen when we call the `draw` method,
just that the value will have that method available for us to call.
-->

Nous n'implémenterons pas une véritable bibliothèque d'interface graphique pour
cet exemple, mais nous verrons comment les morceaux pourraient s'assembler. Au
moment d'écrire la bibliothèque, nous ne pouvons pas savoir ni définir tous les
types que les autres développeurs auraient envie de créer. Mais nous savons que
`gui` doit gérer plusieurs valeurs de types différents et qu'elle
doit appeler la méthode `afficher` sur chacune de ces valeurs de types
différents. Elle n'a pas besoin de savoir exactement ce qui arrivera quand on
appellera la méthode `afficher`, mais seulement de savoir que la valeur
disposera de cette méthode que nous pourrons appeler.

<!--
To do this in a language with inheritance, we might define a class named
`Component` that has a method named `draw` on it. The other classes, such as
`Button`, `Image`, and `SelectBox`, would inherit from `Component` and thus
inherit the `draw` method. They could each override the `draw` method to define
their custom behavior, but the framework could treat all of the types as if
they were `Component` instances and call `draw` on them. But because Rust
doesn’t have inheritance, we need another way to structure the `gui` library to
allow users to extend it with new types.
-->

Pour faire ceci dans un langage avec de l'héritage, nous pourrions définir une
classe `Composant` qui a une méthode `afficher`. Les autres
classes, telles que `Bouton`, `Image` et `ListeDeroulante` hériteraient de
`Composant` et hériteraient ainsi de la méthode `afficher`. Elles pourraient
toutes redéfinir la méthode `afficher` avec leur comportement personnalisé,
mais l'environnement de développement pourrait considérer tous les types comme
des instances de `Composant` et appeler `afficher` sur chacun d'entre eux. Mais
puisque Rust n'a pas d'héritage, il nous faut un autre moyen de structurer la
bibliothèque `gui` pour permettre aux utilisateurs de l'enrichir avec de
nouveaux types.

<!--
### Defining a Trait for Common Behavior
-->

### Définir un trait pour du comportement commun

<!--
To implement the behavior we want `gui` to have, we’ll define a trait named
`Draw` that will have one method named `draw`. Then we can define a vector that
takes a *trait object*. A trait object points to both an instance of a type
implementing our specified trait as well as a table used to look up trait
methods on that type at runtime. We create a trait object by specifying some
sort of pointer, such as a `&` reference or a `Box<T>` smart pointer, then the
`dyn` keyword, and then specifying the relevant trait. (We’ll talk about the
reason trait objects must use a pointer in Chapter 19 in the section
[“Dynamically Sized Types and the `Sized` Trait.”][dynamically-sized]<!--
ignore -- >) We can use trait objects in place of a generic or concrete type.
Wherever we use a trait object, Rust’s type system will ensure at compile time
that any value used in that context will implement the trait object’s trait.
Consequently, we don’t need to know all the possible types at compile time.
-->

Pour implémenter le comportement que nous voulons donner à `gui`, nous
définirons un trait nommé `Affichable` qui aura une méthode nommée `afficher`.
Puis nous définirons un vecteur qui prend un *objet trait*. Un objet trait
pointe à la fois vers une instance d'un type implémentant le trait indiqué ainsi
que vers une table utilisée pour chercher les méthodes de trait de ce type à
l'exécution. Nous créons un objet trait en indiquant une sorte de pointeur, tel
qu'une référence `&` ou un pointeur intelligent `Box<T>`, puis le mot-clé `dyn`
et enfin le trait en question. (Nous expliquerons pourquoi les objets traits
doivent utiliser un pointeur dans [une section][dynamically-sized]<!-- ignore -->
du chapitre 19.) Nous pouvons
utiliser des objets traits à la place d'un type générique ou concret. Partout où
nous utilisons un objet trait, le système de types de Rust s'assurera à la
compilation que n'importe quelle valeur utilisée dans ce contexte implémentera
le trait de l'objet trait. Ainsi, il n'est pas nécessaire de connaître tous les
types possibles à la compilation.

<!--
We’ve mentioned that in Rust, we refrain from calling structs and enums
“objects” to distinguish them from other languages’ objects. In a struct or
enum, the data in the struct fields and the behavior in `impl` blocks are
separated, whereas in other languages, the data and behavior combined into one
concept is often labeled an object. However, trait objects *are* more like
objects in other languages in the sense that they combine data and behavior.
But trait objects differ from traditional objects in that we can’t add data to
a trait object. Trait objects aren’t as generally useful as objects in other
languages: their specific purpose is to allow abstraction across common
behavior.
-->

Nous avons mentionné qu'en Rust, nous nous abstenons de qualifier les structures
et énumérations d'*objets* pour les distinguer des objets des autres langages.
Dans une structure ou une énumération, les données dans les champs de la
structure et le comportement dans les blocs `impl` sont séparés, alors que dans
d'autres langages, les données et le comportement se combinent en un concept
souvent qualifié d'objet. En revanche, les objets traits ressemblent davantage
aux objets des autres langages dans le sens où ils combinent des données et du
comportement. Mais les objets traits diffèrent des objets traditionnels dans le
sens où on ne peut pas ajouter des données à un objet trait. Les objets traits
ne sont généralement pas aussi utiles que les objets des autres langages : leur
but spécifique est de permettre de construire des abstractions de comportements
communs.

<!--
Listing 17-3 shows how to define a trait named `Draw` with one method named
`draw`:
-->

L'encart 17-3 illustre la façon de définir un trait nommé `Affichable` avec une
méthode nommée `afficher` :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-03/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-03/src/lib.rs}}
```

<!--
<span class="caption">Listing 17-3: Definition of the `Draw` trait</span>
-->

<span class="caption">Encart 17-3 : Définition du trait `Affichable`</span>

<!--
This syntax should look familiar from our discussions on how to define traits
in Chapter 10. Next comes some new syntax: Listing 17-4 defines a struct named
`Screen` that holds a vector named `components`. This vector is of type
`Box<dyn Draw>`, which is a trait object; it’s a stand-in for any type inside
a `Box` that implements the `Draw` trait.
-->

Cette syntaxe devrait vous rappeler nos discussions concernant la définition des
traits au chapitre 10. Puis vient une nouvelle syntaxe : l'encart 17-4 définit
une structure `Ecran` qui contient un vecteur `composants`. Ce
vecteur est du type `Box<dyn Affichable>`, qui est un objet trait ; c'est un
substitut pour n'importe quel type au sein d'un `Box` qui implémente le trait
`Affichable`.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-04/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-04/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-4: Definition of the `Screen` struct with a
`components` field holding a vector of trait objects that implement the `Draw`
trait</span>
-->

<span class="caption">Encart 17-4 : Définition de la structure `Ecran` avec un
champ `composants` contenant un vecteur d'objets traits qui implémentent le
trait `Affichable`</span>

<!--
On the `Screen` struct, we’ll define a method named `run` that will call the
`draw` method on each of its `components`, as shown in Listing 17-5:
-->

Sur la structure `Ecran`, nous allons définir une méthode nommée `executer` qui
appellera la méthode `afficher` sur chacun de ses `composants`, comme l'illustre
l'encart 17-5 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-05/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-05/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-5: A `run` method on `Screen` that calls the
`draw` method on each component</span>
-->

<span class="caption">Encart 17-5 : Une méthode `executer` sur `Ecran` qui
appelle la méthode `afficher` sur chaque composant</span>

<!--
This works differently from defining a struct that uses a generic type
parameter with trait bounds. A generic type parameter can only be substituted
with one concrete type at a time, whereas trait objects allow for multiple
concrete types to fill in for the trait object at runtime. For example, we
could have defined the `Screen` struct using a generic type and a trait bound
as in Listing 17-6:
-->

Cela ne fonctionne pas de la même manière que d'utiliser une structure avec un
paramètre de type générique avec des traits liés. Un paramètre de type générique
ne peut être remplacé que par un seul type concret à la fois, tandis que les
objets traits permettent à plusieurs types concrets de remplacer l'objet trait à
l'exécution. Par exemple, nous aurions pu définir la structure `Ecran` en
utilisant un type générique et un trait lié comme dans l'encart 17-6 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-06/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-06/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-6: An alternate implementation of the `Screen`
struct and its `run` method using generics and trait bounds</span>
-->

<span class="caption">Encart 17-6 : Une implémentation différente de la
structure `Ecran` et de sa méthode `executer` en utilisant la généricité et les
traits liés</span>

<!--
This restricts us to a `Screen` instance that has a list of components all of
type `Button` or all of type `TextField`. If you’ll only ever have homogeneous
collections, using generics and trait bounds is preferable because the
definitions will be monomorphized at compile time to use the concrete types.
-->

Cela nous restreint à une instance de `Ecran` qui a une liste de composants qui
sont soit tous de type `Bouton`, soit tous de type `ChampDeTexte`. Si vous ne
voulez que des collections homogènes, il est préférable d'utiliser la généricité
et les traits liés parce que les définitions seront monomorphisées à la
compilation pour utiliser les types concrets.

<!--
On the other hand, with the method using trait objects, one `Screen` instance
can hold a `Vec<T>` that contains a `Box<Button>` as well as a
`Box<TextField>`. Let’s look at how this works, and then we’ll talk about the
runtime performance implications.
-->

D'un autre côté, en utilisant des objets traits, une instance de `Ecran` peut
contenir un `Vec<T>` qui contient à la fois un `Box<Bouton>` et un
`Box<ChampDeTexte>`. Regardons comment cela fonctionne, puis nous parlerons
ensuite du coût en performances à l'exécution.

<!--
### Implementing the Trait
-->

### Implémenter le trait

<!--
Now we’ll add some types that implement the `Draw` trait. We’ll provide the
`Button` type. Again, actually implementing a GUI library is beyond the scope
of this book, so the `draw` method won’t have any useful implementation in its
body. To imagine what the implementation might look like, a `Button` struct
might have fields for `width`, `height`, and `label`, as shown in Listing 17-7:
-->

Ajoutons maintenant quelques types qui implémentent le trait `Affichable`. Nous
fournirons le type `Bouton`. Encore une fois, implémenter une vraie bibliothèque
d'interface graphique dépasse la portée de ce livre, alors la méthode `afficher`
n'aura pas d'implémentation utile dans son corps. Pour imaginer à quoi pourrait
ressembler l'implémentation, une structure `Bouton` pourrait avoir des champs
`largeur`, `hauteur` et `libelle`, comme l'illustre l'encart 17-7 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-07/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-07/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-7: A `Button` struct that implements the
`Draw` trait</span>
-->

<span class="caption">Encart 17-7 : Une structure `Bouton` qui implémente le
trait `Affichable`</span>

<!--
The `width`, `height`, and `label` fields on `Button` will differ from the
fields on other components, such as a `TextField` type, that might have those
fields plus a `placeholder` field instead. Each of the types we want to draw on
the screen will implement the `Draw` trait but will use different code in the
`draw` method to define how to draw that particular type, as `Button` has here
(without the actual GUI code, which is beyond the scope of this chapter). The
`Button` type, for instance, might have an additional `impl` block containing
methods related to what happens when a user clicks the button. These kinds of
methods won’t apply to types like `TextField`.
-->

Les champs `largeur`, `hauteur` et `libelle` de `Bouton` pourront ne pas être
les mêmes que ceux d'autres composants, comme un type `ChampDeTexte`, qui
pourrait avoir ces champs plus un champ `texte_de_substitution` à la place.
Chacun des types que nous voudrons afficher à l'écran implémentera le trait
`Affichable` mais utilisera du code différent dans la méthode `afficher` pour
définir comment afficher ce type en particulier, comme c'est le cas de `Bouton`
ici (sans le vrai code d'implémentation, qui dépasse le cadre de ce chapitre).
Le type `Bouton`, par exemple, pourrait avoir un bloc `impl` supplémentaire
contenant des méthodes en lien à ce qui arrive quand un utilisateur clique sur
le bouton. Ce genre de méthodes ne s'applique pas à des types comme
`ChampDeTexte`.

<!--
If someone using our library decides to implement a `SelectBox` struct that has
`width`, `height`, and `options` fields, they implement the `Draw` trait on the
`SelectBox` type as well, as shown in Listing 17-8:
-->

Si un utilisateur de notre bibliothèque décide d'implémenter une structure
`ListeDeroulante` avec des champs `largeur`, `hauteur` et `options`, il
implémentera également le trait `Affichable` sur le type `ListeDeroulante`,
comme dans l'encart 17-8 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-08/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-08/src/main.rs:here}}
```

<!--
<span class="caption">Listing 17-8: Another crate using `gui` and implementing
the `Draw` trait on a `SelectBox` struct</span>
-->

<span class="caption">Encart 17-8 : Une autre *crate* utilisant `gui` et
implémentant le trait `Affichable` sur une structure `ListeDeroulante`</span>

<!--
Our library’s user can now write their `main` function to create a `Screen`
instance. To the `Screen` instance, they can add a `SelectBox` and a `Button`
by putting each in a `Box<T>` to become a trait object. They can then call the
`run` method on the `Screen` instance, which will call `draw` on each of the
components. Listing 17-9 shows this implementation:
-->

L'utilisateur de notre bibliothèque peut maintenant écrire sa fonction `main`
pour créer une instance de `Ecran`. Il peut ajouter à l'instance de `Ecran` une
`ListeDeroulante` ou un `Bouton` en les mettant chacun dans un `Box<T>` pour en
faire des objets traits. Il peut ensuite appeler la méthode `executer` sur
l'instance de `Ecran`, qui appellera `afficher` sur chacun de ses composants.
L'encart 17-9 montre cette implémentation :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-09/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 17-9: Using trait objects to store values of
different types that implement the same trait</span>
-->

<span class="caption">Encart 17-9 : Utilisation d'objets traits pour stocker des
valeurs de types différents qui implémentent le même trait</span>

<!--
When we wrote the library, we didn’t know that someone might add the
`SelectBox` type, but our `Screen` implementation was able to operate on the
new type and draw it because `SelectBox` implements the `Draw` trait, which
means it implements the `draw` method.
-->

Quand nous avons écrit la bibliothèque, nous ne savions pas que quelqu'un
pourrait y ajouter le type `ListeDeroulante`, mais notre implémentation de
`Ecran` a pu opérer sur le nouveau type et l'afficher parce que
`ListeDeroulante` implémente le trait `Affichable`, ce qui veut dire qu'elle
implémente la méthode `afficher`.

<!--
This concept—of being concerned only with the messages a value responds to
rather than the value’s concrete type—is similar to the concept of *duck
typing* in dynamically typed languages: if it walks like a duck and quacks
like a duck, then it must be a duck! In the implementation of `run` on `Screen`
in Listing 17-5, `run` doesn’t need to know what the concrete type of each
component is. It doesn’t check whether a component is an instance of a `Button`
or a `SelectBox`, it just calls the `draw` method on the component. By
specifying `Box<dyn Draw>` as the type of the values in the `components`
vector, we’ve defined `Screen` to need values that we can call the `draw`
method on.
-->

Ce concept — se préoccuper uniquement des messages auxquels une valeur répond
plutôt que du type concret de la valeur — est similaire au concept du *duck
typing* (“typage canard”) dans les langages typés dynamiquement : si ça marche
comme un canard et que ça fait coin-coin comme un canard, alors ça doit être un
canard ! Dans l'implémentation de `executer` sur `Ecran` dans l'encart 17-5,
`executer` n'a pas besoin de connaître le type concret de chaque composant. Elle
ne vérifie pas si un composant est une instance de `Bouton` ou de
`ListeDeroulante`, elle ne fait qu'appeler la méthode `afficher` sur le
composant. En spécifiant `Box<dyn Affichable>` comme type des valeurs dans le
vecteur `composants`, nous avons défini que `Ecran` n'avait besoin que de valeurs
sur lesquelles on peut appeler la méthode `afficher`.

<!--
The advantage of using trait objects and Rust’s type system to write code
similar to code using duck typing is that we never have to check whether a
value implements a particular method at runtime or worry about getting errors
if a value doesn’t implement a method but we call it anyway. Rust won’t compile
our code if the values don’t implement the traits that the trait objects need.
-->

L'avantage d'utiliser les objets traits et le système de types de Rust pour
écrire du code semblable à celui utilisant le *duck typing* est que nous n'avons
jamais besoin de vérifier si une valeur implémente une méthode en particulier à
l'exécution, ni de nous inquiéter d'avoir des erreurs si une valeur n'implémente
pas une méthode mais qu'on l'appelle quand même. Rust ne compilera pas notre
code si les valeurs n'implémentent pas les traits requis par les objets traits.

<!--
For example, Listing 17-10 shows what happens if we try to create a `Screen`
with a `String` as a component:
-->

Par exemple, l'encart 17-10 montre ce qui arrive si on essaie de créer un
`Ecran` avec une `String` comme composant :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-10/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-10/src/main.rs}}
```

<!--
<span class="caption">Listing 17-10: Attempting to use a type that doesn’t
implement the trait object’s trait</span>
-->

<span class="caption">Encart 17-10 : Tentative d'utiliser un type qui
n'implémente pas le trait de l'objet trait</span>

<!--
We’ll get this error because `String` doesn’t implement the `Draw` trait:
-->

Nous aurons cette erreur parce que `String` n'implémente pas le trait
`Affichable` :

<!--
```console
{{#include ../listings-sources/ch17-oop/listing-17-10/output.txt}}
```
-->

```console
{{#include ../listings/ch17-oop/listing-17-10/output.txt}}
```

<!--
This error lets us know that either we’re passing something to `Screen` we
didn’t mean to pass and we should pass a different type or we should implement
`Draw` on `String` so that `Screen` is able to call `draw` on it.
-->

L'erreur nous fait savoir que soit nous passons quelque chose à `Ecran` que nous
ne voulions pas lui passer et que nous devrions lui passer un type différent, soit
nous devrions implémenter `Affichable` sur `String` de sorte que `Ecran` puisse
appeler `afficher` dessus.

<!--
### Trait Objects Perform Dynamic Dispatch
-->

### Les objets traits effectuent de la répartition dynamique

<!--
Recall in the [“Performance of Code Using
Generics”][performance-of-code-using-generics]<!-- ignore -- > section in
Chapter 10 our discussion on the monomorphization process performed by the
compiler when we use trait bounds on generics: the compiler generates
nongeneric implementations of functions and methods for each concrete type
that we use in place of a generic type parameter. The code that results from
monomorphization is doing *static dispatch*, which is when the compiler knows
what method you’re calling at compile time. This is opposed to *dynamic
dispatch*, which is when the compiler can’t tell at compile time which method
you’re calling. In dynamic dispatch cases, the compiler emits code that at
runtime will figure out which method to call.
-->

Rappelez-vous de notre discussion dans [une
section][performance-of-code-using-generics]<!-- ignore --> du chapitre 10 à
propos du processus de monomorphisation effectué par le compilateur quand nous
utilisons des traits liés sur des génériques : le compilateur génère des
implémentations non génériques de fonctions et de méthodes pour chaque type
concret que nous utilisons à la place d'un paramètre de type générique. Le code
résultant de la monomorphisation effectue du *dispatch statique* (*répartition
statique*), qui peut être mis en place quand le compilateur sait, au moment de
la compilation, quelle méthode vous appelez. Cela s'oppose au *dispatch
dynamique* (*répartition dynamique*), qui est mis en place quand le compilateur
ne peut pas déterminer à la compilation quelle méthode vous appelez. Dans le
cas de la répartition dynamique, le compilateur produit du code qui devra
déterminer à l'exécution quelle méthode appeler.

<!--
When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t
know all the types that might be used with the code that is using trait
objects, so it doesn’t know which method implemented on which type to call.
Instead, at runtime, Rust uses the pointers inside the trait object to know
which method to call. There is a runtime cost when this lookup happens that
doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler
from choosing to inline a method’s code, which in turn prevents some
optimizations. However, we did get extra flexibility in the code that we wrote
in Listing 17-5 and were able to support in Listing 17-9, so it’s a trade-off
to consider.
-->

Quand nous utilisons des objets traits, Rust doit utiliser de la répartition
dynamique. Le compilateur ne connaît pas tous les types qui pourraient être
utilisés avec le code qui utilise des objets traits, donc il ne sait pas quelle
méthode implémentée sur quel type il doit appeler. À la place, lors de
l'exécution, Rust utilise les pointeurs à l'intérieur de l'objet trait pour
savoir quelle méthode appeler. Il y a un coût à l'exécution lors de la
recherche de cette méthode qui n'a pas lieu avec la répartition statique. La
répartition dynamique empêche en outre le compilateur de choisir de remplacer
un appel de méthode par le code de cette méthode, ce qui empêche par ricochet
certaines optimisations. Cependant, cela a permis de rendre plus flexible le
code que nous avons écrit dans l'encart 17-5 et que nous avons pu gérer
dans l'encart 17-9, donc c'est un compromis à envisager.

<!--
[performance-of-code-using-generics]:
ch10-01-syntax.html#performance-of-code-using-generics
[dynamically-sized]: ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
-->

[performance-of-code-using-generics]: ch10-01-syntax.html
[dynamically-sized]: ch19-04-advanced-types.html
