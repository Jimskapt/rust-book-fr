<!--
## Control Flow
-->

## Les structures de contrôle

<!--
Deciding whether or not to run some code depending on if a condition is true
and deciding to run some code repeatedly while a condition is true are basic
building blocks in most programming languages. The most common constructs that
let you control the flow of execution of Rust code are `if` expressions and
loops.
-->

Choisir d'exécuter ou non du code selon qu'une condition est vérifiée et
choisir d'exécuter du code de façon répétée tant qu'une condition est vérifiée
sont des constructions élémentaires dans la plupart des langages de
programmation. Les structures de contrôle les plus courantes en Rust sont les
expressions `if` et les boucles.

<!--
### `if` Expressions
-->

### Les expressions `if`

<!--
An `if` expression allows you to branch your code depending on conditions. You
provide a condition and then state, “If this condition is met, run this block
of code. If the condition is not met, do not run this block of code.”
-->

Une expression `if` vous permet de diviser votre code en fonction de conditions.
Vous précisez une condition et vous choisissez ensuite : “Si cette condition est
remplie, alors exécuter ce bloc de code. Si la condition n'est pas remplie,
ne pas exécuter ce bloc de code.”

<!--
Create a new project called *branches* in your *projects* directory to explore
the `if` expression. In the *src/main.rs* file, input the following:
-->

Créez un nouveau projet appelé *branches* dans votre dossier *projects* pour
découvrir les expressions `if`. Dans le fichier *src/main.rs*, écrivez ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

<!--
All `if` expressions start with the keyword `if`, which is followed by a
condition. In this case, the condition checks whether or not the variable
`number` has a value less than 5. The block of code we want to execute if the
condition is true is placed immediately after the condition inside curly
brackets. Blocks of code associated with the conditions in `if` expressions are
sometimes called *arms*, just like the arms in `match` expressions that we
discussed in the [“Comparing the Guess to the Secret
Number”][comparing-the-guess-to-the-secret-number]<!-- ignore -- > section of
Chapter 2.
-->

Une expression `if` commence par le mot-clé `if`, suivi d'une condition.
Dans notre cas, la condition vérifie si oui ou non la variable `nombre` a une
valeur inférieure à 5. Le bloc de code que nous voulons exécuter si la condition
est vérifiée est placé immédiatement après la condition entre des accolades.
Les blocs de code associés à une condition dans une expression `if` sont parfois
appelés des *branches*, exactement comme les branches dans les expressions
`match` que nous avons vu dans la section [“Comparer le nombre saisi au
nombre secret”][comparing-the-guess-to-the-secret-number]<!-- ignore --> du
chapitre 2.

<!--
Optionally, we can also include an `else` expression, which we chose
to do here, to give the program an alternative block of code to execute should
the condition evaluate to false. If you don’t provide an `else` expression and
the condition is false, the program will just skip the `if` block and move on
to the next bit of code.
-->

Éventuellement, vous pouvez aussi ajouter une expression `else`, ce que nous
avons fait ici, pour préciser un bloc alternatif de code qui sera exécuté dans
le cas où la condition est fausse (elle n'est pas vérifiée). Si
vous ne renseignez pas d'expression `else` et que la condition n'est pas
vérifiée, le programme va simplement sauter le bloc de `if` et passer au
prochain morceau de code.

<!--
Try running this code; you should see the following output:
-->

Essayez d'exécuter ce code ; vous verrez ceci :

<!--
```console
{{#include ../listings-sources/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```
-->

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

<!--
Let’s try changing the value of `number` to a value that makes the condition
`false` to see what happens:
-->

Essayons de changer la valeur de `nombre` pour une valeur qui rend la condition
non vérifiée pour voir ce qui se passe :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

<!--
Run the program again, and look at the output:
-->

Exécutez à nouveau le programme, et regardez le résultat :

<!--
```console
{{#include ../listings-sources/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```
-->

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

<!--
It’s also worth noting that the condition in this code *must* be a `bool`. If
the condition isn’t a `bool`, we’ll get an error. For example, try running the
following code:
-->

Il est aussi intéressant de noter que la condition dans ce code *doit* être un
`bool`. Si la condition n'est pas un `bool`, nous aurons une erreur. Par
exemple, essayez d'exécuter le code suivant :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

<!--
The `if` condition evaluates to a value of `3` this time, and Rust throws an
error:
-->

La condition `if` vaut `3` cette fois, et Rust lève une erreur :

<!--
```console
{{#include ../listings-sources/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```
-->

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

<!--
The error indicates that Rust expected a `bool` but got an integer. Unlike
languages such as Ruby and JavaScript, Rust will not automatically try to
convert non-Boolean types to a Boolean. You must be explicit and always provide
`if` with a Boolean as its condition. If we want the `if` code block to run
only when a number is not equal to `0`, for example, we can change the `if`
expression to the following:
-->

Cette erreur explique que Rust attendait un `bool` mais a obtenu un entier
*(integer)*. Contrairement à des langages comme Ruby et JavaScript, Rust
ne va pas essayer de convertir automatiquement les types non booléens en
booléens. Vous devez être précis et toujours fournir un booléen à la condition
d'un `if`. Si nous voulons que le bloc de code du `if` soit exécuté quand le
nombre est différent de `0`, par exemple, nous pouvons changer l'expression `if`
par la suivante :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier: src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

<!--
Running this code will print `number was something other than zero`.
-->

Exécuter ce code va bien afficher `Le nombre valait autre chose que zéro`.

<!--
#### Handling Multiple Conditions with `else if`
-->

#### Gérer plusieurs conditions avec `else if`

<!--
You can have multiple conditions by combining `if` and `else` in an `else if`
expression. For example:
-->

Vous pouvez utiliser plusieurs conditions en combinant `if` et `else` dans une
expression `else if`. Par exemple :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

<!--
This program has four possible paths it can take. After running it, you should
see the following output:
-->

Ce programme peut choisir entre quatre chemins différents. Après l'avoir
exécuté, vous devriez voir le résultat suivant :

<!--
```console
{{#include ../listings-sources/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```
-->

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

<!--
When this program executes, it checks each `if` expression in turn and executes
the first body for which the condition holds true. Note that even though 6 is
divisible by 2, we don’t see the output `number is divisible by 2`, nor do we
see the `number is not divisible by 4, 3, or 2` text from the `else` block.
That’s because Rust only executes the block for the first true condition, and
once it finds one, it doesn’t even check the rest.
-->

Quand ce programme s'exécute, il vérifie chaque expression `if` à tour de rôle
et exécute le premier bloc dont la condition est vérifiée. Notez que même si 6
est divisible par 2, nous ne voyons pas le message `Le nombre est divisible par
2`, ni le message `Le nombre n'est pas divisible par 4, 3 ou 2` du bloc `else`.
C'est parce que Rust n'exécute que le bloc de la première condition vérifiée,
et dès lors qu'il en a trouvé une, il ne va pas chercher à vérifier les
suivantes.

<!--
Using too many `else if` expressions can clutter your code, so if you have more
than one, you might want to refactor your code. Chapter 6 describes a powerful
Rust branching construct called `match` for these cases.
-->

Utiliser trop d'expressions `else if` peut encombrer votre code, donc si vous
en avez plus d'une, vous devriez envisager de remanier votre code. Le chapitre 6
présente une construction puissante appelée `match` pour de tels cas.

<!--
#### Using `if` in a `let` Statement
-->

#### Utiliser `if` dans une instruction `let`

<!--
Because `if` is an expression, we can use it on the right side of a `let`
statement, as in Listing 3-2.
-->

Comme `if` est une expression, nous pouvons l'utiliser à droite d'une
instruction `let`, comme dans l'encart 3-2.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<!--
<span class="caption">Listing 3-2: Assigning the result of an `if` expression
to a variable</span>
-->

<span class="caption">Encart 3-2 : assigner le résultat d'une expression `if` à
une variable</span>

<!--
The `number` variable will be bound to a value based on the outcome of the `if`
expression. Run this code to see what happens:
-->

La variable `nombre` va avoir la valeur du résultat de l'expression `if`.
Exécutez ce code pour découvrir ce qui va se passer :

<!--
```console
{{#include ../listings-sources/ch03-common-programming-concepts/listing-03-02/output.txt}}
```
-->

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

<!--
Remember that blocks of code evaluate to the last expression in them, and
numbers by themselves are also expressions. In this case, the value of the
whole `if` expression depends on which block of code executes. This means the
values that have the potential to be results from each arm of the `if` must be
the same type; in Listing 3-2, the results of both the `if` arm and the `else`
arm were `i32` integers. If the types are mismatched, as in the following
example, we’ll get an error:
-->

Souvenez-vous que les blocs de code s'exécutent jusqu'à la dernière expression
qu'ils contiennent, et que les nombres tout seuls sont aussi des expressions.
Dans notre cas, la valeur de toute l'expression `if` dépend de quel bloc de code
elle va exécuter. Cela veut dire que chaque valeur qui peut être le résultat de
chaque branche du `if` doivent être du même type ; dans l'encart 3-2, les
résultats des branches `if` et `else` sont tous deux des entiers `i32`. Si
les types ne sont pas identiques, comme dans l'exemple suivant, nous allons
obtenir une erreur :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

<!--
When we try to compile this code, we’ll get an error. The `if` and `else` arms
have value types that are incompatible, and Rust indicates exactly where to
find the problem in the program:
-->

Lorsque nous essayons de compiler ce code, nous obtenons une erreur. Les
branches `if` et `else` ont des types de valeurs qui ne sont pas compatibles, et
Rust indique exactement où trouver le problème dans le programme :

<!--
```console
{{#include ../listings-sources/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```
-->

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

<!--
The expression in the `if` block evaluates to an integer, and the expression in
the `else` block evaluates to a string. This won’t work because variables must
have a single type. Rust needs to know at compile time what type the `number`
variable is, definitively, so it can verify at compile time that its type is
valid everywhere we use `number`. Rust wouldn’t be able to do that if the type
of `number` was only determined at runtime; the compiler would be more complex
and would make fewer guarantees about the code if it had to keep track of
multiple hypothetical types for any variable.
-->

L'expression dans le bloc `if` donne un entier, et l'expression dans le bloc
`else` donne une chaîne de caractères. Ceci ne fonctionne pas car les variables
doivent avoir un seul type. Rust a besoin de savoir de quel type est la variable
`nombre` au moment de la compilation, assurément, afin de vérifier au moment
de la compilation que son type est valable n'importe où nous utilisons `nombre`.
Rust ne serait pas capable de faire cela si le type de `nombre` était déterminé
uniquement à l'exécution ; car le compilateur deviendrait plus complexe et nous
donnerait moins de garanties sur le code s'il devait prendre en compte tous les
types hypothétiques pour une variable.

<!--
### Repetition with Loops
-->

### Les répétitions avec les boucles

<!--
It’s often useful to execute a block of code more than once. For this task,
Rust provides several *loops*. A loop runs through the code inside the loop
body to the end and then starts immediately back at the beginning. To
experiment with loops, let’s make a new project called *loops*.
-->

Il est parfois utile d'exécuter un bloc de code plus d'une seule fois. Dans ce
but, Rust propose plusieurs types de *boucles*. Une boucle parcourt le code à
l'intérieur du corps de la boucle jusqu'à la fin et recommence immédiatement du
début. Pour tester les boucles, créons un nouveau projet appelé *loops*.

<!--
Rust has three kinds of loops: `loop`, `while`, and `for`. Let’s try each one.
-->

Rust a trois types de boucles : `loop`, `while`, et `for`. Essayons chacune
d'elles.

<!--
#### Repeating Code with `loop`
-->

#### Répéter du code avec `loop`

<!--
The `loop` keyword tells Rust to execute a block of code over and over again
forever or until you explicitly tell it to stop.
-->

Le mot-clé `loop` demande à Rust d'exécuter un bloc de code encore et encore
jusqu'à l'infini ou jusqu'à ce que vous lui demandiez explicitement de
s'arrêter.

<!--
As an example, change the *src/main.rs* file in your *loops* directory to look
like this:
-->

Par exemple, changez le fichier *src/main.rs* dans votre dossier *loops* comme
ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

<!--
When we run this program, we’ll see `again!` printed over and over continuously
until we stop the program manually. Most terminals support a keyboard shortcut,
<span class="keystroke">ctrl-c</span>, to interrupt a program that is stuck in
a continual loop. Give it a try:
-->

Quand nous exécutons ce programme, nous voyons `À nouveau !` s'afficher encore
et encore en continu jusqu'à ce qu'on arrête le programme manuellement. La
plupart des terminaux utilisent un raccourci clavier, <span class="keystroke">
ctrl-c</span>, pour arrêter un programme qui est bloqué dans une boucle infinie.
Essayons cela :

<!--
<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-- >
-->

<!--
```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
À nouveau !
À nouveau !
À nouveau !
À nouveau !
^CÀ nouveau !
```

<!--
The symbol `^C` represents where you pressed <span class="keystroke">ctrl-c
</span>. You may or may not see the word `again!` printed after the `^C`,
depending on where the code was in the loop when it received the interrupt
signal.
-->

Le symbole `^C` représente le moment où vous avez appuyé sur
<span class="keystroke">ctrl-c</span>. Vous devriez voir ou non le texte
`À nouveau !` après le `^C`, en fonction de là où la boucle en était dans votre
code quand elle a reçu le signal d'arrêt.

<!--
Fortunately, Rust provides another, more reliable way to break out of a loop.
You can place the `break` keyword within the loop to tell the program when to
stop executing the loop. Recall that we did this in the guessing game in the
[“Quitting After a Correct Guess”][quitting-after-a-correct-guess]<!-- ignore
-- > section of Chapter 2 to exit the program when the user won the game by
guessing the correct number.
-->

Heureusement, Rust fournit un autre moyen, plus fiable, de sortir d'une boucle.
Vous pouvez ajouter le mot-clé `break` à l'intérieur de la boucle pour demander
au programme d'arrêter la boucle. Souvenez-vous que nous avions fait ceci dans
le jeu de devinettes, dans la section [“Arrêter le programme après avoir
gagné”][quitting-after-a-correct-guess]<!-- ignore --> du chapitre 2 afin de
quitter le programme quand l'utilisateur gagne le jeu en devinant le bon nombre.

<!--
#### Returning Values from Loops
-->

#### Retourner des valeurs d'une boucle

<!--
One of the uses of a `loop` is to retry an operation you know might fail, such
as checking whether a thread has completed its job. However, you might need to
pass the result of that operation to the rest of your code. To do this, you can
add the value you want returned after the `break` expression you use to stop
the loop; that value will be returned out of the loop so you can use it, as
shown here:
-->

L'une des utilisations d'une boucle `loop` est de réessayer une opération qui
peut échouer, comme vérifier si une tâche a terminé son travail. Cependant, vous
aurez peut-être besoin de passer le résultat de l'opération au reste de votre
code. Pour ce faire, vous pouvez ajouter la valeur que vous voulez retourner
après l'expression `break` que vous utilisez pour stopper la boucle ; cette
valeur sera retournée de la boucle pour que vous puissiez l'utiliser, comme
ci-dessous :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

<!--
Before the loop, we declare a variable named `counter` and initialize it to
`0`. Then we declare a variable named `result` to hold the value returned from
the loop. On every iteration of the loop, we add `1` to the `counter` variable,
and then check whether the counter is equal to `10`. When it is, we use the
`break` keyword with the value `counter * 2`. After the loop, we use a
semicolon to end the statement that assigns the value to `result`. Finally, we
print the value in `result`, which in this case is 20.
-->

Avant la boucle, nous déclarons une variable avec le nom `compteur` et nous
l'initialisons à `0`. Ensuite, nous déclarons une variable `resultat` pour
stocker la valeur retournée de la boucle. À chaque itération de la boucle, nous
ajoutons `1` à la variable `compteur`, et ensuite nous vérifions si le compteur
est égal à `10`. Lorsque c'est le cas, nous utilisons le mot-clé `break` avec la
valeur `compteur * 2`. Après la boucle, nous utilisons un point-virgule pour
terminer l'instruction qui assigne la valeur à `resultat`. Enfin, nous
affichons la valeur de `resultat`, qui est 20 dans ce cas-ci.

<!--
#### Conditional Loops with `while`
-->

#### Les boucles conditionnelles avec `while`

<!--
It’s often useful for a program to evaluate a condition within a loop. While
the condition is true, the loop runs. When the condition ceases to be true, the
program calls `break`, stopping the loop. This loop type could be implemented
using a combination of `loop`, `if`, `else`, and `break`; you could try that
now in a program, if you’d like.
-->

Il est souvent utile pour un programme d'évaluer une condition dans une boucle.
Tant que la condition est vraie, la boucle tourne. Quand la condition arrête
d'être vraie, le programme appelle `break`, ce qui arrête la boucle. Ce type de
boucle peut être implémenté en combinant `loop`, `if`, `else` et `break` ; vous
pouvez essayer de le faire, si vous voulez.

<!--
However, this pattern is so common that Rust has a built-in language construct
for it, called a `while` loop. Listing 3-3 uses `while`: the program loops
three times, counting down each time, and then, after the loop, it prints
another message and exits.
-->

Cependant, cette utilisation est si fréquente que Rust a une construction pour
cela, intégrée dans le langage, qui s'appelle une boucle `while`. L'encart 3-3
utilise `while` : le programme va boucler trois fois, en décrémentant à chaque
fois, et ensuite, après la boucle, il va afficher un message et se fermer.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<!--
<span class="caption">Listing 3-3: Using a `while` loop to run code while a
condition holds true</span>
-->

<span class="caption">Encart 3-3: utiliser une boucle `while` pour exécuter du
code tant qu'une condition est vraie</span>

<!--
This construct eliminates a lot of nesting that would be necessary if you used
`loop`, `if`, `else`, and `break`, and it’s clearer. While a condition holds
true, the code runs; otherwise, it exits the loop.
-->

Cette construction élimine beaucoup d'imbrications qui seraient nécessaires si
vous utilisiez `loop`, `if`, `else` et `break`, et c'est aussi plus clair. Tant
que la condition est vraie, le code est exécuté ; sinon, il quitte la boucle.

<!--
#### Looping Through a Collection with `for`
-->

#### Boucler dans une collection avec `for`

<!--
You could use the `while` construct to loop over the elements of a collection,
such as an array. For example, let’s look at Listing 3-4.
-->

Vous pouvez utiliser la construction `while` pour itérer sur les
éléments d'une collection, comme les tableaux. Par exemple, analysons l'encart
3-4.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<!--
<span class="caption">Listing 3-4: Looping through each element of a collection
using a `while` loop</span>
-->

<span class="caption">Encart 3-4 : itération sur les éléments d'une collection
en utilisant une boucle `while`</span>

<!--
Here, the code counts up through the elements in the array. It starts at index
`0`, and then loops until it reaches the final index in the array (that is,
when `index < 5` is no longer true). Running this code will print every element
in the array:
-->

Ici, le code parcourt le tableau élément par élément.
Il commence à l'indice `0`, et ensuite boucle jusqu'à ce qu'il atteigne l'indice
final du tableau (ce qui correspond au moment où la condition `index < 5` n'est
plus vraie). Exécuter ce code va afficher chaque élément du tableau :

<!--
```text
{{#include ../listings-sources/ch03-common-programming-concepts/listing-03-04/output.txt}}
```
-->

```text
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

<!--
All five array values appear in the terminal, as expected. Even though `index`
will reach a value of `5` at some point, the loop stops executing before trying
to fetch a sixth value from the array.
-->

Les cinq valeurs du tableau s'affichent toutes dans le terminal, comme attendu.
Même si `indice` va atteindre la valeur `5` à un moment, la boucle arrêtera de
s'exécuter avant d'essayer de récupérer une sixième valeur du tableau.

<!--
But this approach is error prone; we could cause the program to panic if the
index length is incorrect. It’s also slow, because the compiler adds runtime
code to perform the conditional check on every element on every iteration
through the loop.
-->

Mais cette approche pousse à l'erreur ; nous pourrions faire paniquer le
programme si l'indice est trop grand. De plus, c'est lent, car le compilateur
ajoute du code à l'exécution pour effectuer des vérifications sur chaque élément
à chaque itération de la boucle.

<!--
As a more concise alternative, you can use a `for` loop and execute some code
for each item in a collection. A `for` loop looks like the code in Listing 3-5.
-->

Pour une alternative plus concise, vous pouvez utiliser une boucle `for` et
exécuter du code pour chaque élément dans une collection. Une boucle `for`
s'utilise comme dans le code de l'encart 3-5.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<!--
<span class="caption">Listing 3-5: Looping through each element of a collection
using a `for` loop</span>
-->

<span class="caption">Encart 3-5 : itérer sur chaque élément d'une collection
en utilisant une boucle `for`</span>

<!--
When we run this code, we’ll see the same output as in Listing 3-4. More
importantly, we’ve now increased the safety of the code and eliminated the
chance of bugs that might result from going beyond the end of the array or not
going far enough and missing some items.
-->

Lorsque nous exécutons ce code, nous obtenons les mêmes messages que dans
l'encart 3-4. Mais ce qui est plus important, c'est que nous avons amélioré la
sécurité de notre code et éliminé le risque de bogues qui pourraient survenir
si on dépassait la fin du tableau, ou si on n'allait pas jusqu'au bout
et qu'on ratait quelques éléments.

<!--
For example, in the code in Listing 3-4, if you changed the definition of the
`a` array to have four elements but forgot to update the condition to `while
index < 4`, the code would panic. Using the `for` loop, you wouldn’t need to
remember to change any other code if you changed the number of values in the
array.
-->

Par exemple, dans le code de l'encart 3-4, si vous changez la définition du
tableau `a` pour qu'il stocke quatre éléments mais que vous oubliez de mettre à
jour la condition tel que `while indice < 4`, le code va paniquer. En utilisant
la boucle `for`, vous n'aurez pas à vous rappeler de changer le code si vous
changez le nombre de valeurs dans le tableau.

<!--
The safety and conciseness of `for` loops make them the most commonly used loop
construct in Rust. Even in situations in which you want to run some code a
certain number of times, as in the countdown example that used a `while` loop
in Listing 3-3, most Rustaceans would use a `for` loop. The way to do that
would be to use a `Range`, which is a type provided by the standard library
that generates all numbers in sequence starting from one number and ending
before another number.
-->

La sécurité et la concision de la boucle `for` en font la construction de boucle
la plus utilisée avec Rust. Même dans des situations dans lesquelles vous
voudriez exécuter du code plusieurs fois, comme l'exemple du décompte qui
utilisait une boucle `while` dans l'encart 3-3, la plupart des Rustacés
utiliseraient une boucle `for`. Il faut pour cela utiliser un intervalle
`Range`, qui est un type fourni par la bibliothèque standard qui génère dans
l'ordre tous les nombres compris entre un certain nombre et un autre nombre.

<!--
Here’s what the countdown would look like using a `for` loop and another method
we’ve not yet talked about, `rev`, to reverse the range:
-->

Voici ce que le décompte aurait donné en utilisant une boucle `for` et une autre
méthode que nous n'avons pas encore vue, `rev`, qui inverse l'intervalle :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

<!--
This code is a bit nicer, isn’t it?
-->

Ce code est un peu plus sympa, non ?

<!--
## Summary
-->

## Résumé

<!--
You made it! That was a sizable chapter: you learned about variables, scalar
and compound data types, functions, comments, `if` expressions, and loops! If
you want to practice with the concepts discussed in this chapter, try building
programs to do the following:
-->

Vous y êtes arrivé ! C'était un chapitre important : vous avez appris les
variables, les types scalaires et composés, les fonctions, les commentaires, les
expressions `if`, et les boucles ! Si vous voulez pratiquer un peu les concepts
abordés dans ce chapitre, voici quelques programmes que vous pouvez essayer de
créer :

<!--
* Convert temperatures between Fahrenheit and Celsius.
* Generate the nth Fibonacci number.
* Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
  taking advantage of the repetition in the song.
-->

* Convertir des températures entre les degrés Fahrenheit et Celsius.
* Générer le *n*-ième nombre de Fibonacci.
* Afficher les paroles de la chanson de Noël *The Twelve Days of Christmas* en
  profitant de l'aspect répétitif de la chanson.

<!--
When you’re ready to move on, we’ll talk about a concept in Rust that *doesn’t*
commonly exist in other programming languages: ownership.
-->

Quand vous serez prêt à aller plus loin, nous aborderons une notion de Rust
qui n'existe *pas* dans les autres langages de programmation : la possession
*(ownership)*.

<!--
[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
-->

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparer-le-nombre-saisi-au-nombre-secret
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.html#arrêter-le-programme-après-avoir-gagné
