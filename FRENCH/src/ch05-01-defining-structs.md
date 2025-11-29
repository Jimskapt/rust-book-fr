<!--
## Defining and Instantiating Structs
-->

## Définir et instancier des structures

<!--
Structs are similar to tuples, discussed in [“The Tuple Type”][tuples]<!--
ignore -- > section, in that both hold multiple related values. Like tuples, the
pieces of a struct can be different types. Unlike with tuples, in a struct
you’ll name each piece of data so it’s clear what the values mean. Adding these
names means that structs are more flexible than tuples: you don’t have to rely
on the order of the data to specify or access the values of an instance.
-->

Les structures sont similaires aux tuples, qu'on a vus dans [une section du
chapitre 3][tuples]<!-- ignore -->, car tous les deux portent plusieurs valeurs
associées. Comme pour les tuples, les éléments d'une structure peuvent être de
différents types. Contrairement aux tuples, dans une structure on doit nommer
chaque élément des données afin de clarifier le rôle de chaque valeur. L'ajout
de ces noms fait que les structures sont plus flexibles que les tuples : on n'a
pas à utiliser l'ordre des données pour spécifier ou accéder aux valeurs d'une
instance.

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
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-1: A `User` struct definition</span>
-->

<span class="caption">Encart 5-1 : Définition d'une structure
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
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-2: Creating an instance of the `User`
struct</span>
-->

<span class="caption">Encart 5-2 : Création d'une instance de la structure
`Utilisateur`</span>

<!--
To get a specific value from a struct, we use dot notation. If we wanted
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
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
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
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
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
<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>
### Using the Field Init Shorthand
-->

<a id="utiliser-le-raccourci-dinitialisation-des-champs-lorsque-les-variables-et-les-champs-ont-le-même-nom"></a>

### Utiliser le raccourci d'initialisation des champs

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
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
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
It’s often useful to create a new instance of a struct that includes most of
the values from another instance, but changes some. You can do this using
*struct update syntax*.
-->

Il est souvent utile de créer une nouvelle instance de structure qui comporte
la plupart des valeurs d'une autre instance tout en en changeant certaines.
Vous pouvez utiliser pour cela la *syntaxe de mise à jour de structure*.

<!--
First, in Listing 5-6 we show how to create a new `User` instance in `user2`
regularly, without the update syntax. We set a new value for `email` but
otherwise use the same values from `user1` that we created in Listing 5-2.
-->

Tout d'abord, dans l'encart 5-6 nous montrons comment créer une nouvelle
instance de `Utilisateur` dans `utilisateur2` sans la syntaxe de mise à jour de
structure. On donne de nouvelles valeurs à `email` et `pseudo` mais on utilise
pour les autres champs les mêmes valeurs que dans `utilisateur1` qu'on a créé à
l'encart 5-2.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-6: Creating a new `User` instance using one of
the values from `user1`</span>
-->

<span class="caption">Encart 5-6 : Création d'une nouvelle instance de
`Utilisateur` en utilisant une des valeurs de `utilisateur1`.</span>

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
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-7: Using struct update syntax to set a new
`email` value for a `User` instance but use the rest of the values from
`user1`</span>
-->

<span class="caption">Encart 5-7 : Utilisation de la syntaxe de mise à jour de
structure pour assigner de nouvelles valeurs à `email` d'une nouvelle instance
de `Utilisateur` tout en utilisant les autres valeurs de `utilisateur1`</span>

<!--
The code in Listing 5-7 also creates an instance in `user2` that has a
different value for `email` but has the same values for the `username`,
`active`, and `sign_in_count` fields from `user1`. The `..user1` must come last
to specify that any remaining fields should get their values from the
corresponding fields in `user1`, but we can choose to specify values for as
many fields as we want in any order, regardless of the order of the fields in
the struct’s definition.
-->

Le code dans l'encart 5-7 crée aussi une instance dans `utilisateur2` qui a une
valeur différente pour `email`, mais qui a les mêmes valeurs pour les champs
`pseudo`, `actif` et `nombre_de_connexions` que `utilisateur1`. Le
`..utilisateur1` doit être inséré à la fin pour préciser que tous les champs
restants obtiendront les valeurs des champs correspondants de `utilisateur1`,
mais nous pouvons renseigner les valeurs des champs dans n'importe quel ordre,
peu importe leur position dans la définition de la structure.

<!--
Note that the struct update syntax uses `=` like an assignment; this is
because it moves the data, just as we saw in the [“Ways Variables and Data
Interact: Move”][move]<!-- ignore -- > section. In this example, we can no
longer use `user1` after creating `user2` because the `String` in the
`username` field of `user1` was moved into `user2`. If we had given `user2` new
`String` values for both `email` and `username`, and thus only used the
`active` and `sign_in_count` values from `user1`, then `user1` would still be
valid after creating `user2`. The types of `active` and `sign_in_count` are
types that implement the `Copy` trait, so the behavior we discussed in the
[“Stack-Only Data: Copy”][copy]<!-- ignore -- > section would apply.
-->

Veuillez notez que la syntaxe de la mise à jour de structure utilise un `=`
comme le ferait une assignation ; car cela déplace les données, comme nous
l'avons vu dans [une des sections au chapitre 4][move]<!-- ignore -->. Dans cet
exemple, nous ne pouvons plus utiliser `utilisateur1` après avoir créé
`utilisateur2` car la `String` dans le champ `pseudo` de `utilisateur1` a été
déplacée dans `utilisateur2`. Si nous avions donné des nouvelles valeurs pour
chacune des `String` `email` et `pseudo`, et que par conséquent nous aurions
déplacé uniquement les valeurs de `actif` et de `nombre_de_connexions` à partir
de `utilisateur1`, alors `utilisateur1` resterait en vigueur après la création de
`utilisateur2`. Les types de `actif` et de `nombre_de_connexions` sont des types
qui implémentent le trait `Copy`, donc le comportement décrit dans [la section
à propos de copy][copy]<!-- ignore --> aura lieu ici.

<!--
### Using Tuple Structs without Named Fields to Create Different Types
-->

### Utilisation de structures tuples sans champs nommés pour créer des types différents

<!--
Rust also supports structs that look similar to tuples, called *tuple
structs*. Tuple structs have the added meaning the struct name provides but
don’t have names associated with their fields; rather, they just have the types
of the fields. Tuple structs are useful when you want to give the whole tuple a
name and make the tuple a different type from other tuples, and when naming each
field as in a regular struct would be verbose or redundant.
-->

Rust prend aussi en charge des structures qui ressemblent à des tuples,
appelées *structures tuples*. La signification d'une structure tuple est donnée
par son nom. En revanche, ses champs ne sont pas nommés ; on ne précise que
leurs types. Les structures tuples servent lorsqu'on veut donner un nom à un
tuple pour qu'il soit d'un type différent des autres tuples, et lorsque nommer
chaque champ comme dans une structure classique serait trop verbeux ou
redondant.

<!--
To define a tuple struct, start with the `struct` keyword and the struct name
followed by the types in the tuple. For example, here we define and use
two tuple structs named `Color` and `Point`:
-->

La définition d'une structure tuple commence par le mot-clé `struct` et le nom
de la structure suivis des types des champs du tuple. Par exemple ci-dessous,
nous définissons et utilisons deux structures tuples nommées `Couleur` et
`Point` :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
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
*unit-like structs* because they behave similarly to `()`, the unit type that
we mentioned in [“The Tuple Type”][tuples]<!-- ignore -- > section. Unit-like
structs can be useful when you need to implement a trait on some type but don’t
have any data that you want to store in the type itself. We’ll discuss traits
in Chapter 10. Here’s an example of declaring and instantiating a unit struct
named `AlwaysEqual`:
-->

On peut aussi définir des structures qui n'ont pas de champs ! Cela s'appelle
des *structures unité* parce qu'elles se comportent d'une façon analogue au type
unité, `()`, que nous avons vu dans [la section sur les
tuples][tuples]<!-- ignore -->. Les structures unité sont utiles lorsqu'on doit
implémenter un trait sur un type mais qu'on n'a aucune donnée à stocker dans le
type en lui-même. Nous aborderons les traits au chapitre 10. Voici un exemple
de déclaration et d'instanciation d'une structure unité `ToujoursEgal` :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

<!--
To define `AlwaysEqual`, we use the `struct` keyword, the name we want, then a
semicolon. No need for curly brackets or parentheses! Then we can get an
instance of `AlwaysEqual` in the `subject` variable in a similar way: using the
name we defined, without any curly brackets or parentheses. Imagine that later
we’ll implement behavior for this type such that every instance of
`AlwaysEqual` is always equal to every instance of any other type, perhaps to
have a known result for testing purposes. We wouldn’t need any data to
implement that behavior! You’ll see in Chapter 10 how to define traits and
implement them on any type, including unit-like structs.
-->

Pour définir `ToujoursEgal`, nous utilisons le mot-clé `struct`, puis le nom que
nous voulons lui donner, et enfin un point-virgule. Pas besoin d'accolades ou de
parenthèses ! Ensuite, nous pouvons obtenir une instance de `ToujourEgal` dans
la variable `sujet` de la même manière : utilisez le nom que vous avez défini,
sans aucune accolade ou parenthèse. Imaginez que plus tard nous allons
implémenter un comportement pour ce type pour que toutes les instances de
`ToujourEgal` soient toujours égales à chaque instance de n'importe quel autre
type, peut-être pour avoir un résultat connu pour des besoins de tests. Nous
n'avons besoin d'aucune donnée pour implémenter ce comportement ! Vous verrez
au chapitre 10 comment définir des traits et les implémenter sur n'importe quel
type, y compris sur les structures unité.

<!--
> ### Ownership of Struct Data
>
> In the `User` struct definition in Listing 5-1, we used the owned `String`
> type rather than the `&str` string slice type. This is a deliberate choice
> because we want each instance of this struct to own all of its data and for
> that data to be valid for as long as the entire struct is valid.
>
> It’s also possible for structs to store references to data owned by something
> else, but to do so requires the use of *lifetimes*, a Rust feature that we’ll
> discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct
> is valid for as long as the struct is. Let’s say you try to store a reference
> in a struct without specifying lifetimes, like the following; this won’t work:
>
> <span class="filename">Filename: src/main.rs</span>
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -- >
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
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
> The compiler will complain that it needs lifetime specifiers:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  -- > src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  -- > src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` due to 2 previous errors
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
> caractères `&str`. Il s'agit d'un choix délibéré puisque nous voulons que
> chacune des instances de cette structure possède toutes leurs données et
> que ces données restent valides tant que la structure tout entière est
> valide.
>
> Il est aussi possible pour les structures de stocker des références vers des
> données possédées par autre chose, mais cela nécessiterait d'utiliser des
> *durées de vie*, une fonctionnalité de Rust que nous aborderons au
> chapitre 10. Les durées de vie assurent que les données référencées par une
> structure restent valides tant que la structure l'est aussi. Disons que vous
> essayiez de stocker une référence dans une structure sans indiquer de durées
> de vie, comme ce qui suit, ce qui ne fonctionnera pas :
>
> <span class="filename">Fichier : src/main.rs</span>
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct Utilisateur {
>     actif: bool,
>     pseudo: &str,
>     email: &str,
>     nombre_de_connexions: u64,
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
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     pseudo: &str,
>   |             ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct Utilisateur<'a> {
> 2 |     actif: bool,
> 3 ~     pseudo: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct Utilisateur<'a> {
> 2 |     actif: bool,
> 3 |     pseudo: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` due to 2 previous errors
> ```
>
> Au chapitre 10, nous aborderons la façon de corriger ces erreurs pour qu'on
> puisse stocker des références dans des structures, mais pour le moment, nous
> résoudrons les erreurs comme celles-ci en utilisant des types possédés comme
> `String` plutôt que des références comme `&str`.

<!--
<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -- >
-->

<!--
[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
-->

[tuples]: ch03-02-data-types.html
[move]: ch04-01-what-is-ownership.html
[copy]: ch04-01-what-is-ownership.html
