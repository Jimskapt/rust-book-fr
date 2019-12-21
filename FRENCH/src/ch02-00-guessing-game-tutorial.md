<!--
# Programming a Guessing Game
-->

# Programmer un jeu de devinettes

<!--
Let’s jump into Rust by working through a hands-on project together! This
chapter introduces you to a few common Rust concepts by showing you how to use
them in a real program. You’ll learn about `let`, `match`, methods, associated
functions, using external crates, and more! The following chapters will explore
these ideas in more detail. In this chapter, you’ll practice the fundamentals.
-->

Entrons dans le vif du sujet en travaillant ensemble sur un projet concret !
Ce chapitre présente quelques concepts couramment utilisés en Rust en vous
montrant comment les utiliser dans un véritable programme. Nous aborderons
notamment les instructions `let` et `match`, les méthodes et fonctions
associées, l'utilisation des *crates*, et bien plus encore ! Dans les chapitres
suivants, nous approfondirons ces notions. Dans ce chapitre, vous n'allez
exercer que les principes de base.

<!--
We’ll implement a classic beginner programming problem: a guessing game. Here’s
how it works: the program will generate a random integer between 1 and 100. It
will then prompt the player to enter a guess. After a guess is entered, the
program will indicate whether the guess is too low or too high. If the guess is
correct, the game will print a congratulatory message and exit.
-->

Nous allons coder un programme fréquemment réalisé par les débutants en
programmation : un jeu de devinettes. Le principe de ce jeu est le suivant :
le programme va tirer au sort un nombre entre 1 et 100. Il invitera ensuite le
joueur à saisir un nombre qu'il pense deviner. Après la saisie, le programme
indiquera si le nombre saisi par le joueur est trop grand ou trop petit. Si le
nombre saisi est le bon, le jeu affichera un message de félicitations et se
fermera.

<!--
## Setting Up a New Project
-->

## Mise en place d'un nouveau projet

<!--
To set up a new project, go to the *projects* directory that you created in
Chapter 1 and make a new project using Cargo, like so:
-->

Pour créer un nouveau projet, rendez-vous dans le dossier *projects* que
vous avez créé au chapitre 1 et utilisez Cargo pour créer votre projet, comme
ceci :

```text
$ cargo new guessing_game
$ cd guessing_game
```

<!--
The first command, `cargo new`, takes the name of the project (`guessing_game`)
as the first argument. The second command changes to the new project’s
directory.
-->

La première commande, `cargo new`, prend comme premier argument le nom de notre
projet (`guessing_game`). La seconde commande nous déplace dans le dossier de
notre nouveau projet créé par Cargo.

<!--
Look at the generated *Cargo.toml* file:
-->

Regardons le fichier *Cargo.toml* qui a été généré :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

<!--
If the author information that Cargo obtained from your environment is not
correct, fix that in the file and save it again.
-->

Si le nom de l'auteur que Cargo a déduit de votre environnement est incorrect,
vous pouvez le changer dans ce fichier et le sauvegarder.

<!--
As you saw in Chapter 1, `cargo new` generates a “Hello, world!” program for
you. Check out the *src/main.rs* file:
-->

Comme vous l'avez expérimenté dans le chapitre 1, `cargo new` génère un
programme *“Hello, world!”* pour vous. Ouvrez le fichier *src/main.rs* :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

<!--
Now let’s compile this “Hello, world!” program and run it in the same step
using the `cargo run` command:
-->

Maintenant, lançons la compilation de ce programme “Hello, world!” et
son exécution en une seule commande avec `cargo run` :

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/guessing_game`
Hello, world!
```

<!--
The `run` command comes in handy when you need to rapidly iterate on a project,
as we’ll do in this game, quickly testing each iteration before moving on to
the next one.
-->

Cette commande `run` est très pratique lorsqu'on souhaite itérer rapidement
sur un projet, comme c'est le cas ici, pour tester rapidement chaque
modification avant de passer à la suivante.

<!--
Reopen the *src/main.rs* file. You’ll be writing all the code in this file.
-->

Ouvrez à nouveau le fichier *src/main.rs*. C'est dans ce fichier que nous
écrirons la totalité de notre code.

<!--
## Processing a Guess
-->

## Traitement d'un nombre saisi

<!--
The first part of the guessing game program will ask for user input, process
that input, and check that the input is in the expected form. To start, we’ll
allow the player to input a guess. Enter the code in Listing 2-1 into
*src/main.rs*.
-->

La première partie du programme consiste à demander au joueur de saisir du
texte, à traiter cette saisie, et à vérifier que la saisie correspond au format
attendu.
Commençons par permettre au joueur de saisir son nombre. Entrez le
code de l'encart 2-1 dans le fichier *src/main.rs*.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```
-->

```rust,ignore
use std::io;

fn main() {
    println!("Devinez le nombre !");

    println!("Veuillez entrer un nombre.");

    let mut supposition = String::new();

    io::stdin().read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");

    println!("Votre nombre : {}", supposition);
}
```

<!--
<span class="caption">Listing 2-1: Code that gets a guess from the user and
prints it</span>
-->

<span class="caption">Encart 2-1 : Code permettant de récupérer une saisie
utilisateur et de l'afficher</span>

<!--
This code contains a lot of information, so let’s go over it line by line. To
obtain user input and then print the result as output, we need to bring the
`io` (input/output) library into scope. The `io` library comes from the
standard library (which is known as `std`):
-->

Ce code contient beaucoup d'informations, nous allons donc l'analyser petit
à petit. Pour obtenir la saisie utilisateur et ensuite l'afficher, nous avons
besoin d'importer la bibliothèque `io` (pour *input/output*, entrée/sortie) afin
de pouvoir l'utiliser. La bibliothèque `io` provient de la bibliothèque standard
(qui est aussi connue sous le nom de `std`).

```rust,ignore
use std::io;
```

<!--
By default, Rust brings only a few types into the scope of every program in
[the *prelude*][prelude]<!-- ignore -- >. If a type you want to use isn’t in the
prelude, you have to bring that type into scope explicitly with a `use`
statement. Using the `std::io` library provides you with a number of useful
features, including the ability to accept user input.
-->

Par défaut, Rust n'importe que très peu de types dans les programmes, qui sont
présents dans [*l'étape préliminaire (the prelude)*][prelude]<!-- ignore -->. Si
vous voulez utiliser un type qui ne s'y trouve pas, vous devrez l'importer
explicitement avec l'instruction `use`. L'utilisation de la bibliothèque
`std::io` vous apporte de nombreuses fonctionnalités utiles, comme ici la
possibilité de récupérer une saisie utilisateur.

<!-- [prelude]: ../std/prelude/index.html -->
[prelude]: https://doc.rust-lang.org/std/prelude/index.html

<!--
As you saw in Chapter 1, the `main` function is the entry point into the
program:
-->

Comme vous l'avez vu au chapitre 1, la fonction `main` est le point d'entrée
du programme :

```rust,ignore
fn main() {
```

<!--
The `fn` syntax declares a new function, the parentheses, `()`, indicate there
are no parameters, and the curly bracket, `{`, starts the body of the function.
-->

Le mot clé `fn` déclare une nouvelle fonction, les parenthèses `()` indiquent
que cette fonction n'accepte aucun paramètre, et l'accolade ouvrante `{` marque
le début du corps de la fonction.

<!--
As you also learned in Chapter 1, `println!` is a macro that prints a string to
the screen:
-->

Comme vous l'avez également appris au chapitre 1, `println!` est une macro qui
affiche une chaîne de caractères à l'écran :

<!--
```rust,ignore
println!("Guess the number!");

println!("Please input your guess.");
```
-->

```rust,ignore
println!("Devinez le nombre !");

println!("Veuillez entrer un nombre.");
```

<!--
This code is printing a prompt stating what the game is and requesting input
from the user.
-->

Ce code affiche du texte qui indique le titre de notre jeu, et un autre qui
demande au joueur d'entrer un nombre.

<!--
### Storing Values with Variables
-->

### Enregistrer des données dans des variables

<!--
Next, we’ll create a place to store the user input, like this:
-->

Ensuite, on crée un endroit où stocker la saisie de l'utilisateur, comme ceci :

<!--
```rust,ignore
let mut guess = String::new();
```
-->

```rust,ignore
let mut supposition = String::new();
```

<!--
Now the program is getting interesting! There’s a lot going on in this little
line. Notice that this is a `let` statement, which is used to create a
*variable*. Here’s another example:
-->

Le programme commence à devenir intéressant ! Il se passe beaucoup de choses
dans cette petite ligne. Vous remarquerez qu'elle commence par le mot-clé `let`,
qui sert à créer une *variable*. Voici un autre exemple :

```rust,ignore
let foo = bar;
```

<!--
This line creates a new variable named `foo` and binds it to the value of the
`bar` variable. In Rust, variables are immutable by default. We’ll be
discussing this concept in detail in the [“Variables and
Mutability”][variables-and-mutability]<!-- ignore -- > section in Chapter 3.
The following example shows how to use `mut` before the variable name to make
a variable mutable:
-->

Cette ligne permet de créer une nouvelle variable nommée `foo` et à lui
assigner la valeur de `bar`. Par défaut en Rust, les variables sont immuables.
Nous aborderons plus en détail cette notion dans la section [“Variables et
Mutabilité”][variables-and-mutability]<!-- ignore --> au chapitre 3. L'exemple
suivant montre comment utiliser le mot-clé `mut` avant le nom de la variable
pour rendre une variable mutable *(c'est-à-dire modifiable)* :

<!--
```rust,ignore
let foo = 5; // immutable
let mut bar = 5; // mutable
```
-->

```rust
let foo = 5; // immuable
let mut bar = 5; // mutable, modifiable
```

<!--
> Note: The `//` syntax starts a comment that continues until the end of the
> line. Rust ignores everything in comments, which are discussed in more detail
> in Chapter 3.
-->

> Remarque : La syntaxe `//` permet de commencer un commentaire qui s'étend
> jusqu'à la fin de la ligne. Rust ignore tout ce qu'il y a dans un commentaire,
> ceci sera développé plus en détail dans le chapitre 3.

<!--
Let’s return to the guessing game program. You now know that `let mut guess`
will introduce a mutable variable named `guess`. On the other side of the equal
sign (`=`) is the value that `guess` is bound to, which is the result of
calling `String::new`, a function that returns a new instance of a `String`.
[`String`][string]<!-- ignore -- > is a string type provided by the standard
library that is a growable, UTF-8 encoded bit of text.
-->

Mais revenons à notre jeu de devinettes. Vous comprenez donc maintenant que la
ligne `let mut supposition` permet de créer une variable mutable nommée
`supposition`. De l'autre côté du signe égal (`=`) se trouve la valeur de cette
variable, et il s'agit ici du résultat de l'utilisation de `String::new`, qui
est une fonction qui retourne une nouvelle instance de `String`.
[`String`][string]<!-- ignore --> est un type de chaîne de caractères fourni
par la bibliothèque standard, qui est une portion de texte encodée en UTF-8 et
dont la longueur peut augmenter.

<!-- [string]: ../std/string/struct.String.html -->
[string]: https://doc.rust-lang.org/std/string/struct.String.html

<!--
The `::` syntax in the `::new` line indicates that `new` is an *associated
function* of the `String` type. An associated function is implemented on a type,
in this case `String`, rather than on a particular instance of a `String`. Some
languages call this a *static method*.
-->

La syntaxe `::` dans `String::new()` indique que `new` est une
*fonction associée* au type `String`. Une fonction associée est implémentée sur
un type, ici `String`, plutôt que sur une instance de `String`. Ce concept est
parfois appelé une *méthode statique* dans d'autres langages.

<!--
This `new` function creates a new, empty string. You’ll find a `new` function
on many types, because it’s a common name for a function that makes a new value
of some kind.
-->

Cette fonction `new` crée une nouvelle chaîne de caractères vide, une nouvelle
`String`. Vous trouverez fréquemment une fonction `new` sur d'autres types, car
c'est un nom souvent donné à une fonction qui crée une nouvelle valeur ou
instance d'un type.

<!--
To summarize, the `let mut guess = String::new();` line has created a mutable
variable that is currently bound to a new, empty instance of a `String`. Whew!
-->

Pour résumer, la ligne `let mut supposition = String::new();` crée une nouvelle
variable mutable qui contient une nouvelle chaîne de caractères vide, une
instance de `String`. Ouf !

<!--
Recall that we included the input/output functionality from the standard
library with `use std::io;` on the first line of the program. Now we’ll call
the `stdin` function from the `io` module:
-->

Rappelez-vous que nous avons importé les fonctionnalités d'entrée/sortie de la
bibliothèque standard avec `use std::io;` à la première ligne de notre
programme. Nous allons maintenant appeler la fonction `stdin` du
module `io` :

<!--
```rust,ignore
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```
-->

```rust,ignore
io::stdin().read_line(&mut supposition)
    .expect("Échec de la lecture de l'entrée utilisateur");
```

<!--
If we hadn’t put the `use std::io` line at the beginning of the program, we
could have written this function call as `std::io::stdin`. The `stdin` function
returns an instance of [`std::io::Stdin`][iostdin]<!-- ignore -- >, which is a
type that represents a handle to the standard input for your terminal.
-->

Si la ligne `use std::io` n'était pas présente au début du programme, on aurait
dû écrire l'appel à la fonction de cette manière : `std::io::stdin`. La fonction
`stdin` retourne une instance de [`std::io::Stdin`][iostdin]<!-- ignore -->, qui
est un type qui représente une référence abstraite *(handle)* vers l'entrée
standard du terminal dans lequel vous avez lancé le programme.

<!-- [iostdin]: ../std/io/struct.Stdin.html -->
[iostdin]: https://doc.rust-lang.org/std/io/struct.Stdin.html

<!--
The next part of the code, `.read_line(&mut guess)`, calls the
[`read_line`][read_line]<!-- ignore -- > method on the standard input handle to
get input from the user. We’re also passing one argument to `read_line`: `&mut
guess`.
-->

La partie suivante du code, `.read_line(&mut supposition)`, appelle la méthode
[`read_line`][read_line]<!-- ignore --> sur l'entrée standard afin d'obtenir
la saisie utilisateur. De plus, on passe à cette méthode l'argument
`&mut supposition`.

<!-- [read_line]: ../std/io/struct.Stdin.html#method.read_line -->
[read_line]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line

<!--
The job of `read_line` is to take whatever the user types into standard input
and place that into a string, so it takes that string as an argument. The
string argument needs to be mutable so the method can change the string’s
content by adding the user input.
-->

Le rôle de `read_line` est de récupérer tout ce que l'utilisateur écrit dans
l'entrée standard et de le stocker dans une chaîne de caractères ; c'est
pourquoi cette méthode prend une `String` comme argument. Cet argument doit être
mutable pour que `read_line` puisse en modifier le contenu en y ajoutant
la saisie de l'utilisateur.

<!--
The `&` indicates that this argument is a *reference*, which gives you a way to
let multiple parts of your code access one piece of data without needing to
copy that data into memory multiple times. References are a complex feature,
and one of Rust’s major advantages is how safe and easy it is to use
references. You don’t need to know a lot of those details to finish this
program. For now, all you need to know is that like variables, references are
immutable by default. Hence, you need to write `&mut guess` rather than
`&guess` to make it mutable. (Chapter 4 will explain references more
thoroughly.)
-->

Le `&` indique que cet argument est une *référence*, ce qui permet de laisser
plusieurs morceaux de votre code accéder à une même donnée sans avoir besoin
de copier ces données dans la mémoire plusieurs fois. Les références sont une
fonctionnalité complexe, et un des avantages majeurs de Rust est qu'il rend sûr
et simple l'utilisation des références. Il n'est pas nécessaire de trop
s'apesantir sur les références pour terminer ce programme.
Pour l'instant, tout ce que vous devez savoir est que comme les variables, les
références sont immuables par défaut.
D'où la nécessité d'écrire `&mut supposition` au lieu de `&supposition` pour la
rendre mutable. (Le chapitre 4 expliquera plus en détail les références.)

<!--
### Handling Potential Failure with the `Result` Type
-->

### Gérer les erreurs potentielles avec le type `Result`

<!--
We’re not quite done with this line of code. Although what we’ve discussed so
far is a single line of text, it’s only the first part of the single logical
line of code. The second part is this method:
-->

Nous n'en avons pas tout à fait fini avec cette ligne de code.
Nous avons abordé jusqu'ici une seule ligne de texte, mais ce n'est que la
première partie de la ligne de code.
La deuxième partie est cette méthode :

<!--
```rust,ignore
.expect("Failed to read line");
```
-->

```rust,ignore
.expect("Échec de la lecture de l'entrée utilisateur");
```

<!--
When you call a method with the `.foo()` syntax, it’s often wise to introduce a
newline and other whitespace to help break up long lines. We could have
written this code as:
-->

Lorsque l'on appelle une méthode avec la syntaxe `.foo()`, il est généralement
préférable d'ajouter un retour à la ligne puis d'indenter à l'aide d'espaces
pour décomposer les longues lignes de code.
Nous aurions pu écrire ce code de cette manière :

<!--
```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```
-->

```rust,ignore
io::stdin().read_line(&mut supposition).expect("Échec de la lecture de l'entrée utilisateur");
```

<!--
However, one long line is difficult to read, so it’s best to divide it: two
lines for two method calls. Now let’s discuss what this line does.
-->

Cependant, une longue ligne de code n'est pas toujours facile à lire, c'est donc
une bonne pratique de la diviser : deux lignes de texte pour deux appels de
méthodes. Mais maintenant, voyons à quoi sert cette ligne.

<!--
As mentioned earlier, `read_line` puts what the user types into the string
we’re passing it, but it also returns a value—in this case, an
[`io::Result`][ioresult]<!-- ignore -- >. Rust has a number of types named
`Result` in its standard library: a generic [`Result`][result]<!-- ignore -- >
as well as specific versions for submodules, such as `io::Result`.
-->

Comme expliqué précédemment, `read_line` stocke ce que l'utilisateur a saisi
dans la variable qu'on lui passe en argument, mais cette fonction retourne
aussi une valeur − dans notre cas, de type
[`io::Result`][ioresult]<!-- ignore -->. Il existe plusieurs types nommés
`Result` dans la bibliothèque standard de Rust : un type générique
[`Result`][result]<!-- ignore --> ainsi que des déclinaisons spécifiques à
des sous-modules, comme `io::Result`.

<!-- [ioresult]: ../std/io/type.Result.html -->
[ioresult]: https://doc.rust-lang.org/std/io/type.Result.html
<!-- [result]: ../std/result/enum.Result.html -->
[result]: https://doc.rust-lang.org/std/result/enum.Result.html

<!--
The `Result` types are [*enumerations*][enums]<!-- ignore -- >, often referred
to as *enums*. An enumeration is a type that can have a fixed set of values,
and those values are called the enum’s *variants*. Chapter 6 will cover enums
in more detail.
-->

Les types `Result` sont des [*énumérations*][enums]<!-- ignore -->, aussi
appelées *enums*. Une énumération est un type qui peut avoir un certain nombre
de valeurs prédéfinies, et ces valeurs sont appelées des *variantes*
d'énumération. Le chapitre 6 explorera les énumérations plus en détail.

[enums]: ch06-00-enums.html

<!--
For `Result`, the variants are `Ok` or `Err`. The `Ok` variant indicates the
operation was successful, and inside `Ok` is the successfully generated value.
The `Err` variant means the operation failed, and `Err` contains information
about how or why the operation failed.
-->

Avec `Result`, les variantes sont `Ok` ou `Err`. La variante `Ok` signifie que
l'opération a fonctionné, et à l'intérieur de `Ok` se trouve la valeur générée
avec succès. La variante `Err` signifie que l'opération a échoué, et `Err`
contient les informations décrivant comment ou pourquoi l'opération a échoué.

<!--
The purpose of these `Result` types is to encode error-handling information.
Values of the `Result` type, like values of any type, have methods defined on
them. An instance of `io::Result` has an [`expect` method][expect]<!-- ignore
-- > that you can call. If this instance of `io::Result` is an `Err` value,
`expect` will cause the program to crash and display the message that you
passed as an argument to `expect`. If the `read_line` method returns an `Err`,
it would likely be the result of an error coming from the underlying operating
system. If this instance of `io::Result` is an `Ok` value, `expect` will take
the return value that `Ok` is holding and return just that value to you so you
can use it. In this case, that value is the number of bytes in what the user
entered into standard input.
-->

L'objectif de ces types `Result` est d'encoder des informations utiles à la
gestion des erreurs.
Les valeurs du type `Result`, comme pour tous les types, ont des méthodes
qui leur sont associées. Par exemple, une instance de `io::Result` a une
[méthode `expect`][expect]<!-- ignore --> que vous pouvez utiliser. Si cette
instance de `io::Result` a pour valeur la variante `Err`, l'appel à `expect`
fera planter le programme et affichera le message que vous avez passé en
argument de `expect`. Si l'appel à `read_line` retourne une variante `Err`, ce
sera probablement dû à une erreur du système d'exploitation. Si en revanche
`read_line` a pour valeur la variante `Ok`, `expect` récupèrera le
contenu du `Ok`, qui est le résultat de l'opération, et vous le retournera afin
que vous puissiez l'utiliser. Dans notre exemple, ce résultat est le nombre
d'octets que l'utilisateur a saisi dans l'entrée standard.

<!-- [expect]: ../std/result/enum.Result.html#method.expect -->
[expect]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect

<!--
If you don’t call `expect`, the program will compile, but you’ll get a warning:
-->

Si on n'appelle pas `expect`, le programme compilera, mais avec un
avertissement :

<!--
```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
  -- > src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```
-->

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
  -- > src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut supposition);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```

<!--
Rust warns that you haven’t used the `Result` value returned from `read_line`,
indicating that the program hasn’t handled a possible error.
-->

Rust nous prévient que l'on ne fait rien du `Result` que nous fournit
`read_line`, et que par conséquent notre programme ne gère pas une erreur
potentielle.

<!--
The right way to suppress the warning is to actually write error handling, but
because you just want to crash this program when a problem occurs, you can use
`expect`. You’ll learn about recovering from errors in Chapter 9.
-->

La meilleure façon de masquer cet avertissement est de réellement écrire le
code permettant de gérer l'erreur, mais dans notre cas on a seulement besoin de
faire planter le programme si un problème survient, on utilise donc `expect`.
Nous verrons dans le chapitre 9 comment gérer correctement les erreurs.

<!--
### Printing Values with `println!` Placeholders
-->

### Afficher des valeurs grâce aux espaces réservés de `println!`

<!--
Aside from the closing curly brackets, there’s only one more line to discuss in
the code added so far, which is the following:
-->

Mis à part l'accolade fermante, il ne nous reste plus qu'une seule ligne à
étudier dans le code que nous avons pour l'instant :

<!--
```rust,ignore
println!("You guessed: {}", guess);
```
-->

```rust,ignore
println!("Votre nombre : {}", supposition);
```

<!--
This line prints the string we saved the user’s input in. The set of curly
brackets, `{}`, is a placeholder: think of `{}` as little crab pincers that
hold a value in place. You can print more than one value using curly brackets:
the first set of curly brackets holds the first value listed after the format
string, the second set holds the second value, and so on. Printing multiple
values in one call to `println!` would look like this:
-->

Cette ligne affiche la chaîne de caractères où est stocké ce que l'utilisateur
a saisi. La paire d'accolades `{}` représente un espace réservé : imaginez
qu'il s'agit de pinces de crabes qui gardent la place d'une valeur. Vous pouvez
afficher plusieurs valeurs en utilisant des accolades : la première paire
d'accolades affichera la première valeur listée après la chaîne de formatage,
la deuxième paire d'accolades affichera la deuxième valeur, et ainsi de suite.
Pour afficher plusieurs valeurs en appelant `println!` une seule fois, on ferait
comme ceci :

<!--
```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```
-->

```rust
let x = 5;
let y = 10;

println!("x = {} et y = {}", x, y);
```

<!--
This code would print `x = 5 and y = 10`.
-->

Ce code afficherait `x = 5 et y = 10`.

<!--
### Testing the First Part
-->

### Test de la première partie

<!--
Let’s test the first part of the guessing game. Run it using `cargo run`:
-->

Pour tester notre début de programme, lançons-le à l'aide de la commande
`cargo run` :

<!--
```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```
-->

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Devinez le nombre !
Veuillez entrer un nombre.
6
Votre nombre : 6
```

<!--
At this point, the first part of the game is done: we’re getting input from the
keyboard and then printing it.
-->

À ce stade, la première partie de notre programme est terminée : nous avons
récupéré la saisie du clavier et nous l'affichons à l'écran.

<!--
## Generating a Secret Number
-->

## Générer le nombre secret

<!--
Next, we need to generate a secret number that the user will try to guess. The
secret number should be different every time so the game is fun to play more
than once. Let’s use a random number between 1 and 100 so the game isn’t too
difficult. Rust doesn’t yet include random number functionality in its standard
library. However, the Rust team does provide a [`rand` crate][randcrate].
-->

Maintenant, il nous faut générer un nombre secret que notre joueur va devoir
deviner. Ce nombre devra être différent à chaque fois pour qu'on puisse
s'amuser à y jouer plusieurs fois. Tirons au sort un nombre compris entre
1 et 100 pour que le jeu ne soit pas trop difficile. Rust n'embarque pas pour
l'instant de fonctionnalité de génération de nombres aléatoires dans sa
bibliothèque standard. Cependant, l'équipe de Rust propose une
[*crate* `rand`][randcrate].

[randcrate]: https://crates.io/crates/rand

<!--
### Using a Crate to Get More Functionality
-->

### Étendre les fonctionnalités de Rust avec une *crate*

<!--
Remember that a crate is a collection of Rust source code files.
The project we’ve been building is a *binary crate*, which is an executable.
The `rand` crate is a *library crate*, which contains code intended to be
used in other programs.
-->

Souvenez-vous, une *crate* est un ensemble de fichiers de code source Rust.
Le projet sur lequel nous travaillons est une *crate* binaire, qui est un
programme exécutable.
La *crate* `rand` est une *crate de bibliothèque*, qui contient du
code qui peut être utilisé dans d'autres programmes.

<!--
Cargo’s use of external crates is where it really shines. Before we can write
code that uses `rand`, we need to modify the *Cargo.toml* file to include the
`rand` crate as a dependency. Open that file now and add the following line to
the bottom beneath the `[dependencies]` section header that Cargo created for
you:
-->

L'utilisation des *crates* externes est un domaine dans lequel Cargo excelle.
Avant d'écrire le code qui utilisera `rand`, il nous faut éditer le fichier
*Cargo.toml* pour y spécifier `rand` en tant que dépendance. Ouvrez donc
maintenant ce fichier et ajoutez la ligne suivante à la fin, en dessous de
l'en-tête de section `[dependencies]` que Cargo a créé pour vous :

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

```toml
[dependencies]

rand = "0.5.5"
```

<!--
In the *Cargo.toml* file, everything that follows a header is part of a section
that continues until another section starts. The `[dependencies]` section is
where you tell Cargo which external crates your project depends on and which
versions of those crates you require. In this case, we’ll specify the `rand`
crate with the semantic version specifier `0.5.5`. Cargo understands [Semantic
Versioning][semver]<!-- ignore -- > (sometimes called *SemVer*), which is a
standard for writing version numbers. The number `0.5.5` is actually shorthand
for `^0.5.5`, which means “any version that has a public API compatible with
version 0.5.5.”
-->

Dans le fichier *Cargo.toml*, tout ce qui suit une en-tête fait partie de cette
section, et ce jusqu'à ce qu'une autre section débute. La section
`[dependencies]` permet d'indiquer à Cargo de quelles *crates* externes votre
projet dépend, et de quelle version de ces *crates* vous avez besoin.
Dans notre cas, on ajoute comme dépendance la crate `rand` avec la version
sémantique `0.5.5`. Cargo arrive à interpréter le
[versionnage sémantique][semver]<!-- ignore --> (aussi appelé *SemVer*), qui
est une convention d'écriture de numéros de version. En réalité, `0.5.5` est
une abréviation pour `^0.5.5`, ce qui signifie “toute version qui propose une
API publique compatible avec la version 0.5.5”.

[semver]: http://semver.org

<!--
Now, without changing any of the code, let’s build the project, as shown in
Listing 2-2.
-->

Maintenant, sans apporter le moindre changement au code, lançons une compilation
du projet, comme dans l'encart 2-2 :

```text
$ cargo build
    Updating crates.io index
  Downloaded rand v0.5.5
  Downloaded libc v0.2.62
  Downloaded rand_core v0.2.2
  Downloaded rand_core v0.3.1
  Downloaded rand_core v0.4.2
   Compiling rand_core v0.4.2
   Compiling libc v0.2.62
   Compiling rand_core v0.3.1
   Compiling rand_core v0.2.2
   Compiling rand v0.5.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 s
```

<!--
<span class="caption">Listing 2-2: The output from running `cargo build` after
adding the rand crate as a dependency</span>
-->

<span class="caption">Encart 2-2 : Résultat du lancement de `cargo build` après
avoir ajouté la *crate* `rand` comme dépendance</span>

<!--
You may see different version numbers (but they will all be compatible with
the code, thanks to SemVer!), and the lines may be in a different order.
-->

Il est possible que vous ne voyiez pas exactement les mêmes numéros de version,
(mais ils seront compatibles avec votre code, grâce au *versionnage
sémantique* !), et les lignes ne seront pas forcément affichées dans le même
ordre.

<!--
Now that we have an external dependency, Cargo fetches the latest versions of
everything from the *registry*, which is a copy of data from
[Crates.io][cratesio]. Crates.io is where people in the Rust ecosystem post
their open source Rust projects for others to use.
-->

Maintenant que nous avons une dépendance externe, Cargo récupère la dernière
version de tout ce qui nous faut depuis le *registre*, qui est une copie des
données de [Crates.io][cratesio]. Crates.io est là où les développeurs de
l'écosystème Rust publient leurs projets open source afin de les rendre
disponibles aux autres.

[cratesio]: https://crates.io/

<!--
After updating the registry, Cargo checks the `[dependencies]` section and
downloads any crates you don’t have yet. In this case, although we only listed
`rand` as a dependency, Cargo also grabbed `libc` and `rand_core`, because
`rand` depends on those to work. After downloading the crates, Rust compiles
them and then compiles the project with the dependencies available.
-->

Une fois le registre mis à jour, Cargo lit la section `[dependencies]` et se
charge de télécharger les *crates* que vous n'avez pas encore. Dans notre cas,
bien que nous n'ayons spécifié qu'une seule dépendance, `rand`, Cargo a aussi
téléchargé la *crate* `libc` et `rand_core`, car `rand` dépend d'elles pour
fonctionner. Une fois le téléchargement terminé des *crates*, Rust les compile,
puis compile notre projet avec les dépendances disponibles.

<!--
If you immediately run `cargo build` again without making any changes, you
won’t get any output aside from the `Finished` line. Cargo knows it has already
downloaded and compiled the dependencies, and you haven’t changed anything
about them in your *Cargo.toml* file. Cargo also knows that you haven’t changed
anything about your code, so it doesn’t recompile that either. With nothing to
do, it simply exits.
-->

Si vous relancez tout de suite `cargo build` sans changer quoi que ce soit, vous
n'obtiendrez rien d'autre que la ligne `Finished`. Cargo sait qu'il a déjà
téléchargé et compilé les dépendances, et que vous n'avez rien changé dans votre
fichier *Cargo.toml*. Cargo sait aussi que vous n'avez rien changé dans votre
code, donc il ne le recompile pas non plus. Étant donné qu'il n'a rien à faire,
Cargo se termine tout simplement.

<!--
If you open up the *src/main.rs* file, make a trivial change, and then save it
and build again, you’ll only see two lines of output:
-->

Si vous ouvrez le fichier *src/main.rs*, faites un changement très simple,
enregistrez le fichier, et relancez la compilation, vous verrez s'afficher
uniquement deux lignes :

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<!--
These lines show Cargo only updates the build with your tiny change to the
*src/main.rs* file. Your dependencies haven’t changed, so Cargo knows it can
reuse what it has already downloaded and compiled for those. It just rebuilds
your part of the code.
-->

Ces lignes nous informent que Cargo a recompilé uniquement à cause de notre
petit changement dans le fichier *src/main.rs*. Les dépendances n'ayant pas
changé, Cargo sait qu'il peut simplement réutiliser ce qu'il a déjà téléchargé
et compilé précédemment. Il se contente donc de ne recompiler que notre partie
du code.

<!--
#### Ensuring Reproducible Builds with the *Cargo.lock* File
-->

#### Assurer la reproductibilité des compilations avec le fichier *Cargo.lock*

<!--
Cargo has a mechanism that ensures you can rebuild the same artifact every time
you or anyone else builds your code: Cargo will use only the versions of the
dependencies you specified until you indicate otherwise. For example, what
happens if next week version 0.5.6 of the `rand` crate comes out and
contains an important bug fix but also contains a regression that will break
your code?
-->

Cargo embarque une fonctionnalité qui garantie que vous pouvez recompiler le
même artéfact à chaque fois que vous ou quelqu'un d'autre compile votre code :
Cargo va utiliser uniquement les versions de dépendances que vous avez
utilisées jusqu'à ce que vous indiquiez le contraire.
Par exemple, que se passe-t-il si la semaine prochaine, la version 0.5.6 de la
*crate* `rand` est publiée et qu'elle apporte une correction importante, mais
aussi qu'elle produit une régression qui va casser votre code ?

<!--
The answer to this problem is the *Cargo.lock* file, which was created the
first time you ran `cargo build` and is now in your *guessing_game* directory.
When you build a project for the first time, Cargo figures out all the
versions of the dependencies that fit the criteria and then writes them to
the *Cargo.lock* file. When you build your project in the future, Cargo will
see that the *Cargo.lock* file exists and use the versions specified there
rather than doing all the work of figuring out versions again. This lets you
have a reproducible build automatically. In other words, your project will
remain at `0.5.5` until you explicitly upgrade, thanks to the *Cargo.lock*
file.
-->

La réponse à ce problème est le fichier *Cargo.lock*, qui a été créé la première
fois que vous avez utilisé `cargo build` et qui se trouve désormais dans votre
dossier *guessing_game*. Quand vous compilez un projet pour la première fois,
Cargo détermine toutes les versions de dépendances qui correspondent à vos
critères et les écrit dans le fichier *Cargo.lock*.
Quand vous recompilerez votre projet plus tard, Cargo verra que le fichier
*Cargo.lock* existe et utilisera les versions précisées à l'intérieur au lieu
de recommencer à déterminer toutes les versions demandées.
Ceci vous permet d'avoir automatiquement des compilations reproductibles.
En d'autres termes, votre projet va rester sur la version `0.5.5` jusqu'à ce
que vous le mettiez à jour explicitement, grâce au fichier *Cargo.lock*.

<!--
#### Updating a Crate to Get a New Version
-->

#### Mettre à jour une *crate* vers sa nouvelle version

<!--
When you *do* want to update a crate, Cargo provides another command, `update`,
which will ignore the *Cargo.lock* file and figure out all the latest versions
that fit your specifications in *Cargo.toml*. If that works, Cargo will write
those versions to the *Cargo.lock* file.
-->

Lorsque vous souhaitez réellement mettre à jour une *crate*, Cargo vous fournit
une autre commande, `update`, qui va ignorer le fichier *Cargo.lock* et va
rechercher toutes les versions qui correspondent à vos critères dans
*Cargo.toml*. Si cela se passe bien, Cargo va écrire ces versions dans le
fichier *Cargo.lock*.

<!--
But by default, Cargo will only look for versions greater than `0.5.5` and less
than `0.6.0`. If the `rand` crate has released two new versions, `0.5.6` and
`0.6.0`, you would see the following if you ran `cargo update`:
-->

Mais par défaut, Cargo va rechercher uniquement les versions plus grandes que
`0.5.5` et inférieures à `0.6.0`. Si la *crate* `rand` a été publiée en deux
nouvelles versions, `0.5.6` et `0.6.0`, alors vous verrez ceci si vous
lancez `cargo update` :

```text
$ cargo update
    Updating crates.io index
    Updating rand v0.5.5 -> v0.5.6
```

<!--
At this point, you would also notice a change in your *Cargo.lock* file noting
that the version of the `rand` crate you are now using is `0.5.6`.
-->

À partir de ce moment, vous pouvez aussi constater un changement dans le fichier
*Cargo.lock* indiquant que la version de la *crate* `rand` que vous utilisez
maintenant est la `0.5.6`.

<!--
If you wanted to use `rand` version `0.6.0` or any version in the `0.6.x`
series, you’d have to update the *Cargo.toml* file to look like this instead:
-->

Si vous vouliez utiliser `rand` en version `0.4.0` ou toute autre version dans
la série des `0.4.x`, il vous faut mettre à jour le fichier *Cargo.toml* comme
ceci :

```toml
[dependencies]
rand = "0.6.0"
```

<!--
The next time you run `cargo build`, Cargo will update the registry of crates
available and reevaluate your `rand` requirements according to the new version
you have specified.
-->

La prochaine fois que vous lancerez `cargo build`, Cargo mettra à jour son
registre de *crates* disponibles et réévaluera vos exigences vis-à-vis de `rand`
selon la nouvelle version que vous avez spécifiée.

<!--
There’s a lot more to say about [Cargo][doccargo]<!-- ignore -- > and [its
ecosystem][doccratesio]<!-- ignore -- > which we’ll discuss in Chapter 14, but
for now, that’s all you need to know. Cargo makes it very easy to reuse
libraries, so Rustaceans are able to write smaller projects that are assembled
from a number of packages.
-->

Il y a encore plus à dire à propos de [Cargo][doccargo]<!-- ignore --> et de
[son écosystème][doccratesio]<!-- ignore --> que nous aborderons au chapitre 14,
mais pour l'instant, c'est tout ce qu'il vous faut savoir. Cargo
facilite la réutilisation des bibliothèques, pour que les Rustacés soient
capables d'écrire des petits projets issus d'un assemblage d'un certain
nombre de paquets.

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

<!--
### Generating a Random Number
-->

### Générer un nombre aléatoire

<!--
Now that you’ve added the `rand` crate to *Cargo.toml*, let’s start using
`rand`. The next step is to update *src/main.rs*, as shown in Listing 2-3.
-->

Maintenant que vous avez ajouté la *crate* `rand` dans *Cargo.toml*, commençons
à utiliser `rand`. La prochaine étape est de modifier *src/main.rs* comme dans
l'encart 2-3.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```
-->

```rust,ignore
use std::io;
use rand::Rng;

fn main() {
    println!("Devinez le nombre !");

    let nombre_secret = rand::thread_rng().gen_range(1, 101);

    println!("Le nombre secret est : {}", nombre_secret);

    println!("Veuillez entrer un nombre.");

    let mut supposition = String::new();

    io::stdin().read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");

    println!("Votre nombre : {}", supposition);
}
```

<!--
<span class="caption">Listing 2-3: Adding code to generate a random
number</span>
-->

<span class="caption">Encart 2-3 : Ajout du code pour générer un nombre
aléatoire</span>

<!--
First, we add a `use` line: `use rand::Rng`. The `Rng` trait defines
methods that random number generators implement, and this trait must be in
scope for us to use those methods. Chapter 10 will cover traits in detail.
-->

D'abord, nous avons ajouté une ligne `use` : `use rand::Rng`. Le *trait*
`Rng` définit les méthodes implémentées par les générateurs de nombres
aléatoires, et ce *trait* doit être accessible à notre code pour qu'on puisse
utiliser ces méthodes. Le chapitre 10 expliquera plus en détail les *traits*.

<!--
Next, we’re adding two lines in the middle. The `rand::thread_rng` function
will give us the particular random number generator that we’re going to use:
one that is local to the current thread of execution and seeded by the
operating system. Then we call the `gen_range` method on the random number
generator. This method is defined by the `Rng` trait that we brought into
scope with the `use rand::Rng` statement. The `gen_range` method takes two
numbers as arguments and generates a random number between them. It’s inclusive
on the lower bound but exclusive on the upper bound, so we need to specify `1`
and `101` to request a number between 1 and 100.
-->

Ensuite, nous ajoutons deux lignes au milieu. La fonction `rand::thread_rng`
nous fournit le générateur de nombres aléatoires particulier que nous allons
utiliser : il est propre au fil d'exécution courant et généré par le
système d'exploitation. Ensuite, nous appelons la méthode `gen_range` sur le
générateur de nombres aléatoires. Cette méthode est définie par le *trait* `Rng`
que nous avons importé avec l'instruction `use rand::Rng`. La méthode
`gen_range` prend deux nombres en paramètres et génère un nombre aléatoire entre
ces deux bornes. Elle inclut la borne inférieure mais exclut la borne
supérieure, nous avons donc besoin de préciser `1` et `101` pour demander un
nombre entre 1 et 100.

<!--
> Note: You won’t just know which traits to use and which methods and functions
> to call from a crate. Instructions for using a crate are in each crate’s
> documentation. Another neat feature of Cargo is that you can run the `cargo
> doc --open` command, which will build documentation provided by all of your
> dependencies locally and open it in your browser. If you’re interested in
> other functionality in the `rand` crate, for example, run `cargo doc --open`
> and click `rand` in the sidebar on the left.
-->

> Remarque : vous ne pourrez pas deviner quels *traits*, méthodes et
> fonctions utiliser avec une *crate*. Les instructions pour l'utilisation
> d'une *crate* se trouvent dans la documentation propre à chaque *crate*. Une
> autre fonctionnalité intéressante de Cargo est que vous pouvez utiliser la
> commande `cargo doc --open`, qui va construire localement la documentation
> intégrée par toutes vos dépendances et va l'ouvrir dans votre navigateur.
> Si vous vous intéressez à d'autres fonctionnalités de la *crate* `rand`,
> par exemple, vous pouvez lancer `cargo doc --open` et cliquer sur `rand`
> dans la barre latérale sur la gauche.

<!--
The second line that we added to the middle of the code prints the secret
number. This is useful while we’re developing the program to be able to test
it, but we’ll delete it from the final version. It’s not much of a game if the
program prints the answer as soon as it starts!
-->

La seconde ligne que nous avons ajoutée au milieu du code affiche le nombre
secret. C'est pratique lors du développement pour pouvoir le tester, mais nous
l'enlèverons dans la version finale. Ce n'est pas vraiment un jeu si le
programme affiche la réponse dès qu'il démarre !

<!--
Try running the program a few times:
-->

Essayez de lancer le programme plusieurs fois :

<!--
```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4
$ cargo run
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```
-->

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Devinez le nombre !
Le nombre secret est : 7
Veuillez entrer un nombre.
4
Votre nombre : 4
$ cargo run
     Running `target/debug/guessing_game`
Devinez le nombre !
Le nombre secret est : 83
Veuillez entrer un nombre.
5
Votre nombre : 5
```

<!--
You should get different random numbers, and they should all be numbers between
1 and 100. Great job!
-->

Vous devriez obtenir des nombres aléatoires différents, et ils devraient être
tous compris entre 1 et 100. Beau travail !

<!--
## Comparing the Guess to the Secret Number
-->

## Comparer le nombre saisi au nombre secret

<!--
Now that we have user input and a random number, we can compare them. That step
is shown in Listing 2-4. Note that this code won’t compile quite yet, as we
will explain.
-->

Maintenant que nous avons une saisie utilisateur et un nombre aléatoire, nous
pouvons les comparer. Cette étape est écrite dans l'encart 2-4. Sachez toutefois
que le code ne se compile pas encore, nous allons l'expliquer par la suite.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // ---snip---

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```
-->

```rust,ignore,does_not_compile
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // --- code inchangé masqué ici ---

    println!("Votre nombre : {}", supposition);

    match supposition.cmp(&nombre_secret) {
        Ordering::Less => println!("C'est plus !"),
        Ordering::Greater => println!("C'est moins !"),
        Ordering::Equal => println!("Vous avez gagné !"),
    }
}
```

<!--
<span class="caption">Listing 2-4: Handling the possible return values of
comparing two numbers</span>
-->

<span class="caption">Encart 2-4 : Traitement des valeurs possibles saisies en
comparant les deux nombres</span>

<!--
The first new bit here is another `use` statement, bringing a type called
`std::cmp::Ordering` into scope from the standard library. Like `Result`,
`Ordering` is another enum, but the variants for `Ordering` are `Less`,
`Greater`, and `Equal`. These are the three outcomes that are possible when you
compare two values.
-->

La première nouveauté ici est une nouvelle utilisation de l'instruction `use`,
qui importe `std::cmp::Ordering` à portée de notre code depuis la
bibliothèque standard. Comme `Result`, `Ordering` est une autre énumération,
mais les variantes pour `Ordering` sont `Less` *(inférieur)*, `Greater`
*(supérieur)* et `Equal` *(égal)*.
Ce sont les trois issues possibles lorsqu'on compare deux valeurs.

<!--
Then we add five new lines at the bottom that use the `Ordering` type. The
`cmp` method compares two values and can be called on anything that can be
compared. It takes a reference to whatever you want to compare with: here it’s
comparing the `guess` to the `secret_number`. Then it returns a variant of the
`Ordering` enum we brought into scope with the `use` statement. We use a
[`match`][match]<!-- ignore -- > expression to decide what to do next based on
which variant of `Ordering` was returned from the call to `cmp` with the values
in `guess` and `secret_number`.
-->

Ensuite, nous ajoutons cinq nouvelles lignes à la fin qui utilisent le type
`Ordering`. La méthode `cmp` compare deux valeurs et peut être appelée sur
tout ce qui peut être comparé. Elle prend en paramètre une référence de ce qu'on
veut comparer : ici, nous voulons comparer `supposition` et `nombre_secret`.
Ensuite, cela retourne une variante de l'énumération `Ordering` que nous avons
importée avec l'instruction `use`. Nous utilisons une expression
[`match`][match]<!-- ignore --> pour décider quoi faire ensuite en fonction de
quelle variante de `Ordering` a été retournée à l'appel de `cmp` avec
`supposition` et `nombre_secret`.

[match]: ch06-02-match.html

<!--
A `match` expression is made up of *arms*. An arm consists of a *pattern* and
the code that should be run if the value given to the beginning of the `match`
expression fits that arm’s pattern. Rust takes the value given to `match` and
looks through each arm’s pattern in turn. The `match` construct and patterns
are powerful features in Rust that let you express a variety of situations your
code might encounter and make sure that you handle them all. These features
will be covered in detail in Chapter 6 and Chapter 18, respectively.
-->

Une expression `match` est composée de *branches*. Une branche est constituée
d'un *motif (pattern)* et du code qui sera exécuté si la valeur donnée
au début de l'expression `match` correspond bien au motif de cette
branche. Rust prend la valeur donnée à `match` et la compare au motif de
chaque branche à tour de rôle.
La structure de contrôle `match` et les motifs sont des
fonctionnalités puissantes de Rust qui vous permettent de décrire une multitude
de scénarios que votre code peut rencontrer et de s'assurer que vous les gérez
toutes. Ces fonctionnalités seront expliquées plus en détail respectivement dans
le chapitre 6 et le chapitre 18.

<!--
Let’s walk through an example of what would happen with the `match` expression
used here. Say that the user has guessed 50 and the randomly generated secret
number this time is 38. When the code compares 50 to 38, the `cmp` method will
return `Ordering::Greater`, because 50 is greater than 38. The `match`
expression gets the `Ordering::Greater` value and starts checking each arm’s
pattern. It looks at the first arm’s pattern, `Ordering::Less`, and sees that
the value `Ordering::Greater` does not match `Ordering::Less`, so it ignores
the code in that arm and moves to the next arm. The next arm’s pattern,
`Ordering::Greater`, *does* match `Ordering::Greater`! The associated code in
that arm will execute and print `Too big!` to the screen. The `match`
expression ends because it has no need to look at the last arm in this scenario.
-->

Voyons un exemple de ce qui se passerait avec l'expression `match` utilisée
ici. Disons que l'utilisateur a saisi le nombre 50 et que le nombre secret
généré aléatoirement a cette fois-ci comme valeur 38. Quand le code compare 50 à
38, la méthode `cmp` va retourner `Ordering::Greater`, car 50 est plus grand
que 38. L'expression `match` obtient la valeur `Ordering::Greater` et commence à
vérifier le motif de chaque branche.
Elle consulte le motif de la première branche, `Ordering::Less` et remarque que
la valeur `Ordering::Greater` ne correspond pas au motif `Ordering::Less` ;
elle ignore donc le code de cette branche et passe à la suivante.
Le motif de la branche suivante, `Ordering::Greater`, correspond à
`Ordering::Greater` ! Le code associé à cette branche va être exécuté et va
afficher à l'écran `C'est moins !`. L'expression `match` se termine ensuite, car
elle n'a pas besoin de consulter les autres branches de ce scénario.

<!--
However, the code in Listing 2-4 won’t compile yet. Let’s try it:
-->

Cependant, notre code dans l'encart 2-4 ne compile pas encore. Essayons de le
faire :

<!-- markdownlint-disable -->
<!--
```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  -- > src/main.rs:23:21
   |
23 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `std::string::String`, found integer
   |
   = note: expected type `&std::string::String`
   = note:    found type `&{integer}`

error: aborting due to previous error
Could not compile `guessing_game`.
```
-->
<!-- markdownlint-restore -->

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  -- > src/main.rs:23:21
   |
23 |     match supposition.cmp(&nombre_secret) {
   |                           ^^^^^^^^^^^^^^ expected struct `std::string::String`, found integer
   |
   = note: expected type `&std::string::String`
   = note:    found type `&{integer}`

error: aborting due to previous error
Could not compile `guessing_game`.
```

<!--
The core of the error states that there are *mismatched types*. Rust has a
strong, static type system. However, it also has type inference. When we wrote
`let mut guess = String::new()`, Rust was able to infer that `guess` should be
a `String` and didn’t make us write the type. The `secret_number`, on the other
hand, is a number type. A few number types can have a value between 1 and 100:
`i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a 64-bit
number; as well as others. Rust defaults to an `i32`, which is the type of
`secret_number` unless you add type information elsewhere that would cause Rust
to infer a different numerical type. The reason for the error is that Rust
cannot compare a string and a number type.
-->

Le message d'erreur nous indique que nous sommes dans un cas de types non
compatibles *(mismatched types)*. Rust a un système de types fort et statique.
Cependant, il a aussi une fonctionnalité d'inférence de type. Quand nous avons
écrit `let mut supposition = String::new()`, Rust a pu en déduire que
`supposition` devait être une `String` et ne nous a pas demandé d'écrire le
type. D'autre part, `nombre_secret` est d'un type de nombre.
Quelques types de nombres peuvent avoir une valeur entre 1 et 100 : `i32`, un
nombre entier encodé sur 32 bits ; `u32`, un nombre entier de 32 bits non signé
(positif ou nul) ; `i64`, un nombre entier encodé sur 64 bits ; parmi
tant d'autres. Rust utilise par défaut un `i32`, qui est le type
de `nombre_secret`, à moins que vous précisiez quelque part une information de
type qui amènerait Rust à inférer un type de nombre différent. La raison de
cette erreur est que Rust ne peut pas comparer une chaîne de caractères à un
nombre.

<!--
Ultimately, we want to convert the `String` the program reads as input into a
real number type so we can compare it numerically to the secret number. We can
do that by adding the following two lines to the `main` function body:
-->

Au bout du compte, nous voulons convertir la `String` que le programme récupère
de la saisie utilisateur en un nombre, pour qu'on puisse la comparer
numériquement au nombre secret. Nous pouvons faire ceci en ajoutant les deux
lignes suivantes dans le corps de la fonction `main` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
// --snip--

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```
-->

```rust,ignore
// -- code inchangé masqué ici --

    let mut supposition = String::new();

    io::stdin().read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");

    let supposition: u32 = supposition.trim().parse()
        .expect("Veuillez entrer un nombre !");

    println!("Votre nombre : {}", supposition);

    match supposition.cmp(&nombre_secret) {
        Ordering::Less => println!("C'est plus !"),
        Ordering::Greater => println!("C'est moins !"),
        Ordering::Equal => println!("Vous avez gagné !"),
    }
}
```

<!--
The two new lines are:
-->

Les deux nouvelles lignes sont :

<!--
```rust,ignore
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```
-->

```rust,ignore
let supposition: u32 = supposition.trim().parse()
    .expect("Veuillez entrer un nombre !");
```

<!--
We create a variable named `guess`. But wait, doesn’t the program already have
a variable named `guess`? It does, but Rust allows us to *shadow* the previous
value of `guess` with a new one. This feature is often used in situations in
which you want to convert a value from one type to another type. Shadowing lets
us reuse the `guess` variable name rather than forcing us to create two unique
variables, such as `guess_str` and `guess` for example. (Chapter 3 covers
shadowing in more detail.)
-->

Nous créons une variable qui s'appelle `supposition`. Mais attendez, le
programme n'a-t-il pas déjà une variable qui s'appelle `supposition` ?
C'est le cas, mais Rust nous permet de *masquer* la valeur précédente de
`supposition` avec une nouvelle.
Cette fonctionnalité est souvent utilisée dans des
situations où on veut convertir une valeur d'un type à un autre.
Le masquage *(shadowing)* nous permet de réutiliser le nom de variable
`supposition`, plutôt que de nous forcer à créer deux variables distinctes,
telles que `supposition_str` et `supposition` par exemple.
(Le chapitre 3 expliquera plus en détail le masquage.)

<!--
We bind `guess` to the expression `guess.trim().parse()`. The `guess` in the
expression refers to the original `guess` that was a `String` with the input in
it. The `trim` method on a `String` instance will eliminate any whitespace at
the beginning and end. Although `u32` can contain only numerical characters,
the user must press <span class="keystroke">enter</span> to satisfy
`read_line`. When the user presses <span class="keystroke">enter</span>, a
newline character is added to the string. For example, if the user types <span
class="keystroke">5</span> and presses <span class="keystroke">enter</span>,
`guess` looks like this: `5\n`. The `\n` represents “newline,” the result of
pressing <span class="keystroke">enter</span>. The `trim` method eliminates
`\n`, resulting in just `5`.
-->

Nous lions `supposition` à l'expression `supposition.trim().parse()`. Le
`supposition` dans l'expression se réfère au `supposition` initial qui était une
`String` contenant la saisie utilisateur. La méthode `trim` sur une instance
de `String` va enlever les espaces et autres *whitespaces* au début et à la fin.
Même si `u32` ne peut contenir que des chiffres, l'utilisateur doit
appuyer sur <span class="keystroke">entrée</span> pour mettre fin à `read_line`.
Lorsque l'utilisateur appuie sur
<span class="keystroke">entrée</span>, un caractère de fin de ligne
est ajouté à la chaîne de caractères. Par exemple, si l'utilisateur écrit
<span class="keystroke">5</span> et appuie sur <span class="keystroke">
entrée</span>, `supposition` aura alors cette valeur : `5\n`.
Le `\n` représente une fin de ligne, qui résulte de l'appui sur
<span class="keystroke">entrée</span>. La méthode `trim` enlève `\n`, il ne
reste donc plus que `5`.

<!--
The [`parse` method on strings][parse]<!-- ignore -- > parses a string into some
kind of number. Because this method can parse a variety of number types, we
need to tell Rust the exact number type we want by using `let guess: u32`. The
colon (`:`) after `guess` tells Rust we’ll annotate the variable’s type. Rust
has a few built-in number types; the `u32` seen here is an unsigned, 32-bit
integer. It’s a good default choice for a small positive number. You’ll learn
about other number types in Chapter 3. Additionally, the `u32` annotation in
this example program and the comparison with `secret_number` means that Rust
will infer that `secret_number` should be a `u32` as well. So now the
comparison will be between two values of the same type!
-->

La [méthode `parse` des chaînes de caractères][parse]<!-- ignore --> interprète
une chaîne de caractères en une sorte de nombre. Comme cette méthode peut
interpréter plusieurs types de nombres, nous devons indiquer à Rust le type
exact de nombre que nous voulons en utilisant `let supposition: u32`.
Le deux-points (`:`) après `supposition` indique à Rust que nous voulons
préciser le type de la variable.
Rust embarque quelques types de nombres ; le `u32` utilisé ici est un
entier non signé sur 32 bits.
C'est un bon choix par défaut pour un petit nombre positif.
Vous découvrirez d'autres types de nombres dans le chapitre 3.
De plus, l'annotation `u32` dans ce programme d'exemple et la
comparaison avec `nombre_secret` permet à Rust d'en déduire que `nombre_secret`
doit être lui aussi un `u32`. Donc maintenant, la comparaison se fera
entre deux valeurs du même type !

<!-- [parse]: ../std/primitive.str.html#method.parse -->
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse

<!--
The call to `parse` could easily cause an error. If, for example, the string
contained `A👍%`, there would be no way to convert that to a number. Because it
might fail, the `parse` method returns a `Result` type, much as the `read_line`
method does (discussed earlier in [“Handling Potential Failure with the
`Result` Type”](#handling-potential-failure-with-the-result-type)<!-- ignore
-- >). We’ll treat this `Result` the same way by using the `expect` method
again. If `parse` returns an `Err` `Result` variant because it couldn’t create
a number from the string, the `expect` call will crash the game and print the
message we give it. If `parse` can successfully convert the string to a number,
it will return the `Ok` variant of `Result`, and `expect` will return the
number that we want from the `Ok` value.
-->

L'utilisation de `parse` peut facilement mener à une erreur. Si par exemple,
le texte contient `A👍%`, il ne sera pas possible de le convertir en nombre.
Comme elle peut échouer, la méthode `parse` retourne un type `Result`, comme
celui que la méthode `read_line` retourne (comme nous l'avons vu plus tôt dans
[“Gérer les erreurs potentielles avec le type
`Result`”](#gérer-les-erreurs-potentielles-avec-le-type-result)<!-- ignore-->).
Nous allons gérer ce `Result` de la même manière, avec à nouveau la méthode
`expect`. Si `parse` retourne une variante `Err` de `Result` car elle ne peut
pas créer un nombre à partir de la chaîne de caractères, l'appel à
`expect` va faire planter le jeu et va afficher le message que nous lui avons
passé en paramètre. Si `parse` arrive à convertir la chaîne de caractères en
nombre, alors elle retournera la variante `Ok` de `Result`, et `expect` va
retourner le nombre qu'il nous faut qui est stocké dans la variante `Ok`.

<!--
Let’s run the program now!
-->

Exécutons ce programme, maintenant !

<!--
```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```
-->

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running `target/debug/guessing_game`
Devinez le nombre !
Le nombre secret est : 58
Veuillez entrer un nombre.
  76
Votre nombre : 76
C'est moins !
```

<!--
Nice! Even though spaces were added before the guess, the program still figured
out that the user guessed 76. Run the program a few times to verify the
different behavior with different kinds of input: guess the number correctly,
guess a number that is too high, and guess a number that is too low.
-->

Très bien !
Même si des espaces ont été ajoutées avant la supposition, le programme a quand
même compris que l'utilisateur a saisi 76. Lancez le programme plusieurs
fois pour vérifier qu'il se comporte correctement avec différentes saisies :
devinez le nombre correctement, saisissez un nombre qui est trop grand, et
saisissez un nombre qui est trop petit.

<!--
We have most of the game working now, but the user can make only one guess.
Let’s change that by adding a loop!
-->

La majeure partie du jeu fonctionne désormais, mais l'utilisateur ne peut faire
qu'une seule supposition. Corrigeons cela en ajoutant une boucle !

<!--
## Allowing Multiple Guesses with Looping
-->

## Permettre plusieurs suppositions avec les boucles

<!--
The `loop` keyword creates an infinite loop. We’ll add that now to give users
more chances at guessing the number:
-->

Le mot-clé `loop` crée une boucle infinie. C'est ce que nous allons ajouter pour
donner aux utilisateurs plus de chances de deviner le nombre :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
// --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```
-->

```rust,ignore
// -- code inchangé masqué ici --

    println!("Le nombre secret est : {}", nombre_secret);

    loop {
        println!("Veuillez entrer un nombre.");

        // -- code inchangé masqué ici --

        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => println!("Vous avez gagné !"),
        }
    }
}
```

<!--
As you can see, we’ve moved everything into a loop from the guess input prompt
onward. Be sure to indent the lines inside the loop another four spaces each
and run the program again. Notice that there is a new problem because the
program is doing exactly what we told it to do: ask for another guess forever!
It doesn’t seem like the user can quit!
-->

Comme vous pouvez le remarquer, nous avons déplacé dans une boucle tout le
code à partir de l'invite à entrer le nombre. Assurez-vous d'indenter
correctement les lignes dans la boucle avec quatre nouvelles espaces pour
chacune, et lancez à nouveau le programme. Notez bien qu'il y a un nouveau
problème, car le programme fait exactement ce que nous lui avons demandé de
faire : demander un nombre à l'infini ! Il n'est pas possible pour l'utilisateur
de l'arrêter !

<!--
The user could always interrupt the program by using the keyboard shortcut <span
class="keystroke">ctrl-c</span>. But there’s another way to escape this
insatiable monster, as mentioned in the `parse` discussion in [“Comparing the
Guess to the Secret Number”](#comparing-the-guess-to-the-secret-number)<!--
ignore -- >: if the user enters a non-number answer, the program will crash. The
user can take advantage of that in order to quit, as shown here:
-->

L'utilisateur pourrait quand même interrompre le programme en utilisant le
raccourci clavier <span class="keystroke">ctrl-c</span>.
Mais il y a une autre façon d'échapper à ce monstre insatiable, comme nous
l'avons abordé dans la partie [“Comparer le nombre saisi au nombre
secret”](#comparer-le-nombre-saisi-au-nombre-secret)<!-- ignore --> : si
l'utilisateur saisit quelque chose qui n'est pas un nombre, le programme va
planter. L'utilisateur peut procéder ainsi pour le quitter, comme ci-dessous :

<!-- markdownlint-disable -->
<!--
```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/guess` (exit code: 101)
```
-->
<!-- markdownlint-restore -->

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/guessing_game`
Devinez le nombre !
Le nombre secret est : 59
Veuillez entrer un nombre.
45
Votre nombre : 45
C'est plus !
Veuillez entrer un nombre.
60
Votre nombre : 60
C'est moins !
Veuillez entrer un nombre.
59
Votre nombre : 59
Vous avez gagné !
Veuillez entrer un nombre.
quitter
thread 'main' panicked at 'Veuillez entrer un nombre !: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/supposition` (exit code: 101)
```

<!--
Typing `quit` actually quits the game, but so will any other non-number input.
However, this is suboptimal to say the least. We want the game to automatically
stop when the correct number is guessed.
-->

Taper `quitter` ferme bien le jeu, mais toute autre saisie qui n'est pas un
nombre le ferait aussi.
Cependant, ce mécanisme laisse franchement à désirer. Nous voudrions que le jeu
s'arrête automatiquement lorsque le bon nombre est deviné.

<!--
### Quitting After a Correct Guess
-->

### Arrêter le programme après avoir gagné

<!--
Let’s program the game to quit when the user wins by adding a `break` statement:
-->

Faisons en sorte que le jeu s'arrête quand le joueur gagne en ajoutant
l'instruction `break` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
// --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```
-->

```rust,ignore
// -- code inchangé masqué ici --

        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}
```

<!--
Adding the `break` line after `You win!` makes the program exit the loop when
the user guesses the secret number correctly. Exiting the loop also means
exiting the program, because the loop is the last part of `main`.
-->

Ajouter la ligne `break` après `Vous avez gagné !` fait sortir le programme de
la boucle quand le joueur a correctement deviné le nombre secret. Et quitter la
boucle veut aussi dire terminer le programme, car ici la boucle est la dernière
partie de `main`.

<!--
### Handling Invalid Input
-->

### Gérer les saisies invalides

<!--
To further refine the game’s behavior, rather than crashing the program when
the user inputs a non-number, let’s make the game ignore a non-number so the
user can continue guessing. We can do that by altering the line where `guess`
is converted from a `String` to a `u32`, as shown in Listing 2-5.
-->

Pour améliorer le comportement du jeu, plutôt que de faire planter le programme
quand l'utilisateur saisit quelque chose qui n'est pas un nombre, faisons en
sorte que le jeu ignore ce qui n'est pas un nombre afin que l'utilisateur puisse
continuer à essayer de deviner. Nous pouvons faire ceci en modifiant la ligne où
`supposition` est converti d'une `String` en un `u32`, comme dans l'encart 2-5 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
// --snip--

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {}", guess);

// --snip--
```
-->

```rust,ignore
// -- code inchangé masqué ici --

io::stdin().read_line(&mut supposition)
    .expect("Échec de la lecture de l'entrée utilisateur");

let supposition: u32 = match supposition.trim().parse() {
    Ok(nombre) => nombre,
    Err(_) => continue,
};

println!("Votre nombre : {}", supposition);

// -- code inchangé masqué ici --
```

<!--
<span class="caption">Listing 2-5: Ignoring a non-number guess and asking for
another guess instead of crashing the program</span>
-->

<span class="caption">Encart 2-5 : Ignorer une saisie qui n'est pas un nombre
et demander un nouveau nombre plutôt que de faire planter le programme</span>

<!--
Switching from an `expect` call to a `match` expression is how you generally
move from crashing on an error to handling the error. Remember that `parse`
returns a `Result` type and `Result` is an enum that has the variants `Ok` or
`Err`. We’re using a `match` expression here, as we did with the `Ordering`
result of the `cmp` method.
-->

Remplacer un appel à `expect` par une expression `match` est la technique qu'on
utilise en général pour passer d'une erreur qui fait planter le programme à une
erreur proprement gérée. N'oubliez pas que
`parse` retourne un type `Result` et que `Result` est une énumération qui a pour
variantes `Ok` ou `Err`. Nous utilisons ici une expression `match` comme nous
l'avons déjà fait avec le résultat de type `Ordering` de la méthode `cmp`.

<!--
If `parse` is able to successfully turn the string into a number, it will
return an `Ok` value that contains the resulting number. That `Ok` value will
match the first arm’s pattern, and the `match` expression will just return the
`num` value that `parse` produced and put inside the `Ok` value. That number
will end up right where we want it in the new `guess` variable we’re creating.
-->

Si `parse` arrive à convertir la chaîne de caractères en nombre, cela va
retourner la variante `Ok` qui contient le nombre qui en résulte. Cette variante
va correspondre au motif de la première branche, et l'expression `match` va
simplement retourner la valeur de `nombre` que `parse` a trouvée et qu'elle a
mise dans la variante `Ok`.
Ce nombre va se retrouver là où nous en avons besoin,
dans la variable `supposition` que nous sommes en train de créer.

<!--
If `parse` is *not* able to turn the string into a number, it will return an
`Err` value that contains more information about the error. The `Err` value
does not match the `Ok(num)` pattern in the first `match` arm, but it does
match the `Err(_)` pattern in the second arm. The underscore, `_`, is a
catchall value; in this example, we’re saying we want to match all `Err`
values, no matter what information they have inside them. So the program will
execute the second arm’s code, `continue`, which tells the program to go to the
next iteration of the `loop` and ask for another guess. So, effectively, the
program ignores all errors that `parse` might encounter!
-->

Si `parse` n'arrive *pas* à convertir la chaîne de caractères en nombre, elle
va retourner la variante `Err` qui contient plus d'informations sur l'erreur. La
variante `Err` ne correspond pas au motif `Ok(nombre)` de la première branche,
mais elle correspond au motif `Err(_)` de la seconde branche. Le tiret bas,
`_`, est une valeur passe-partout ; dans notre exemple, nous disons
que nous voulons correspondre à toutes les valeurs de `Err`, peu importe quelle
information elles ont à l'intérieur d'elles-mêmes. Donc le programme va exécuter
le code de la seconde branche, `continue`, qui indique au programme de se rendre
à la prochaine itération de `loop` et de demander un nouveau nombre. Ainsi, le
programme ignore toutes les erreurs que `parse` pourrait rencontrer !

<!--
Now everything in the program should work as expected. Let’s try it:
-->

Maintenant, le programme devrait fonctionner correctement. Essayons-le :

<!--
```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```
-->

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Devinez le nombre !
Le nombre secret est : 61
Veuillez entrer un nombre.
10
Votre nombre : 10
C'est plus !
Veuillez entrer un nombre.
99
Votre nombre : 99
C'est moins !
Veuillez entrer un nombre.
foo
Veuillez entrer un nombre.
61
Votre nombre : 61
Vous avez gagné !
```

<!--
Awesome! With one tiny final tweak, we will finish the guessing game. Recall
that the program is still printing the secret number. That worked well for
testing, but it ruins the game. Let’s delete the `println!` that outputs the
secret number. Listing 2-6 shows the final code.
-->

Super ! Avec notre petite touche finale, nous avons fini notre jeu de
devinettes. Rappelez-vous que le programme affiche toujours le nombre secret.
C'était pratique pour les tests, mais cela gâche le jeu. Supprimons le
`println!` qui affiche le nombre secret. L'encart 2-6 représente le code final.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```
-->

```rust,ignore
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Devinez le nombre !");

    let nombre_secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Veuillez entrer un nombre.");

        let mut supposition = String::new();

        io::stdin().read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        let supposition: u32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        println!("Votre nombre : {}", supposition);

        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}
```

<!--
<span class="caption">Listing 2-6: Complete guessing game code</span>
-->

<span class="caption">Encart 2-6 : Code complet du jeu de devinettes</span>

<!--
## Summary
-->

## Résumé

<!--
At this point, you’ve successfully built the guessing game. Congratulations!
-->

Si vous êtes arrivé jusqu'ici, c'est que vous avez construit avec succès le jeu
de devinettes. Félicitations !

<!--
This project was a hands-on way to introduce you to many new Rust concepts:
`let`, `match`, methods, associated functions, the use of external crates, and
more. In the next few chapters, you’ll learn about these concepts in more
detail. Chapter 3 covers concepts that most programming languages have, such as
variables, data types, and functions, and shows how to use them in Rust.
Chapter 4 explores ownership, a feature that makes Rust different from other
languages. Chapter 5 discusses structs and method syntax, and Chapter 6
explains how enums work.
-->

Ce projet était une mise en pratique pour vous initier à de nombreux concepts de
Rust : `let`, `match`, les méthodes, les fonctions associées, l'utilisation de
*crates* externes, et bien plus. Dans les prochains chapitres, vous allez en
apprendre plus sur ces concepts. Le chapitre 3 va traiter des concepts utilisés
par la plupart des langages de programmation, comme les variables, les types de
données, et les fonctions, et vous montrera comment les utiliser avec Rust. Le
chapitre 4 expliquera la possession *(ownership)*, qui est une fonctionnalité
qui distingue Rust des autres langages. Le chapitre 5 abordera les structures et
les syntaxes des méthodes, et le chapitre 6 expliquera comment les énumérations
fonctionnent.

<!--
[variables-and-mutability]:
ch03-01-variables-and-mutability.html#variables-and-mutability
-->

[variables-and-mutability]:
ch03-01-variables-and-mutability.html
