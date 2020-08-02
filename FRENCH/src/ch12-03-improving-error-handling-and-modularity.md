<!--
## Refactoring to Improve Modularity and Error Handling
-->

## Remanier le code pour améliorer sa modularité et la gestion des erreurs

<!--
To improve our program, we’ll fix four problems that have to do with the
program’s structure and how it’s handling potential errors.
-->

Pour améliorer notre programme, nous allons résoudre quatre problèmes liés
à la structure du programme et comment il gère de potentielles erreurs.

<!--
First, our `main` function now performs two tasks: it parses arguments and
reads files. For such a small function, this isn’t a major problem. However, if
we continue to grow our program inside `main`, the number of separate tasks the
`main` function handles will increase. As a function gains responsibilities, it
becomes more difficult to reason about, harder to test, and harder to change
without breaking one of its parts. It’s best to separate functionality so each
function is responsible for one task.
-->

Premièrement, notre fonction `main` assure deux tâches : elle interprète les
arguments et elle lit des fichiers. Pour une fonction aussi petite, ce n'est
pas un problème majeur. Cependant, si nous continuons à faire grossir notre
programme dans le `main`, le nombre des différentes tâches qu'assure la
fonction `main` va continuer à s'agrandir. Plus une fonction assure des
tâches différentes, plus cela devient difficile de la comprendre, de la tester,
et d'y faire des changements sans casser ses autres constituants. Cela est
mieux de séparer les fonctionnalités afin que chaque fonction n'assure qu'une
seule tâche.

<!--
This issue also ties into the second problem: although `query` and `filename`
are configuration variables to our program, variables like `contents` are used
to perform the program’s logic. The longer `main` becomes, the more variables
we’ll need to bring into scope; the more variables we have in scope, the harder
it will be to keep track of the purpose of each. It’s best to group the
configuration variables into one structure to make their purpose clear.
-->

Cette problématique est aussi liée au second problème : bien que `recherche` et
`nom_fichier` soient des variables de configuration de notre programme, les
variables telles que `contenu` sont utilisées pour appuyer la logique du
programme. Plus `main` est grand, plus nous aurons des variables à importer
dans la portée ; plus nous avons des variables dans notre portée, plus il sera
difficile de se souvenir à quoi elles servent. Il est préférable de regrouper
les variables de configuration dans une structure pour clarifier leur usage.

<!--
The third problem is that we’ve used `expect` to print an error message when
reading the file fails, but the error message just prints `Something went wrong
reading the file`. Reading a file can fail in a number of ways: for example,
the file could be missing, or we might not have permission to open it. Right
now, regardless of the situation, we’d print the `Something went wrong reading
the file` error message, which wouldn’t give the user any information!
-->

Le troisième problème est que nous avons utilisé `expect` pour afficher un
message d'erreur lorsque la lecture du fichier échoue, mais le message affiche
uniquement `Quelque chose s'est mal passé lors de la lecture du fichier`. Lire
un fichier peut échouer pour de nombreuses raisons : par exemple, le fichier
peut ne pas exister, ou nous n'avons pas le droit de l'ouvrir. Pour le moment,
quelle que soit la raison, nous affichons le message d'erreur `Quelque chose
s'est mal passé lors de la lecture du fichier`, ce qui ne donne aucune
information à l'utilisateur !

<!--
Fourth, we use `expect` repeatedly to handle different errors, and if the user
runs our program without specifying enough arguments, they’ll get an `index out
of bounds` error from Rust that doesn’t clearly explain the problem. It would
be best if all the error-handling code were in one place so future maintainers
had only one place to consult in the code if the error-handling logic needed to
change. Having all the error-handling code in one place will also ensure that
we’re printing messages that will be meaningful to our end users.
-->

Quatrièmement, nous utilisons `expect` à répétition pour gérer les différentes
erreurs, et si l'utilisateur lance notre programme sans renseigner d'arguments,
il va avoir une erreur `index out of bounds` provenant de Rust, qui n'explique
pas clairement le problème. Il serait plus judicieux que tout le code de gestion
des erreurs se trouve au même endroit afin que les futurs mainteneurs n'aient
qu'un seul endroit à consulter dans le code si la logique de gestion des
erreurs doit être modifiée. Avoir tout le code de gestion des erreurs dans un
seul endroit va aussi garantir que nous affichons des messages qui ont du sens
pour les utilisateurs.

<!--
Let’s address these four problems by refactoring our project.
-->

Corrigeons ces quatre problèmes en remaniant notre projet.

<!--
### Separation of Concerns for Binary Projects
-->

### Séparation des tâches des projets de binaires

<!--
The organizational problem of allocating responsibility for multiple tasks to
the `main` function is common to many binary projects. As a result, the Rust
community has developed a process to use as a guideline for splitting the
separate concerns of a binary program when `main` starts getting large. The
process has the following steps:
-->

Le problème de l'organisation de la répartition des tâches multiples dans la
fonction `main` est commun à de nombreux projets binaires. En conséquence, la
communauté Rust a développé une procédure à utiliser comme ligne conductrice
pour partager les tâches d'un programme binaire lorsque `main` commence à
grossir. Le processus se décompose selon les étapes suivantes :

<!--
* Split your program into a *main.rs* and a *lib.rs* and move your program’s
  logic to *lib.rs*.
* As long as your command line parsing logic is small, it can remain in
  *main.rs*.
* When the command line parsing logic starts getting complicated, extract it
  from *main.rs* and move it to *lib.rs*.
-->

* Diviser votre programme dans un *main.rs* et un *lib.rs* et déplacer la
  logique de votre programme dans *lib.rs*.
* Tant que votre logique d'interprétation de la ligne de commande est peu
  volumineuse, elle peut rester dans le *main.rs*
* Lorsque la logique d'interprétation de la ligne de commande commence à devenir
  compliquée, il faut la déplacer du *main.rs* vers le *lib.rs*.

<!--
The responsibilities that remain in the `main` function after this process
should be limited to the following:
-->

Les fonctionnalités qui restent dans la fonction `main` après cette procédure
seront les suivantes :

<!--
* Calling the command line parsing logic with the argument values
* Setting up any other configuration
* Calling a `run` function in *lib.rs*
* Handling the error if `run` returns an error
-->

* Appeler la logique d'interprétation de ligne de commande avec les valeurs des
  arguments
* Régler toutes les autres configurations
* Appeler une fonction `run` de *lib.rs*
* Gérer l'erreur si `run` retourne une erreur

<!--
This pattern is about separating concerns: *main.rs* handles running the
program, and *lib.rs* handles all the logic of the task at hand. Because you
can’t test the `main` function directly, this structure lets you test all of
your program’s logic by moving it into functions in *lib.rs*. The only code
that remains in *main.rs* will be small enough to verify its correctness by
reading it. Let’s rework our program by following this process.
-->

Cette structure permet de séparer les responsabilités : *main.rs* se charge de
lancer le programme, et *lib.rs* renferme toute la logique des tâches à
accomplir. Comme vous ne pouvez pas directement tester la fonction `main`, cette
structure vous permet de tester toute la logique de votre programme en les
déplaçant dans des fonctions dans *lib.rs*. Le seul code qui restera dans le
*main.rs* sera suffisamment petit pour s'assurer qu'il soit correcte en le
lisant. Lançons-nous dans le remaniement de notre programme en suivant cette
procédure.

<!--
#### Extracting the Argument Parser
-->

#### Extraction de l'interpréteur des arguments

<!--
We’ll extract the functionality for parsing arguments into a function that
`main` will call to prepare for moving the command line parsing logic to
*src/lib.rs*. Listing 12-5 shows the new start of `main` that calls a new
function `parse_config`, which we’ll define in *src/main.rs* for the moment.
-->

Nous allons déplacer la fonctionnalité de l'interprétation des arguments dans
une fonction que `main` va appeler afin de préparer le déplacement de la logique
de l'interpréteur dans *src/lib.rs*. L'encart 12-5 montre le nouveau début du
`main` qui appelle une nouvelle fonction `interpreter_config`, que nous allons
définir dans *src/main.rs* pour le moment.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 12-5: Extracting a `parse_config` function from
`main`</span>
-->

<span class="caption">Encart 12-5 : Extraction d'une fonction
`interpreter_config` à partir de `main`</span>

<!--
We’re still collecting the command line arguments into a vector, but instead of
assigning the argument value at index 1 to the variable `query` and the
argument value at index 2 to the variable `filename` within the `main`
function, we pass the whole vector to the `parse_config` function. The
`parse_config` function then holds the logic that determines which argument
goes in which variable and passes the values back to `main`. We still create
the `query` and `filename` variables in `main`, but `main` no longer has the
responsibility of determining how the command line arguments and variables
correspond.
-->

Nous continuons à récupérer les arguments de la ligne de commande dans un
vecteur, mais au lieu d'assigner la valeur de l'argument d'indice 1 à la
variable `recherche` et la valeur de l'argument d'indice 2 à la variable
`nom_fichier` dans la fonction `main`, nous passons le vecteur entier à la
fonction `interpreter_config`. La fonction `interpreter_config` renferme la
logique qui détermine quel argument va dans quelle variable et renvoie les
valeurs au `main`. Nous continuons à créer les variables `recherche` et
`nom_fichier` dans le `main`, mais `main` n'a plus la responsabilité de
déterminer quelles sont les variables qui correspondent aux arguments de la
ligne de commande.

<!--
This rework may seem like overkill for our small program, but we’re refactoring
in small, incremental steps. After making this change, run the program again to
verify that the argument parsing still works. It’s good to check your progress
often, to help identify the cause of problems when they occur.
-->

Ce remaniement peut sembler excessif pour notre petit programme, mais nous
remanions avec de manière incrémentale par de petites étapes. Après avoir fait
ces changements, lancez à nouveau le programme pour vérifier que l'envoi des
arguments fonctionne toujours. C'est une bonne chose de vérifier souvent lorsque
vous avancez, pour vous aider à mieux identifier les causes de problèmes
lorsqu'ils apparaissent.

<!--
#### Grouping Configuration Values
-->

#### Grouper les valeurs de configuration

<!--
We can take another small step to improve the `parse_config` function further.
At the moment, we’re returning a tuple, but then we immediately break that
tuple into individual parts again. This is a sign that perhaps we don’t have
the right abstraction yet.
-->

Nous pouvons appliquer une nouvelle petite étape pour améliorer la fonction
`interpreter_config`. Pour le moment, nous retournons un tuple, mais ensuite
nous divisons immédiatement ce tuple à nouveau en plusieurs éléments. C'est un
signe que nous n'avons peut-être la bonne approche.

<!--
Another indicator that shows there’s room for improvement is the `config` part
of `parse_config`, which implies that the two values we return are related and
are both part of one configuration value. We’re not currently conveying this
meaning in the structure of the data other than by grouping the two values into
a tuple; we could put the two values into one struct and give each of the
struct fields a meaningful name. Doing so will make it easier for future
maintainers of this code to understand how the different values relate to each
other and what their purpose is.
-->

Un autre signe qui indique qu'il y a encore de la place pour de l'amélioration
est que la partie `config` de `interpreter_config`, ce qui sous-entend que les
deux valeurs que nous retournons sont liées et font partie d'une même valeur de
configuration. Actuellement, nous ne donnons pas de signification à cela dans la
structure des données autrement qu'en regroupant les deux valeurs dans un
tuple ; nous pourrions mettre les deux valeurs dans une seule structure et
donner un nom significatif à chacun des champs de la structure. Faire ainsi
permet de faciliter la compréhension du code par les futurs développeurs de ce
code pour comprendre le lien entre les deux et quels sont leurs rôles.

<!--
> Note: Using primitive values when a complex type would be more appropriate is
> an anti-pattern known as *primitive obsession*.
-->

> Remarque : l'utilisation de valeurs primitives à la place d'un type
> sophistiqué lorsque c'est nécessaire est un anti-patron connu sous le nom
> *d'obsession primitive*.

<!--
Listing 12-6 shows the improvements to the `parse_config` function.
-->

L'encart 12-6 montre les améliorations apportées à la fonction
`interpreter_config`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,should_panic
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```
-->

```rust,should_panic
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 12-6: Refactoring `parse_config` to return an
instance of a `Config` struct</span>
-->

<span class="caption">Encart 12-6 : Remaniement de `interpreter_config` pour
retourner une instance de la structure `Config`</span>

<!--
We’ve added a struct named `Config` defined to have fields named `query` and
`filename`. The signature of `parse_config` now indicates that it returns a
`Config` value. In the body of `parse_config`, where we used to return string
slices that reference `String` values in `args`, we now define `Config` to
contain owned `String` values. The `args` variable in `main` is the owner of
the argument values and is only letting the `parse_config` function borrow
them, which means we’d violate Rust’s borrowing rules if `Config` tried to take
ownership of the values in `args`.
-->

Nous avons ajouté une structure `Config` qui a deux champs `recherche` et
`nom_fichier`. La signature de `interpreter_config` indique maintenant qu'elle
retourne une valeur `Config`. Dans le corps de `interpreter_config`, où nous
retournions une slice de chaînes de caractères qui pointaient sur des valeurs
`String` présentes dans `args`, nous définissons maintenant la structure
`Config` pour contenir des valeurs `String` qu'elle possède. La variable `args`
du `main` est la propriétaire des valeurs des arguments et permet uniquement à
la fonction `interpreter_config` de les emprunter, ce qui signifie que nous
violons les règles d'emprunt de Rust si `Config` essaye de prendre possession
des valeurs provenant de `args`.

<!--
We could manage the `String` data in a number of different ways, but the
easiest, though somewhat inefficient, route is to call the `clone` method on
the values. This will make a full copy of the data for the `Config` instance to
own, which takes more time and memory than storing a reference to the string
data. However, cloning the data also makes our code very straightforward
because we don’t have to manage the lifetimes of the references; in this
circumstance, giving up a little performance to gain simplicity is a worthwhile
trade-off.
-->

Nous pourrions gérer les données `String` de plusieurs manières, mais la façon
la plus facile, bien que non optimisée, est d'appeler la méthode `clone` sur
les valeurs. Cela va produire une copie complète des données pour que
l'instance de `Config` puisse se les approprier, ce qui va prendre plus de
temps et de mémoire que de stocker une référence vers les données de la chaîne
de caractères. Cependant le clonage des données rend votre code très simple
car nous n'avons pas à gérer les durées de vie des références ; dans ces
circonstances, sacrifier un peu de performances pour gagner en simplicité est
un compromis qui en vaut la peine.

<!--
> ### The Trade-Offs of Using `clone`
>
> There’s a tendency among many Rustaceans to avoid using `clone` to fix
> ownership problems because of its runtime cost. In
> [Chapter 13][ch13]<!-- ignore -- >, you’ll learn how to use more efficient
> methods in this type of situation. But for now, it’s okay to copy a few
> strings to continue making progress because you’ll make these copies only
> once and your filename and query string are very small. It’s better to have
> a working program that’s a bit inefficient than to try to hyperoptimize code
> on your first pass. As you become more experienced with Rust, it’ll be
> easier to start with the most efficient solution, but for now, it’s
> perfectly acceptable to call `clone`.
-->

> ### Les contre-parties de l'utilisation de `clone`
>
> Il y a une tendance chez les Rustacés de s'interdire l'utilisation de `clone`
> pour régler les problèmes d'appartenance à cause du coût à l'exécution. Dans
> le [chapitre 13][ch13]<!-- ignore -->, vous allez apprendre à utiliser des
> méthodes plus efficaces dans ce genre de situation. Mais pour le moment, ce
> n'est pas un problème de copier quelques chaînes de caractères pour continuer
> à progresser car vous allez le faire une seule fois et votre `nom_fichier` et
> `recherche` sont très courts. Il est plus important d'avoir un programme
> fonctionnel qui est n'est très optimisé plutôt que d'essayer d'optimiser à
> outrance le code dès sa première écriture. Plus vous deviendrez expérimenté
> en Rust, plus il sera facile de commencer par la solution la plus
> performante, mais pour le moment, il est parfaitement acceptable de faire
> appel à `clone`.

<!--
We’ve updated `main` so it places the instance of `Config` returned by
`parse_config` into a variable named `config`, and we updated the code that
previously used the separate `query` and `filename` variables so it now uses
the fields on the `Config` struct instead.
-->

Nous avons actualisé `main` pour qu'il utilise l'instance de `Config` retournée
par `interpreter_config` dans une variable `config`, et nous avons rafraîchi le
code qui utilisait les variables séparées `recherche` et `nom_fichier` pour
qu'il utilise maintenant les champs de la structure `Config` à la place.

<!--
Now our code more clearly conveys that `query` and `filename` are related and
that their purpose is to configure how the program will work. Any code that
uses these values knows to find them in the `config` instance in the fields
named for their purpose.
-->

Maintenant, notre code indique clairement que `recherche` et `nom_fichier` sont
reliés et que leur but est de configurer le fonctionnement du programme.
N'importe quel code qui utilise ces valeurs sait comment les retrouver dans les
champs de l'instance `config` grâce à leurs noms donnés à cet effet.

<!--
#### Creating a Constructor for `Config`
-->

#### Créer un constructeur pour `Config`

<!--
So far, we’ve extracted the logic responsible for parsing the command line
arguments from `main` and placed it in the `parse_config` function. Doing so
helped us to see that the `query` and `filename` values were related and that
relationship should be conveyed in our code. We then added a `Config` struct to
name the related purpose of `query` and `filename` and to be able to return the
values’ names as struct field names from the `parse_config` function.
-->

Pour l'instant, nous avons extrait la logique en charge d'interpréter les
arguments de la ligne de commande à partir du `main` et nous l'avons placé dans
la fonction `interpreter_config`. Cela nous a aidé à découvrir que les valeurs
`recherche` et `nom_fichier` étaient liées et que ce lien devait être
retranscrit dans notre code. Nous avons ensuite créé une structure `Config`
afin de donner un nom au rôle apparenté à `recherche` et à `nom_fichier`, et
pour pouvoir retourner les noms des valeurs sous la forme de noms de champs à
partir de la fonction `interpreter_config`.

<!--
So now that the purpose of the `parse_config` function is to create a `Config`
instance, we can change `parse_config` from a plain function to a function
named `new` that is associated with the `Config` struct. Making this change
will make the code more idiomatic. We can create instances of types in the
standard library, such as `String`, by calling `String::new`. Similarly, by
changing `parse_config` into a `new` function associated with `Config`, we’ll
be able to create instances of `Config` by calling `Config::new`. Listing 12-7
shows the changes we need to make.
-->

Maintenant que le but de la fonction `interpreter_config` est de créer une
instance de `Config`, nous pouvons transformer `interpreter_config` d'une
simple fonction à une fonction `new` qui est associée à la structure `Config`.
Ce changement rendra le code plus familier. Habituellement, nous créons des
instances de types de la bibliothèque standard, comme `String`, en appelant
`String::new`. Si on change le `interpreter_config` en une fonction `new`
associée à `Config`, nous pourrons créer de la même façon des instances de
`Config` en appelant `Config::new`. L'encart 12-7 nous montre les changements
que nous devons faire pour cela.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,should_panic
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```
-->

```rust,should_panic
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

<!--
<span class="caption">Listing 12-7: Changing `parse_config` into
`Config::new`</span>
-->

<span class="caption">Encart 12-7 : Transformer `interpreter_config` en
`Config::new`</span>

<!--
We’ve updated `main` where we were calling `parse_config` to instead call
`Config::new`. We’ve changed the name of `parse_config` to `new` and moved it
within an `impl` block, which associates the `new` function with `Config`. Try
compiling this code again to make sure it works.
-->

Nous avons actualisé le `main` où nous appelions `interpreter_config` pour
appeler à la place le `Config::new`. Nous avons changé le nom de
`interpreter_config` par `new` et nous l'avons déplacé dans un bloc `impl`,
ce qui relie la fonction `new` à `Config`. Essayez à nouveau de compiler ce
code pour vous assurer qu'il fonctionne.

<!--
### Fixing the Error Handling
-->

### Corriger la gestion des erreurs

<!--
Now we’ll work on fixing our error handling. Recall that attempting to access
the values in the `args` vector at index 1 or index 2 will cause the program to
panic if the vector contains fewer than three items. Try running the program
without any arguments; it will look like this:
-->

Maintenant, nous allons nous pencher sur la correction de la gestion des
erreurs. Rappellez-vous que la tentative d'accéder aux valeurs dans le vecteur
`args` aux indices 1 ou 2 va faire paniquer le programme si le vecteur contient
moins de trois éléments. Essayez de lancer le programme sans aucun argument ;
cela donnera quelque chose comme ceci :

<!--
```console
{{#include ../listings-sources/ch12-an-io-project/listing-12-07/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

<!--
The line `index out of bounds: the len is 1 but the index is 1` is an error
message intended for programmers. It won’t help our end users understand what
happened and what they should do instead. Let’s fix that now.
-->

La ligne `index out of bounds: the len is 1 but the index is 1` est un
message d'erreur destiné aux développeurs. Il n'aidera pas nos utilisateurs
finaux à comprendre ce qu'il s'est passé et ce qu'ils devraient faire à la
place. Corrigeons cela dès maintenant.

<!--
#### Improving the Error Message
-->

#### Améliorer le message d'erreur

<!--
In Listing 12-8, we add a check in the `new` function that will verify that the
slice is long enough before accessing index 1 and 2. If the slice isn’t long
enough, the program panics and displays a better error message than the `index
out of bounds` message.
-->

Dans l'encart 12-8, nous ajoutons une vérification dans la fonction `new`, qui
va vérifier que le slice est suffisamment grand avant d'accéder aux indices 1
et 2. Si le slice n'est pas suffisamment grand, le programme va paniquer et
afficher un meilleur message d'erreur que le message `index out of bounds`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

```rust,ignore
// -- partie masquée ici --
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("il n'y a pas assez d'arguments");
    }
    // -- partie masquée ici --
```

<!--
<span class="caption">Listing 12-8: Adding a check for the number of
arguments</span>
-->

<span class="caption">Encart 12-8 : Ajout d'une vérification du nombre
d'arguments</span>

<!--
This code is similar to [the `Guess::new` function we wrote in Listing
9-10][ch9-custom-types]<!-- ignore -- >, where we called `panic!` when the
`value` argument was out of the range of valid values. Instead of checking for
a range of values here, we’re checking that the length of `args` is at least 3
and the rest of the function can operate under the assumption that this
condition has been met. If `args` has fewer than three items, this condition
will be true, and we call the `panic!` macro to end the program immediately.
-->

Ce code est similaire à [la fonction Supposition::new que nous avons écrit
dans l'encart 9-10][ch9-custom-types]<!-- ignore -->, dans lequel nous
appelions `panic!` lorsque l'argument `valeur` était hors de l'intervalle des
valeurs valides. Plutôt que de vérifier un intervalle de valeurs dans le cas
présent, nous vérifions que la taille de `args` est au moins de 3 et que le
reste de la fonction puisse fonctionner en s'appuyant sur l'affirmation que
cette condition a bien été remplie. Si `args` avait moins de trois éléments,
cette fonction serait vraie, et nous appellerions alors la macro `panic!`
pour mettre fin au programme immédiatement.

<!--
With these extra few lines of code in `new`, let’s run the program without any
arguments again to see what the error looks like now:
-->

Avec ces quelques lignes de code en plus dans `new`, lançons le programme sans
aucun argument à nouveau pour voir à quoi ressemble désormais l'erreur :

<!--
```console
{{#include ../listings-sources/ch12-an-io-project/listing-12-08/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

<!--
This output is better: we now have a reasonable error message. However, we also
have extraneous information we don’t want to give to our users. Perhaps using
the technique we used in Listing 9-10 isn’t the best to use here: a call to
`panic!` is more appropriate for a programming problem than a usage problem,
[as discussed in Chapter 9][ch9-error-guidelines]<!-- ignore -- >. Instead, we
can use the other technique you learned about in Chapter 9—[returning a
`Result`][ch9-result]<!-- ignore -- > that indicates either success or an error.
-->

Cette sortie est meilleure : nous avons maintenant un message d'erreur
compréhensible. Cependant, nous avons aussi des informations superflues que
nous ne souhaitons pas afficher à nos utilisateurs. Peut-être que la technique
que nous avons utilisé dans l'encart 9-10 n'est pas la plus appropriée dans ce
cas : un appel à `panic!` est plus approprié pour un problème de développement
qu'un problème d'utilisation, [comme nous l'avons appris au chapitre
9][ch9-error-guidelines]<!-- ignore -->. A la place, nous pourrions utiliser
une autre technique que vous avez appris au chapitre 9 — [retourner un
`Result`][ch9-result]<!-- ignore --> qui indique si c'est un succès ou une
erreur.

<!--
#### Returning a `Result` from `new` Instead of Calling `panic!`
-->

#### Retourner un `Result` à partir de `new` plutôt que d'appeler `panic!`

<!--
We can instead return a `Result` value that will contain a `Config` instance in
the successful case and will describe the problem in the error case. When
`Config::new` is communicating to `main`, we can use the `Result` type to
signal there was a problem. Then we can change `main` to convert an `Err`
variant into a more practical error for our users without the surrounding text
about `thread 'main'` and `RUST_BACKTRACE` that a call to `panic!` causes.
-->

Nous pouvons à la place retourner une valeur `Result` qui contiendra une
instance de `Config` dans le cas d'un succès et va décrire le problème dans le
cas d'une erreur. Lorsque `Config::new` communiquera avec le `main`, nous
pourrons utiliser le type de `Result` pour signaler où il y a un problème.
Ensuite, nous pourrons changer le `main` pour convertir une variante de `Err`
dans une erreur plus pratique pour nos utilisateurs sans avoir le texte à
propos de `thread 'main'` et de `RUST_BACKTRACE` qui sont provoqués par l'appel
à `panic!`.

<!--
Listing 12-9 shows the changes we need to make to the return value of
`Config::new` and the body of the function needed to return a `Result`. Note
that this won’t compile until we update `main` as well, which we’ll do in the
next listing.
-->

L'encart 12-9 nous montre les changements que nous devons faire pour à la
valeur de retour de `Config::new` et le corps de la fonction qui doit retourner
un `Result`. Notez que cela ne va pas se compiler tant que nous ne corrigeons
pas aussi le `main`, ce que nous allons faire dans le prochain encart.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 12-9: Returning a `Result` from
`Config::new`</span>
-->

<span class="caption">Encart 12-9 : Retourner un `Result` à partir de
`Config::new`</span>

<!--
Our `new` function now returns a `Result` with a `Config` instance in the
success case and a `&'static str` in the error case. Recall from [“The Static
Lifetime”][the-static-lifetime]<!-- ignore -- > section in Chapter 10 that
`&'static str` is the type of string literals, which is our error message type
for now.
-->

Notre fonction `new` retourne désormais un `Result` contenant une instance de
`Config` dans le cas d'un succès et une `&'static str` dans le cas d'une
erreur. Pour rappel, nous avons vu dans une section du
[chapitre 10][the-static-lifetime]<!-- ignore --> que `&'static str` est le
type des chaînes de caractères littérales, ce qui est désormais le type de
notre message d'erreur.

<!--
We’ve made two changes in the body of the `new` function: instead of calling
`panic!` when the user doesn’t pass enough arguments, we now return an `Err`
value, and we’ve wrapped the `Config` return value in an `Ok`. These changes
make the function conform to its new type signature.
-->

Nous avons fait deux changements dans le corps de notre fonction `new` :
plutôt que d'avoir à appeler `panic!` lorsque l'utilisateur n'envoie pas assez
d'arguments, nous retournons maintenant une valeur `Err`, et nous avons intégré
la valeur de retour `Config` dans un `Ok`. Ces modifications font en sorte que
la fonction soit désormais conformes à son nouveau type de signature.

<!--
Returning an `Err` value from `Config::new` allows the `main` function to
handle the `Result` value returned from the `new` function and exit the process
more cleanly in the error case.
-->

Retourner une valeur `Err` à partir de `Config::new` permet à la fonction
`main` de gérer la valeur `Result` retournée par la fonction `new` et de
terminer plus proprement le processus dans cas d'une erreur.

<!--
#### Calling `Config::new` and Handling Errors
-->

#### Appeler `Config::new` et gérer les erreurs

<!--
To handle the error case and print a user-friendly message, we need to update
`main` to handle the `Result` being returned by `Config::new`, as shown in
Listing 12-10. We’ll also take the responsibility of exiting the command line
tool with a nonzero error code from `panic!` and implement it by hand. A
nonzero exit status is a convention to signal to the process that called our
program that the program exited with an error state.
-->

Pour gérer les cas d'erreurs et afficher un message correct pour
l'utilisateur, nous devons mettre à jour `main` pour gérer le `Result`
retourné par `Config::new`, comme dans l'encart 12-10. Nous allons aussi
prendre la décision de quitter l'outil en ligne de commande avec un code
d'erreur différent de zéro avec `panic!` et nous allons l'implémenter
manuellement. Un statut de sortie différent de zéro est une convention pour
signaler au processus qui a appelé notre programme que le programme s'est
terminé dans un état d'erreur.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

<!--
<span class="caption">Listing 12-10: Exiting with an error code if creating a
new `Config` fails</span>
-->

<span class="caption">Encart 12-10 : Quitter avec un code d'erreur si la
création d'une nouvelle `Config` échoue.
</span>

<!--
In this listing, we’ve used a method we haven’t covered before:
`unwrap_or_else`, which is defined on `Result<T, E>` by the standard library.
Using `unwrap_or_else` allows us to define some custom, non-`panic!` error
handling. If the `Result` is an `Ok` value, this method’s behavior is similar
to `unwrap`: it returns the inner value `Ok` is wrapping. However, if the value
is an `Err` value, this method calls the code in the *closure*, which is an
anonymous function we define and pass as an argument to `unwrap_or_else`. We’ll
cover closures in more detail in [Chapter 13][ch13]<!-- ignore -- >. For now,
you just need to know that `unwrap_or_else` will pass the inner value of the
`Err`, which in this case is the static string `not enough arguments` that we
added in Listing 12-9, to our closure in the argument `err` that appears
between the vertical pipes. The code in the closure can then use the `err`
value when it runs.
-->

Dans cet encart, nous avons utilisé une méthode que nous n'avons pas encore
abordé : `unwrap_or_else`, qui est défini sur `Result<T, E>` par la bibliothèque
standard. L'utilisation de `unwrap_or_else` nous permet de définir une gestion
des erreurs personnalisée, exempt de `panic!`. Si le `Result` est une valeur
`Ok`, le comportement de cette méthode est similaire à `unwrap` : elle retourne
la valeur à l'intérieur du `Ok`. Cependant, si la valeur est une valeur `Err`,
cette méthode appelle le code dans la *fermeture*, qui est une fonction anonyme
que nous définissons et passons en argument de `unwrap_or_else`. Nous verrons
les fermetures plus en détail dans le [chapitre 13][ch13]<!-- ignore -->. Pour
l'instant, vous avez juste à savoir que le `unwrap_or_else` va passer la valeur
interne du `Err` (qui dans ce cas est la chaîne de caractères statique
`pas assez d'arguments` que nous avons ajouté dans l'encart 12-9) à notre
fermeture dans l'argument `err` qui est présent entre deux barres verticales. Le
code dans la fermeture peut ensuite utiliser la valeur `err` lorsqu'il est
exécuté.

<!--
We’ve added a new `use` line to bring `process` from the standard library into
scope. The code in the closure that will be run in the error case is only two
lines: we print the `err` value and then call `process::exit`. The
`process::exit` function will stop the program immediately and return the
number that was passed as the exit status code. This is similar to the
`panic!`-based handling we used in Listing 12-8, but we no longer get all the
extra output. Let’s try it:
-->

Nous avons ajouté une nouvelle ligne `use` pour importer `porté` dans la portée
à partir de la bibliothèque standard. Le code dans la fermeture qui sera exécuté
dans le cas d'une erreur fait uniquement deux lignes : nous affichons la valeur
de `err` et nous appelons ensuite `process::exit`. La fonction `process::exit`
va stopper le programme immédiatement et retourner le nombre qui lui a été donné
en paramètre comme code de statut de sortie. C'est semblable à la gestion basée
sur `panic!` que nous avons utilisé à l'encart 12-8, mais nous n'avons plus tout
le texte en plus. Essayons cela :

<!--
```console
{{#include ../listings-sources/ch12-an-io-project/listing-12-10/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

<!--
Great! This output is much friendlier for our users.
-->

Très bien ! Cette sortie est bien plus compréhensible pour nos utilisateurs.

<!--
### Extracting Logic from `main`
-->

### Extraction de la logique du `main`

<!--
Now that we’ve finished refactoring the configuration parsing, let’s turn to
the program’s logic. As we stated in [“Separation of Concerns for Binary
Projects”](#separation-of-concerns-for-binary-projects)<!-- ignore -- >, we’ll
extract a function named `run` that will hold all the logic currently in the
`main` function that isn’t involved with setting up configuration or handling
errors. When we’re done, `main` will be concise and easy to verify by
inspection, and we’ll be able to write tests for all the other logic.
-->

Maintenant que nous avons fini le remaniement de l'interprétation de la
configuration, occupons-nous de logique du programme. Comme nous l'avons dit
dans [“Séparation des tâches des projets de
binaires”](#separation-of-concerns-for-binary-projects)<!-- ignore -->, nous
allons extraire une fonction `run` qui va contenir toute la logique qui est
actuellement dans la fonction `main` qui n'est pas liée au réglage de la
configuration ou la gestion des erreurs. Lorsque nous aurons terminé, `main`
sera plus concise et facile à vérifier en l'inspectant, et nous pourrons écrire
des tests pour toutes les autres logiques.

<!--
Listing 12-11 shows the extracted `run` function. For now, we’re just making
the small, incremental improvement of extracting the function. We’re still
defining the function in *src/main.rs*.
-->

L'encart 12-11 montre la fonction `run` extraite. Pour le moment, nous faisons
des petites améliorations progressives pour extraire les fonctions. Nous
continuons à définir la fonction dans *src/main.rs*.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 12-11: Extracting a `run` function containing the
rest of the program logic</span>
-->

<span class="caption">Encart 12-11 : Extraction d'une fonction `run` qui
contient le reste de la logique du programme</span>

<!--
The `run` function now contains all the remaining logic from `main`, starting
from reading the file. The `run` function takes the `Config` instance as an
argument.
-->

La fonction `run` contient maintenant toute la logique qui restait dans le
`main`, en commençant par la lecture du fichier. La fonction `run` prend
l'instance de `Config` en argument.

<!--
#### Returning Errors from the `run` Function
-->

#### Retourner des erreurs avec la fonction `run`

<!--
With the remaining program logic separated into the `run` function, we can
improve the error handling, as we did with `Config::new` in Listing 12-9.
Instead of allowing the program to panic by calling `expect`, the `run`
function will return a `Result<T, E>` when something goes wrong. This will let
us further consolidate into `main` the logic around handling errors in a
user-friendly way. Listing 12-12 shows the changes we need to make to the
signature and body of `run`.
-->

Avec le restant de la logique du programme maintenant séparée dans la fonction
`run`, nous pouvons améliorer la gestion des erreurs, comme nous l'avons fait
avec `Config::new` dans l'encart 12-9. Plutôt que de permettre au programme à
paniquer en appelant `expect`, la fonction `run` va retourner un `Result<T, E>`
lorsque quelque chose se passe mal. Cela va nous permettre de consolider
davantage la logique de gestion des erreurs dans le `main` pour qu'elle soit
plus conviviale pour l'utilisateur. L'encart 12-12 montre les changements que
nous devons appliquer à la signature et au corps du `run`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

<!--
<span class="caption">Listing 12-12: Changing the `run` function to return
`Result`</span>
-->

<span class="caption">Encart 12-12 : Changer la fonction `run` pour retourner
un `Result`</span>

<!--
We’ve made three significant changes here. First, we changed the return type of
the `run` function to `Result<(), Box<dyn Error>>`. This function previously
returned the unit type, `()`, and we keep that as the value returned in the
`Ok` case.
-->

Nous avons fait trois changements significatifs ici. Premièrement, nous avons
changé le type de retour de la fonction `run` en `Result<(), Box<dyn Error>>`.
Cette fonction renvoyait précédemment le type unité, `()`, que nous gardons
comme valeur de retour dans le cas de `Ok`.

<!--
For the error type, we used the *trait object* `Box<dyn Error>` (and we’ve
brought `std::error::Error` into scope with a `use` statement at the top).
We’ll cover trait objects in [Chapter 17][ch17]<!-- ignore -- >. For now, just
know that `Box<dyn Error>` means the function will return a type that
implements the `Error` trait, but we don’t have to specify what particular type
the return value will be. This gives us flexibility to return error values that
may be of different types in different error cases. The `dyn` keyword is short
for “dynamic.”
-->

En ce qui concerne le type d'erreur, nous avons utilisé *l'objet trait*
`Box<dyn Error>` (et nous avons importé `std::error::Error` dans la portée avec
une instruction `use` en haut). Nous allons voir les objets trait dans le
[chapitre 17][ch17]<!-- ignore -->. Pour l'instant, retenez juste que
`Box<dyn Error>` signifie que la fonction va retourner un type qui implémente
le trait `Error`, mais que nous n'avons pas à préciser quel sera précisément le
type de la valeur de retour. Cela nous donne de la flexibilité sur les valeurs
d'erreurs de retour qui peuvent être différentes dans différents cas d'erreurs.
Le mot-clé `dyn` est un raccourci pour “dynamique”.

<!--
Second, we’ve removed the call to `expect` in favor of the `?` operator, as we
talked about in [Chapter 9][ch9-question-mark]<!-- ignore -- >. Rather than
`panic!` on an error, `?` will return the error value from the current function
for the caller to handle.
-->

Deuxièmement, nous avons enlevé l'appel à `expect` pour privilégier l'opérateur
`?`, que nous avons vu dans le [chapitre 9][ch9-question-mark]<!-- ignore -->.
Au lieu de faire un `panic!` sur une erreur, `?` va retourner la valeur d'erreur
de la fonction courante vers le code qui l'a appelé pour qu'il la gère.

<!--
Third, the `run` function now returns an `Ok` value in the success case. We’ve
declared the `run` function’s success type as `()` in the signature, which
means we need to wrap the unit type value in the `Ok` value. This `Ok(())`
syntax might look a bit strange at first, but using `()` like this is the
idiomatic way to indicate that we’re calling `run` for its side effects only;
it doesn’t return a value we need.
-->

Troisièmement, la fonction `run` retourne maintenant une valeur `Ok` dans les
cas de succès. Nous avons déclaré dans la signature que le type de succès de la
fonction `run` était `()`, ce qui signifie que nous avons envelopper la valeur
de type unité dans la valeur `Ok`. Cette syntaxe `Ok(())` peut sembler un peu
étrange au départ, mais utiliser `()` de cette manière est la façon idéale
d'indiquer que nous appelons `run` uniquement pour ses effets secondaires ; elle
ne retourne pas de valeur que nous pourrions avoir besoin.

<!--
When you run this code, it will compile but will display a warning:
-->

Lorsque vous exécutez ce code, il va se compiler mais il va afficher un
avertissement :

<!--
```console
{{#include ../listings-sources/ch12-an-io-project/listing-12-12/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

<!--
Rust tells us that our code ignored the `Result` value and the `Result` value
might indicate that an error occurred. But we’re not checking to see whether or
not there was an error, and the compiler reminds us that we probably meant to
have some error-handling code here! Let’s rectify that problem now.
-->

Rust nous informe que notre code ignore la valeur `Result` et que cette valeur
`Result` pourrait indiquer qu'une erreur s'est passée. Mais nous ne vérifions
pas pour savoir si oui ou non il y a eu une erreur, et le compilateur nous
rappelle que nous devrions avoir du code de gestion des erreurs ici !
Corrigeons dès à présent ce problème.

<!--
#### Handling Errors Returned from `run` in `main`
-->

#### Gérer les erreurs retournées par `run` dans `main`

<!--
We’ll check for errors and handle them using a technique similar to one we used
with `Config::new` in Listing 12-10, but with a slight difference:
-->

Nous allons vérifier les erreurs et les gérer en utilisant une technique
similaire à celle que nous avons utilisé avec `Config::new` dans l'encart
12-10, mais avec une légère différence :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

<!--
We use `if let` rather than `unwrap_or_else` to check whether `run` returns an
`Err` value and call `process::exit(1)` if it does. The `run` function doesn’t
return a value that we want to `unwrap` in the same way that `Config::new`
returns the `Config` instance. Because `run` returns `()` in the success case,
we only care about detecting an error, so we don’t need `unwrap_or_else` to
return the unwrapped value because it would only be `()`.
-->

Nous utilisons `if let` plutôt que `unwrap_or_else` pour vérifier si `run`
retourne un valeure `Err` et appeler `process::exit(1)` le cas échéant. La
fonction `run` ne retourne pas de valeur que nous avons besoin de `unwrap`
comme nous l'avions fait avec le `Config::new` qui retournait une instance de
`Config`. Comme `run` retourne `()` dans le cas d'un succès, nous nous
préoccupons uniquement de détecter les erreurs, donc n'avons pas besoin de
`unwrap_or_else` pour retourner la valeur extraite car elle sera toujours
`()`.

<!--
The bodies of the `if let` and the `unwrap_or_else` functions are the same in
both cases: we print the error and exit.
-->

Les corps du `if let` et de la fonction `unwrap_or_else` sont identiques dans
les deux cas : nous affichons l'erreur et nous quittons.

<!--
### Splitting Code into a Library Crate
-->

### Déplacer le code dans une crate de bibliothèque

<!--
Our `minigrep` project is looking good so far! Now we’ll split the
*src/main.rs* file and put some code into the *src/lib.rs* file so we can test
it and have a *src/main.rs* file with fewer responsibilities.
-->

Notre projet `minigrep` se présente plutôt bien pour le moment ! Maintenant,
nous allons diviser notre fichier *src/main.rs* et déplacer du code dans le
fichier *src/lib.rs* pour que nous puissions le tester et avoir fichier
*src/main.rs* qui héberge moins de fonctionnalités.

<!--
Let’s move all the code that isn’t the `main` function from *src/main.rs* to
*src/lib.rs*:
-->

Déplaçons tout le code qui ne fait pas partie de la fonction `main` dans le
*src/main.rs* vers le *src/lib.rs* :

<!--
* The `run` function definition
* The relevant `use` statements
* The definition of `Config`
* The `Config::new` function definition
-->

* La définition de la fonction `run`
* Les instructions `use` correspondantes
* La définition de `Config`
* La définition de la fonction `Config::new`

<!--
The contents of *src/lib.rs* should have the signatures shown in Listing 12-13
(we’ve omitted the bodies of the functions for brevity). Note that this won’t
compile until we modify *src/main.rs* in Listing 12-14.
-->

Le contenu du *src/lib.rs* devrait contenir les signatures de l'encart 12-13
(nous avons enlevé les corps des fonctions pour des raisons de brièveté). Notez
que cela ne va pas se compiler jusqu'à ce que nous modifions le *src/main.rs*
dans l'encart 12-14.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-13/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-13: Moving `Config` and `run` into
*src/lib.rs*</span>
-->

<span class="caption">Encart 12-13 : Déplacement de `Config` et de `run` dans
*src/lib.rs*</span>

<!--
We’ve made liberal use of the `pub` keyword: on `Config`, on its fields and its
`new` method, and on the `run` function. We now have a library crate that has a
public API that we can test!
-->

Nous avons fait un usage généreux du mot-clé `pub` : sur `Config`, sur ses
champs et sur la méthode `new`, et sur la fonction `run`. Nous avons maintenant
une crate de bibliothèque qui a une API publique que nous pouvons tester !

<!--
Now we need to bring the code we moved to *src/lib.rs* into the scope of the
binary crate in *src/main.rs*, as shown in Listing 12-14.
-->

Maintenant nous devons importer le code que nous avons déplacé dans
*src/lib.rs* dans la portée de la crate binaire dans *src/main.rs*, comme dans
l'encart 12-14.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

<!--
<span class="caption">Listing 12-14: Using the `minigrep` library crate in
*src/main.rs*</span>
-->

<span class="caption">Encart 12-14 : Utilisation de la crate de bibliothèque
`minigrep` dans *src/main.rs*</span>

<!--
We add a `use minigrep::Config` line to bring the `Config` type from the
library crate into the binary crate’s scope, and we prefix the `run` function
with our crate name. Now all the functionality should be connected and should
work. Run the program with `cargo run` and make sure everything works
correctly.
-->

Nous avons ajouté une ligne `use minigrep::Config` pour importer le type
`Config` de la crate de bibliothèque dans la portée de la crate binaire, et
nous avons avons préfixé la fonction `run` avec le nom de notre crate.
Maintenant, toutes les fonctionnalités devraient être connectées et devraient
fonctionner. Lancez le programme avec `cargo run` pour vous assurer que tout
fonctionne correctement.

<!--
Whew! That was a lot of work, but we’ve set ourselves up for success in the
future. Now it’s much easier to handle errors, and we’ve made the code more
modular. Almost all of our work will be done in *src/lib.rs* from here on out.
-->

Ouah ! C'était pas mal de travail, mais nous sommes organisés pour nous assurer
le succès à venir. Maintenant il est bien plus facile de gérer les erreurs, et
nous avons rendu le code plus modulaire. A partir de maintenant, l'essentiel de
notre travail sera effectué dans *src/lib.rs*.

<!--
Let’s take advantage of this newfound modularity by doing something that would
have been difficult with the old code but is easy with the new code: we’ll
write some tests!
-->

Profitons de cette nouvelle modularité en accomplissant quelque chose qui
aurait été difficile à faire avec l'ancien code, mais qui est facile avec ce
nouveau code : nous allons écrire des tests !

<!--
[the-static-lifetime]: ch10-03-lifetime-syntax.html#the-static-lifetime
[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch17]: ch17-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
-->

[the-static-lifetime]: ch10-03-lifetime-syntax.html
[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch17]: ch17-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html
