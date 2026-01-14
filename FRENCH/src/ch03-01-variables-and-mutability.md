<!--
## Variables and Mutability
-->

## Les variables et la mutabilité

<!--
As mentioned in the [“Storing Values with
Variables”][storing-values-with-variables]<!-- ignore -- > section, by default
variables are immutable. This is one of many nudges Rust gives you to write
your code in a way that takes advantage of the safety and easy concurrency that
Rust offers. However, you still have the option to make your variables mutable.
Let’s explore how and why Rust encourages you to favor immutability and why
sometimes you might want to opt out.
-->

Tel qu'abordé au [chapitre 2][storing-values-with-variables]<!-- ignore -->,
par défaut, les variables sont *immuables*. C'est un des nombreux coups de pouce
de Rust pour écrire votre code de façon à garantir la sécurité et la concurrence
sans problème. Cependant, vous avez quand même la possibilité de rendre vos
variables mutables *(modifiables)*. Explorons comment et pourquoi Rust vous
encourage à favoriser l'immuabilité, et pourquoi parfois vous pourriez choisir
d'y renoncer.

<!--
When a variable is immutable, once a value is bound to a name, you can’t change
that value. To illustrate this, let’s generate a new project called *variables*
in your *projects* directory by using `cargo new variables`.
-->

Lorsqu'une variable est immuable, cela signifie qu'une fois qu'une valeur est
liée à un nom, vous ne pouvez pas changer cette valeur. À titre d'illustration,
générons un nouveau projet appelé *variables* dans votre dossier *projects* en
utilisant `cargo new variables`.

<!--
Then, in your new *variables* directory, open *src/main.rs* and replace its
code with the following code. This code won’t compile just yet, we’ll first
examine the immutability error.
-->

Ensuite, dans votre nouveau dossier *variables*, ouvrez *src/main.rs* et
remplacez son code par le code suivant. Ce code ne se compile pas pour le
moment, nous allons commencer par étudier l'erreur d'immutabilité.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

<!--
Save and run the program using `cargo run`. You should receive an error
message, as shown in this output:
-->

Sauvegardez et lancez le programme en utilisant `cargo run`. Vous devriez
avoir un message d'erreur comme celui-ci :

<!--
```console
{{#include ../listings-sources/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```
-->

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

<!--
This example shows how the compiler helps you find errors in your programs.
Compiler errors can be frustrating, but really they only mean your program
isn’t safely doing what you want it to do yet; they do *not* mean that you’re
not a good programmer! Experienced Rustaceans still get compiler errors.
-->

Cet exemple montre comment le compilateur vous aide à trouver les erreurs dans
vos programmes. Les erreurs de compilation peuvent s'avérer frustrantes, mais
elles signifient en réalité que, pour le moment, votre programme n'est pas en
train de faire ce que vous voulez qu'il fasse en toute sécurité ; elles ne
signifient *pas* que vous êtes un mauvais développeur ! Même les Rustacés
expérimentés continuent d'avoir des erreurs de compilation.

<!--
The error message indicates that the cause of the error is that you `` cannot
assign twice to immutable variable `x` ``, because you tried to assign a second
value to the immutable `x` variable.
-->

Ce message d'erreur indique que la cause du problème est qu'il est
`` impossible d'assigner à deux reprises la variable immuable `x` ``
(`` cannot assign twice to immutable variable `x` ``).

<!--
It’s important that we get compile-time errors when we attempt to change a
value that’s designated as immutable because this very situation can lead to
bugs. If one part of our code operates on the assumption that a value will
never change and another part of our code changes that value, it’s possible
that the first part of the code won’t do what it was designed to do. The cause
of this kind of bug can be difficult to track down after the fact, especially
when the second piece of code changes the value only *sometimes*. The Rust
compiler guarantees that when you state a value won’t change, it really won’t
change, so you don’t have to keep track of it yourself. Your code is thus
easier to reason through.
-->

Il est important que nous obtenions des erreurs au moment de la compilation
lorsque nous essayons de changer une valeur qui a été déclarée comme immuable,
car cette situation particulière peut donner lieu à des bogues. Si une partie
de notre code part du principe qu'une valeur ne changera jamais et qu'une autre
partie de notre code modifie cette valeur, il est possible que la première
partie du code ne fasse pas ce pour quoi elle a été conçue. La cause de ce
genre de bogue peut être difficile à localiser après coup, en particulier
lorsque la seconde partie du code ne modifie que *parfois* cette valeur. Le
compilateur Rust garantit que lorsque vous déclarez qu'une valeur ne change
pas, elle ne va jamais changer, donc vous n'avez pas à vous en soucier. Votre
code est ainsi plus facile à maîtriser.

<!--
But mutability can be very useful, and can make code more convenient to write.
Variables are immutable only by default; as you did in Chapter 2, you can make
them mutable by adding `mut` in front of the variable name. Adding `mut` also
conveys intent to future readers of the code by indicating that other parts of
the code will be changing this variable’s value.
-->

Mais la mutabilité peut s'avérer très utile, et peut faciliter la rédaction du
code. Les variables sont immuables par défaut ; mais comme vous l'avez fait au
chapitre 2, vous pouvez les rendre mutables en ajoutant `mut` devant le nom de
la variable. L'ajout de `mut` va aussi signaler l'intention aux futurs lecteurs
de ce code que d'autres parties du code vont modifier la valeur de cette
variable.

<!--
For example, let’s change *src/main.rs* to the following:
-->

Par exemple, modifions *src/main.rs* ainsi :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

<!--
When we run the program now, we get this:
-->

Lorsque nous exécutons le programme, nous obtenons :

<!--
```console
{{#include ../listings-sources/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```
-->

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

<!--
We’re allowed to change the value bound to `x` from `5` to `6` when `mut`
is used. There are multiple trade-offs to consider in addition to the
prevention of bugs. For example, in cases where you’re using large data
structures, mutating an instance in place may be faster than copying and
returning newly allocated instances. With smaller data structures, creating new
instances and writing in a more functional programming style may be easier to
think through, so lower performance might be a worthwhile penalty for gaining
that clarity.
-->

En utilisant `mut`, nous avons permis à la valeur liée à `x` de passer de `5` à
`6`. Il y a d'autres compromis à envisager, en plus de la prévention des
bogues. Par exemple, dans le cas où vous utiliseriez des grosses structures de
données, muter une instance déjà existante peut être plus rapide que copier et
retourner une instance nouvellement allouée. Avec des structures de données
plus petites, créer de nouvelles instances avec un style de programmation
fonctionnelle peut rendre le code plus facile à comprendre, donc il peut valoir
le coup de sacrifier un peu de performance pour que le code gagne en clarté.

<!--
### Constants
-->

### Les constantes

<!--
Like immutable variables, *constants* are values that are bound to a name and
are not allowed to change, but there are a few differences between constants
and variables.
-->

Comme les variables immuables, les *constantes* sont des valeurs qui sont liées
à un nom et qui ne peuvent être modifiées, mais il y a quelques différences
entre les constantes et les variables.

<!--
First, you aren’t allowed to use `mut` with constants. Constants aren’t just
immutable by default—they’re always immutable. You declare constants using the
`const` keyword instead of the `let` keyword, and the type of the value *must*
be annotated. We’re about to cover types and type annotations in the next
section, [“Data Types,”][data-types]<!-- ignore -- > so don’t worry about the
details right now. Just know that you must always annotate the type.
-->

D'abord, vous ne pouvez pas utiliser `mut` avec les constantes. Les constantes
ne sont pas seulement immuables par défaut − elles sont toujours immuables. On
déclare les constantes en utilisant le mot-clé `const` à la place du mot-clé
`let`, et le type de la valeur *doit* être indiqué. Nous allons aborder les
types et les annotations de types dans la prochaine section, [“Les types de
données”][data-types]<!-- ignore -->, donc ne vous souciez pas des détails pour
le moment. Sachez seulement que vous devez toujours indiquer le type.

<!--
Constants can be declared in any scope, including the global scope, which makes
them useful for values that many parts of code need to know about.
-->

Les constantes peuvent être déclarées à n'importe quel endroit du code, y
compris la portée globale, ce qui les rend très utiles pour des valeurs que de
nombreuses parties de votre code ont besoin de connaître.

<!--
The last difference is that constants may be set only to a constant expression,
not the result of a value that could only be computed at runtime.
-->

La dernière différence est que les constantes ne peuvent être définies que par
une expression constante, et non pas le résultat d'une valeur qui ne pourrait
être calculée qu'à l'exécution.

<!--
Here’s an example of a constant declaration:
-->

Voici un exemple d'une déclaration de constante :

<!--
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
-->

```rust
const TROIS_HEURES_EN_SECONDES: u32 = 60 * 60 * 3;
```

<!--
The constant’s name is `THREE_HOURS_IN_SECONDS` and its value is set to the
result of multiplying 60 (the number of seconds in a minute) by 60 (the number
of minutes in an hour) by 3 (the number of hours we want to count in this
program). Rust’s naming convention for constants is to use all uppercase with
underscores between words. The compiler is able to evaluate a limited set of
operations at compile time, which lets us choose to write out this value in a
way that’s easier to understand and verify, rather than setting this constant
to the value 10,800. See the [Rust Reference’s section on constant
evaluation][const-eval] for more information on what operations can be used
when declaring constants.
-->

Le nom de la constante est `TROIS_HEURES_EN_SECONDES` et sa valeur est définie
comme étant le résultat de la multiplication de 60 (le nombre de secondes dans
une minute) par 60 (le nombre de minutes dans une heure) par 3 (le nombre
d'heures que nous voulons calculer dans ce programme).
En Rust, la convention de nommage des constantes est de les écrire tout en
majuscule avec des tirets bas entre les mots. Le compilateur peut calculer un
certain nombre d'opérations à la compilation, ce qui nous permet d'écrire
cette valeur de façon à la comprendre plus facilement et à la vérifier, plutôt
que de définir cette valeur à 10 800. Vous pouvez consulter la [section de la
référence Rust à propos des évaluations des constantes][const-eval] pour en
savoir plus sur les opérations qui peuvent être utilisées pour déclarer des
constantes.

<!--
Constants are valid for the entire time a program runs, within the scope they
were declared in. This property makes constants useful for values in your
application domain that multiple parts of the program might need to know about,
such as the maximum number of points any player of a game is allowed to earn or
the speed of light.
-->

Les constantes sont valables pendant toute la durée d'exécution du programme
au sein de la portée dans laquelle elles sont déclarées. Cette caractéristique
rend les constantes très utiles lorsque plusieurs parties du programme doivent
connaître certaines valeurs, comme par exemple le nombre maximum de points
qu'un joueur est autorisé à gagner ou encore la vitesse de la lumière.

<!--
Naming hardcoded values used throughout your program as constants is useful in
conveying the meaning of that value to future maintainers of the code. It also
helps to have only one place in your code you would need to change if the
hardcoded value needed to be updated in the future.
-->

Déclarer des valeurs codées en dur et utilisées tout le long de votre programme
en tant que constantes est utile pour faire comprendre la signification de ces
valeurs dans votre code aux futurs développeurs. Cela permet également de
n'avoir qu'un seul endroit de votre code à modifier si cette valeur codée en dur
doit être mise à jour à l'avenir.

<!--
### Shadowing
-->

### Le masquage

<!--
As you saw in the guessing game tutorial in [Chapter
2][comparing-the-guess-to-the-secret-number]<!-- ignore -- >, you can declare a
new variable with the same name as a previous variable. Rustaceans say that the
first variable is *shadowed* by the second, which means that the second
variable’s value is what the program sees when the variable is used. We can
shadow a variable by using the same variable’s name and repeating the use of
the `let` keyword as follows:
-->

Comme nous l'avons vu dans le [Chapitre
2][comparing-the-guess-to-the-secret-number]<!-- ignore -->, on peut déclarer
une nouvelle variable avec le même nom qu'une variable précédente. Les Rustacés
disent que la première variable est *masquée* par la seconde, ce qui signifie
que la valeur de la seconde variable sera ce que le programme verra lorsque
nous utiliserons cette variable. Nous pouvons créer un masque d'une variable en
utilisant le même nom de variable et en réutilisant le mot-clé `let` comme
ci-dessous :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

<!--
This program first binds `x` to a value of `5`. Then it shadows `x` by
repeating `let x =`, taking the original value and adding `1` so the value of
`x` is then `6`. Then, within an inner scope, the third `let` statement also
shadows `x`, multiplying the previous value by `2` to give `x` a value of `12`.
When that scope is over, the inner shadowing ends and `x` returns to being `6`.
When we run this program, it will output the following:
-->

Au début, ce programme lie `x` à la valeur `5`. Puis il crée un masque de `x`
en répétant `let x =`, en récupérant la valeur d'origine et lui ajoutant `1` :
la valeur de `x` est désormais `6`. Ensuite, à l'intérieur de la portée interne,
la troisième instruction `let` crée un autre masque de `x`, en récupérant la
précédente valeur et en la multipliant par `2` pour donner à `x` la valeur
finale de `12`. Dès que nous sortons de cette portée, le masque prends fin, et
`x` revient à la valeur `6`. Lorsque nous exécutons ce programme, nous obtenons
ceci :

<!--
```console
{{#include ../listings-sources/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```
-->

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

<!--
Shadowing is different from marking a variable as `mut`, because we’ll get a
compile-time error if we accidentally try to reassign to this variable without
using the `let` keyword. By using `let`, we can perform a few transformations
on a value but have the variable be immutable after those transformations have
been completed.
-->

Créer un masque est différent que de marquer une variable comme étant `mut`,
car à moins d'utiliser une nouvelle fois le mot-clé `let`, nous obtiendrons une
erreur de compilation si nous essayons de réassigner cette variable par
accident. Nous pouvons effectuer quelques transformations sur une valeur en
utilisant `let`, mais faire en sorte que la variable soit immuable après que ces
transformations ont été appliquées.

<!--
The other difference between `mut` and shadowing is that because we’re
effectively creating a new variable when we use the `let` keyword again, we can
change the type of the value but reuse the same name. For example, say our
program asks a user to show how many spaces they want between some text by
inputting space characters, and then we want to store that input as a number:
-->

Comme nous créons une nouvelle variable lorsque nous utilisons le mot-clé `let`
une nouvelle fois, l'autre différence entre le `mut` et la création d'un masque
est que cela nous permet de changer le type de la valeur, mais en réutilisant
le même nom. Par exemple, imaginons un programme qui demande à l'utilisateur
le nombre d'espaces qu'il souhaite entre deux portions de texte en saisissant
des espaces, et ensuite nous voulons stocker cette saisie sous forme de
nombre :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

<!--
The first `spaces` variable is a string type and the second `spaces` variable
is a number type. Shadowing thus spares us from having to come up with
different names, such as `spaces_str` and `spaces_num`; instead, we can reuse
the simpler `spaces` name. However, if we try to use `mut` for this, as shown
here, we’ll get a compile-time error:
-->

La première variable `espaces` est du type chaîne de caractères *(string)* et
la seconde variable `espaces` est du type nombre. L'utilisation du masquage
nous évite ainsi d'avoir à trouver des noms différents, comme `espaces_str` et
`espaces_num` ; nous pouvons plutôt simplement réutiliser le nom `espaces`.
Cependant, si nous essayons d'utiliser `mut` pour faire ceci, comme ci-dessous,
nous avons une erreur de compilation :

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

<!--
The error says we’re not allowed to mutate a variable’s type:
-->

L'erreur indique que nous ne pouvons pas muter le type d'une variable :

<!--
```console
{{#include ../listings-sources/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```
-->

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

<!--
Now that we’ve explored how variables work, let’s look at more data types they
can have.
-->

Maintenant que nous avons découvert comment fonctionnent les variables, étudions
les types de données qu'elles peuvent prendre.

<!--
[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: ../reference/const_eval.html
-->

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparer-le-nombre-saisi-au-nombre-secret
[data-types]: ch03-02-data-types.html#les-types-de-données
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html
[const-eval]: https://doc.rust-lang.org/reference/const_eval.html
