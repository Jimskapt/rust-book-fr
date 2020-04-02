<!--
## Characteristics of Object-Oriented Languages
-->

## Les caractéristiques des langages orientés objet

<!--
There is no consensus in the programming community about what features a
language must have to be considered object oriented. Rust is influenced by many
programming paradigms, including OOP; for example, we explored the features
that came from functional programming in Chapter 13. Arguably, OOP languages
share certain common characteristics, namely objects, encapsulation, and
inheritance. Let’s look at what each of those characteristics means and whether
Rust supports it.
-->

Il n'y a aucun consensus parmi les programmeurs sur les fonctionnalités qu'un
langage doit avoir pour être considéré orienté objet. Rust est influencé par
beaucoup de paradigmes de programmation, y compris la POO ; par exemple, nous
avons examiné les fonctionnalités issues de la programmation fonctionnelle au
chapitre 13. On peut vraisemblablement dire que les langages orientés objet ont
plusieurs caractéristiques en commun, à savoir les objets, l'encapsulation et
l'héritage. Examinons chacune de ces caractéristiques et déterminons si Rust
les supporte.

<!--
### Objects Contain Data and Behavior
-->

### Les objets contiennent des données et du comportement

<!--
The book *Design Patterns: Elements of Reusable Object-Oriented Software* by
Erich Gamma, Richard Helm, Ralph Johnson, and John Vlissides (Addison-Wesley
Professional, 1994) colloquially referred to as *The Gang of Four* book, is a
catalog of object-oriented design patterns. It defines OOP this way:
-->

Le livre *Design Patterns: Elements of Reusable Object-Oriented Software*
d'Erich Gamma, Richard Helm, Ralph Johnson, et John Vlissides (Addison-Wesley
Professional, 1994) que l'on surnomme le livre du *Gang of Four*, est un
catalogue de patrons de conception orientés objet. Il définit la POO ainsi :

<!--
> Object-oriented programs are made up of objects. An *object* packages both
> data and the procedures that operate on that data. The procedures are
> typically called *methods* or *operations*.
-->

> Les programmes orientés objet sont constitués d'objets. Un *objet* regroupe
> des données ainsi que les procédures qui opèrent sur ces données. Ces
> procédures sont typiquement appelées *méthodes* ou *opérations*.

<!--
Using this definition, Rust is object oriented: structs and enums have data,
and `impl` blocks provide methods on structs and enums. Even though structs and
enums with methods aren’t *called* objects, they provide the same
functionality, according to the Gang of Four’s definition of objects.
-->

Si l'on se tient à cette définition, Rust est orienté objet : les structures et
les énumérations ont des données, et les blocs `impl` leur fournissent des
méthodes. Bien que les structures et les énumérations ne soient pas qualifiées
d'objets, elles en ont les fonctionnalités, selon la définition des objets par
le *Gang of Four*.

<!--
### Encapsulation that Hides Implementation Details
-->

### L'encapsulation qui masque les détails d'implémentation

<!--
Another aspect commonly associated with OOP is the idea of *encapsulation*,
which means that the implementation details of an object aren’t accessible to
code using that object. Therefore, the only way to interact with an object is
through its public API; code using the object shouldn’t be able to reach into
the object’s internals and change data or behavior directly. This enables the
programmer to change and refactor an object’s internals without needing to
change the code that uses the object.
-->

Un autre aspect qu'on associe souvent à la POO est l'idée d'*encapsulation*, ce
qui signifie que les détails d'implémentation d'un objet ne sont pas accessibles
au code utilisant cet objet. Ainsi, la seule façon d'interagir avec un objet est
via son API publique ; le code qui utilise l'objet ne devrait pas pouvoir
accéder aux éléments internes d'un objet et changer directement ses données ou
son comportement. Cela permet au programmeur de changer et remanier les éléments
internes d'un objet sans avoir à changer le code qui utilise cet objet.

<!--
We discussed how to control encapsulation in Chapter 7: we can use the `pub`
keyword to decide which modules, types, functions, and methods in our code
should be public, and by default everything else is private. For example, we
can define a struct `AveragedCollection` that has a field containing a vector
of `i32` values. The struct can also have a field that contains the average of
the values in the vector, meaning the average doesn’t have to be computed
on demand whenever anyone needs it. In other words, `AveragedCollection` will
cache the calculated average for us. Listing 17-1 has the definition of the
`AveragedCollection` struct:
-->

Nous avons abordé la façon de contrôler l'encapsulation au chapitre 7 : on peut
utiliser le mot-clé `pub` pour décider quels modules, types, fonctions et
méthodes de notre code devraient être publics ; par défaut, tout le reste est
privé. Par exemple, nous pouvons définir une structure `CollectionMoyennee` qui
a un champ contenant un vecteur de valeurs `i32`. La structure peut aussi avoir
un champ qui contient la moyenne des valeurs dans le vecteur de sorte qu'il ne
soit pas nécessaire de recalculer la moyenne à chaque fois que quelqu'un en a
besoin. En d'autres termes, `CollectionMoyennee` va mettre en cache la moyenne
calculée pour nous. L'encart 17-1 contient la définition de la structure
`CollectionMoyennee` :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```
-->

```rust
pub struct CollectionMoyennee {
    liste: Vec<i32>,
    moyenne: f64,
}
```

<!--
<span class="caption">Listing 17-1: An `AveragedCollection` struct that
maintains a list of integers and the average of the items in the
collection</span>
-->

<span class="caption">Encart 17-1 : Une structure `CollectionMoyennee` qui
maintient une liste d'entiers et la moyenne des éléments de la collection</span>

<!--
The struct is marked `pub` so that other code can use it, but the fields within
the struct remain private. This is important in this case because we want to
ensure that whenever a value is added or removed from the list, the average is
also updated. We do this by implementing `add`, `remove`, and `average` methods
on the struct, as shown in Listing 17-2:
-->

La structure est marquée `pub` de sorte que d'autres codes puissent l'utiliser,
mais les champs au sein de la structure restent privés. C'est important dans ce
cas puisque nous voulons nous assurer que lorsqu'une valeur est ajoutée ou
retirée dans la liste, la moyenne soit aussi mise à jour. Nous le faisons en
implémentant les méthodes `ajouter`, `retirer` et `moyenne` sur la structure,
comme le montre l'encart 17-2 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
# pub struct AveragedCollection {
#     list: Vec<i32>,
#     average: f64,
# }
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```
-->

```rust
# pub struct CollectionMoyennee {
#     liste: Vec<i32>,
#     moyenne: f64,
# }
impl CollectionMoyennee {
    pub fn ajouter(&mut self, valeur: i32) {
        self.liste.push(valeur);
        self.mettre_a_jour_moyenne();
    }

    pub fn retirer(&mut self) -> Option<i32> {
        let resultat = self.liste.pop();
        match resultat {
            Some(valeur) => {
                self.mettre_a_jour_moyenne();
                Some(valeur)
            },
            None => None,
        }
    }

    pub fn moyenne(&self) -> f64 {
        self.moyenne
    }

    fn mettre_a_jour_moyenne(&mut self) {
        let total: i32 = self.liste.iter().sum();
        self.moyenne = total as f64 / self.liste.len() as f64;
    }
}
```

<!--
<span class="caption">Listing 17-2: Implementations of the public methods
`add`, `remove`, and `average` on `AveragedCollection`</span>
-->

<span class="caption">Encart 17-2 : Implémentations des méthodes publiques
`ajouter`, `retirer` et `moyenne` sur `CollectionMoyennee`</span>

<!--
The public methods `add`, `remove`, and `average` are the only ways to access
or modify data in an instance of `AveragedCollection`. When an item is added
to `list` using the `add` method or removed using the `remove` method, the
implementations of each call the private `update_average` method that handles
updating the `average` field as well.
-->

Les méthodes publiques `ajouter`, `retirer` et `moyenne` sont les seules façons
d'accéder ou de modifier les données d'une instance de `CollectionMoyennee`.
Lorsqu'un élément est ajouté à `liste` en utilisant la méthode `ajouter` ou
retiré en utilisant la méthode `retirer`, l'implémentation de chacune de ces
méthodes appelle la méthode privée `mettre_a_jour_moyenne` qui met à jour le
champ `moyenne` également.

<!--
We leave the `list` and `average` fields private so there is no way for
external code to add or remove items to the `list` field directly; otherwise,
the `average` field might become out of sync when the `list` changes. The
`average` method returns the value in the `average` field, allowing external
code to read the `average` but not modify it.
-->

Nous laissons les champs `liste` et `moyenne` privés pour qu'il soit impossible
pour du code externe d'ajouter ou de retirer des éléments dans notre champ
`liste` directement ; sinon, le champ `moyenne` pourrait ne plus être
synchronisé lorsque la liste change. La méthode `moyenne` renvoie la valeur du
champ `moyenne`, ce qui permet au code externe de lire le champ `moyenne` mais
pas de le modifier.

<!--
Because we’ve encapsulated the implementation details of the struct
`AveragedCollection`, we can easily change aspects, such as the data structure,
in the future. For instance, we could use a `HashSet<i32>` instead of a
`Vec<i32>` for the `list` field. As long as the signatures of the `add`,
`remove`, and `average` public methods stay the same, code using
`AveragedCollection` wouldn’t need to change. If we made `list` public instead,
this wouldn’t necessarily be the case: `HashSet<i32>` and `Vec<i32>` have
different methods for adding and removing items, so the external code would
likely have to change if it were modifying `list` directly.
-->

Puisque nous avons encapsulé les détails d'implémentation de la structure
`CollectionMoyennee`, nous pourrons aisément en changer quelques aspects, tels
que la structure de données, à l'avenir. Par exemple, nous pourrions utiliser
un `HashSet<i32>` plutôt qu'un `Vec<i32>` pour le champ `liste`. Du moment que
les signatures des méthodes publiques `ajouter`, `retirer` et `moyenne` restent
les mêmes, du code qui utilise `CollectionMoyennee` n'aurait pas besoin de
changer. En revanche, si nous avions rendu `liste` public, cela n'aurait pas été
forcément le cas : `HashSet<i32>` et `Vec<i32>` ont des méthodes différentes
pour ajouter et retirer des éléments, donc il aurait vraisemblablement fallu
changer le code externe s'il modifiait `liste` directement.

<!--
If encapsulation is a required aspect for a language to be considered object
oriented, then Rust meets that requirement. The option to use `pub` or not for
different parts of code enables encapsulation of implementation details.
-->

Si l'encapsulation est une condition nécessaire pour qu'un langage soit
considéré orienté objet, alors Rust satisfait cette condition. La possibilité
d'utiliser `pub` ou non pour différentes parties de notre code permet
d'encapsuler les détails d'implémentation.

<!--
### Inheritance as a Type System and as Code Sharing
-->

### L'héritage comme système de type et comme partage de code

<!--
*Inheritance* is a mechanism whereby an object can inherit from another
object’s definition, thus gaining the parent object’s data and behavior without
you having to define them again.
-->

L'*héritage* est un mécanisme selon lequel un objet peut hériter de la
définition d'un autre objet, acquérant ainsi les données et le comportement de
l'objet père sans que l'on ait besoin de les redéfinir.

<!--
If a language must have inheritance to be an object-oriented language, then
Rust is not one. There is no way to define a struct that inherits the parent
struct’s fields and method implementations. However, if you’re used to having
inheritance in your programming toolbox, you can use other solutions in Rust,
depending on your reason for reaching for inheritance in the first place.
-->

Si un langage doit avoir de l'héritage pour être un langage orienté objet, alors
Rust n'en est pas un. Il est impossible de définir une structure qui hérite des
champs et de l'implémentation des méthodes de la structure mère. Cependant, si
vous avez l'habitude d'utiliser l'héritage dans vos programmes, vous pouvez
utiliser d'autres solutions en Rust, selon la raison pour laquelle vous vous
êtes tourné vers l'héritage en premier lieu.

<!--
You choose inheritance for two main reasons. One is for reuse of code: you can
implement particular behavior for one type, and inheritance enables you to
reuse that implementation for a different type. You can share Rust code using
default trait method implementations instead, which you saw in Listing 10-14
when we added a default implementation of the `summarize` method on the
`Summary` trait. Any type implementing the `Summary` trait would have the
`summarize` method available on it without any further code. This is similar to
a parent class having an implementation of a method and an inheriting child
class also having the implementation of the method. We can also override the
default implementation of the `summarize` method when we implement the
`Summary` trait, which is similar to a child class overriding the
implementation of a method inherited from a parent class.
-->

Il y a deux principales raisons de choisir l'héritage. La première raison est la
réutilisation de code : vous pouvez implémenter un comportement particulier pour
un type et l'héritage vous permet de réutiliser cette implémentation pour un
autre type. À la place, vous pouvez partager du code Rust en utilisant des
implémentations de méthodes de trait par défaut, comme nous l'avons vu dans
l'encart 10-14 lorsque nous avons ajouté une implémentation par défaut de la
méthode `resumer` sur le trait `Resumable`. La méthode `resumer` serait alors
disponible sur tout type implémentant le trait `Resumable` sans avoir besoin de
rajouter du code. C'est comme si vous aviez une classe mère avec
l'implémentation d'une méthode et une classe fille avec une autre implémentation
de cette méthode. On peut aussi masquer l'implémentation par défaut de la
méthode `resumer` quand on implémente le trait `Resumable`, un peu comme une
classe fille qui masque l'implémentation d'une méthode héritée d'une classe
mère.

<!--
The other reason to use inheritance relates to the type system: to enable a
child type to be used in the same places as the parent type. This is also
called *polymorphism*, which means that you can substitute multiple objects for
each other at runtime if they share certain characteristics.
-->

L'autre raison d'utiliser l'héritage concerne le système de types : pour
permettre à un type fils d'être utilisé à la place d'un type père. Cela
s'appelle le *polymorphisme*, ce qui veut dire qu'on peut substituer plusieurs
objets entre eux à l'exécution s'ils partagent certaines caractéristiques.

<!--
> ### Polymorphism
>
> To many people, polymorphism is synonymous with inheritance. But it’s
> actually a more general concept that refers to code that can work with data
> of multiple types. For inheritance, those types are generally subclasses.
>
> Rust instead uses generics to abstract over different possible types and
> trait bounds to impose constraints on what those types must provide. This is
> sometimes called *bounded parametric polymorphism*.
-->

> ### Polymorphisme
>
> Pour beaucoup de gens, le polymorphisme est synonyme d'héritage. Mais il
> s'agit en fait d'un principe plus général qui se rapporte au code manipulant
> des données de divers types. Pour l'héritage, ces types sont généralement des
> classes filles (ou *sous-classes*).
>
> À la place, Rust utilise la généricité pour abstraire les différents types et
> traits liés possibles pour imposer des contraintes sur ce que ces types
> doivent fournir. Cela est parfois appelé *polymorphisme paramétrique borné*.

<!--
Inheritance has recently fallen out of favor as a programming design solution
in many programming languages because it’s often at risk of sharing more code
than necessary. Subclasses shouldn’t always share all characteristics of their
parent class but will do so with inheritance. This can make a program’s design
less flexible. It also introduces the possibility of calling methods on
subclasses that don’t make sense or that cause errors because the methods don’t
apply to the subclass. In addition, some languages will only allow a subclass
to inherit from one class, further restricting the flexibility of a program’s
design.
-->

L'héritage est récemment tombé en disgrâce en tant que solution de conception
dans plusieurs langages de programmation parce qu'il conduit souvent à partager
plus de code que nécessaire. Les classes mères ne devraient pas toujours
partager toutes leurs caractéristiques avec leurs classes filles, mais elles y
sont obligées avec l'héritage. Cela peut rendre la conception d'un programme
moins flexible. De plus, cela introduit la possibilité d'appeler des méthodes
sur des classes filles qui n'ont aucun sens ou qui entraînent des erreurs parce
que les méthodes ne s'appliquent pas à la classe fille. De plus, certains
langages ne permettront à une classe fille d'hériter que d'une seule classe, ce
qui restreint d'autant plus la flexibilité de la conception d'un programme.

<!--
For these reasons, Rust takes a different approach, using trait objects instead
of inheritance. Let’s look at how trait objects enable polymorphism in Rust.
-->

Voilà pourquoi Rust prend une autre approche, en utilisant des objets de trait
plutôt que l'héritage. Jetons un œil à la façon dont les objets de trait
permettent le polymorphisme en Rust.
