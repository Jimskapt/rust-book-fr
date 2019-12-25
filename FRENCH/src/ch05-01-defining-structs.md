<!--
## Defining and Instantiating Structs
-->

## Définir et instancier des structures

<!--
Structs are similar to tuples, which were discussed in Chapter 3. Like tuples,
the pieces of a struct can be different types. Unlike with tuples, you’ll name
each piece of data so it’s clear what the values mean. As a result of these
names, structs are more flexible than tuples: you don’t have to rely on the
order of the data to specify or access the values of an instance.
-->

Les structures sont similaires aux tuples, qu'on a vus au chapitre 3. Comme pour
les tuples, les éléments d'une structure peuvent être de différents types.
Contrairement aux tuples, on doit nommer chaque élément des données afin de
clarifier le rôle de chaque valeur. Grâce à ces noms, les structures sont plus
flexibles que les tuples : on n'a pas à utiliser l'ordre des données pour
spécifier ou accéder aux valeurs d'une instance.

<!--
To define a struct, we enter the keyword `struct` and name the entire struct. A
struct’s name should describe the significance of the pieces of data being
grouped together. Then, inside curly brackets, we define the names and types of
the pieces of data, which we call *fields*. For example, Listing 5-1 shows a
struct that stores information about a user account.
-->

Pour définir une structure, on tape le mot-clé `struct` et on donne un nom à
toute la structure. Le nom d'une structure devrait décrire l'utilisation des
éléments des données regroupés. Ensuite, entre des accolades, on définit le nom
et le type de chaque élément des données, qu'on appelle un *champ*. Par exemple,
l'encart 5-1 montre une structure qui stocke des informations à propos d'un
compte d'utilisateur.

<!--
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
-->

```rust
struct Utilisateur {
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
    actif: bool,
}
```

<!--
<span class="caption">Listing 5-1: A `User` struct definition</span>
-->

<span class="caption">Encart 5-1 : La définition d'une structure
`Utilisateur`</span>

<!--
To use a struct after we’ve defined it, we create an *instance* of that struct
by specifying concrete values for each of the fields. We create an instance by
stating the name of the struct and then add curly brackets containing `key:
value` pairs, where the keys are the names of the fields and the values are the
data we want to store in those fields. We don’t have to specify the fields in
the same order in which we declared them in the struct. In other words, the
struct definition is like a general template for the type, and instances fill
in that template with particular data to create values of the type. For
example, we can declare a particular user as shown in Listing 5-2.
-->

Pour utiliser une structure après l'avoir définie, on crée une *instance* de
cette structure en indiquant des valeurs concrètes pour chacun des champs.
On crée une instance en indiquant le nom de la structure puis en ajoutant des
accolades qui contiennent des paires de `clé: valeur`, où les clés sont les noms
des champs et les valeurs sont les données que l'on souhaite stocker dans ces
champs. Nous n'avons pas à préciser les champs dans le même ordre qu'on les a
déclarés dans la structure. En d'autres termes, la définition de la structure
décrit un gabarit pour le type, et les instances remplissent ce gabarit avec des
données précises pour créer des valeurs de ce type. Par exemple, nous pouvons
déclarer un utilisateur précis comme dans l'encart 5-2.

<!--
```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```
-->

```rust
# struct Utilisateur {
#     pseudo: String,
#     email: String,
#     nombre_de_connexions: u64,
#     actif: bool,
# }
#
let utilisateur1 = Utilisateur {
    email: String::from("quelquun@example.com"),
    pseudo: String::from("pseudoquelconque123"),
    actif: true,
    nombre_de_connexions: 1,
};
```

<!--
<span class="caption">Listing 5-2: Creating an instance of the `User`
struct</span>
-->

<span class="caption">Encart 5-2 : Création d'une instance de la structure
`Utilisateur`</span>

<!--
To get a specific value from a struct, we can use dot notation. If we wanted
just this user’s email address, we could use `user1.email` wherever we wanted
to use this value. If the instance is mutable, we can change a value by using
the dot notation and assigning into a particular field. Listing 5-3 shows how
to change the value in the `email` field of a mutable `User` instance.
-->

Pour obtenir une valeur spécifique depuis une structure, on utilise la notation
avec le point. Si nous voulions seulement l'adresse e-mail de cet utilisateur,
on pourrait utiliser `utilisateur1.email` partout où on voudrait utiliser cette
valeur. Si l'instance est mutable, nous pourrions changer une valeur en
utilisant la notation avec le point et assigner une valeur à ce champ en
particulier. L'encart 5-3 montre comment changer la valeur du champ `email`
d'une instance mutable de `Utilisateur`.

<!--
```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```
-->

```rust
# struct Utilisateur {
#     pseudo: String,
#     email: String,
#     nombre_de_connexions: u64,
#     actif: bool,
# }
#
let mut utilisateur1 = Utilisateur {
    email: String::from("quelquun@example.com"),
    pseudo: String::from("pseudoquelconque123"),
    actif: true,
    nombre_de_connexions: 1,
};

utilisateur1.email = String::from("unautremail@example.com");
```

<!--
<span class="caption">Listing 5-3: Changing the value in the `email` field of a
`User` instance</span>
-->

<span class="caption">Encart 5-3 : Changement de la valeur du champ `email`
d'une instance de `Utilisateur`</span>

<!--
Note that the entire instance must be mutable; Rust doesn’t allow us to mark
only certain fields as mutable. As with any expression, we can construct a new
instance of the struct as the last expression in the function body to
implicitly return that new instance.
-->

À noter que l'instance tout entière doit être mutable ; Rust ne nous permet pas
de marquer seulement certains champs comme mutables. Comme pour toute
expression, nous pouvons construire une nouvelle instance de la structure comme
dernière expression du corps d'une fonction pour retourner implicitement cette
nouvelle instance.

<!--
Listing 5-4 shows a `build_user` function that returns a `User` instance with
the given email and username. The `active` field gets the value of `true`, and
the `sign_in_count` gets a value of `1`.
-->

L'encart 5-4 montre une fonction `creer_utilisateur` qui retourne une instance
de `Utilisateur` avec l'adresse e-mail et le pseudo fournis. Le champ `actif`
prend la valeur `true` et le `nombre_de_connexions` prend la valeur `1`.

<!--
```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```
-->

```rust
# struct Utilisateur {
#     pseudo: String,
#     email: String,
#     nombre_de_connexions: u64,
#     actif: bool,
# }
#
fn creer_utilisateur(email: String, pseudo: String) -> Utilisateur {
    Utilisateur {
        email: email,
        pseudo: pseudo,
        actif: true,
        nombre_de_connexions: 1,
    }
}
```

<!--
<span class="caption">Listing 5-4: A `build_user` function that takes an email
and username and returns a `User` instance</span>
-->

<span class="caption">Encart 5-4 : Une fonction `creer_utilisateur` qui prend
en entrée une adresse e-mail et un pseudo et retourne une instance de
`Utilisateur`</span>

<!--
It makes sense to name the function parameters with the same name as the struct
fields, but having to repeat the `email` and `username` field names and
variables is a bit tedious. If the struct had more fields, repeating each name
would get even more annoying. Luckily, there’s a convenient shorthand!
-->

Il est logique de nommer les paramètres de fonction avec le même nom que les
champs de la structure, mais devoir répéter les noms de variables et de champs
`email` et `pseudo` est un peu pénible. Si la structure avait plus de champs,
répéter chaque nom serait encore plus fatigant. Heureusement, il existe un
raccourci pratique !

<!--
### Using the Field Init Shorthand when Variables and Fields Have the Same Name
-->

### Utiliser le raccourci d'initialisation des champs lorsque les variables et les champs ont le même nom

<!--
Because the parameter names and the struct field names are exactly the same in
Listing 5-4, we can use the *field init shorthand* syntax to rewrite
`build_user` so that it behaves exactly the same but doesn’t have the
repetition of `email` and `username`, as shown in Listing 5-5.
-->

Puisque les noms des paramètres et les noms de champs de la structure sont
exactement les mêmes dans l'encart 5-4, on peut utiliser la syntaxe de
*raccourci d'initialisation des champs* pour réécrire `creer_utilisateur` de
sorte qu'elle se comporte exactement de la même façon sans avoir à répéter
`email` et `pseudo`, comme le montre l'encart 5-5.

<!--
```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```
-->

```rust
# struct Utilisateur {
#     pseudo: String,
#     email: String,
#     nombre_de_connexions: u64,
#     actif: bool,
# }
#
fn creer_utilisateur(email: String, pseudo: String) -> Utilisateur {
    Utilisateur {
        email,
        pseudo,
        actif: true,
        nombre_de_connexions: 1,
    }
}
```

<!--
<span class="caption">Listing 5-5: A `build_user` function that uses field init
shorthand because the `email` and `username` parameters have the same name as
struct fields</span>
-->

<span class="caption">Encart 5-5 : Une fonction `creer_utilisateur` qui utilise
le raccourci d'initialisation des champs parce que les paramètres `email` et
`pseudo` ont le même nom que les champs de la structure</span>

<!--
Here, we’re creating a new instance of the `User` struct, which has a field
named `email`. We want to set the `email` field’s value to the value in the
`email` parameter of the `build_user` function. Because the `email` field and
the `email` parameter have the same name, we only need to write `email` rather
than `email: email`.
-->

Ici, on crée une nouvelle instance de la structure `Utilisateur`, qui possède
un champ nommé `email`. On veut donner au champ `email` la valeur du paramètre
`email` de la fonction `creer_utilisateur`. Comme le champ `email` et le
paramètre `email` ont le même nom, on a uniquement besoin d'écrire `email`
plutôt que `email: email`.

<!--
### Creating Instances From Other Instances With Struct Update Syntax
-->

### Créer des instances à partir d'autres instances avec la syntaxe de mise à jour de structure

<!--
It’s often useful to create a new instance of a struct that uses most of an old
instance’s values but changes some. You’ll do this using *struct update syntax*.
-->

Il est souvent utile de créer une nouvelle instance de structure qui utilise la
plupart des valeurs d'une ancienne instance tout en en changeant certaines. On
utilisera pour cela la *syntaxe de mise à jour de structure*.

<!--
First, Listing 5-6 shows how we create a new `User` instance in `user2` without
the update syntax. We set new values for `email` and `username` but otherwise
use the same values from `user1` that we created in Listing 5-2.
-->

Tout d'abord, l'encart 5-6 nous montre comment créer une nouvelle instance de
`Utilisateur` dans `utilisateur2` sans la syntaxe de mise à jour de structure.
On donne de nouvelles valeurs à `email` et `pseudo` mais on utilise pour les
autres champs les mêmes valeurs que dans `utilisateur1` qu'on a créé à
l'encart 5-2.

<!--
```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```
-->

```rust
# struct Utilisateur {
#     pseudo: String,
#     email: String,
#     nombre_de_connexions: u64,
#     actif: bool,
# }
#
# let utilisateur1 = Utilisateur {
#     email: String::from("quelquun@example.com"),
#     pseudo: String::from("pseudoquelconque123"),
#     actif: true,
#     nombre_de_connexions: 1,
# };
#
let utilisateur2 = Utilisateur {
    email: String::from("quelquundautre@example.com"),
    pseudo: String::from("autrepseudo567"),
    actif: utilisateur1.actif,
    nombre_de_connexions: utilisateur1.nombre_de_connexions,
};
```

<!--
<span class="caption">Listing 5-6: Creating a new `User` instance using some of
the values from `user1`</span>
-->

<span class="caption">Encart 5-6 : Création d'une nouvelle instance de
`Utilisateur` en utilisant certaines valeurs de `utilisateur1`.</span>

<!--
Using struct update syntax, we can achieve the same effect with less code, as
shown in Listing 5-7. The syntax `..` specifies that the remaining fields not
explicitly set should have the same value as the fields in the given instance.
-->

En utilisant la syntaxe de mise à jour de structure, on peut produire le même
résultat avec moins de code, comme le montre l'encart 5-7. La syntaxe `..`
indique que les autres champs auxquels on ne donne pas explicitement de valeur
devraient avoir la même valeur que dans l'instance précisée.

<!--
```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```
-->

```rust
# struct Utilisateur {
#     pseudo: String,
#     email: String,
#     nombre_de_connexions: u64,
#     actif: bool,
# }
#
# let utilisateur1 = Utilisateur {
#     email: String::from("quelquun@example.com"),
#     pseudo: String::from("pseudoquelconque123"),
#     actif: true,
#     nombre_de_connexions: 1,
# };
#
let utilisateur2 = Utilisateur {
    email: String::from("quelquundautre@example.com"),
    pseudo: String::from("autrepseudo567"),
    ..utilisateur1
};
```

<!--
<span class="caption">Listing 5-7: Using struct update syntax to set new
`email` and `username` values for a `User` instance but use the rest of the
values from the fields of the instance in the `user1` variable</span>
-->

<span class="caption">Encart 5-7 : Utilisation de la syntaxe de mise à jour de
structure pour assigner de nouvelles valeurs à `email` et `pseudo` à une
nouvelle instance de `Utilisateur` tout en utilisant les autres valeurs des
champs de l'instance de la variable `utilisateur1`</span>

<!--
The code in Listing 5-7 also creates an instance in `user2` that has a
different value for `email` and `username` but has the same values for the
`active` and `sign_in_count` fields from `user1`.
-->

Le code dans l'encart 5-7 crée aussi une instance dans `utilisateur2` qui a une
valeur différente pour `email` et `pseudo` mais qui a les mêmes valeurs pour les
champs `actif` et `nombre_de_connexions` que `utilisateur1`.

<!--
### Using Tuple Structs without Named Fields to Create Different Types
-->

### Utilisation de structures tuples sans champ nommé pour créer des types différents

<!--
You can also define structs that look similar to tuples, called *tuple
structs*. Tuple structs have the added meaning the struct name provides but
don’t have names associated with their fields; rather, they just have the types
of the fields. Tuple structs are useful when you want to give the whole tuple a
name and make the tuple be a different type from other tuples, and naming each
field as in a regular struct would be verbose or redundant.
-->

On peut aussi définir des structures qui ressemblent à des tuples, appelées
*structures tuples*. La signification d'une structure tuple est donnée par son
nom. En revanche, ses champs ne sont pas nommés ; on ne précise que leurs types.
Les structures tuples servent lorsqu'on veut donner un nom à un tuple pour qu'il
ait un type différent des autres tuples, mais que nommer chaque champ comme dans
une structure classique serait trop verbeux ou redondant.

<!--
To define a tuple struct, start with the `struct` keyword and the struct name
followed by the types in the tuple. For example, here are definitions and
usages of two tuple structs named `Color` and `Point`:
-->

La définition d'une structure tuple commence par le mot-clé `struct` et le nom
de la structure suivis des types des champs du tuple. Par exemple, voici une
définition et une utilisation de deux structures tuples nommées `Couleur` et
`Point` :

<!--
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
-->

```rust
struct Couleur(i32, i32, i32);
struct Point(i32, i32, i32);

let noir = Couleur(0, 0, 0);
let origine = Point(0, 0, 0);
```

<!--
Note that the `black` and `origin` values are different types, because they’re
instances of different tuple structs. Each struct you define is its own type,
even though the fields within the struct have the same types. For example, a
function that takes a parameter of type `Color` cannot take a `Point` as an
argument, even though both types are made up of three `i32` values. Otherwise,
tuple struct instances behave like tuples: you can destructure them into their
individual pieces, you can use a `.` followed by the index to access an
individual value, and so on.
-->

Notez que les valeurs `noir` et `origine` sont de types différents parce que ce
sont des instances de structures tuples différentes. Chaque structure que l'on
définit constitue son propre type, même si les champs au sein de la structure
ont les mêmes types. Par exemple, une fonction qui prend un paramètre de type
`Couleur` ne peut pas prendre un argument de type `Point` à la place, bien que
ces deux types soient tous les deux constitués de trois valeurs `i32`. Mis à
part cela, les instances de stuctures tuples se comportent comme des tuples : on
peut les déstructurer en éléments individuels, on peut utiliser un `.` suivi de
l'indice pour accéder individuellement à une valeur, et ainsi de suite.

<!--
### Unit-Like Structs Without Any Fields
-->

### Les structures unité sans champs

<!--
You can also define structs that don’t have any fields! These are called
*unit-like structs* because they behave similarly to `()`, the unit type.
Unit-like structs can be useful in situations in which you need to implement a
trait on some type but don’t have any data that you want to store in the type
itself. We’ll discuss traits in Chapter 10.
-->

On peut aussi définir des structures qui n'ont pas de champs ! Cela s'appelle
des *structures unité* parce qu'elles se comportent d'une façon analogue au type
unité, `()`. Les structures unité sont utiles lorsqu'on doit implémenter un
trait sur un type mais qu'on n'a aucune donnée à stocker dans le type en
lui-même. Nous aborderons les traits au chapitre 10.

<!--
> ### Ownership of Struct Data
>
> In the `User` struct definition in Listing 5-1, we used the owned `String`
> type rather than the `&str` string slice type. This is a deliberate choice
> because we want instances of this struct to own all of its data and for that
> data to be valid for as long as the entire struct is valid.
>
> It’s possible for structs to store references to data owned by something else,
> but to do so requires the use of *lifetimes*, a Rust feature that we’ll
> discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct
> is valid for as long as the struct is. Let’s say you try to store a reference
> in a struct without specifying lifetimes, like this, which won’t work:
>
> <span class="filename">Filename: src/main.rs</span>
>
> ```rust,ignore,does_not_compile
> struct User {
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
>     active: bool,
> }
>
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
>
> The compiler will complain that it needs lifetime specifiers:
>
> ```text
> error[E0106]: missing lifetime specifier
>  -- >
>   |
> 2 |     username: &str,
>   |               ^ expected lifetime parameter
>
> error[E0106]: missing lifetime specifier
>  -- >
>   |
> 3 |     email: &str,
>   |            ^ expected lifetime parameter
> ```
>
> In Chapter 10, we’ll discuss how to fix these errors so you can store
> references in structs, but for now, we’ll fix errors like these using owned
> types like `String` instead of references like `&str`.
-->

> ### La possession des données d'une structure
>
> Dans la définition de la structure `Utilisateur` de l'encart 5-1, nous avions
> utilisé le type possédé `String` plutôt que le type de *slice* de chaîne de
> caractères `&str`. Il s'agit d'un choix délibéré puisque nous voulons que les
> instances de cette structure possèdent toutes leurs données et que ces données
> restent valides tant que la structure tout entière est valide.
>
> Il est possible pour les structures de stocker des références vers des données
> possédées par autre chose, mais cela nécessiterait d'utiliser des
> *durées de vie*, une fonctionnalité de Rust que nous aborderons au
> chapitre 10. Les durées de vie assurent que les données référencées par une
> structure restent valides tant que la structure l'est aussi. Disons que vous
> essayiez de stocker une référence dans une structure sans indiquer de durées
> de vie, comme ceci, ce qui ne fonctionnera pas :
>
> <span class="filename">Fichier : src/main.rs</span>
>
> ```rust,ignore,does_not_compile
> struct Utilisateur {
>     pseudo: &str,
>     email: &str,
>     nombre_de_connexions: u64,
>     actif: bool,
> }
>
> fn main() {
>     let utilisateur1 = Utilisateur {
>         email: "quelquun@example.com",
>         pseudo: "pseudoquelconque123",
>         actif: true,
>         nombre_de_connexions: 1,
>     };
> }
> ```
>
> Le compilateur réclamera l'ajout des durées de vie :
>
> ```text
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 2 |     username: &str,
>   |               ^ expected lifetime parameter
>
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 3 |     email: &str,
>   |            ^ expected lifetime parameter
> ```
>
> Au chapitre 10, nous aborderons la façon de corriger ces erreurs pour qu'on
> puisse stocker des références dans des structures, mais pour le moment, nous
> résoudrons les erreurs comme celles-ci en utilisant des types possédés comme
> `String` plutôt que des références comme `&str`.
