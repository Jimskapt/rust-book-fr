<!--
# Programming a Guessing Game
-->

# Programmer le jeu du plus ou du moins

<!--
Let’s jump into Rust by working through a hands-on project together! This
chapter introduces you to a few common Rust concepts by showing you how to use
them in a real program. You’ll learn about `let`, `match`, methods, associated
functions, using external crates, and more! In the following chapters, we’ll
explore these ideas in more detail. In this chapter, you’ll practice the
fundamentals.
-->

Entrons dans le vif du sujet en travaillant ensemble sur un projet concret !
Ce chapitre présente quelques concepts couramment utilisés en Rust en vous
montrant comment les utiliser dans un véritable programme. Nous aborderons
notamment les instructions `let` et `match`, les méthodes et fonctions
associées, l'utilisation des _crates_, et bien plus encore ! Dans les chapitres
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
programmation : _le jeu du plus ou du moins_. Le principe de ce jeu est le
suivant : le programme va tirer au sort un nombre entre 1 et 100. Il invitera
ensuite le joueur à saisir un nombre qu'il pense deviner. Après la saisie, le
programme indiquera si le nombre saisi par le joueur est trop grand ou trop
petit. Si le nombre saisi est le bon, le jeu affichera un message de
félicitations et se fermera.

<!--
## Setting Up a New Project
-->

## Mise en place d'un nouveau projet

<!--
To set up a new project, go to the _projects_ directory that you created in
Chapter 1 and make a new project using Cargo, like so:
-->

Pour créer un nouveau projet, rendez-vous dans le répertoire _projects_ que
vous avez créé au chapitre 1 et utilisez Cargo pour créer votre projet, comme
ceci :

<!--
```console
$ cargo new guessing_game
$ cd guessing_game
```
-->

```console
$ cargo new jeu_du_plus_ou_du_moins
$ cd jeu_du_plus_ou_du_moins
```

<!--
The first command, `cargo new`, takes the name of the project (`guessing_game`)
as the first argument. The second command changes to the new project’s
directory.
-->

La première commande, `cargo new`, prend comme premier argument le nom de notre
projet (`jeu_du_plus_ou_du_moins`). La seconde commande nous déplace dans le
répertoire de notre nouveau projet créé par Cargo.

<!--
Look at the generated _Cargo.toml_ file:
-->

Regardons le fichier _Cargo.toml_ qui a été généré :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

<!--
```toml
{{#include ../listings-sources/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```
-->

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

<!--
As you saw in Chapter 1, `cargo new` generates a “Hello, world!” program for
you. Check out the _src/main.rs_ file:
-->

Comme vous l'avez expérimenté dans le chapitre 1, `cargo new` génère un
programme _“Hello, world!”_ pour vous. Ouvrez le fichier _src/main.rs_ :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

<!--
Now let’s compile this “Hello, world!” program and run it in the same step
using the `cargo run` command:
-->

Maintenant, lançons la compilation de ce programme “Hello, world!” et
son exécution en une seule commande avec `cargo run` :

<!--
```console
{{#include ../listings-sources/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
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
Reopen the _src/main.rs_ file. You’ll be writing all the code in this file.
-->

Ouvrez à nouveau le fichier _src/main.rs_. C'est dans ce fichier que nous
écrirons la totalité de notre code.

<!--
## Processing a Guess
-->

## Traitement d'un nombre saisi

<!--
The first part of the guessing game program will ask for user input, process
that input, and check that the input is in the expected form. To start, we’ll
allow the player to input a guess. Enter the code in Listing 2-1 into
_src/main.rs_.
-->

La première partie du programme consiste à demander au joueur de saisir du
texte, à traiter cette saisie, et à vérifier que la saisie correspond au format
attendu.
Commençons par permettre au joueur de saisir son nombre. Entrez le
code de l'encart 2-1 dans le fichier _src/main.rs_.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
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
`io` input/output library into scope. The `io` library comes from the
standard library, known as `std`:
-->

Ce code contient beaucoup d'informations, nous allons donc l'analyser petit
à petit. Pour obtenir la saisie utilisateur et ensuite l'afficher, nous avons
besoin d'importer la bibliothèque d'entrée/sortie `io` (initiales
de _input/output_) afin de pouvoir l'utiliser. La bibliothèque `io` provient de
la bibliothèque standard, connue sous le nom de `std` :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

<!--
By default, Rust has a set of items defined in the standard library that it brings
into the scope of every program. This set is called the _prelude_, and you can
see everything in it [in the standard library documentation][prelude].
-->

Par défaut, Rust importe dans la portée de tous les programmes un ensemble de
fonctionnalités définies dans la bibliothèque standard. Cela s'appelle _l'étape
préliminaire (the prelude)_, et vous pouvez en savoir plus dans sa
[documentation de la bibliothèque standard][prelude]<!-- ignore -->.

<!--
If a type you want to use isn’t in the prelude, you have to bring that type
into scope explicitly with a `use` statement. Using the `std::io` library
provides you with a number of useful features, including the ability to accept
user input.
-->

Si vous
voulez utiliser un type qui ne s'y trouve pas, vous devrez l'importer
explicitement avec l'instruction `use`. L'utilisation de la bibliothèque
`std::io` vous apporte de nombreuses fonctionnalités utiles, comme ici la
possibilité de récupérer une saisie utilisateur.

<!--
As you saw in Chapter 1, the `main` function is the entry point into the
program:
-->

Comme vous l'avez vu au chapitre 1, la fonction `main` est le point d'entrée
du programme :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

<!--
The `fn` syntax declares a new function; the parentheses, `()`, indicate there
are no parameters; and the curly bracket, `{`, starts the body of the function.
-->

Le mot clé `fn` déclare une nouvelle fonction ; les parenthèses `()` indiquent
que cette fonction n'accepte aucun paramètre ; et l'accolade ouvrante `{` marque
le début du corps de la fonction.

<!--
As you also learned in Chapter 1, `println!` is a macro that prints a string to
the screen:
-->

Comme vous l'avez également appris au chapitre 1, `println!` est une macro qui
affiche une chaîne de caractères à l'écran :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
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
Next, we’ll create a _variable_ to store the user input, like this:
-->

Ensuite, on crée une _variable_ pour stocker la saisie de l'utilisateur, comme
ceci :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

<!--
Now the program is getting interesting! There’s a lot going on in this little
line. We use the `let` statement to create the variable. Here’s another example:
-->

Le programme commence à devenir intéressant ! Il se passe beaucoup de choses
dans cette petite ligne. Nous utilisons l'instruction `let` pour créer la
variable. Voici un autre exemple :

<!--
```rust,ignore
let apples = 5;
```
-->

```rust,ignore
let pommes = 5;
```

<!--
This line creates a new variable named `apples` and binds it to the value `5`.
In Rust, variables are immutable by default , meaning once we give the variable
a value, the value won’t change. We’ll be discussing this concept in detail in
the [“Variables and Mutability”][variables-and-mutability]<!-- ignore -- >
section in Chapter 3. To make a variable mutable, we add `mut` before the
variable name:
-->

Cette ligne permet de créer une nouvelle variable nommée `pommmes` et à lui
assigner la valeur `5`. Par défaut en Rust, les variables sont immuables, ce qui
implique qu'à partir du moment où l'on donne une valeur à une variable, cette
valeur ne changera pas. Nous aborderons plus en détail cette notion dans la
section [“Variables et Mutabilité”][variables-and-mutability]<!-- ignore --> 
au chapitre 3. Pour rendre une variable mutable _(c'est-à-dire modifiable)_,
nous ajoutons `mut` devant le nom de la variable :

<!--
```rust,ignore
let apples = 5; // immutable
let mut bananas = 5; // mutable
```
-->

```rust
let pommes = 5; // immuable
let mut bananes = 5; // mutable, modifiable
```

<!--
> Note: The `//` syntax starts a comment that continues until the end of the
> line. Rust ignores everything in comments. We’ll discuss comments in more
> detail in [Chapter 3][comments]<!-- ignore -- >.
-->

> Remarque : La syntaxe `//` permet de commencer un commentaire qui s'étend
> jusqu'à la fin de la ligne. Rust ignore tout ce qu'il y a dans un
> commentaire. Nous verrons plus en détail les commentaires dans le
> [chapitre 3][comments]<!-- ignore -->.

<!--
Returning to the guessing game program, you now know that `let mut guess` will
introduce a mutable variable named `guess`. The equal sign (`=`) tells Rust we
want to bind something to the variable now. On the right of the equal sign is
the value that `guess` is bound to, which is the result of calling
`String::new`, a function that returns a new instance of a `String`.
[`String`][string]<!-- ignore -- > is a string type provided by the standard
library that is a growable, UTF-8 encoded bit of text.
-->

Lorsque vous revenez sur le jeu du plus ou du moins, vous comprenez donc
maintenant que la ligne `let mut supposition` permet de créer une variable
mutable nommée `supposition`. Le signe égal (`=`) indique à Rust que nous
voulons désormais lier quelquechose à la variable. À la droite du signe égal,
nous avons la valeur liée à `supposition`, qui est ici le résultat de
l'utilisation de `String::new`, qui est une fonction qui retourne une nouvelle
instance de `String`.
[`String`][string]<!-- ignore --> est un type de chaîne de caractères fourni
par la bibliothèque standard, qui est une portion de texte encodée en UTF-8 et
dont la longueur peut augmenter.

<!--
The `::` syntax in the `::new` line indicates that `new` is an associated
function of the `String` type. An _associated function_ is a function that’s
implemented on a type, in this case `String`. This `new` function creates a
new, empty string. You’ll find a `new` function on many types because it’s a
common name for a function that makes a new value of some kind.
-->

La syntaxe `::` dans `String::new()` indique que `new` est une fonction
associée au type `String`. Une _fonction associée_ est une fonction qui est
implémentée sur un type, ici `String`. Cette fonction `new` crée une nouvelle
chaîne de caractères vide, une nouvelle `String`. Vous trouverez fréquemment
une fonction `new` sur d'autres types car c'est un nom souvent donné à une
fonction qui crée une nouvelle valeur ou instance d'un type.

<!--
In full, the `let mut guess = String::new();` line has created a mutable
variable that is currently bound to a new, empty instance of a `String`. Whew!
-->

En définitif, la ligne `let mut supposition = String::new();` crée une nouvelle
variable mutable qui contient une nouvelle chaîne de caractères vide, une
instance de `String`. Ouf !

<!--
### Receiving User Input
-->

### Recueillir la saisie utilisateur

<!--
Recall that we included the input/output functionality from the standard
library with `use std::io;` on the first line of the program. Now we’ll call
the `stdin` function from the `io` module, which will allow us to handle user
input:
-->

Rappelez-vous que nous avons importé les fonctionnalités d'entrée/sortie de la
bibliothèque standard avec `use std::io;` à la première ligne de notre
programme. Nous allons maintenant appeler la fonction `stdin` du module `io`,
qui va nous permettre de traiter la saisie utilisateur :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

<!--
If we hadn’t imported the `io` module with `use std::io;` at the beginning of
the program, we could still use the function by writing this function call as
`std::io::stdin`. The `stdin` function returns an instance of
[`std::io::Stdin`][iostdin]<!-- ignore -- >, which is a type that represents a
handle to the standard input for your terminal.
-->

Si nous n'avions pas importé le module `io` avec `use std::io` au début
du programme, on aurait toujours pu utiliser la fonction en écrivant l'appel à
la fonction de cette manière : `std::io::stdin`. La fonction `stdin` retourne
une instance de [`std::io::Stdin`][iostdin]<!-- ignore -->, qui est un type qui
représente une référence abstraite *(handle)* vers l'entrée standard du
terminal dans lequel vous avez lancé le programme.

<!--
Next, the line `.read_line(&mut guess)` calls the [`read_line`][read_line]<!--
ignore -- > method on the standard input handle to get input from the user.
We’re also passing `&mut guess` as the argument to `read_line` to tell it what
string to store the user input in. The full job of `read_line` is to take
whatever the user types into standard input and append that into a string
(without overwriting its contents), so we therefore pass that string as an
argument. The string argument needs to be mutable so that the method can change
the string’s content.
-->

Ensuite, la ligne `.read_line(&mut supposition)` appelle la méthode
[`read_line`][read_line]<!-- ignore --> sur l'entrée standard afin d'obtenir
la saisie utilisateur.
Nous passons aussi `&mut supposition` en argument de `read_line` pour lui
indiquer dans quelle chaîne de caractère il faut stocker la saisie utilisateur.
Le but final de `read_line` est de récupérer tout ce que l'utilisateur écrit
dans l'entrée standard et de l'ajouter à la fin d'une chaîne de caractères
(sans écraser son contenu) ; c'est pourquoi nous passons cette chaîne de
caractères en argument. Cet argument doit être mutable pour que `read_line`
puisse en modifier le contenu.

<!--
The `&` indicates that this argument is a _reference_, which gives you a way to
let multiple parts of your code access one piece of data without needing to
copy that data into memory multiple times. References are a complex feature,
and one of Rust’s major advantages is how safe and easy it is to use
references. You don’t need to know a lot of those details to finish this
program. For now, all you need to know is that, like variables, references are
immutable by default. Hence, you need to write `&mut guess` rather than
`&guess` to make it mutable. (Chapter 4 will explain references more
thoroughly.)
-->

Le `&` indique que cet argument est une _référence_, ce qui permet de laisser
plusieurs morceaux de votre code accéder à une même donnée sans avoir besoin
de copier ces données dans la mémoire plusieurs fois. Les références sont une
fonctionnalité complexe, et un des avantages majeurs de Rust est qu'il rend sûr
et simple l'utilisation des références. Il n'est pas nécessaire de trop
s'apesantir sur les références pour terminer ce programme.
Pour l'instant, tout ce que vous devez savoir est que, comme les variables, les
références sont immuables par défaut.
D'où la nécessité d'écrire `&mut supposition` au lieu de `&supposition` pour la
rendre mutable. (Le chapitre 4 expliquera plus en détail les références.)

<!-- Old headings. Do not remove or links may break. -->

<!--
<a id="handling-potential-failure-with-the-result-type"></a>
-->

<a id="gérer-les-erreurs-potentielles-avec-result"></a>

<!--
### Handling Potential Failure with `Result`
-->

### Gérer les erreurs potentielles avec `Result`

<!--
We’re still working on this line of code. We’re now discussing a third line of
text, but note that it’s still part of a single logical line of code. The next
part is this method:
-->

Nous avons encore du travail sur cette ligne de code. Nous allons discuter d'une
troisième ligne de texte, mais notez bien qu'elle ne fait partie que d'une seule
ligne de code logique. La prochaine partie rajoute cette méthode :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

<!--
We could have written this code as:
-->

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
However, one long line is difficult to read, so it’s best to divide it. It’s
often wise to introduce a newline and other whitespace to help break up long
lines when you call a method with the `.method_name()` syntax. Now let’s
discuss what this line does.
-->

Cependant, une longue ligne de code n'est pas toujours facile à lire, c'est donc
une bonne pratique de la diviser. Il est parfois utile d'ajouter une nouvelle
ligne et des espaces afin de désagréger les longues lignes lorsque vous
appelerez une méthode, comme ici avec la syntaxe `.nom_de_la_methode()`.
Maintenant, voyons à quoi sert cette ligne.

<!--
As mentioned earlier, `read_line` puts whatever the user enters into the string
we pass to it, but it also returns a  `Result` value. [`Result`][result]<!--
ignore -- > is an [_enumeration_][enums]<!-- ignore -- >, often called an _enum_,
which is a type that can be in one of multiple possible states. We call each
possible state a _variant_.
-->

Comme expliqué précédemment, `read_line` stocke tout ce que l'utilisateur
a saisi dans la variable chaîne qu'on lui passe en argument , mais cette
fonction retourne aussi une valeur `Result`. [`Result`][result]<!-- ignore -->
est une [_énumération_][enum]<!-- ignore -->, souvent appelée _enum_, qui est
un type pouvant prendre plusieurs états possibles. Chaque état est appelé une
_variante_.





<!--
[Chapter 6][enums]<!-- ignore -- >  will cover enums in more detail. The purpose
of these `Result` types is to encode error-handling information.
-->

Le [chapitre 6][enums]<!-- ignore -- > explorera les énumérations plus en
détail. La raison d'être du type `Result` est de coder des informations pour la
gestion des erreurs.

<!--
`Result`’s variants are `Ok` and `Err`. The `Ok` variant indicates the
operation was successful, and it contains the successfully generated value.
The `Err` variant means the operation failed, and it contains information
about how or why the operation failed.
-->

Les variantes de `Result` sont `Ok` et `Err`. La variante `Ok` signifie que
l'opération a fonctionné, et elle contient la valeur générée avec succès. La
variante `Err` signifie que l'opération a échoué, et elle contient les 
informations décrivant comment ou pourquoi l'opération a échoué.

<!--
Values of the `Result` type, like values of any type, have methods defined on
them. An instance of `Result` has an [`expect` method][expect]<!-- ignore
-- > that you can call. If this instance of `Result` is an `Err` value, `expect`
will cause the program to crash and display the message that you passed as an
argument to `expect`. If the `read_line` method returns an `Err`, it would
likely be the result of an error coming from the underlying operating system.
If this instance of `Result` is an `Ok` value, `expect` will take the return
value that `Ok` is holding and return just that value to you so that you can
use it. In this case, that value is the number of bytes in the user’s input.
-->

Les valeurs du type `Result`, comme pour tous les types, ont des méthodes
qui leur sont associées. Par exemple, une instance de `Result` a une
[méthode `expect`][expect]<!-- ignore --> que vous pouvez utiliser. Si cette
instance de `Result` a pour valeur la variante `Err`, l'appel à `expect`
fera planter le programme et affichera le message que vous avez passé en
argument de `expect`. Si l'appel à `read_line` retourne une variante `Err`, ce
sera probablement dû à une erreur du système d'exploitation. Si en revanche
`read_line` a pour valeur la variante `Ok`, `expect` récupèrera le
contenu du `Ok`, qui est le résultat de l'opération, et vous le retournera afin
que vous puissiez l'utiliser. Dans notre exemple, ce résultat est le nombre
d'octets de la saisie utilisateur.

<!--
If you don’t call `expect`, the program will compile, but you’ll get a warning:
-->

Si on n'appelle pas `expect`, le programme compilera, mais avec un
avertissement :

<!--
```console
{{#include ../listings-sources/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

<!--
Rust warns that you haven’t used the `Result` value returned from `read_line`,
indicating that the program hasn’t handled a possible error.
-->

Rust nous prévient que l'on ne fait rien du `Result` que nous fournit
`read_line`, et que par conséquent notre programme ne gère pas une erreur
potentielle.

<!--
The right way to suppress the warning is to actually write error-handling code,
but in our case we just want to crash this program when a problem occurs, so we
can use `expect`. You’ll learn about recovering from errors in [Chapter
9][recover]<!-- ignore -- >.
-->

La meilleure façon de masquer cet avertissement est de réellement écrire le
code permettant de gérer l'erreur, mais dans notre cas on a seulement besoin de
faire planter le programme si un problème survient, on utilise donc `expect`.
Nous verrons dans le [chapitre 9][recover]<!-- ignore --> comment gérer
correctement les erreurs.

<!--
### Printing Values with `println!` Placeholders
-->

### Afficher des valeurs grâce aux espaces réservés de `println!`

<!--
Aside from the closing curly bracket, there’s only one more line to discuss in
the code so far:
-->

Mis à part l'accolade fermante, il ne nous reste plus qu'une seule ligne à
étudier dans le code que nous avons pour l'instant :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

<!-- This line prints the string that now contains the user’s input. The `{}`
set of curly brackets is a placeholder: Think of `{}` as little crab pincers
that hold a value in place. When printing the value of a variable, the variable
name can go inside the curly brackets.  You can print more than one value using
curly brackets: the first set of curly brackets holds the first value listed
after the format string, the second set holds the second value, and so on.
Printing multiple values in one call to `println!` would look like this:

-->

Cette ligne affiche la chaîne de caractères qui contient maintenant ce que
l'utilisateur a saisi. La paire d'accolades `{}` représente un espace réservé :
imaginez qu'il s'agit de pinces de crabes qui gardent la place d'une valeur.
Lorsque vous affichez la valeur d'une variable, le nom de la variable peut être
placé entre accolades. Pour afficher le résultat de l'évaluation d'une
expression, placez des accolades vides dans la chaîne de format, puis faites
suivre la chaîne de format d'une liste d'expressions séparées par des virgules,
expressions dont le résultat sera affiché dans chaque espace réservé
d'accolades vides, dans le même ordre. Pour afficher une variable et le
résultat d'une expression en appelant `println!` une seule fois, on ferait
comme ceci :

```rust
let x = 5;
let y = 10;

println!("x = {x} et y + 2 = {}", y + 2);
```


<!--
This code would print `x = 5 and y + 2 = 12`.
-->

Ce code afficherait `x = 5 et y + 2 = 12`.

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
<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -- >
-->

<!--
```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```
-->

```console
$ cargo run
   Compiling jeu_du_plus_ou_du_moins v0.1.0 (file:///projects/jeu_du_plus_ou_du_moins)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/jeu_du_plus_ou_du_moins`
Devinez le nombre !
Veuillez entrer un nombre.
6
Votre nombre : 6
```

<!--
At this point, the first part of the game is done: We’re getting input from the
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
secret number should be different every time so that the game is fun to play
more than once. We’ll use a random number between 1 and 100 so that the game
isn’t too difficult. Rust doesn’t yet include random number functionality in
its standard library. However, the Rust team does provide a [`rand` 
crate][randcrate] with said functionality.
-->

Maintenant, il nous faut générer un nombre secret que notre joueur va devoir
deviner. Ce nombre devra être différent à chaque fois pour qu'on puisse
s'amuser à y jouer plusieurs fois. Nous allons tirer au sort un nombre compris
entre 1 et 100 pour que le jeu ne soit pas trop difficile. Rust n'embarque pas
pour l'instant de fonctionnalité de génération de nombres aléatoires dans sa
bibliothèque standard. Cependant, l'équipe de Rust propose une
[crate `rand`][randcrate] qui offre la possibilité de le faire.

<!-- Old headings. Do not remove or links may break. -->
<a id="using-a-crate-to-get-more-functionality"></a>

<!--
### Increasing Functionality with a Crate
-->

### Étendre les fonctionnalités de Rust avec une _crate_

<!--
Remember that a crate is a collection of Rust source code files. The project
we’ve been building is a *binary crate*, which is an executable. The `rand`
crate is a *library crate*, which contains code that is intended to be used in
other programs and can’t be executed on its own.
-->

Souvenez-vous, une _crate_ est un ensemble de fichiers de code source Rust. Le
projet sur lequel nous travaillons est une _crate_ binaire, qui est un programme
exécutable. La _crate_ `rand` est une _crate de bibliothèque_, qui contient du
code qui peut être utilisé dans d'autres programmes, et qui ne peut pas être
exécuté tout seul.

<!--
Cargo’s coordination of external crates is where Cargo really shines. Before we
can write code that uses `rand`, we need to modify the _Cargo.toml_ file to
include the `rand` crate as a dependency. Open that file now and add the
following line to the bottom, beneath the `[dependencies]` section header that
Cargo created for you. Be sure to specify `rand` exactly as we have here, with
this version number, or the code examples in this tutorial may not work:
-->

La coordination des _crates_ externes est un domaine dans lequel Cargo excelle.
Avant d'écrire le code qui utilisera `rand`, il nous faut éditer le fichier
_Cargo.toml_ pour y spécifier `rand` en tant que dépendance. Ouvrez donc
maintenant ce fichier et ajoutez la ligne suivante à la fin, en dessous de
l'en-tête de section `[dependencies]` que Cargo a créé pour vous. Assurez-vous
de spécifier `rand` exactement comme dans le bout de code suivant, avec ce
numéro de version, ou sinon les exemples de code de ce tutoriel pourraient ne
pas fonctionner:

<!--
<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-- >
-->

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

<!--
In the _Cargo.toml_ file, everything that follows a header is part of that
section that continues until another section starts. In `[dependencies]`, you
tell Cargo which external crates your project depends on and which versions of
those crates you require. In this case, we specify the `rand` crate with the
semantic version specifier `0.8.5`. Cargo understands [Semantic
Versioning][semver]<!-- ignore -- > (sometimes called _SemVer_), which is a
standard for writing version numbers. The specifier `0.8.5` is actually
shorthand for `^0.8.5`, which means any version that is at least `0.8.5` but
below `0.9.0`.
Cargo considers these versions to have public APIs compatible with version 
`0.8.5`, and this specification ensures that you’ll get the latest patch release
that will still compile with the code in this chapter. Any version `0.9.0` or
greater is not guaranteed to have the same API as what the following examples
use.
-->

Dans le fichier _Cargo.toml_, tout ce qui suit une en-tête fait partie de cette
section, et ce jusqu'à ce qu'une autre section débute. Dans `[dependencies]`,
vous indiquez à Cargo de quelles _crates_ externes votre
projet dépend, et de quelle version de ces _crates_ vous avez besoin.
Dans notre cas, on ajoute comme dépendance la crate `rand` avec la version
sémantique `0.8.5`. Cargo arrive à interpréter le
[versionnage sémantique][semver]<!-- ignore --> (aussi appelé _SemVer_), qui
est une convention d'écriture de numéros de version. En réalité, `0.8.5` est
une abréviation pour `^0.8.5`, ce qui signifie “toute version ultérieure ou
égale à `0.8.5` mais strictement antérieure à `0.9.0`”. Cargo considère que ces
versions ont des API publiques compatibles avec la version `0.8.5`, et cette
indication garantit que vous obtiendrez la dernière version de correction qui
compilera encore avec le code de ce chapitre. Il n'est pas garanti que les
versions `0.9.0` et ultérieures aient la même API que celle utilisée dans les
exemples suivants.

<!--
Now, without changing any of the code, let’s build the project, as shown in
Listing 2-2.
-->

Maintenant, sans apporter le moindre changement au code, lançons une compilation
du projet, comme dans l'encart 2-2 :

<!--
<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -- >
-->

```console
$ cargo build
    Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
```

<!--
<span class="caption">Listing 2-2: The output from running `cargo build` after
adding the rand crate as a dependency</span>
-->

<span class="caption">Encart 2-2 : Résultat du lancement de `cargo build` après
avoir ajouté la _crate_ `rand` comme dépendance</span>

<!--
You may see different version numbers (but they will all be compatible with the
code, thanks to SemVer!) and different lines (depending on the operating
system), and the lines may be in a different order.
-->

Il est possible que vous ne voyiez pas exactement les mêmes numéros de version,
(mais ils seront compatibles avec votre code, grâce au _versionnage
sémantique_ !) et différentes lignes (en fonction de votre système
d'exploitation), et les lignes ne seront pas forcément affichées dans le même
ordre.

<!--
When we include an external dependency, Cargo fetches the latest versions of
everything that dependency needs from the _registry_, which is a copy of data
from [Crates.io][cratesio]. Crates.io is where people in the Rust ecosystem
post their open source Rust projects for others to use.
-->

Lorsque nous ajoutons une dépendance externe, Cargo récupère les dernières
versions de tout ce dont cette dépendance a besoin depuis le _registre_, qui
est une copie des données de [Crates.io][cratesio]. Crates.io est là où les
développeurs de l'écosystème Rust publient leurs projets open source afin de
les rendre disponibles aux autres.

<!--
After updating the registry, Cargo checks the `[dependencies]` section and
downloads any crates listed that aren’t already downloaded. In this case,
although we only listed `rand` as a dependency, Cargo also grabbed other crates
that `rand` depends on to work. After downloading the crates, Rust compiles
them and then compiles the project with the dependencies available.
-->

Une fois le registre mis à jour, Cargo lit la section `[dependencies]` et se
charge de télécharger les _crates_ qui y sont listées que vous n'avez pas
encore téléchargées. Dans notre cas, bien que nous n'ayons spécifié qu'une
seule dépendance, `rand`, Cargo a aussi téléchargé d'autres _crates_ dont
dépend `rand` pour fonctionner. Une fois le téléchargement terminé des
_crates_, Rust les compile, puis compile notre projet avec les dépendances
disponibles.

<!--
If you immediately run `cargo build` again without making any changes, you
won’t get any output aside from the `Finished` line. Cargo knows it has already
downloaded and compiled the dependencies, and you haven’t changed anything
about them in your _Cargo.toml_ file. Cargo also knows that you haven’t changed
anything about your code, so it doesn’t recompile that either. With nothing to
do, it simply exits.
-->

Si vous relancez tout de suite `cargo build` sans changer quoi que ce soit,
vous n'obtiendrez rien d'autre que la ligne `Finished`. Cargo sait qu'il a déjà
téléchargé et compilé les dépendances, et que vous n'avez rien changé dans
votre fichier _Cargo.toml_. Cargo sait aussi que vous n'avez rien changé dans
votre code, donc il ne le recompile pas non plus. Étant donné qu'il n'a rien
à faire, Cargo se termine tout simplement.

<!--
If you open the _src/main.rs_ file, make a trivial change, and then save it
and build again, you’ll only see two lines of output:
-->

Si vous ouvrez le fichier _src/main.rs_, faites un changement très simple,
enregistrez le fichier, et relancez la compilation, vous verrez s'afficher
uniquement deux lignes :

<!--
<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -- >
-->

<!--
```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```
-->

```console
$ cargo build
   Compiling jeu_du_plus_ou_du_moins v0.1.0 (file:///projects/jeu_du_plus_ou_du_moins)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

<!--
These lines show that Cargo only updates the build with your tiny change to the
_src/main.rs_ file. Your dependencies haven’t changed, so Cargo knows it can
reuse what it has already downloaded and compiled for those.
-->

Ces lignes nous informent que Cargo a recompilé uniquement à cause de notre
petit changement dans le fichier _src/main.rs_. Les dépendances n'ayant pas
changé, Cargo sait qu'il peut simplement réutiliser ce qu'il a déjà téléchargé
et compilé précédemment.

<!-- Old headings. Do not remove or links may break. -->
<a id="ensuring-reproducible-builds-with-the-cargo-lock-file"></a>

<!--
#### Ensuring Reproducible Builds
-->

#### Assurer la reproductibilité des compilations

<!--
Cargo has a mechanism that ensures that you can rebuild the same artifact every
time you or anyone else builds your code: Cargo will use only the versions of
the dependencies you specified until you indicate otherwise. For example, say
that next week version 0.8.6 of the `rand` crate comes out, and that version
contains an important bug fix, but it also contains a regression that will
break your code. To handle this, Rust creates the _Cargo.lock_ file the first
time you run `cargo build`, so we now have this in the _guessing_game_
directory.
-->

Cargo embarque une fonctionnalité qui garantit que vous pouvez recompiler le
même artéfact à chaque fois que vous ou quelqu'un d'autre compile votre code :
Cargo va utiliser uniquement les versions de dépendances que vous avez
utilisées jusqu'à ce que vous indiquiez le contraire.  Par exemple, imaginons
que la semaine prochaine, la version 0.8.6 de la _crate_ `rand` est publiée, et
qu'elle apporte une correction importante, mais aussi qu'elle produit une
régression qui va casser votre code. Pour éviter cela, Rust crée le fichier
_Cargo.lock_ la première fois que vous utilisez `cargo build`, donc nous
l'avons désormais dans le répertoire _jeu_du_plus_ou_du_moins_.

<!--
When you build a project for the first time, Cargo figures out all the versions
of the dependencies that fit the criteria and then writes them to the
_Cargo.lock_ file. When you build your project in the future, Cargo will see
that the _Cargo.lock_ file exists and will use the versions specified there
rather than doing all the work of figuring out versions again. This lets you
have a reproducible build automatically. In other words, your project will
remain at `0.8.5` until you explicitly upgrade, thanks to the _Cargo.lock_
file.
Because the _Cargo.lock_ file is important for reproducible builds, it’s often
checked into source control with the rest of the code in your project.
-->

Quand vous compilez un projet pour la première fois, Cargo détermine toutes les
versions de dépendances qui correspondent à vos critères et les écrit dans le
fichier _Cargo.lock_. Quand vous recompilerez votre projet plus tard, Cargo
verra que le fichier _Cargo.lock_ existe et utilisera les versions précisées
à l'intérieur au lieu de recommencer à déterminer toutes les versions
demandées.  Ceci vous permet d'avoir automatiquement des compilations
reproductibles.  En d'autres termes, votre projet va rester sur la version
`0.8.5` jusqu'à ce que vous le mettiez à jour explicitement, grâce au fichier
_Cargo.lock_.
Du fait que le fichier _Cargo.lock_ est important pour la reproductibilité des
compilations, il est souvent enregistré dans le système de contrôle de code
source, avec le reste du code de votre projet.





<!--
#### Updating a Crate to Get a New Version
-->

#### Mettre à jour une _crate_ vers sa nouvelle version

<!--
When you _do_ want to update a crate, Cargo provides the command `update`,
which will ignore the _Cargo.lock_ file and figure out all the latest versions
that fit your specifications in _Cargo.toml_. Cargo will then write those
versions to the _Cargo.lock_ file. Otherwise, by default, Cargo will only look
for versions greater than `0.8.5` and less than `0.9.0`. If the `rand` crate
has released the two new versions `0.8.6` and `0.999.0`, you would see the
following if you ran `cargo update`:
-->

Lorsque vous souhaitez réellement mettre à jour une _crate_, Cargo vous fournit
la commande `update`, qui va ignorer le fichier _Cargo.lock_ et va rechercher
toutes les versions qui correspondent à vos critères dans _Cargo.toml_. Cargo
va ensuite écrire ces versions dans le fichier _Cargo.lock_. Sinon par défaut,
Cargo va rechercher uniquement les versions plus grandes que `0.8.5` et
inférieures à `0.9.0`. Si la _crate_ `rand` a été publiée en deux nouvelles
versions `0.8.6` et `0.9.0`, alors vous verrez ceci si vous lancez
`cargo update` :

<!--
<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -- >
-->

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.999.0)
```

<!--
Cargo ignores the `0.999.0` release. At this point, you would also notice a
change in your _Cargo.lock_ file noting that the version of the `rand` crate
you are now using is `0.8.6`. To use `rand` version `0.999.0` or any version in
the `0.999.x` series, you’d have to update the _Cargo.toml_ file to look like
this instead (don’t actually make this change because the following examples assume
you’re using `rand` 0.8):
-->

Cargo ignore la version `0.999.0`. À partir de ce moment, vous pouvez aussi
constater un changement dans le fichier _Cargo.lock_ indiquant que la version
de la _crate_ `rand` que vous utilisez maintenant est la `0.8.6`. Pour utiliser
`rand` en version `0.999.0` ou toute autre version dans la série des `0.999.x`,
il vous faut mettre à jour le fichier _Cargo.toml_ comme ceci (ne faites pas
réellement ce changement, car les exemples suivants supposent que vous utilisez
`rand` 0.8) :

```toml
[dependencies]
rand = "0.999.0"
```

<!--
The next time you run `cargo build`, Cargo will update the registry of crates
available and reevaluate your `rand` requirements according to the new version
you have specified.
-->

La prochaine fois que vous lancerez `cargo build`, Cargo mettra à jour son
registre de _crates_ disponibles et réévaluera vos exigences vis-à-vis de
`rand` selon la nouvelle version que vous avez spécifiée.

<!--
There’s a lot more to say about [Cargo][doccargo]<!-- ignore -- > and [its
ecosystem][doccratesio]<!-- ignore -- >, which we’ll discuss in Chapter 14, but
for now, that’s all you need to know. Cargo makes it very easy to reuse
libraries, so Rustaceans are able to write smaller projects that are assembled
from a number of packages.
-->

Il y a encore plus à dire à propos de [Cargo][doccargo]<!-- ignore --> et de
[son écosystème][doccratesio]<!-- ignore -->, que nous aborderons au chapitre
14, mais pour l'instant, c'est tout ce qu'il vous faut savoir. Cargo facilite
la réutilisation des bibliothèques, pour que les Rustacés soient capables
d'écrire des petits projets issus d'un assemblage d'un certain nombre de
paquets.

<!--
### Generating a Random Number
-->

### Générer un nombre aléatoire

<!--
Let’s start using `rand` to generate a number to guess. The next step is to
update _src/main.rs_, as shown in Listing 2-3.
-->

Commençons désormais à utiliser `rand` pour générer un nombre à deviner. La
prochaine étape est de modifier _src/main.rs_ comme dans l'encart 2-3.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">Encart 2-3 : Ajout du code pour générer un nombre
aléatoire</span>

<!--
First, we add the line `use rand::Rng;`. The `Rng` trait defines methods that
random number generators implement, and this trait must be in scope for us to
use those methods. Chapter 10 will cover traits in detail.
-->

D'abord, nous avons ajouté la ligne `use rand::Rng;`. Le _trait_ `Rng` définit
les méthodes implémentées par les générateurs de nombres aléatoires, et ce
_trait_ doit être accessible à notre code pour qu'on puisse utiliser ces
méthodes. Le chapitre 10 expliquera plus en détail les _traits_.

<!--
Next, we’re adding two lines in the middle. In the first line, we call the
`rand::thread_rng` function that gives us the particular random number
generator we’re going to use: one that is local to the current thread of
execution and is seeded by the operating system. Then, we call the `gen_range`
method on the random number generator. This method is defined by the `Rng`
trait that we brought into scope with the `use rand::Rng;` statement. The
`gen_range` method takes a range expression as an argument and generates a
random number in the range. The kind of range expression we’re using here takes
the form `start..=end` and is inclusive on the lower and upper bounds, so we
need to specify `1..=100` to request a number between 1 and 100.
-->

Ensuite, nous ajoutons deux lignes au milieu. À la première ligne, nous
appelons la fonction `rand::thread_rng` qui nous fournit le générateur de
nombres aléatoires particulier que nous allons utiliser : il est propre au fil
d'exécution courant et généré par le système d'exploitation. Ensuite, nous
appelons la méthode `gen_range` sur le générateur de nombres aléatoires. Cette
méthode est définie par le _trait_ `Rng` que nous avons importé avec
l'instruction `use rand::Rng;`. La méthode `gen_range` prend une expression
d'intervalle en paramètre et génère un nombre aléatoire au sein de
l'intervalle. Le genre d'expression d'intervalle utilisé ici est de la forme
`début..=fin` et inclut les bornes inférieure et supérieure,
nous avons donc besoin de préciser `1..=100` pour demander un nombre entre 1
et 100.

<!--
> Note: You won’t just know which traits to use and which methods and functions
> to call from a crate, so each crate has documentation with instructions for
> using it. Another neat feature of Cargo is that running the `cargo doc
> --open` command will build documentation provided by all your dependencies
> locally and open it in your browser. If you’re interested in other
> functionality in the `rand` crate, for example, run `cargo doc --open` and
> click `rand` in the sidebar on the left.
-->

> Remarque : vous ne pourrez pas deviner quels _traits_, méthodes et
> fonctions utiliser avec une _crate_, donc chaque _crate_ a une documentation
> qui donne des indications sur son utilisation. Une autre fonctionnalité
> intéressante de Cargo est que vous pouvez utiliser la commande
> `cargo doc --open`, qui va construire localement la documentation intégrée
> par toutes vos dépendances et va l'ouvrir dans votre navigateur. Si vous vous
> intéressez à d'autres fonctionnalités de la _crate_ `rand`, par exemple, vous
> pouvez lancer `cargo doc --open` et cliquer sur `rand` dans la barre latérale
> sur la gauche.

<!--
The second new line prints the secret number. This is useful while we’re
developing the program to be able to test it, but we’ll delete it from the
final version. It’s not much of a game if the program prints the answer as soon
as it starts!
-->

La seconde nouvelle ligne affiche le nombre secret. C'est pratique lors du
développement pour pouvoir le tester, mais nous l'enlèverons dans la version
finale. Ce n'est pas vraiment un jeu si le programme affiche la réponse dès
qu'il démarre !

<!--
Try running the program a few times:
-->

Essayez de lancer le programme plusieurs fois :

<!--
<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-- >
-->

<!--
```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```
-->

```console
$ cargo run
   Compiling jeu_du_plus_ou_du_moins v0.1.0 (file:///projects/jeu_du_plus_ou_du_moins)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/jeu_du_plus_ou_du_moins`
Devinez le nombre !
Le nombre secret est : 7
Veuillez entrer un nombre.
4
Votre nombre : 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/jeu_du_plus_ou_du_moins`
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
is shown in Listing 2-4. Note that this code won’t compile just yet, as we will
explain.
-->

Maintenant que nous avons une saisie utilisateur et un nombre aléatoire, nous
pouvons les comparer. Cette étape est écrite dans l'encart 2-4. Sachez
toutefois que le code ne se compilera pas tout de suite, comme nous allons
l'expliquer par la suite.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 2-4: Handling the possible return values of
comparing two numbers</span>
-->

<span class="caption">Encart 2-4 : Traitement des valeurs possibles saisies en
comparant les deux nombres</span>

<!--
First, we add another `use` statement, bringing a type called
`std::cmp::Ordering` into scope from the standard library. The `Ordering` type
is another enum and has the variants `Less`, `Greater`, and `Equal`. These are
the three outcomes that are possible when you compare two values.
-->

Premièrement, nous ajoutons une autre instruction `use`, qui importe
`std::cmp::Ordering` à portée de notre code depuis la bibliothèque standard.
Le type `Ordering` est une autre énumération et a les variantes `Less`
_(inférieur)_, `Greater` _(supérieur)_ et `Equal` _(égal)_. Ce sont les trois
issues possibles lorsqu'on compare deux valeurs.

<!--
Then, we add five new lines at the bottom that use the `Ordering` type. The
`cmp` method compares two values and can be called on anything that can be
compared. It takes a reference to whatever you want to compare with: Here, it’s
comparing `guess` to `secret_number`. Then, it returns a variant of the
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

<!--
A `match` expression is made up of _arms_. An arm consists of a _pattern_ to
match against, and the code that should be run if the value given to `match`
fits that arm’s pattern. Rust takes the value given to `match` and looks
through each arm’s pattern in turn. Patterns and the `match` construct are
powerful Rust features: They let you express a variety of situations your code
might encounter, and they make sure you handle them all. These features will be
covered in detail in Chapter 6 and Chapter 19, respectively.
-->

Une expression `match` est composée de _branches_. Une branche est constituée
d'un _motif (pattern)_ avec lequel elle doit correspondre et du code qui sera
exécuté si la valeur donnée au `match` correspond bien au motif de cette
branche. Rust prend la valeur donnée à `match` et la compare au motif de chaque
branche à tour de rôle. Les motifs et la structure de contrôle `match` sont des
fonctionnalités puissantes de Rust : elles vous permettent de décrire une
multitude de scénarios que votre code peut rencontrer, et elles s'assurent que
vous les gérez toutes. Ces fonctionnalités seront expliquées plus en détail
respectivement dans le chapitre 6 et le chapitre 19.

<!--
Let’s walk through an example with the `match` expression we use here. Say that
the user has guessed 50 and the randomly generated secret number this time is
38.

When the code compares 50 to 38, the `cmp` method will return
`Ordering::Greater` because 50 is greater than 38. The `match` expression gets
the `Ordering::Greater` value and starts checking each arm’s pattern. It looks
at the first arm’s pattern, `Ordering::Less`, and sees that the value
`Ordering::Greater` does not match `Ordering::Less`, so it ignores the code in
that arm and moves to the next arm. The next arm’s pattern is
`Ordering::Greater`, which _does_ match `Ordering::Greater`! The associated
code in that arm will execute and print `Too big!` to the screen. The `match`
expression ends after the first successful match, so it won’t look at the last
arm in this scenario.
-->

Voyons un exemple avec l'expression `match` que nous avons utilisée ici. Disons
que l'utilisateur a saisi le nombre 50 et que le nombre secret généré
aléatoirement a cette fois-ci comme valeur 38.

Quand le code compare 50 à 38, la méthode `cmp` va retourner
`Ordering::Greater`, car 50 est plus grand que 38. L'expression `match` obtient
la valeur `Ordering::Greater` et commence à vérifier le motif de chaque
branche. Elle consulte le motif de la première branche, `Ordering::Less` et
remarque que la valeur `Ordering::Greater` ne correspond pas au motif
`Ordering::Less` ; elle ignore donc le code de cette branche et passe à la
suivante. Le motif de la branche suivante est `Ordering::Greater`, qui
correspond à `Ordering::Greater` ! Le code associé à cette branche va être
exécuté et va afficher à l'écran `C'est moins !`.  L'expression `match` se
termine après la première correspondance, elle n'a donc pas besoin de consulter
les autres branches de ce scénario.

<!--
However, the code in Listing 2-4 won’t compile yet. Let’s try it:
-->

Cependant, notre code dans l'encart 2-4 ne compile pas encore. Essayons de le
faire :

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

<!--
```console
{{#include ../listings-sources/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

<!--
The core of the error states that there are _mismatched types_. Rust has a
strong, static type system. However, it also has type inference. When we wrote
`let mut guess = String::new()`, Rust was able to infer that `guess` should be
a `String` and didn’t make us write the type. The `secret_number`, on the other
hand, is a number type. A few of Rust’s number types can have a value between 1
and 100: `i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a
64-bit number; as well as others. Unless otherwise specified, Rust defaults to
an `i32`, which is the type of `secret_number` unless you add type information
elsewhere that would cause Rust to infer a different numerical type. The reason
for the error is that Rust cannot compare a string and a number type.
-->

Le message d'erreur nous indique que nous sommes dans un cas de types non
compatibles _(mismatched types)_. Rust a un système de types fort et statique.
Cependant, il a aussi une fonctionnalité d'inférence de type. Quand nous avons
écrit `let mut supposition = String::new()`, Rust a pu en déduire que
`supposition` devait être une `String` et ne nous a pas demandé d'écrire le
type. D'autre part, `nombre_secret` est d'un type de nombre. Quelques types de
nombres de Rust peuvent avoir une valeur entre 1 et 100 : `i32`, un nombre
entier encodé sur 32 bits ; `u32`, un nombre entier de 32 bits non signé
(positif ou nul) ; `i64`, un nombre entier encodé sur 64 bits ; parmi tant
d'autres. Rust utilise par défaut un `i32`, qui est le type de `nombre_secret`,
à moins que vous précisiez quelque part une information de type qui amènerait
Rust à inférer un type de nombre différent. La raison de cette erreur est que
Rust ne peut pas comparer une chaîne de caractères à un nombre.

<!--
Ultimately, we want to convert the `String` the program reads as input into a
number type so that we can compare it numerically to the secret number. We do
so by adding this line to the `main` function body:
-->

Au bout du compte, nous voulons convertir la `String` que le programme récupère
de la saisie utilisateur en un nombre, pour qu'on puisse la comparer
numériquement au nombre secret. Nous allons faire ceci en ajoutant cette ligne
supplémentaire dans le corps de la fonction `main` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

<!--
The line is:
-->

La nouvelle ligne est :

<!--
```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```
-->

```rust,ignore
let supposition: u32 = supposition.trim().parse().expect("Veuillez entrer un nombre !");
```

<!--
We create a variable named `guess`. But wait, doesn’t the program already have
a variable named `guess`? It does, but helpfully Rust allows us to _shadow_ the
previous value of `guess` with a new one. Shadowing lets us reuse the `guess`
variable name rather than forcing us to create two unique variables, such as
`guess_str` and `guess`, for example. We’ll cover this in more detail in
[Chapter 3][shadowing]<!-- ignore -- >, but for now, know that this feature is
often used when you want to convert a value from one type to another type.
-->

Nous créons une variable qui s'appelle `supposition`. Mais attendez, le
programme n'a-t-il pas déjà une variable qui s'appelle `supposition` ?
C'est le cas, mais heureusement Rust nous permet de _masquer_ la valeur
précédente de `supposition` avec une nouvelle.
Le masquage _(shadowing)_ nous permet de réutiliser le nom de variable
`supposition`, plutôt que de nous forcer à créer deux variables distinctes,
telles que `supposition_str` et `supposition` par exemple.
Nous verrons cela plus en détails au [chapitre 3][shadowing]<!-- ignore -->,
mais pour le moment cette fonctionnalité est souvent utilisée dans des
situations où on veut convertir une valeur d'un type à un autre.

<!--
We bind this new variable to the expression `guess.trim().parse()`. The `guess`
in the expression refers to the original `guess` variable that contained the
input as a string. The `trim` method on a `String` instance will eliminate any
whitespace at the beginning and end, which we must do before we can convert the
string to a `u32`, which can only contain numerical data. The user must press
<span class="keystroke">enter</span> to satisfy `read_line` and input their
guess, which adds a newline character to the string. For example, if the user
types <span class="keystroke">5</span> and presses <span
class="keystroke">enter</span>, `guess` looks like this: `5\n`. The `\n`
represents “newline”. (On Windows, pressing <span
class="keystroke">enter</span> results in a carriage return and a newline,
`\r\n`). The `trim` method eliminates `\n` or `\r\n`, resulting in just `5`.
-->

Nous lions cette nouvelle variable à l'expression `supposition.trim().parse()`.
Le `supposition` dans l'expression se réfère à la variable `supposition`
initiale qui contenait la saisie utilisateur en tant que chaîne de caractères.
La méthode `trim` sur une instance de `String` va enlever les espaces et autres
_whitespaces_ au début et à la fin, ce que nous devons faire avant de pouvoir
convertir la chaîne en un `u32`, qui ne peut être constitué que de chiffres.
L'utilisateur doit appuyer sur <span class="keystroke">entrée</span> pour
mettre fin à `read_line` et récupérer sa supposition, ce qui va rajouter un
caractère de fin de ligne à la chaîne de caractères. Par exemple, si
l'utilisateur écrit <span class="keystroke">5</span> et appuie sur <span
class="keystroke">entrée </span>, `supposition` aura alors cette valeur :
`5\n`.  Le `\n` représente une fin de ligne (à noter que sur Windows, appuyer
sur <span class="keystroke">entrée</span> résulte en un retour chariot suivi
d'un saut de ligne, `\r\n`). La méthode `trim` enlève `\n` et `\r\n`, il ne
reste donc plus que `5`.

<!--
The [`parse` method on strings][parse]<!-- ignore -- > converts a string to
another type. Here, we use it to convert from a string to a number. We need to
tell Rust the exact number type we want by using `let guess: u32`. The colon
(`:`) after `guess` tells Rust we’ll annotate the variable’s type. Rust has a
few built-in number types; the `u32` seen here is an unsigned, 32-bit integer.
It’s a good default choice for a small positive number. You’ll learn about
other number types in [Chapter 3][integers]<!--ignore -- >.

Additionally, the `u32` annotation in this example program and the comparison
with `secret_number` means Rust will infer that `secret_number` should be a
`u32` as well. So, now the comparison will be between two values of the same
type!
-->

La [méthode `parse` des chaînes de caractères][parse]<!-- ignore --> convertit
une chaîne de caractères vers un autre type. Ici, elle est utilisée pour
convertir d'une chaîne vers un nombre. Nous devons indiquer à Rust le type
exact de nombre que nous voulons en utilisant `let supposition: u32`.  Le
deux-points (`:`) après `supposition` indique à Rust que nous voulons préciser
le type de la variable. Rust embarque quelques types de nombres ; le `u32`
utilisé ici est un entier non signé sur 32 bits.  C'est un bon choix par défaut
pour un petit nombre positif.  Vous découvrirez d'autres types de nombres dans
le [chapitre 3][integers]<!-- ignore -->.

De plus, l'annotation `u32` dans ce programme d'exemple et la comparaison avec
`nombre_secret` permet à Rust d'en déduire que `nombre_secret` doit être lui
aussi un `u32`. Donc maintenant, la comparaison se fera entre deux valeurs du
même type !

<!--
The `parse` method will only work on characters that can logically be converted
into numbers and so can easily cause errors. If, for example, the string
contained `A👍%`, there would be no way to convert that to a number. Because it
might fail, the `parse` method returns a `Result` type, much as the `read_line`
method does (discussed earlier in [“Handling Potential Failure with
`Result`”](#handling-potential-failure-with-result)<!-- ignore -- >). We’ll treat
this `Result` the same way by using the `expect` method again. If `parse`
returns an `Err` `Result` variant because it couldn’t create a number from the
string, the `expect` call will crash the game and print the message we give it.
If `parse` can successfully convert the string to a number, it will return the
 `Ok` variant of `Result`, and `expect` will return the number that we want from
 the `Ok` value.
-->

La méthode `parse` va fonctionner uniquement sur des caractères qui peuvent
être logiquement convertis en nombres et donc peut facilement mener à une
erreur. Si par exemple, le texte contient `A👍%`, il ne sera pas possible de le
convertir en nombre. Comme elle peut échouer, la méthode `parse` retourne un
type `Result`, comme celui que la méthode `read_line` retourne (comme nous
l'avons vu plus tôt dans [“Gérer les erreurs potentielles avec `Result`”]
(#gérer-les-erreurs-potentielles-avec-result)<!-- ignore-->).  Nous allons
gérer ce `Result` de la même manière, avec à nouveau la méthode `expect`. Si
`parse` retourne une variante `Err` de `Result` car elle ne peut pas créer un
nombre à partir de la chaîne de caractères, l'appel à `expect` va faire planter
le jeu et va afficher le message que nous lui avons passé en paramètre. Si
`parse` arrive à convertir la chaîne de caractères en nombre, alors elle
retournera la variante `Ok` de `Result`, et `expect` va retourner le nombre
qu'il nous faut qui est stocké dans la variante `Ok`.

<!--
Let’s run the program now:
-->

Exécutons ce programme, maintenant :

<!--
<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-- >
-->

<!--
```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```
-->

```console
$ cargo run
   Compiling jeu_du_plus_ou_du_moins v0.1.0 (file:///projects/jeu_du_plus_ou_du_moins)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/jeu_du_plus_ou_du_moins`
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
The `loop` keyword creates an infinite loop. We’ll add a loop to give users
more chances at guessing the number:
-->

Le mot-clé `loop` crée une boucle infinie. Nous allons ajouter une boucle pour
donner aux utilisateurs plus de chances de deviner le nombre :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

<!--
As you can see, we’ve moved everything from the guess input prompt onward into
a loop. Be sure to indent the lines inside the loop another four spaces each
and run the program again. The program will now ask for another guess forever,
which actually introduces a new problem. It doesn’t seem like the user can quit!
-->

Comme vous pouvez le remarquer, nous avons déplacé dans une boucle tout le code
de l'invite à entrer le nombre. Assurez-vous d'indenter correctement les lignes
dans la boucle avec quatre nouvelles espaces pour chacune, et lancez à nouveau
le programme. Le programme va désormais demander un nombre à l'infini, ce qui
est un nouveau problème. Il n'est pas possible pour l'utilisateur de
l'arrêter !

<!--
The user could always interrupt the program by using the keyboard shortcut
<span class="keystroke">ctrl-C</span>. But there’s another way to escape this
insatiable monster, as mentioned in the `parse` discussion in [“Comparing the
Guess to the Secret Number”](#comparing-the-guess-to-the-secret-number)<!--
ignore -- >: if the user enters a non-number answer, the program will crash. We
can take advantage of that to allow the user to quit, as shown here:
-->

L'utilisateur pourrait quand même interrompre le programme en utilisant le
raccourci clavier <span class="keystroke">ctrl-C</span>.
Mais il y a une autre façon d'échapper à ce monstre insatiable, comme nous
l'avons abordé dans la partie [“Comparer le nombre saisi au nombre
secret”](#comparer-le-nombre-saisi-au-nombre-secret)<!-- ignore --> : si
l'utilisateur saisit quelque chose qui n'est pas un nombre, le programme va
planter. Nous pouvons procéder ainsi pour permettre à l'utilisateur de quitter,
comme ci-dessous :

<!--
<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
touch src/main.rs
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-- >
-->

<!-- markdownlint-disable -->
<!--
```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
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
thread 'main' panicked at src/main.rs:28:47:
Please type a number!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
-->
<!-- markdownlint-restore -->

```console
$ cargo run
   Compiling jeu_du_plus_ou_du_moins v0.1.0 (file:///projects/jeu_du_plus_ou_du_moins)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/jeu_du_plus_ou_du_moins`
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
thread 'main' panicked at 'Veuillez entrer un nombre !: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: Run with `RUST_BACKTRACE=1` for a backtrace
```

<!--
Typing `quit` will quit the game, but as you’ll notice, so will entering any
other non-number input. This is suboptimal, to say the least; we want the game
to also stop when the correct number is guessed.
-->

Taper `quitter` va bien fermer le jeu, mais comme vous pouvez le remarquer,
toute autre saisie qui n'est pas un nombre le ferait aussi. Ce mécanisme laisse
franchement à désirer ; nous voudrions que le jeu s'arrête aussi lorsque le bon
nombre est deviné.

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
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

___JEANSUILÀ___
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
the user inputs a non-number, let’s make the game ignore a non-number so that
the user can continue guessing. We can do that by altering the line where
`guess` is converted from a `String` to a `u32`, as shown in Listing 2-5.
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
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 2-5: Ignoring a non-number guess and asking for
another guess instead of crashing the program</span>
-->

<span class="caption">Encart 2-5 : Ignorer une saisie qui n'est pas un nombre
et demander un nouveau nombre plutôt que de faire planter le programme</span>

<!--
We switch from an `expect` call to a `match` expression to move from crashing
on an error to handling the error. Remember that `parse` returns a `Result`
type and `Result` is an enum that has the variants `Ok` and `Err`. We’re using a
`match` expression here, as we did with the `Ordering` result of the `cmp`
method.
-->

Nous remplaçons un appel à `expect` par une expression `match` pour passer
d'une erreur qui fait planter le programme à une erreur proprement gérée.
N'oubliez pas que `parse` retourne un type `Result` et que `Result` est une
énumération qui a pour variantes `Ok` et `Err`. Nous utilisons ici une
expression `match` comme nous l'avons déjà fait avec le résultat de type
`Ordering` de la méthode `cmp`.

<!--
If `parse` is able to successfully turn the string into a number, it will
return an `Ok` value that contains the resultant number. That `Ok` value will
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
If `parse` is _not_ able to turn the string into a number, it will return an
`Err` value that contains more information about the error. The `Err` value
does not match the `Ok(num)` pattern in the first `match` arm, but it does
match the `Err(_)` pattern in the second arm. The underscore, `_`, is a
catch-all value; in this example, we’re saying we want to match all `Err`
values, no matter what information they have inside them. So, the program will
execute the second arm’s code, `continue`, which tells the program to go to the
next iteration of the `loop` and ask for another guess. So, effectively, the
program ignores all errors that `parse` might encounter!
-->

Si `parse` n'arrive _pas_ à convertir la chaîne de caractères en nombre, elle
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
<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-- >
-->

<!--
```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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

```console
$ cargo run
   Compiling jeu_du_plus_ou_du_moins v0.1.0 (file:///projects/jeu_du_plus_ou_du_moins)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/jeu_du_plus_ou_du_moins`
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

Super ! Avec notre petite touche finale, nous avons fini notre jeu du plus ou du
moins. Rappelez-vous que le programme affiche toujours le nombre secret. C'était
pratique pour les tests, mais cela gâche le jeu. Supprimons le `println!` qui
affiche le nombre secret. L'encart 2-6 représente le code final.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<!--
<span class="caption">Listing 2-6: Complete guessing game code</span>
-->

<span class="caption">Encart 2-6 : Code complet du jeu du plus ou du moins
</span>

<!--
At this point, you’ve successfully built the guessing game. Congratulations!
-->

Si vous êtes arrivé jusqu'ici, c'est que vous avez construit avec succès le jeu
du plus ou du moins. Félicitations !

<!--
## Summary
-->

## Résumé


<!--
This project was a hands-on way to introduce you to many new Rust concepts:
`let`, `match`, functions, the use of external crates, and more. In the next
few chapters, you’ll learn about these concepts in more detail. Chapter 3
covers concepts that most programming languages have, such as variables, data
types, and functions, and shows how to use them in Rust. Chapter 4 explores
ownership, a feature that makes Rust different from other languages. Chapter 5
discusses structs and method syntax, and Chapter 6 explains how enums work.
-->

Ce projet était une mise en pratique pour vous initier à de nombreux concepts de
Rust : `let`, `match`, les méthodes, les fonctions associées, l'utilisation de
_crates_ externes, et bien plus. Dans les prochains chapitres, vous allez en
apprendre plus sur ces concepts. Le chapitre 3 va traiter des concepts utilisés
par la plupart des langages de programmation, comme les variables, les types de
données, et les fonctions, et vous montrera comment les utiliser avec Rust. Le
chapitre 4 expliquera la possession _(ownership)_, qui est une fonctionnalité
qui distingue Rust des autres langages. Le chapitre 5 abordera les structures et
les syntaxes des méthodes, et le chapitre 6 expliquera comment les énumérations
fonctionnent.

[prelude]: https://doc.rust-lang.org/std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: https://doc.rust-lang.org/std/string/struct.String.html
[iostdin]: https://doc.rust-lang.org/std/io/struct.Stdin.html
[read_line]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
