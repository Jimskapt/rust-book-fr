<!--
## Appendix C: Derivable Traits
-->

## Annexe C : les traits dérivables

<!--
In various places in the book, we’ve discussed the `derive` attribute, which
you can apply to a struct or enum definition. The `derive` attribute generates
code that will implement a trait with its own default implementation on the
type you’ve annotated with the `derive` syntax.
-->

Dans de nombreux endroits du livre, nous avons vu l'attribut `derive`, que vous
pouvez appliquer à une définition de structure ou d'énumération. L'attribut
`derive` génère du code qui va implémenter un trait avec sa propre
implémentation par défaut sur le type que vous avez annoté avec la syntaxe
`derive`.

<!--
In this appendix, we provide a reference of all the traits in the standard
library that you can use with `derive`. Each section covers:
-->

Dans cette annexe, nous allons produire une référence de tous les traits de la
bibliothèque standard que vous pouvez utiliser avec `derive`. Chaque section
va donner :

<!--
* What operators and methods deriving this trait will enable
* What the implementation of the trait provided by `derive` does
* What implementing the trait signifies about the type
* The conditions in which you’re allowed or not allowed to implement the trait
* Examples of operations that require the trait
-->

* Quels opérateurs et méthodes seront activés en dérivant de ce trait
* Ce que fait l'implémentation du trait appliqué par le `derive`
* Ce que l'implémentation du trait implique sur le type concerné
* Les conditions dans lesquelles vous pouvez ou non implémenter le trait
* Des exemples d'opérations qui nécessitent que le trait soit implémenté

<!--
If you want different behavior from that provided by the `derive` attribute,
consult the [standard library documentation](../std/index.html)<!-- ignore -- >
for each trait for details of how to manually implement them.
-->

Si vous souhaitez appliquer un comportement différent de celui fourni par
l'attribut `derive`, consultez [la documentation de la bibliothèque
standard](https://doc.rust-lang.org/std/index.html)<!-- ignore --> pour le trait
concerné afin d'en savoir plus sur son implémentation manuelle.

<!--
The rest of the traits defined in the standard library can’t be implemented on
your types using `derive`. These traits don’t have sensible default behavior,
so it’s up to you to implement them in the way that makes sense for what you’re
trying to accomplish.
-->

Le reste des traits définis dans la bibliothèque standard ne peuvent pas être
implémentés sur des types en utilisant `derive`. Ces traits n'ont pas de
comportement logique par défaut, donc c'est à vous de les implémenter de la
façon la plus appropriée pour ce que vous souhaitez accomplir.

<!--
An example of a trait that can’t be derived is `Display`, which handles
formatting for end users. You should always consider the appropriate way to
display a type to an end user. What parts of the type should an end user be
allowed to see? What parts would they find relevant? What format of the data
would be most relevant to them? The Rust compiler doesn’t have this insight, so
it can’t provide appropriate default behavior for you.
-->

Un exemple de trait qui ne peut pas être dérivé est `Display`, qui permet de
formater la donnée pour les utilisateurs finaux. Vous devez toujours réfléchir
au formatage du type le plus approprié pour un utilisateur final. Quelles
parties d'un type un utilisateur final devrait pouvoir voir ? Sous quelle forme
les données devraient être les plus intéressantes pour eux ? Le compilateur de
Rust n'a pas cette intuition, donc il ne peut pas fournir un comportement par
défaut à votre place.

<!--
The list of derivable traits provided in this appendix is not comprehensive:
libraries can implement `derive` for their own traits, making the list of
traits you can use `derive` with truly open-ended. Implementing `derive`
involves using a procedural macro, which is covered in the
[“Macros”][macros]<!-- ignore -- > section of Chapter 19.
-->

La liste des traits dérivables fournis dans cette annexe n'est pas exhaustive :
les bibliothèques peuvent implémenter `derive` pour leurs propres traits,
étendant potentiellement à l'infini la liste de traits que vous pouvez utiliser
avec `derive`. L'implémentation de `derive` implique l'utilisation d'une macro
procédurale, que nous avons vu dans [une section][macros]<!-- ignore --> du
chapitre 19.

<!--
### `Debug` for Programmer Output
-->

### `Debug` pour l'affichage au développeur

<!--
The `Debug` trait enables debug formatting in format strings, which you
indicate by adding `:?` within `{}` placeholders.
-->

Le trait `Debug` permet le formatage de déboguage pour mettre en forme en tant
que chaînes de caractères, que vous pouvez utiliser en ajoutant `:?` dans un
espace réservé `{}`.

<!--
The `Debug` trait allows you to print instances of a type for debugging
purposes, so you and other programmers using your type can inspect an instance
at a particular point in a program’s execution.
-->

Le trait `Debug` vous permet d'afficher des instances d'un type pour des besoins
de déboguage, afin que vous et les autres développeurs qui utilisent votre type
puissent inspecter une de ses instances à un endroit précis de l'exécution du
programme.

<!--
The `Debug` trait is required, for example, in use of the `assert_eq!` macro.
This macro prints the values of instances given as arguments if the equality
assertion fails so programmers can see why the two instances weren’t equal.
-->

Le trait `Debug` est nécessaire, par exemple, pour l'utilisation de la macro
`assert_eq!`. Cette macro affiche les valeurs des instances passées en argument
dans le cas où l'affirmation échoue afin que le développeur puisse voir pourquoi
les deux instances ne sont pas égales.

<!--
### `PartialEq` and `Eq` for Equality Comparisons
-->

### `PartialEq` et `Eq` pour comparer l'égalité

<!--
The `PartialEq` trait allows you to compare instances of a type to check for
equality and enables use of the `==` and `!=` operators.
-->

Le trait `PartialEq` vous permet de comparer des instances d'un type pour
vérifier leur égalité et permet l'utilisation des opérateurs `==` et `!=`.

<!--
Deriving `PartialEq` implements the `eq` method. When `PartialEq` is derived on
structs, two instances are equal only if *all* fields are equal, and the
instances are not equal if any fields are not equal. When derived on enums,
each variant is equal to itself and not equal to the other variants.
-->

L'application de `derive` avec `PartialEq` implémente la méthode `eq`. Lorsque
`PartialEq` est dérivé sur une structure, deux instances ne peuvent être égales
seulement si *tous* leurs champs sont égaux, et les instances ne sont pas égales
si un des champs n'est pas égal. Lorsque ce trait est dérivé sur une
énumération, chaque variante est égale à elle-même et n'est pas égale aux autres
variantes.

<!--
The `PartialEq` trait is required, for example, with the use of the
`assert_eq!` macro, which needs to be able to compare two instances of a type
for equality.
-->

Le trait `Eq` est nécessaire, par exemple, pour utiliser la macro `assert_eq!`,
qui nécessite de pouvoir comparer l'égalité de deux instances d'un type.

<!--
The `Eq` trait has no methods. Its purpose is to signal that for every value of
the annotated type, the value is equal to itself. The `Eq` trait can only be
applied to types that also implement `PartialEq`, although not all types that
implement `PartialEq` can implement `Eq`. One example of this is floating point
number types: the implementation of floating point numbers states that two
instances of the not-a-number (`NaN`) value are not equal to each other.
-->

Le trait `Eq` n'a pas de méthode. Son rôle est de signaler que pour chaque
valeur du type annoté, la valeur est égale à elle-même. Le trait `Eq` peut
seulement être appliqué sur des types qui implémentent `PartialEq`, bien que
tous les types qui implémentent `PartialEq` ne puissent pas implémenter `Eq`. Un
exemple de ceci sont les types de nombres à virgule flottante : l'implémentation
des nombres à virgule flottante stipule que deux instances ayant la valeur
“not-a-number” (`NaN`, c'est-à-dire “ceci n'est pas un nombre”) ne sont pas
égales entre elles.

<!--
An example of when `Eq` is required is for keys in a `HashMap<K, V>` so the
`HashMap<K, V>` can tell whether two keys are the same.
-->

Par exemple, `Eq` est nécessaire est pour les clés dans un `HashMap<K, V>` afin
que le `HashMap<K, V>` puisse déterminer si deux clés sont identiques.

<!--
### `PartialOrd` and `Ord` for Ordering Comparisons
-->

### `PartialOrd` et `Ord` pour comparer les ordres de grandeur

<!--
The `PartialOrd` trait allows you to compare instances of a type for sorting
purposes. A type that implements `PartialOrd` can be used with the `<`, `>`,
`<=`, and `>=` operators. You can only apply the `PartialOrd` trait to types
that also implement `PartialEq`.
-->

Le trait `PartialOrd` vous permet de comparer des instances d'un type pour
pouvoir les trier. Un type qui implémente `PartialOrd` peut être utilisé avec
les opérateurs `<`, `>`, `<=`, et `>=`. Vous pouvez appliquer uniquement le
trait `PartialOrd` aux types qui implémentent aussi `PartialEq`.

<!--
Deriving `PartialOrd` implements the `partial_cmp` method, which returns an
`Option<Ordering>` that will be `None` when the values given don’t produce an
ordering. An example of a value that doesn’t produce an ordering, even though
most values of that type can be compared, is the not-a-number (`NaN`) floating
point value. Calling `partial_cmp` with any floating point number and the `NaN`
floating point value will return `None`.
-->

L'application de `derive` avec `PartialOrd` implémente la méthode `partial_cmp`,
qui retourne un `Option<Ordering>` qui vaudra `None` lorsque les valeurs
fournies ne fournissent pas un ordre. Un exemple de valeur qui ne produit pas
d'ordre, même si la plupart des valeurs de ce type peuvent être comparées, est
la valeur “not-a-number” (`NaN`) des virgules flottantes. L'appel à
`partial_cmp` entre n'importe quel nombre à virgule flottante et la valeur `NaN`
de virgule flottante va retourner `None`.

<!--
When derived on structs, `PartialOrd` compares two instances by comparing the
value in each field in the order in which the fields appear in the struct
definition. When derived on enums, variants of the enum declared earlier in the
enum definition are considered less than the variants listed later.
-->

Lorsqu'il est dérivé sur une structure, `PartialOrd` compare deux instances en
comparant les valeurs de chaque champ dans l'ordre dans lequel les champs
apparaissent dans la définition de la structure. Lorsqu'il est dérivé sur des
énumérations, les variantes de l'énumération déclarées plus tôt dans la
définition de l'énumération sont considérées inférieures aux variantes déclarées
ensuite.

<!--
The `PartialOrd` trait is required, for example, for the `gen_range` method
from the `rand` crate that generates a random value in the range specified by a
low value and a high value.
-->

Le trait `PartialOrd` est nécessaire, par exemple, pour la méthode `gen_range`
de la crate `rand` qui génère une valeur aléatoire dans l'intervalle contrainte
par une valeur minimale et une valeur maximale.

<!--
The `Ord` trait allows you to know that for any two values of the annotated
type, a valid ordering will exist. The `Ord` trait implements the `cmp` method,
which returns an `Ordering` rather than an `Option<Ordering>` because a valid
ordering will always be possible. You can only apply the `Ord` trait to types
that also implement `PartialOrd` and `Eq` (and `Eq` requires `PartialEq`). When
derived on structs and enums, `cmp` behaves the same way as the derived
implementation for `partial_cmp` does with `PartialOrd`.
-->

Le trait `Ord` vous permet de savoir si un ordre valide existe toujours entre
deux valeurs du type annoté. Le trait `Ord` implémente la méthode `cmp`, qui
retourne un `Ordering` plutôt qu'une `Option<Ordering>` car un ordre valide sera
toujours possible. Vous pouvez appliquer le trait `Ord` uniquement sur les types
qui implémentent aussi `PartialOrd` et `Eq` (et `Eq` nécessite `PartialEq`).
Lorsqu'il est dérivé sur des structures et des énumérations, `cmp` se comporte
de la même manière que l'implémentation de `partial_cmp` dérivée de
`PartialOrd`.

<!--
An example of when `Ord` is required is when storing values in a `BTreeSet<T>`,
a data structure that stores data based on the sort order of the values.
-->

Par exemple, `Ord` doit être implémenté sur le type de valeurs que nous stockons
dans un `BTreeSet<T>`, qui est une structure de donnée qui stocke des données en
fonction de l'ordre de tri de ces valeurs.

<!--
### `Clone` and `Copy` for Duplicating Values
-->

### `Clone` et `Copy` pour dupliquer des valeurs

<!--
The `Clone` trait allows you to explicitly create a deep copy of a value, and
the duplication process might involve running arbitrary code and copying heap
data. See the [“Ways Variables and Data Interact:
Clone”][ways-variables-and-data-interact-clone]<!-- ignore -- > section in
Chapter 4 for more information on `Clone`.
-->

Le trait `Clone` vous permet de créer explicitement une copie profonde d'une
valeur, et le processus de duplication peut impliquer l'exécution d'un code
arbitraire pour copier les données stockées dans le tas. Rendez-vous à la
section [“Les interactions entre les variables et les données : le
déplacement”][ways-variables-and-data-interact-clone]<!-- ignore --> du
chapitre 4 pour plus d'informations sur `Clone`.

<!--
Deriving `Clone` implements the `clone` method, which when implemented for the
whole type, calls `clone` on each of the parts of the type. This means all the
fields or values in the type must also implement `Clone` to derive `Clone`.
-->

Utiliser `derive` avec `Clone` implémente la méthode `clone`, qui, lorsqu'elle
est implémentée sur tout le type, fait appel à `clone` sur chaque constituant du
type. Cela signifie que tous les champs ou les valeurs dans le type doivent
aussi implémenter `Clone` pour dériver de `Clone`.

<!--
An example of when `Clone` is required is when calling the `to_vec` method on a
slice. The slice doesn’t own the type instances it contains, but the vector
returned from `to_vec` will need to own its instances, so `to_vec` calls
`clone` on each item. Thus, the type stored in the slice must implement `Clone`.
-->

`Clone` est par exemple nécessaire lorsque nous appelons la méthode `to_vec` sur
une slice. La slice ne prend pas possession des instances du type qu'il
contient, mais le vecteur retourné par `to_vec` va avoir besoin de prendre
possession de ses instances, donc `to_vec` fait appel à `clone` sur chaque
élément. C'est pourquoi le type stocké dans la slice doit implémenter `Clone`.

<!--
The `Copy` trait allows you to duplicate a value by only copying bits stored on
the stack; no arbitrary code is necessary. See the [“Stack-Only Data:
Copy”][stack-only-data-copy]<!-- ignore -- > section in Chapter 4 for more
information on `Copy`.
-->

Le trait `Copy` vous permet de dupliquer une valeur en copiant uniquement les
éléments stockés sur la pile ; il n'est pas nécessaire d'avoir de code
arbitraire. Rendez-vous à la section [“Données uniquement sur la pile : la
copie”][stack-only-data-copy]<!-- ignore --> du chapitre 4 pour plus
d'informations sur `Copy`.

<!--
The `Copy` trait doesn’t define any methods to prevent programmers from
overloading those methods and violating the assumption that no arbitrary code
is being run. That way, all programmers can assume that copying a value will be
very fast.
-->

Le trait `Copy` ne définit pas de méthode, volontairement pour empêcher les
développeurs de surcharger ces méthodes et ainsi violer l'affirmation qu'aucun
code arbitraire est exécuté à la copie. Ainsi, tous les développeurs peuvent
compter sur le fait qu'une copie de valeur est très rapide.

<!--
You can derive `Copy` on any type whose parts all implement `Copy`. You can
only apply the `Copy` trait to types that also implement `Clone`, because a
type that implements `Copy` has a trivial implementation of `Clone` that
performs the same task as `Copy`.
-->

Vous pouvez utiliser `derive` avec `Copy` sur n'importe quel type constitué
d'éléments qui implémentent aussi `Copy`. Vous ne pouvez appliquer le trait
`Copy` que sur des types qui implémentent aussi `Clone`, car un type qui
implémente `Copy` a aussi une implémentation triviale de `Clone` qui procède
aux mêmes actions que `Copy`.

<!--
The `Copy` trait is rarely required; types that implement `Copy` have
optimizations available, meaning you don’t have to call `clone`, which makes
the code more concise.
-->

Le trait `Copy` est rarement nécessaire ; les types qui implémentent `Copy`
peuvent être optimisés, ce qui veut dire que vous n'avez pas à appeler `clone`,
ce qui rend le code plus concis.

<!--
Everything possible with `Copy` you can also accomplish with `Clone`, but the
code might be slower or have to use `clone` in places.
-->

Tout ce que vous pouvez accomplir avec `Copy`, vous pouvez le faire avec
`Clone`, mais le code risque d'être plus lent ou doit parfois utiliser `clone`.

<!--
### `Hash` for Mapping a Value to a Value of Fixed Size
-->

### `Hash` pour faire correspondre une valeur avec une valeur de taille fixe

<!--
The `Hash` trait allows you to take an instance of a type of arbitrary size and
map that instance to a value of fixed size using a hash function. Deriving
`Hash` implements the `hash` method. The derived implementation of the `hash`
method combines the result of calling `hash` on each of the parts of the type,
meaning all fields or values must also implement `Hash` to derive `Hash`.
-->

Le trait `Hash` vous permet d'obtenir une valeur à taille fixe en utilisant une
fonction de hachage sur une instance d'un type d'une taille quelconque. Utiliser
`derive` avec `Hash` implémente la méthode `hash`. L'implémentation dérive de la
méthode `hash` combine le résultat de l'appel de `hash` sur chaque élément du
type, ce qui signifie que tous ses champs ou valeurs doivent aussi implémenter
`Hash` pour pouvoir lui appliquer le trait `Hash`.

<!--
An example of when `Hash` is required is in storing keys in a `HashMap<K, V>`
to store data efficiently.
-->

Pour stocker des clés efficacement dans un `HashMap<K, V>`, les clés doivent
nécessairement implémenter `Hash`.

<!--
### `Default` for Default Values
-->

### `Default` pour des valeurs par défaut

<!--
The `Default` trait allows you to create a default value for a type. Deriving
`Default` implements the `default` function. The derived implementation of the
`default` function calls the `default` function on each part of the type,
meaning all fields or values in the type must also implement `Default` to
derive `Default`.
-->

Le trait `Default` vous permet de créer une valeur par défaut pour un type.
Implémenter `Default` avec `derive` ajoute la fonction `default`. Cette fonction
`default` fait elle-même appel à la fonction `default` sur chaque élément du
type, ce qui signifie que tous les champs ou les valeurs dans le type doit aussi
implémenter `Default` pour que ce type puisse dériver de `Default`.

<!-- markdownlint-disable -->
<!--
The `Default::default` function is commonly used in combination with the struct
update syntax discussed in the [“Creating Instances From Other Instances With
Struct Update
Syntax”][creating-instances-from-other-instances-with-struct-update-syntax]<!-- ignore -- >
section in Chapter 5. You can customize a few fields of a struct and then
set and use a default value for the rest of the fields by using
`..Default::default()`.
-->


La fonction `Default::default` est couramment utilisé en association avec la
syntaxe de modification de structures que nous avons vu dans la section [“Créer
des instances à partir d'autres instances avec la syntaxe de mise à jour de
structure”][creating-instances-from-other-instances-with-struct-update-syntax]<!-- ignore -->
du chapitre 5. Vous pouvez personnaliser quelques champs d'une structure et
ensuite définir et utiliser une valeur par défaut pour le reste des champs en
utilisant `..Default::default()`.

<!--
The `Default` trait is required when you use the method `unwrap_or_default` on
`Option<T>` instances, for example. If the `Option<T>` is `None`, the method
`unwrap_or_default` will return the result of `Default::default` for the type
`T` stored in the `Option<T>`.
-->

Le trait `Default` est nécessaire lorsque vous utilisez la méthode
`unwrap_or_default` sur les instances de `Option<T>`, par exemple. Si le
`Option<T>` vaut `None`, la méthode `unwrap_or_default` va retourner le résultat
de `Default::default` sur le type `T` provenant du `Option<T>`.

<!--
[creating-instances-from-other-instances-with-struct-update-syntax]:
ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
[stack-only-data-copy]:
ch04-01-what-is-ownership.html#stack-only-data-copy
[ways-variables-and-data-interact-clone]:
ch04-01-what-is-ownership.html#ways-variables-and-data-interact-clone
[macros]: ch19-06-macros.html#macros
-->

[creating-instances-from-other-instances-with-struct-update-syntax]:
ch05-01-defining-structs.html#cr%C3%A9er-des-instances-%C3%A0-partir-dautres-instances-avec-la-syntaxe-de-mise-%C3%A0-jour-de-structure
[stack-only-data-copy]:
ch04-01-what-is-ownership.html#donn%C3%A9es-uniquement-sur-la-pile--la-copie
[ways-variables-and-data-interact-clone]:
ch04-01-what-is-ownership.html#les-interactions-entre-les-variables-et-les-donn%C3%A9es--le-d%C3%A9placement
[macros]: ch19-06-macros.html
<!-- markdownlint-enable -->
