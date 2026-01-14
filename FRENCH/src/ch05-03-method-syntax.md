<!--
## Method Syntax
-->

## La syntaxe des méthodes

<!--
*Methods* are similar to functions: we declare them with the `fn` keyword and a
name, they can have parameters and a return value, and they contain some code
that’s run when the method is called from somewhere else. Unlike functions,
methods are defined within the context of a struct (or an enum or a trait
object, which we cover in Chapters 6 and 17, respectively), and their first
parameter is always `self`, which represents the instance of the struct the
method is being called on.
-->

Les *méthodes* sont similaires aux fonctions : nous les déclarons avec le
mot-clé `fn` et un nom, elles peuvent avoir des paramètres et une valeur de
retour, et elles contiennent du code qui est exécuté quand la méthode est
appellée depuis un autre endroit. Contrairement aux fonctions, les méthodes
diffèrent des fonctions parce qu'elles sont définies dans le contexte d'une
structure (ou d'une énumération ou d'un objet de trait, que nous aborderons
respectivement aux chapitres 6 et 17) et que leur premier paramètre est
toujours `self`, un mot-clé qui représente l'instance de la structure sur
laquelle on appelle la méthode.

<!--
### Defining Methods
-->

### Définir des méthodes

<!--
Let’s change the `area` function that has a `Rectangle` instance as a parameter
and instead make an `area` method defined on the `Rectangle` struct, as shown
in Listing 5-13.
-->

Remplaçons la fonction `aire` qui prend une instance de `Rectangle` en paramètre
par une méthode `aire` définie sur la structure `Rectangle`, comme dans
l'encart 5-13.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<!--
<span class="caption">Listing 5-13: Defining an `area` method on the
`Rectangle` struct</span>
-->

<span class="caption">Encart 5-13 : Définition d'une méthode `aire` sur la
structure `Rectangle`</span>

<!--
To define the function within the context of `Rectangle`, we start an `impl`
(implementation) block for `Rectangle`. Everything within this `impl` block
will be associated with the `Rectangle` type. Then we move the `area` function
within the `impl` curly brackets and change the first (and in this case, only)
parameter to be `self` in the signature and everywhere within the body. In
`main`, where we called the `area` function and passed `rect1` as an argument,
we can instead use *method syntax* to call the `area` method on our `Rectangle`
instance. The method syntax goes after an instance: we add a dot followed by
the method name, parentheses, and any arguments.
-->

Pour définir la fonction dans le contexte de `Rectangle`, nous démarrons un bloc
`impl` (*implémentation*) pour `Rectangle`. Tout ce qui sera dans ce bloc `impl`
sera lié au type `Rectangle`. Puis nous déplaçons la fonction `aire` entre les
accolades du `impl` et nous remplaçons le premier paramètre (et dans notre cas,
le seul) par `self` dans la signature et dans tout le corps. Dans `main`, où
nous avons appelé la fonction `aire` et passé `rect1` en argument, nous pouvons
utiliser à la place la *syntaxe des méthodes* pour appeler la méthode `aire` sur
notre instance de `Rectangle`. La syntaxe des méthodes se place après
l'instance : on ajoute un point suivi du nom de la méthode et des parenthèses
contenant les arguments s'il y en a.

<!--
In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle`.
The `&self` is actually short for `self: &Self`. Within an `impl` block, the
type `Self` is an alias for the type that the `impl` block is for. Methods must
have a parameter named `self` of type `Self` for their first parameter, so Rust
lets you abbreviate this with only the name `self` in the first parameter spot.
Note that we still need to use the `&` in front of the `self` shorthand to
indicate this method borrows the `Self` instance, just as we did in `rectangle:
&Rectangle`. Methods can take ownership of `self`, borrow `self` immutably as
we’ve done here, or borrow `self` mutably, just as they can any other parameter.
-->

Dans la signature de `aire`, nous utilisons `&self` à la place de
`rectangle: &Rectangle`. Le `&self` est un raccourci pour `self: &Self`. Au
sein d'un bloc `impl`, le type de `Self` est un alias pour le type sur lequel
porte le `impl`. Les méthodes doivent avoir un paramètre `self` du type `Self`
comme premier paramètre afin que Rust puisse vous permettre d'abréger en
renseignant uniquement `self` en premier paramètre. Veuillez noter qu'il nous
faut quand même utiliser le `&` devant le raccourci `self`, pour indiquer que
cette méthode emprunte l'instance de `Self`, comme nous l'avions fait pour
`rectangle: &Rectangle`. Les méthodes peuvent prendre possession de `self`,
emprunter `self` de façon immuable comme nous l'avons fait ici, ou emprunter
`self` de façon mutable, comme pour n'importe quel autre paramètre.

<!--
We’ve chosen `&self` here for the same reason we used `&Rectangle` in the
function version: we don’t want to take ownership, and we just want to read the
data in the struct, not write to it. If we wanted to change the instance that
we’ve called the method on as part of what the method does, we’d use `&mut
self` as the first parameter. Having a method that takes ownership of the
instance by using just `self` as the first parameter is rare; this technique is
usually used when the method transforms `self` into something else and you want
to prevent the caller from using the original instance after the transformation.
-->

Nous avons choisi `&self` ici pour la même raison que nous avions utilisé
`&Rectangle` quand il s'agissait d'une fonction ; nous ne voulons pas en prendre
possession, et nous voulons seulement lire les données de la structure, pas les
modifier. Si nous voulions que la méthode modifie l'instance sur laquelle on
l'appelle, on utiliserait `&mut self` comme premier paramètre. Il est rare
d'avoir une méthode qui prend possession de l'instance en utilisant uniquement
`self` comme premier argument ; cette technique est généralement utilisée
lorsque la méthode transforme `self` en quelque chose d'autre et que vous voulez
empêcher le code appelant d'utiliser l'instance d'origine après la
transformation.

<!--
The main reason for using methods instead of functions, in addition to providing
method syntax and not having to repeat the type of `self` in every method’s
signature, is for organization. We’ve put all the things we can do with an
instance of a type in one `impl` block rather than making future users of our
code search for capabilities of `Rectangle` in various places in the library we
provide.
-->

En complément de l'application de la syntaxe des méthodes et ainsi de ne pas
être obligé de répéter le type de `self` dans la signature de chaque méthode,
la principale raison d'utiliser les méthodes plutôt que de fonctions est pour
l'organisation. Nous avons mis tout ce qu'on pouvait faire avec une instance de
notre type dans un bloc `impl` plutôt que d'imposer aux futurs utilisateurs de
notre code à rechercher les fonctionnalités de `Rectangle` à divers endroits de
la bibliothèque que nous fournissons.

<!--
Note that we can choose to give a method the same name as one of the struct’s
fields. For example, we can define a method on `Rectangle` also named `width`:
-->

Notez que nous pourions faire en sorte qu'une méthode porte le même nom qu'un
des champs de la structure. Par exemple, nous pourions définir une méthode sur
`Rectangle` qui s'appelle elle aussi `largeur` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

<!--
Here, we’re choosing to make the `width` method return `true` if the value in
the instance’s `width` field is greater than 0, and `false` if the value is 0:
we can use a field within a method of the same name for any purpose. In `main`,
when we follow `rect1.width` with parentheses, Rust knows we mean the method
`width`. When we don’t use parentheses, Rust knows we mean the field `width`.
-->

Ici, nous avons défini la méthode `largeur` pour qu'elle retourne `true` si la
valeur dans le champ `largeur` est supérieure à 0, et `false` si la
valeur est 0 : nous pouvons utiliser un champ à l'intérieur d'une méthode du
même nom, pour n'importe quel usage. Dans le `main`, lorsque nous ajoutons des
parenthèses après `rect1.largeur`, Rust comprend que nous parlons de la méthode
`largeur`. Lorsque nous n'utilisons pas les parenthèses, Rust sait nous parlons
du champ `largeur`.

<!--
Often, but not always, when we give methods with the same name as a field we
want it to only return the value in the field and do nothing else. Methods like
this are called *getters*, and Rust does not implement them automatically for
struct fields as some other languages do. Getters are useful because you can
make the field private but the method public and thus enable read-only access
to that field as part of the type’s public API. We will be discussing what
public and private are and how to designate a field or method as public or
private in Chapter 7.
-->

Souvent, mais pas toujours, lorsque nous appellons une méthode avec le même nom
qu'un champ, nous voulons qu'elle renvoie uniquement la valeur de ce champ et
ne fasse rien d'autre. Ces méthodes sont appelées des *accesseurs*, et Rust ne
les implémente pas automatiquement pour les champs des structures comme le font
certains langages. Les accesseurs sont utiles pour rendre le champ privé mais
rendre la méthode publique et ainsi donner un accès en lecture seule à ce champ
dans l'API publique de ce type. Nous développerons les notions de publique et
privé et comment définir un champ ou une méthode publique ou privée au
chapitre 7.

<!--
> ### Where’s the `->` Operator?
>
> In C and C++, two different operators are used for calling methods: you use
> `.` if you’re calling a method on the object directly and `->` if you’re
> calling the method on a pointer to the object and need to dereference the
> pointer first. In other words, if `object` is a pointer,
> `object->something()` is similar to `(*object).something()`.
>
> Rust doesn’t have an equivalent to the `->` operator; instead, Rust has a
> feature called *automatic referencing and dereferencing*. Calling methods is
> one of the few places in Rust that has this behavior.
>
> Here’s how it works: when you call a method with `object.something()`, Rust
> automatically adds in `&`, `&mut`, or `*` so `object` matches the signature of
> the method. In other words, the following are the same:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -- >
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> The first one looks much cleaner. This automatic referencing behavior works
> because methods have a clear receiver—the type of `self`. Given the receiver
> and name of a method, Rust can figure out definitively whether the method is
> reading (`&self`), mutating (`&mut self`), or consuming (`self`). The fact
> that Rust makes borrowing implicit for method receivers is a big part of
> making ownership ergonomic in practice.
-->

> ### Où est l'opérateur `->` ?
>
> En C et en C++, deux opérateurs différents sont utilisés pour appeler les
> méthodes : on utilise `.` si on appelle une méthode directement sur l'objet
> et `->` si on appelle la méthode sur un pointeur vers l'objet et qu'il faut
> d'abord déréférencer le pointeur. En d'autres termes, si `objet` est un
> pointeur, `objet->methode()` est similaire à `(*objet).methode()`.
>
> Rust n'a pas d'équivalent à l'opérateur `->` ; à la place, Rust a une
> fonctionnalité appelée *référencement et déréférencement automatiques*.
> L'appel de méthodes est l'un des rares endroits de Rust où on retrouve ce
> comportement.
>
> Voilà comment cela fonctionne : quand on appelle une méthode avec
> `objet.methode()`, Rust ajoute automatiquement le `&`, `&mut` ou `*` pour que
> `objet` corresponde à la signature de la méthode. Autrement dit, ces deux
> lignes sont identiques :
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, autre: &Point) -> f64 {
> #        let x_carre = f64::powi(autre.x - self.x, 2);
> #        let y_carre = f64::powi(autre.y - self.y, 2);
> #
> #        f64::sqrt(x_carre + y_carre)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> La première ligne semble bien plus propre. Ce comportement du
> (dé)référencement automatique fonctionne parce que les méthodes ont une
> cible claire : le type de `self`. Compte tenu du nom de la méthode et
> de l'instance sur laquelle elle s'applique, Rust peut déterminer de manière
> irréfutable si la méthode lit (`&self`), modifie (`&mut self`) ou consomme
> (`self`) l'instance. Le fait que Rust rend implicite l'emprunt pour les
> instances sur lesquelles on appelle les méthodes améliore significativement
> l'ergonomie de la possession.

<!--
### Methods with More Parameters
-->

### Les méthodes avec davantage de paramètres

<!--
Let’s practice using methods by implementing a second method on the `Rectangle`
struct. This time, we want an instance of `Rectangle` to take another instance
of `Rectangle` and return `true` if the second `Rectangle` can fit completely
within `self` (the first `Rectangle`); otherwise it should return `false`. That
is, once we’ve defined the `can_hold` method, we want to be able to write the
program shown in Listing 5-14.
-->

Entraînons-nous à utiliser des méthodes en implémentant une seconde méthode sur
la structure `Rectangle`. Cette fois-ci, nous voulons qu'une instance de
`Rectangle` prenne une autre instance de `Rectangle` et qu'on retourne `true` si
le second `Rectangle` peut se dessiner intégralement à l'intérieur de `self`
(le premier `Rectangle`) ; sinon, on renverra `false`. En d'autres termes, une
fois qu'on aura défini la méthode `peut_contenir`, on veut pouvoir écrire le
programme de l'encart 5-14.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<!--
<span class="caption">Listing 5-14: Using the as-yet-unwritten `can_hold`
method</span>
-->

<span class="caption">Encart 5-14 : Utilisation de la méthode `peut_contenir`
qui reste à écrire</span>

<!--
And the expected output would look like the following, because both dimensions
of `rect2` are smaller than the dimensions of `rect1` but `rect3` is wider than
`rect1`:
-->

Et on s'attend à ce que le texte suivant s'affiche, puisque les deux dimensions
de `rect2` sont plus petites que les dimensions de `rect1`, mais `rect3` est
plus large que `rect1` :

<!--
```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```
-->

```text
rect1 peut-il contenir rect2 ? true
rect1 peut-il contenir rect3 ? false
```

<!--
We know we want to define a method, so it will be within the `impl Rectangle`
block. The method name will be `can_hold`, and it will take an immutable borrow
of another `Rectangle` as a parameter. We can tell what the type of the
parameter will be by looking at the code that calls the method:
`rect1.can_hold(&rect2)` passes in `&rect2`, which is an immutable borrow to
`rect2`, an instance of `Rectangle`. This makes sense because we only need to
read `rect2` (rather than write, which would mean we’d need a mutable borrow),
and we want `main` to retain ownership of `rect2` so we can use it again after
calling the `can_hold` method. The return value of `can_hold` will be a
Boolean, and the implementation will check whether the width and height of
`self` are both greater than the width and height of the other `Rectangle`,
respectively. Let’s add the new `can_hold` method to the `impl` block from
Listing 5-13, shown in Listing 5-15.
-->

Nous voulons définir une méthode, donc elle doit se trouver dans le bloc
`impl Rectangle`. Le nom de la méthode sera `peut_contenir` et elle prendra une
référence immuable vers un autre `Rectangle` en paramètre. On peut déterminer le
type du paramètre en regardant le code qui appelle la méthode :
`rect1.peut_contenir(&rect2)` prend en argument `&rect2`, une référence immuable
vers `rect2`, une instance de `Rectangle`. Cela est logique puisque nous voulons
uniquement lire `rect2` (plutôt que de la modifier, ce qui aurait nécessité une
référence mutable) et nous souhaitons que `main` garde possession de `rect2`
pour qu'on puisse le réutiliser après avoir appelé la méthode `peut_contenir`.
La valeur de retour de `peut_contenir` sera un booléen et l'implémentation de la
méthode vérifiera si la largeur et la hauteur de `self` sont respectivement plus
grandes que la largeur et la hauteur de l'autre `Rectangle`. Ajoutons la
nouvelle méthode `peut_contenir` dans le bloc `impl` de l'encart 5-13, comme le
montre l'encart 5-15.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-15: Implementing the `can_hold` method on
`Rectangle` that takes another `Rectangle` instance as a parameter</span>
-->

<span class="caption">Encart 5-15 : Implémentation de la méthode `peut_contenir`
sur `Rectangle` qui prend une autre instance de `Rectangle` en paramètre</span>

<!--
When we run this code with the `main` function in Listing 5-14, we’ll get our
desired output. Methods can take multiple parameters that we add to the
signature after the `self` parameter, and those parameters work just like
parameters in functions.
-->

Lorsque nous exécutons ce code avec la fonction `main` de l'encart 5-14, nous
obtenons l'affichage attendu. Les méthodes peuvent prendre plusieurs paramètres
qu'on peut ajouter à la signature après le paramètre `self`, et ces paramètres
fonctionnent de la même manière que les paramètres des fonctions.

<!--
### Associated Functions
-->

### Les fonctions associées

<!--
All functions defined within an `impl` block are called *associated functions*
because they’re associated with the type named after the `impl`. We can define
associated functions that don’t have `self` as their first parameter (and thus
are not methods) because they don’t need an instance of the type to work with.
We’ve already used one function like this: the `String::from` function that’s
defined on the `String` type.
-->

Toutes les fonctions définies dans un bloc `impl` s'appellent des *fonctions
associées* car elles sont associées au type renseigné après le `impl`. Nous
pouvons aussi y définir des fonctions associées qui n'ont pas de `self` en
premier paramètre (et donc ce ne sont pas des méthodes) car elles n'ont pas
besoin d'une instance du type sur lequel elles travaillent. Nous avons déjà
utilisé une fonction comme celle-ci : la fonction `String::from` qui est
définie sur le type `String`.

<!--
Associated functions that aren’t methods are often used for constructors that
will return a new instance of the struct. For example, we could provide an
associated function that would have one dimension parameter and use that as
both width and height, thus making it easier to create a square `Rectangle`
rather than having to specify the same value twice:
-->

Les fonctions associées qui ne ne sont pas des méthodes sont souvent utilisées
comme constructeurs qui vont retourner une nouvelle instance de la structure.
Par exemple, on pourrait écrire une fonction associée qui prend une unique
dimension en paramètre et l'utilise à la fois pour la largeur et pour la
hauteur, ce qui rend plus aisé la création d'un `Rectangle` carré plutôt que
d'avoir à indiquer la même valeur deux fois :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

<!--
To call this associated function, we use the `::` syntax with the struct name;
`let sq = Rectangle::square(3);` is an example. This function is namespaced by
the struct: the `::` syntax is used for both associated functions and
namespaces created by modules. We’ll discuss modules in Chapter 7.
-->

Pour appeler cette fonction associée, on utilise la syntaxe `::` avec le nom de
la structure ; `let mon_carre = Rectangle::carre(3);` en est un exemple. Cette
fonction est cloisonnée dans l'espace de noms de la structure : la syntaxe `::`
s'utilise aussi bien pour les fonctions associées que pour les espaces de noms
créés par des modules. Nous aborderons les modules au chapitre 7.

<!--
### Multiple `impl` Blocks
-->

### Plusieurs blocs `impl`

<!--
Each struct is allowed to have multiple `impl` blocks. For example, Listing
5-15 is equivalent to the code shown in Listing 5-16, which has each method
in its own `impl` block.
-->

Chaque structure peut avoir plusieurs blocs `impl`. Par exemple, l'encart 5-15
est équivalent au code de l'encart 5-16, où chaque méthode est dans son propre
bloc `impl`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-16: Rewriting Listing 5-15 using multiple `impl`
blocks</span>
-->

<span class="caption">Encart 5-16 : Réécriture de l'encart 5-15 en utilisant
plusieurs blocs `impl`</span>

<!--
There’s no reason to separate these methods into multiple `impl` blocks here,
but this is valid syntax. We’ll see a case in which multiple `impl` blocks are
useful in Chapter 10, where we discuss generic types and traits.
-->

Il n'y a aucune raison de séparer ces méthodes dans plusieurs blocs `impl` dans
notre exemple, mais c'est une syntaxe valide. Nous verrons un exemple de
l'utilité d'avoir plusieurs blocs `impl` au chapitre 10, où nous aborderons les
types génériques et les traits.

<!--
## Summary
-->

## Résumé

<!--
Structs let you create custom types that are meaningful for your domain. By
using structs, you can keep associated pieces of data connected to each other
and name each piece to make your code clear. In `impl` blocks, you can define
functions that are associated with your type, and methods are a kind of
associated function that let you specify the behavior that instances of your
structs have.
-->

Les structures vous permettent de créer des types personnalisés significatifs
pour votre domaine. En utilisant des structures, on peut relier entre elles
des données associées et nommer chaque donnée pour rendre le code plus clair.
Dans des blocs `impl`, vous pouvez définir des fonctions qui sont associées à
votre type, et les méthodes sont un genre de fonction associée qui vous permet
de renseigner le comportement que doivent suivre les instances de votre
structure.

<!--
But structs aren’t the only way you can create custom types: let’s turn to
Rust’s enum feature to add another tool to your toolbox.
-->

Mais les structures ne sont pas le seul moyen de créer des types personnalisés :
nous allons maintenant voir les énumérations de Rust, une fonctionnalité que
vous pourrez bientôt ajouter à votre boîte à outils.
