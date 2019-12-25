<!--
## Method Syntax
-->

## La syntaxe des méthodes

<!--
*Methods* are similar to functions: they’re declared with the `fn` keyword and
their name, they can have parameters and a return value, and they contain some
code that is run when they’re called from somewhere else. However, methods are
different from functions in that they’re defined within the context of a struct
(or an enum or a trait object, which we cover in Chapters 6 and 17,
respectively), and their first parameter is always `self`, which represents the
instance of the struct the method is being called on.
-->

Les *méthodes* sont similaires aux fonctions : on les déclare avec le mot-clé
`fn` et leur nom, elles peuvent avoir des paramètres et une valeur de retour, et
elles contiennent du code qui est exécuté quand on les appelle depuis un autre
endroit. Cependant, les méthodes diffèrent des fonctions parce qu'elles sont
définies dans le contexte d'une structure (ou d'une énumération ou d'un objet de
trait, que nous aborderons respectivement aux chapitres 6 et 17) et que leur
premier paramètre est toujours `self`, un mot-clé qui représente l'instance de
la structure sur laquelle on appelle la méthode.

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
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
-->

```rust
#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

impl Rectangle {
    fn aire(&self) -> u32 {
        self.largeur * self.hauteur
    }
}

fn main() {
    let rect1 = Rectangle { largeur: 30, hauteur: 50 };

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        rect1.aire()
    );
}
```

<!--
<span class="caption">Listing 5-13: Defining an `area` method on the
`Rectangle` struct</span>
-->

<span class="caption">Encart 5-13 : Définition d'une méthode `aire` sur la
structure `Rectangle`</span>

<!--
To define the function within the context of `Rectangle`, we start an `impl`
(implementation) block. Then we move the `area` function within the `impl`
curly brackets and change the first (and in this case, only) parameter to be
`self` in the signature and everywhere within the body. In `main`, where we
called the `area` function and passed `rect1` as an argument, we can instead
use *method syntax* to call the `area` method on our `Rectangle` instance.
The method syntax goes after an instance: we add a dot followed by the method
name, parentheses, and any arguments.
-->

Pour définir la fonction dans le contexte de `Rectangle`, nous démarrons un bloc
`impl` (*implémentation*). Puis nous déplaçons la fonction `aire` entre les
accolades du `impl` et nous remplaçons le premier paramètre (et dans notre cas,
le seul) par `self` dans la signature et dans tout le corps. Dans `main`, où
nous avons appelé la fonction `aire` et passé `rect1` en argument, nous pouvons
utiliser à la place la *syntaxe des méthodes* pour appeler la méthode `aire` sur
notre instance de `Rectangle`. La syntaxe des méthodes se place après
l'instance : on ajoute un point suivi du nom de la méthode et des parenthèses
contenant les arguments s'il y en a.

<!--
In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle`
because Rust knows the type of `self` is `Rectangle` due to this method’s being
inside the `impl Rectangle` context. Note that we still need to use the `&`
before `self`, just as we did in `&Rectangle`. Methods can take ownership of
`self`, borrow `self` immutably as we’ve done here, or borrow `self` mutably,
just as they can any other parameter.
-->

Dans la signature de `aire`, nous utilisons `&self` à la place de
`rectangle: &Rectangle` parce que Rust sait que le type de `self` est
`Rectangle` puisque la méthode se trouve au sein du contexte `impl Rectangle`.
Veuillez noter qu'il nous faut quand même utiliser le `&` avant le `self`, comme
nous l'avions fait pour `&Rectangle`. Les méthodes peuvent prendre possession de
`self`, emprunter `self` de façon immuable comme nous l'avons fait ici, ou
emprunter `self` de façon mutable, comme pour n'importe quel autre paramètre.

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
The main benefit of using methods instead of functions, in addition to using
method syntax and not having to repeat the type of `self` in every method’s
signature, is for organization. We’ve put all the things we can do with an
instance of a type in one `impl` block rather than making future users of our
code search for capabilities of `Rectangle` in various places in the library we
provide.
-->

Outre l'utilisation de la syntaxe des méthodes et le fait de ne pas être obligé
de répéter le type de `self` dans la signature de chaque méthode, le principal
avantage de l'utilisation de méthodes plutôt que de fonctions est pour
l'organisation. Nous avons mis tout ce qu'on pouvait faire avec une instance de
notre type dans un bloc `impl` plutôt que d'imposer aux futurs utilisateurs de
notre code à rechercher les fonctionnalités de `Rectangle` à divers endroits de
la bibliothèque que nous fournissons.

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
within `self`; otherwise it should return `false`. That is, we want to be able
to write the program shown in Listing 5-14, once we’ve defined the `can_hold`
method.
-->

Entraînons-nous à utiliser des méthodes en implémentant une seconde méthode sur
la structure `Rectangle`. Cette fois-ci, nous voulons qu'une instance de
`Rectangle` prenne une autre instance de `Rectangle` et qu'on retourne `true` si
le second `Rectangle` peut se dessiner intégralement à l'intérieur de `self` ;
sinon, on renverra `false`. En d'autres termes, on veut pouvoir écrire le
programme de l'encart 5-14 une fois qu'on aura défini la méthode
`peut_contenir`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```
-->

```rust,ignore
fn main() {
    let rect1 = Rectangle { largeur: 30, hauteur: 50 };
    let rect2 = Rectangle { largeur: 10, hauteur: 40 };
    let rect3 = Rectangle { largeur: 60, hauteur: 45 };

    println!("rect1 peut-il contenir rect2 ? {}", rect1.peut_contenir(&rect2));
    println!("rect1 peut-il contenir rect3 ? {}", rect1.peut_contenir(&rect3));
}
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
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```
-->

```rust
# #[derive(Debug)]
# struct Rectangle {
#     largeur: u32,
#     hauteur: u32,
# }
#
impl Rectangle {
    fn aire(&self) -> u32 {
        self.largeur * self.hauteur
    }

    fn peut_contenir(&self, autre: &Rectangle) -> bool {
        self.largeur > autre.largeur && self.hauteur > autre.hauteur
    }
}
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
Another useful feature of `impl` blocks is that we’re allowed to define
functions within `impl` blocks that *don’t* take `self` as a parameter. These
are called *associated functions* because they’re associated with the struct.
They’re still functions, not methods, because they don’t have an instance of
the struct to work with. You’ve already used the `String::from` associated
function.
-->

Une autre fonctionnalité utile des blocs `impl` est qu'on peut définir des
fonctions dans des blocs `impl` qui ne prennent *pas* `self` en paramètre. Cela
s'appelle des *fonctions associées* parce qu'elles sont associées à la
structure. Cela reste des fonctions, pas des méthodes, parce qu'elles ne
s'appliquent pas à une instance de structure. Vous avez déjà utilisé la fonction
associée `String::from`.

<!--
Associated functions are often used for constructors that will return a new
instance of the struct. For example, we could provide an associated function
that would have one dimension parameter and use that as both width and height,
thus making it easier to create a square `Rectangle` rather than having to
specify the same value twice:
-->

Les fonctions associées sont souvent utilisées comme constructeurs qui vont
retourner une nouvelle instance de la structure. Par exemple, on pourrait écrire
une fonction associée qui prend une unique dimension en paramètre et l'utilise
à la fois pour la largeur et pour la hauteur, ce qui rend plus aisé la création
d'un `Rectangle` carré plutôt que d'avoir à indiquer la même valeur deux fois :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```
-->

```rust
# #[derive(Debug)]
# struct Rectangle {
#     largeur: u32,
#     hauteur: u32,
# }
#
impl Rectangle {
    fn carre(cote: u32) -> Rectangle {
        Rectangle { largeur: cote, hauteur: cote }
    }
}
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
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```
-->

```rust
# #[derive(Debug)]
# struct Rectangle {
#     largeur: u32,
#     hauteur: u32,
# }
#
impl Rectangle {
    fn aire(&self) -> u32 {
        self.largeur * self.hauteur
    }
}

impl Rectangle {
    fn peut_contenir(&self, autre: &Rectangle) -> bool {
        self.largeur > autre.largeur && self.hauteur > autre.hauteur
    }
}
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
and name each piece to make your code clear. Methods let you specify the
behavior that instances of your structs have, and associated functions let you
namespace functionality that is particular to your struct without having an
instance available.
-->

Les structures vous permettent de créer des types personnalisés significatifs
pour votre domaine. En utilisant des structures, on peut relier entre elles
des données associées et nommer chaque donnée pour rendre le code plus clair.
Les méthodes vous permettent de définir le comportement des instances de vos
structures, et les fonctions associées vous permettent de cloisonner dans un
espace de noms des fonctionnalités qui sont spécifiques à votre structure sans
avoir besoin d'une instance disponible.

<!--
But structs aren’t the only way you can create custom types: let’s turn to
Rust’s enum feature to add another tool to your toolbox.
-->

Mais les structures ne sont pas le seul moyen de créer des types personnalisés :
nous allons maintenant voir les énumérations de Rust, une fonctionnalité que
vous pourrez bientôt ajouter à votre boîte à outils.
