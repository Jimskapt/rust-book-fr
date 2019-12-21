<!--
## Defining an Enum
-->

## Définir une énumération

<!--
Let’s look at a situation we might want to express in code and see why enums
are useful and more appropriate than structs in this case. Say we need to work
with IP addresses. Currently, two major standards are used for IP addresses:
version four and version six. These are the only possibilities for an IP
address that our program will come across: we can *enumerate* all possible
variants, which is where enumeration gets its name.
-->

Imaginons une situation que nous voudrions exprimer avec du code et regardons
pourquoi les énumérations sont utiles et plus appropriées que les structures
dans ce cas. Disons que nous avons besoin de travailler avec des adresses IP.
Pour le moment, il existe deux normes principales pour les adresses IP : la
version quatre et la version six. Ce seront les seules possibilités d'adresse
IP que notre programme va rencontrer : nous pouvons *énumérer* toutes les
variantes possibles, d'où vient le nom de l'énumération.

<!--
Any IP address can be either a version four or a version six address, but not
both at the same time. That property of IP addresses makes the enum data
structure appropriate, because enum values can only be one of its variants.
Both version four and version six addresses are still fundamentally IP
addresses, so they should be treated as the same type when the code is handling
situations that apply to any kind of IP address.
-->

N'importe quelle adresse IP peut être soit une adresse en version quatre, soit
en version six, mais pas les deux en même temps. Cette propriété des adresses IP
est appropriée à la structure de données d'énumérations, car les valeurs de
l'énumération ne peuvent être qu'une de ses variantes. Les adresses en version
quatre et six sont toujours fondamentalement des adresses IP, donc elles doivent
être traitées comme étant du même type lorsque le code travaille avec des
situations qui s'appliquent à n'importe quelle sorte d'adresse IP.

<!--
We can express this concept in code by defining an `IpAddrKind` enumeration and
listing the possible kinds an IP address can be, `V4` and `V6`. These are the
variants of the enum:
-->

Nous pouvons exprimer ce concept dans le code en définissant une énumération
`SorteAdresseIp` et en listant les différentes sortes possibles d'adresses IP
qu'elle peut avoir, `V4` et `V6`. Ce sont les variantes de l'énumération :

<!--
```rust
enum IpAddrKind {
    V4,
    V6,
}
```
-->

```rust
enum SorteAdresseIp {
    V4,
    V6,
}
```

<!--
`IpAddrKind` is now a custom data type that we can use elsewhere in our code.
-->

`SorteAdresseIp` est maintenant un type de données personnalisé que nous pouvons
utiliser n'importe où dans notre code.

<!--
### Enum Values
-->

### Les valeurs d'énumérations

<!--
We can create instances of each of the two variants of `IpAddrKind` like this:
-->

Nous pouvons créer des instances de chacune des deux variantes de
`SorteAdresseIp` de cette manière :

<!--
```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
-->

```rust
# enum SorteAdresseIp {
#     V4,
#     V6,
# }
#
let quatre = SorteAdresseIp::V4;
let six = SorteAdresseIp::V6;
```

<!--
Note that the variants of the enum are namespaced under its identifier, and we
use a double colon to separate the two. The reason this is useful is that now
both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type:
`IpAddrKind`. We can then, for instance, define a function that takes any
`IpAddrKind`:
-->

Remarquez que les variantes de l'énumération sont dans un espace de nom qui se
situe avant leur nom, et nous utilisons un double deux-points pour les séparer
tous les deux. C'est utile car maintenant les deux valeurs `SorteAdresseIp::V4`
et `SorteAdresseIp::V6` sont du même type : `SorteAdresseIp`. Ensuite, nous
pouvons, par exemple, définir une fonction qui accepte n'importe quelle
`SorteAdresseIp` :

<!--
```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
fn route(ip_kind: IpAddrKind) { }
```
-->

```rust
# enum SorteAdresseIp {
#     V4,
#     V6,
# }
#
fn router(sorte_ip: SorteAdresseIp) { }
```

<!--
And we can call this function with either variant:
-->

Et nous pouvons appeler cette fonction avec chacune des variantes :

<!--
```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
# fn route(ip_kind: IpAddrKind) { }
#
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```
-->

```rust
# enum SorteAdresseIp {
#     V4,
#     V6,
# }
#
# fn router(sorte_ip: SorteAdresseIp) { }
#
router(SorteAdresseIp::V4);
router(SorteAdresseIp::V6);
```

<!--
Using enums has even more advantages. Thinking more about our IP address type,
at the moment we don’t have a way to store the actual IP address *data*; we
only know what *kind* it is. Given that you just learned about structs in
Chapter 5, you might tackle this problem as shown in Listing 6-1.
-->

L'utilisation des énumérations a encore plus d'avantages. En étudiant un peu
plus notre type d'adresse IP, nous constatons que pour le moment, nous ne
pouvons pas stocker *la donnée* de l'adresse IP ; nous savons seulement de
quelle sorte elle est. Avec ce que vous avez appris au chapitre 5, vous pouvez
résoudre ce problème comme dans l'encart 6-1.

<!--
```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```
-->

```rust
enum SorteAdresseIp {
    V4,
    V6,
}

struct AdresseIp {
    sorte: SorteAdresseIp,
    adresse: String,
}

let local = AdresseIp {
    sorte: SorteAdresseIp::V4,
    adresse: String::from("127.0.0.1"),
};

let rebouclage = AdresseIp {
    sorte: SorteAdresseIp::V6,
    adresse: String::from("::1"),
};
```

<!--
<span class="caption">Listing 6-1: Storing the data and `IpAddrKind` variant of
an IP address using a `struct`</span>
-->

<span class="caption">Encart 6-1 : Stockage de la donnée et de la variante de
`SorteAdresseIp` d'une adresse IP en utilisant une `struct`</span>

<!--
Here, we’ve defined a struct `IpAddr` that has two fields: a `kind` field that
is of type `IpAddrKind` (the enum we defined previously) and an `address` field
of type `String`. We have two instances of this struct. The first, `home`, has
the value `IpAddrKind::V4` as its `kind` with associated address data of
`127.0.0.1`. The second instance, `loopback`, has the other variant of
`IpAddrKind` as its `kind` value, `V6`, and has address `::1` associated with
it. We’ve used a struct to bundle the `kind` and `address` values together, so
now the variant is associated with the value.
-->

Ainsi, nous avons défini une structure `AdresseIp` qui a deux champs : un champ
`sorte` qui est du type `SorteAdresseIp` (l'énumération que nous avons définie
précédemment) et un champ `adresse` qui est du type `String`. Nous avons deux
instances de cette structure. La première, `local`, a la valeur
`SorteAdresseIp::V4` pour son champ `sorte`, associé à la donnée d'adresse qui
est `127.0.0.1`. La seconde instance, `rebouclage`, a comme valeur de champ
`sorte` l'autre variante de `SorteAdresseIp`, `V6`, et a l'adresse`::1` qui lui
est associée. Nous avons utilisé une structure pour relier ensemble la `sorte`
et l'`adresse`, donc maintenant la variante est liée à la valeur.

<!--
We can represent the same concept in a more concise way using just an enum,
rather than an enum inside a struct, by putting data directly into each enum
variant. This new definition of the `IpAddr` enum says that both `V4` and `V6`
variants will have associated `String` values:
-->

Nous pouvons appliquer le même principe de manière plus concise en utilisant
uniquement une énumération, plutôt que d'utiliser une énumération dans une
structure, en insérant directement la donnée dans chaque variante de
l'énumération. Cette nouvelle définition de l'énumération `AdresseIp` indique
que chacune des variantes `V4` et `V6` auront des valeurs associées de type
`String` :

<!--
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```
-->

```rust
enum AdresseIp {
    V4(String),
    V6(String),
}

let local = AdresseIp::V4(String::from("127.0.0.1"));

let rebouclage = AdresseIp::V6(String::from("::1"));
```

<!--
We attach data to each variant of the enum directly, so there is no need for an
extra struct.
-->

Nous relions les données de chaque variante directement à l'énumération, donc il
n'est pas nécessaire d'avoir une structure en plus.

<!--
There’s another advantage to using an enum rather than a struct: each variant
can have different types and amounts of associated data. Version four type IP
addresses will always have four numeric components that will have values
between 0 and 255. If we wanted to store `V4` addresses as four `u8` values but
still express `V6` addresses as one `String` value, we wouldn’t be able to with
a struct. Enums handle this case with ease:
-->

Il y a un autre avantage à utiliser une énumération plutôt qu'une structure :
chaque variante peut stocker des types différents, et aussi avoir une quantité
différente de données associées. Les adresses IP version quatre vont toujours
avoir quatre composantes numériques qui auront une valeur entre 0 et 255. Si
nous voulions stocker les adresses `V4` avec quatre valeurs de type `u8` mais
continuer à stocker les adresses `V6` dans une `String`, nous ne pourrions pas
le faire avec une structure. Les énumérations permettent de faire cela
facilement :

<!--
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```
-->

```rust
enum AdresseIp {
    V4(u8, u8, u8, u8),
    V6(String),
}

let local = AdresseIp::V4(127, 0, 0, 1);

let rebouclage = AdresseIp::V6(String::from("::1"));
```

<!--
We’ve shown several different ways to define data structures to store version
four and version six IP addresses. However, as it turns out, wanting to store
IP addresses and encode which kind they are is so common that [the standard
library has a definition we can use!][IpAddr]<!-- ignore -- > Let’s look at how
the standard library defines `IpAddr`: it has the exact enum and variants that
we’ve defined and used, but it embeds the address data inside the variants in
the form of two different structs, which are defined differently for each
variant:
-->

Nous avons vu différentes manières de définir des structures de données pour
enregistrer des adresses IP en version quatre et version six. Cependant, il
s'avère que vouloir stocker des adresses IP et identifier de quelle sorte elles
sont est si fréquent que [la bibliothèque standard a une définition que nous
pouvons utiliser !][IpAddr]<!-- ignore --> Analysons comment la bibliothèque
standard a défini `IpAddr` (l'équivalent de notre `AdresseIp`) : nous retrouvons
la même énumération et les variantes que nous avons définies et utilisées, mais
stocke les données d'adresse dans des variantes dans deux structures
différentes, qui sont définies chacune pour chaque variante :

<!--
[IpAddr]: ../std/net/enum.IpAddr.html
-->

[IpAddr]: https://doc.rust-lang.org/std/net/enum.IpAddr.html

<!--
```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
-->

```rust
struct Ipv4Addr {
    // -- code masqué ici --
}

struct Ipv6Addr {
    // -- code masqué ici --
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

<!--
This code illustrates that you can put any kind of data inside an enum variant:
strings, numeric types, or structs, for example. You can even include another
enum! Also, standard library types are often not much more complicated than
what you might come up with.
-->

Ce code montre comment vous pouvez insérer n'importe quel type de données dans
une variante d'énumération : des chaînes de caractères, des nombres ou des
structures, par exemple. Vous pouvez même y intégrer d'autres énumérations ! Par
ailleurs, les types de la bibliothèque standard ne sont parfois pas plus
compliqués que ce que vous pourriez inventer.

<!--
Note that even though the standard library contains a definition for `IpAddr`,
we can still create and use our own definition without conflict because we
haven’t brought the standard library’s definition into our scope. We’ll talk
more about bringing types into scope in Chapter 7.
-->

Notez aussi que même si la bibliothèque standard embarque une définition de
`IpAddr`, nous pouvons quand même créer et utiliser notre propre définition de
ce type sans avoir de conflit de nom car nous n'avons pas importé cette
définition de la bibliothèque standard dans la portée. Nous verrons plus en
détail comment importer les types dans la portée au chapitre 7.

<!--
Let’s look at another example of an enum in Listing 6-2: this one has a wide
variety of types embedded in its variants.
-->

Analysons un autre exemple d'une énumération dans l'encart 6-2 : celle-ci a une
grande diversité de types dans ses variantes.

<!--
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
-->

```rust
enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}
```

<!--
<span class="caption">Listing 6-2: A `Message` enum whose variants each store
different amounts and types of values</span>
-->

<span class="caption">Encart 6-2 : Une énumération `Message` dont chaque
variante stocke des valeurs de différents types et en différentes
quantités</span>

<!--
This enum has four variants with different types:
-->

Cette énumération a quatre variantes avec des types différents :

<!--
* `Quit` has no data associated with it at all.
* `Move` includes an anonymous struct inside it.
* `Write` includes a single `String`.
* `ChangeColor` includes three `i32` values.
-->

* `Quitter` n'a pas du tout de donnée associée.
* `Deplacer` intègre une structure anonyme.
* `Ecrire` intègre une seule `String`.
* `ChangerCouleur` intègre trois valeurs de type `i32`.

<!--
Defining an enum with variants such as the ones in Listing 6-2 is similar to
defining different kinds of struct definitions, except the enum doesn’t use the
`struct` keyword and all the variants are grouped together under the `Message`
type. The following structs could hold the same data that the preceding enum
variants hold:
-->

Définir une énumération avec des variantes comme celles dans l'encart 6-2
ressemble à la définition de différentes sortes de structures, sauf que
l'énumération n'utilise pas le mot-clé `struct` et que toutes les variantes sont
regroupées ensemble sous le type `Message`. Les structures suivantes peuvent
stocker les mêmes données que celles stockées par les variantes précédentes :

<!--
```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```
-->

```rust
struct MessageQuitter; // une structure unité
struct MessageDeplacer {
    x: i32,
    y: i32,
}
struct MessageEcrire(String); // une structure tuple
struct MessageChangerCouleur(i32, i32, i32); // une structure tuple
```

<!--
But if we used the different structs, which each have their own type, we
couldn’t as easily define a function to take any of these kinds of messages as
we could with the `Message` enum defined in Listing 6-2, which is a single type.
-->

Mais si nous utilisions les différentes structures, qui ont chacune leur propre
type, nous ne pourrions pas définir facilement une fonction qui prend en
paramètre toutes les sortes de messages, tel que nous pourrions le faire avec
l'énumération `Message` que nous avons définie dans l'encart 6-2, qui est un
seul type.

<!--
There is one more similarity between enums and structs: just as we’re able to
define methods on structs using `impl`, we’re also able to define methods on
enums. Here’s a method named `call` that we could define on our `Message` enum:
-->

Il y a un autre point commun entre les énumérations et les structures : tout
comme on peut définir des méthodes sur les structures en utilisant `impl`, on
peut aussi définir des méthodes sur des énumérations. Voici une méthode appelée
`appeler` que nous pouvons définir sur notre énumération `Message` :

<!--
```rust
# enum Message {
#     Quit,
#     Move { x: i32, y: i32 },
#     Write(String),
#     ChangeColor(i32, i32, i32),
# }
#
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```
-->

```rust
# enum Message {
#     Quitter,
#     Deplacer { x: i32, y: i32 },
#     Ecrire(String),
#     ChangerCouleur(i32, i32, i32),
# }
#
impl Message {
    fn appeler(&self) {
        // le corps de la méthode sera défini ici
    }
}

let m = Message::Ecrire(String::from("hello"));
m.appeler();
```

<!--
The body of the method would use `self` to get the value that we called the
method on. In this example, we’ve created a variable `m` that has the value
`Message::Write(String::from("hello"))`, and that is what `self` will be in the
body of the `call` method when `m.call()` runs.
-->

Le corps de la méthode va utiliser `self` pour obtenir la valeur sur laquelle
nous avons utilisé la méthode. Dans cet exemple, nous avons créé une variable
`m` qui a la valeur `Message::Ecrire(String::from("hello"))`, et cela sera ce
que `self` aura comme valeur dans le corps de la méthode `appeler` quand nous
lancerons `m.appeler()`.

<!--
Let’s look at another enum in the standard library that is very common and
useful: `Option`.
-->

Regardons maintenant une autre énumération de la bibliothèque standard qui est
très utilisée et utile : `Option`.

<!--
### The `Option` Enum and Its Advantages Over Null Values
-->

### L'énumération `Option` et ses avantages par rapport à la valeur null

<!--
In the previous section, we looked at how the `IpAddr` enum let us use Rust’s
type system to encode more information than just the data into our program.
This section explores a case study of `Option`, which is another enum defined
by the standard library. The `Option` type is used in many places because it
encodes the very common scenario in which a value could be something or it
could be nothing. Expressing this concept in terms of the type system means the
compiler can check whether you’ve handled all the cases you should be handling;
this functionality can prevent bugs that are extremely common in other
programming languages.
-->

Dans la section précédente, nous avons découvert comment l'énumération
`AdresseIp` nous permet d'utiliser le système de types de Rust pour enregistrer
dans nos programmes encore plus d'informations qu'uniquement la donnée. Cette
section étudie le cas de `Option`, qui est une autre énumération définie dans la
bibliothèque standard. Le type `Option` est utilisé dans de nombreux endroits
car il décrit un scénario très courant où une valeur peut être soit quelque
chose, soit rien du tout. Exprimer ce concept avec le système de types implique
que le compilateur peut vérifier si vous avez géré tous les cas que vous
pourriez rencontrer ; cette fonctionnalité peut éviter des bogues qui sont très
courants dans d'autres langages de programmation.

<!--
Programming language design is often thought of in terms of which features you
include, but the features you exclude are important too. Rust doesn’t have the
null feature that many other languages have. *Null* is a value that means there
is no value there. In languages with null, variables can always be in one of
two states: null or not-null.
-->

La conception d'un langage de programmation est souvent pensée en fonction des
fonctionnalités qu'on inclut, mais les fonctionnalités qu'on refuse sont elles
aussi importantes. Rust n'a pas de fonctionnalité *null* qu'ont de nombreux
langages. *Null* est une valeur qui signifie qu'il n'y a pas de valeur à cet
endroit. Avec les langages qui utilisent null, les variables peuvent toujours
être dans deux états : null ou non null.

<!--
In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony
Hoare, the inventor of null, has this to say:
-->

Dans sa thèse de 2009 “Null References: The Billion Dollar Mistake” (les
références nulles : l'erreur à un milliard de dollars), Tony Hoare, l'inventeur
de null, a écrit ceci :

<!--
> I call it my billion-dollar mistake. At that time, I was designing the first
> comprehensive type system for references in an object-oriented language. My
> goal was to ensure that all use of references should be absolutely safe, with
> checking performed automatically by the compiler. But I couldn’t resist the
> temptation to put in a null reference, simply because it was so easy to
> implement. This has led to innumerable errors, vulnerabilities, and system
> crashes, which have probably caused a billion dollars of pain and damage in
> the last forty years.
-->

> Je l'appelle mon erreur à un milliard de dollars. À cette époque, je concevais
> le premier système de type complet pour des références dans un langage orienté
> objet. Mon objectif était de garantir que toutes les utilisations des
> références soient totalement sûres, et soient vérifiées automatiquement par le
> compilateur. Mais je n'ai pas pu résister à la tentation d'inclure la
> référence nulle, simplement parce que c'était si simple à implémenter. Cela a
> conduit à d'innombrables erreurs, vulnérabilités, et pannes systèmes, qui ont
> probablement causé un milliard de dollars de dommages au cours des quarante
> dernières années.

<!--
The problem with null values is that if you try to use a null value as a
not-null value, you’ll get an error of some kind. Because this null or not-null
property is pervasive, it’s extremely easy to make this kind of error.
-->

Le problème avec les valeurs nulles, c'est que si vous essayez d'utiliser une
valeur nulle comme si elle n'était pas nulle, vous obtiendrez une erreur d'une
façon ou d'une autre. Comme cette propriété nulle ou non nulle est omniprésente,
il est très facile de faire cette erreur.

<!--
However, the concept that null is trying to express is still a useful one: a
null is a value that is currently invalid or absent for some reason.
-->

Cependant, le concept que null essaye d'exprimer reste utile : une valeur nulle
est une valeur qui est actuellement invalide ou absente pour une raison ou une
autre.

<!--
The problem isn’t really with the concept but with the particular
implementation. As such, Rust does not have nulls, but it does have an enum
that can encode the concept of a value being present or absent. This enum is
`Option<T>`, and it is [defined by the standard library][option]<!-- ignore -- >
as follows:
-->

Le problème ne vient pas vraiment du concept, mais de son implémentation. C'est
pourquoi Rust n'a pas de valeurs nulles, mais il a une énumération qui décrit le
concept d'une valeur qui peut être soit présente, soit absente. Cette
énumération est `Option<T>`, et elle est [définie dans la bibliothèque
standard][option]<!-- ignore --> comme ci-dessous :

<!--
[option]: ../std/option/enum.Option.html
-->

[option]: https://doc.rust-lang.org/std/option/enum.Option.html

```rust
enum Option<T> {
    Some(T),
    None,
}
```

<!--
The `Option<T>` enum is so useful that it’s even included in the prelude; you
don’t need to bring it into scope explicitly. In addition, so are its variants:
you can use `Some` and `None` directly without the `Option::` prefix. The
`Option<T>` enum is still just a regular enum, and `Some(T)` and `None` are
still variants of type `Option<T>`.
-->

L'énumération `Option<T>` est tellement utile qu'elle est intégrée dans l'étape
préliminaire ; vous n'avez pas besoin de l'importer explicitement dans la
portée. De plus, voici ses variantes : vous pouvez utiliser directement `Some`
(*quelque chose*) et `None` (*rien*) sans les préfixer par `Option::`.
L'énumération `Option<T>` reste une énumération normale, et `Some(T)` ainsi que
`None` sont toujours des variantes de type `Option<T>`.

<!--
The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a
generic type parameter, and we’ll cover generics in more detail in Chapter 10.
For now, all you need to know is that `<T>` means the `Some` variant of the
`Option` enum can hold one piece of data of any type. Here are some examples of
using `Option` values to hold number types and string types
-->

La syntaxe `<T>` est une fonctionnalité de Rust que nous n'avons pas encore
abordée. Il s'agit d'un paramètre de type générique, et nous verrons la
généricité plus en détail au chapitre 10. Pour le moment, dites-vous que ce
`<T>` signifie que la variante `Some` de l'énumération `Option` peut stocker un
élément de donnée de n'importe quel type. Voici quelques exemples d'utilisation
de valeurs de `Option` pour stocker des types de nombres et des types de chaînes
de caractères :

<!--
```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```
-->

```rust
let un_nombre = Some(5);
let une_chaine = Some("une chaîne");

let nombre_absent: Option<i32> = None;
```

<!--
If we use `None` rather than `Some`, we need to tell Rust what type of
`Option<T>` we have, because the compiler can’t infer the type that the `Some`
variant will hold by looking only at a `None` value.
-->

Si nous utilisons `None` plutôt que `Some`, nous devons indiquer à Rust quel
type de `Option<T>` nous avons, car le compilateur ne peut pas déduire le type
que cette variante `Some` va stocker en considérant uniquement une valeur
`None`.

<!--
When we have a `Some` value, we know that a value is present and the value is
held within the `Some`. When we have a `None` value, in some sense, it means
the same thing as null: we don’t have a valid value. So why is having
`Option<T>` any better than having null?
-->

Lorsque nous avons une valeur `Some`, nous savons que la valeur est présente et
que la valeur est stockée dans le `Some`. Lorsque nous avons une valeur `None`,
en quelque sorte, cela veut dire la même chose que null : nous n'avons pas une
valeur valide. Donc pourquoi obtenir `Option<T>` est meilleur que d'avoir null ?

<!--
In short, because `Option<T>` and `T` (where `T` can be any type) are different
types, the compiler won’t let us use an `Option<T>` value as if it were
definitely a valid value. For example, this code won’t compile because it’s
trying to add an `i8` to an `Option<i8>`:
-->

En bref, comme `Option<T>` et `T` (où `T` représente n'importe quel type) sont
de types différents, le compilateur ne va pas nous autoriser à utiliser une
valeur `Option<T>` comme si cela était bien une valeur valide. Par exemple, le
code suivant ne se compile pas car il essaye d'additionner un `i8` et une
`Option<i8>` :

<!--
```rust,ignore,does_not_compile
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```
-->

```rust,ignore,does_not_compile
let x: i8 = 5;
let y: Option<i8> = Some(5);

let somme = x + y;
```

<!--
If we run this code, we get an error message like this:
-->

Si nous lançons ce code, nous aurons un message d'erreur comme celui-ci :

<!--
```text
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
 -- >
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + std::option::Option<i8>`
  |
```
-->

```text
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
 -- >
  |
5 |     let somme = x + y;
  |                   ^ no implementation for `i8 + std::option::Option<i8>`
  |
```

<!--
Intense! In effect, this error message means that Rust doesn’t understand how
to add an `i8` and an `Option<i8>`, because they’re different types. When we
have a value of a type like `i8` in Rust, the compiler will ensure that we
always have a valid value. We can proceed confidently without having to check
for null before using that value. Only when we have an `Option<i8>` (or
whatever type of value we’re working with) do we have to worry about possibly
not having a value, and the compiler will make sure we handle that case before
using the value.
-->

Intense ! Effectivement, ce message d'erreur signifie que Rust ne comprend pas
comment additionner un `i8` et une `Option<i8>`, car ils sont de types
différents. Quand nous avons une valeur d'un type comme `i8` avec Rust, le
compilateur va s'assurer que nous avons toujours une valeur valide. Nous pouvons
continuer en toute confiance sans avoir à vérifier que cette valeur n'est pas
nulle avant de l'utiliser. Ce n'est que lorsque nous avons une `Option<i8>` (ou
tout autre type de valeur avec lequel nous travaillons) que nous devons nous
inquiéter de ne pas avoir de valeur, et le compilateur va s'assurer que nous
gérons ce cas avant d'utiliser la valeur.

<!--
In other words, you have to convert an `Option<T>` to a `T` before you can
perform `T` operations with it. Generally, this helps catch one of the most
common issues with null: assuming that something isn’t null when it actually
is.
-->

Autrement dit, vous devez convertir une `Option<T>` en `T` pour pouvoir faire
avec elle des opérations du type `T`. Généralement, cela permet de résoudre l'un
des problèmes les plus courants avec null : supposer qu'une valeur n'est pas
nulle alors qu'en réalité, elle l'est.

<!--
Not having to worry about incorrectly assuming a not-null value helps you to be
more confident in your code. In order to have a value that can possibly be
null, you must explicitly opt in by making the type of that value `Option<T>`.
Then, when you use that value, you are required to explicitly handle the case
when the value is null. Everywhere that a value has a type that isn’t an
`Option<T>`, you *can* safely assume that the value isn’t null. This was a
deliberate design decision for Rust to limit null’s pervasiveness and increase
the safety of Rust code.
-->

Ne pas avoir à s'inquiéter que des valeurs nulles puissent être mal gérées vous
aide à être plus confiant en votre code. Pour avoir une valeur qui peut
potentiellement être nulle, vous devez l'indiquer explicitement en déclarant que
le type de cette valeur est `Option<T>`. Ensuite, quand vous utiliserez cette
valeur, il vous faudra gérer explicitement le cas où cette valeur est nulle. Si
vous utilisez une valeur qui n'est pas une `Option<T>`, alors vous *pouvez*
considérer que cette valeur ne sera jamais nulle sans prendre de risques. Il
s'agit d'un choix de conception délibéré de Rust pour limiter l'omniprésence de
null et augmenter la sécurité du code en Rust.

<!--
So, how do you get the `T` value out of a `Some` variant when you have a value
of type `Option<T>` so you can use that value? The `Option<T>` enum has a large
number of methods that are useful in a variety of situations; you can check
them out in [its documentation][docs]<!-- ignore -- >. Becoming familiar with
the methods on `Option<T>` will be extremely useful in your journey with Rust.
-->

Donc, comment récupérer la valeur de type `T` d'une variante `Some` quand vous
avez une valeur de type `Option<T>` afin de l'utiliser ? L'énumération
`Option<T>` a un large choix de méthodes qui sont plus ou moins utiles selon les
cas ; vous pouvez les découvrir dans [sa documentation][docs]<!-- ignore -->. Se
familiariser avec les méthodes de `Option<T>` peut être très utile dans votre
aventure avec Rust.

<!--
[docs]: ../std/option/enum.Option.html
-->

[docs]: https://doc.rust-lang.org/std/option/enum.Option.html

<!--
In general, in order to use an `Option<T>` value, you want to have code that
will handle each variant. You want some code that will run only when you have a
`Some(T)` value, and this code is allowed to use the inner `T`. You want some
other code to run if you have a `None` value, and that code doesn’t have a `T`
value available. The `match` expression is a control flow construct that does
just this when used with enums: it will run different code depending on which
variant of the enum it has, and that code can use the data inside the matching
value.
-->

De manière générale, pour pouvoir utiliser une valeur de `Option<T>`, votre code
doit gérer chaque variante. On veut que du code soit exécuté uniquement quand on
a une valeur `Some(T)`, et que ce code soit autorisé à utiliser la valeur de
type `T` à l'intérieur. On veut aussi qu'un autre code soit exécuté si on a une
valeur `None`, et ce code n'aura pas de valeur de type `T` de disponible.
L'expression `match` est une structure de contrôle qui fait bien ceci
lorsqu'elle est utilisée avec les énumérations : elle va exécuter du code
différent en fonction de quelle variante de l'énumération elle obtient, et ce
code pourra utiliser la donnée présente dans la valeur correspondante.
