<!--
<a id="the-match-control-flow-operator"></a>
## The `match` Control Flow Construct
-->

<a id="la-structure-de-contrôle-match"></a>

## La structure de contrôle de flux `match`

<!--
Rust has an extremely powerful control flow construct called `match` that allows
you to compare a value against a series of patterns and then execute code based
on which pattern matches. Patterns can be made up of literal values, variable
names, wildcards, and many other things; Chapter 18 covers all the different
kinds of patterns and what they do. The power of `match` comes from the
expressiveness of the patterns and the fact that the compiler confirms that all
possible cases are handled.
-->

Rust a une structure de contrôle de flux très puissante appelée `match` qui vous
permet de comparer une valeur avec une série de motifs et d'exécuter du code en
fonction du motif qui correspond. Les motifs peuvent être constitués de valeurs
littérales, de noms de variables, de jokers, parmi tant d'autres ; le
chapitre 18 va couvrir tous les différents types de motifs et ce qu'ils font. Ce
qui fait la puissance de `match` est l'expressivité des motifs et le fait que le
compilateur vérifie que tous les cas possibles sont bien gérés.

<!--
Think of a `match` expression as being like a coin-sorting machine: coins slide
down a track with variously sized holes along it, and each coin falls through
the first hole it encounters that it fits into. In the same way, values go
through each pattern in a `match`, and at the first pattern the value “fits,”
the value falls into the associated code block to be used during execution.
Speaking of coins, let’s use them as an example using `match`! We can write a
function that takes an unknown United States coin and, in a similar way as the
counting machine, determines which coin it is and return its value in cents, as
shown here in Listing 6-3.
-->

Considérez l'expression `match` comme une machine à trier les pièces de
monnaie : les pièces descendent le long d'une piste avec des trous de tailles
différentes, et chaque pièce tombe dans le premier trou à sa taille qu'elle
rencontre. De manière similaire, les valeurs parcourent tous les motifs dans un
`match`, et au premier motif auquel la valeur “correspond”, la valeur va
descendre dans le bloc de code correspondant afin d'être utilisée pendant son
exécution. En parlant des pièces, utilisons-les avec un exemple qui utilise
`match` ! Nous pouvons écrire une fonction qui prend en paramètre une pièce
inconnue des États-Unis d'Amérique et qui peut, de la même manière qu'une
machine à trier, déterminer quelle pièce c'est et retourner sa valeur en
centimes, comme ci-dessous dans l'encart 6-3.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-3: An enum and a `match` expression that has
the variants of the enum as its patterns</span>
-->

<span class="caption">Encart 6-3 : Une énumération et une expression `match` qui
trie les variantes de l'énumération dans ses motifs</span>

<!--
Let’s break down the `match` in the `value_in_cents` function. First, we list
the `match` keyword followed by an expression, which in this case is the value
`coin`. This seems very similar to an expression used with `if`, but there’s a
big difference: with `if`, the expression needs to return a Boolean value, but
here, it can return any type. The type of `coin` in this example is the `Coin`
enum that we defined on the first line.
-->

Décomposons le `match` dans la fonction `valeur_en_centimes`. En premier lieu,
nous utilisons le mot-clé `match` suivi par une expression, qui dans notre cas
est la valeur de `piece`. Cela ressemble beaucoup à une expression utilisée avec
`if`, mais il y a une grosse différence : avec `if`, l'expression doit retourner
une valeur booléenne, mais ici, elle retourne n'importe quel type. Dans cet
exemple, `piece` est de type `PieceUs`, qui est l'énumération que nous avons
définie à la première ligne.

<!--
Next are the `match` arms. An arm has two parts: a pattern and some code. The
first arm here has a pattern that is the value `Coin::Penny` and then the `=>`
operator that separates the pattern and the code to run. The code in this case
is just the value `1`. Each arm is separated from the next with a comma.
-->

Ensuite, nous avons les branches du `match`. Une branche a deux parties : un
motif et du code. La première branche a ici pour motif la valeur
`PieceUs::Penny` et ensuite l'opérateur `=>` qui sépare le motif et le code à
exécuter. Le code dans ce cas est uniquement la valeur `1`. Chaque branche est
séparée de la suivante par une virgule.

<!--
When the `match` expression executes, it compares the resulting value against
the pattern of each arm, in order. If a pattern matches the value, the code
associated with that pattern is executed. If that pattern doesn’t match the
value, execution continues to the next arm, much as in a coin-sorting machine.
We can have as many arms as we need: in Listing 6-3, our `match` has four arms.
-->

Lorsqu'une expression `match` est exécutée, elle compare la valeur de `piece`
avec le motif de chaque branche, dans l'ordre. Si un motif correspond à la
valeur, le code correspondant à ce motif est alors exécuté. Si ce motif ne
correspond pas à la valeur, l'exécution passe à la prochaine branche, un peu
comme dans une machine de tri de pièces. Nous pouvons avoir autant de branches
que nécessaire : dans l'encart 6-3, notre `match` a quatre branches.

<!--
The code associated with each arm is an expression, and the resulting value of
the expression in the matching arm is the value that gets returned for the
entire `match` expression.
-->

Le code correspondant à chaque branche est une expression, et la valeur qui
résulte de l'expression dans la branche correspondante est la valeur qui sera
retournée par l'expression `match`.

<!--
We don’t typically use curly brackets if the match arm code is short, as it is
in Listing 6-3 where each arm just returns a value. If you want to run multiple
lines of code in a match arm, you must use curly brackets. For example, the
following code prints “Lucky penny!” every time the method is called with a
`Coin::Penny`, but still returns the last value of the block, `1`:
-->

Habituellement, nous n'utilisons pas les accolades si le code de la branche
correspondante est court, comme c'est le cas dans l'encart 6-3 où chaque
branche retourne simplement une valeur. Si vous voulez exécuter plusieurs
lignes de code dans une branche d'un `match`, vous devez utiliser les
accolades. Par exemple, le code suivant va afficher “Un centime
porte-bonheur !” à chaque fois que la méthode est appelée avec une valeur
`PieceUs::Penny`, mais va continuer à retourner la dernière valeur du
bloc, `1` :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

<!--
### Patterns that Bind to Values
-->

### Des motifs reliés à des valeurs

<!--
Another useful feature of match arms is that they can bind to the parts of the
values that match the pattern. This is how we can extract values out of enum
variants.
-->

Une autre fonctionnalité intéressante des branches de `match` est qu'elles
peuvent se lier aux valeurs qui correspondent au motif. C'est ainsi que nous
pouvons extraire des valeurs d'une variante d'énumération.

<!--
As an example, let’s change one of our enum variants to hold data inside it.
From 1999 through 2008, the United States minted quarters with different
designs for each of the 50 states on one side. No other coins got state
designs, so only quarters have this extra value. We can add this information to
our `enum` by changing the `Quarter` variant to include a `UsState` value stored
inside it, which we’ve done here in Listing 6-4.
-->

En guise d'exemple, changeons une de nos variantes d'énumération pour stocker
une donnée à l'intérieur. Entre 1999 et 2008, les États-Unis d'Amérique ont
frappé un côté des *quarters* (pièces de 25 centimes) avec des dessins
différents pour chacun des 50 États. Les autres pièces n'ont pas eu de dessins
d'États, donc seul le *quarter* a cette valeur en plus. Nous pouvons ajouter
cette information à notre `enum` en changeant la variante `Quarter` pour y
ajouter une valeur `EtatUs` qui y sera stockée à l'intérieur, comme nous
l'avons fait dans l'encart 6-4.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-4: A `Coin` enum in which the `Quarter` variant
also holds a `UsState` value</span>
-->

<span class="caption">Encart 6-4 : Une énumération `PieceUs` dans laquelle la
variante `Quarter` stocke en plus une valeur de type `EtatUs`</span>

<!--
Let’s imagine that a friend is trying to collect all 50 state quarters. While
we sort our loose change by coin type, we’ll also call out the name of the
state associated with each quarter so if it’s one our friend doesn’t have, they
can add it to their collection.
-->

Imaginons qu'un de vos amis essaye de collectionner tous les *quarters* des 50
États. Pendant que nous trions notre monnaie en vrac par type de pièce, nous
mentionnerons aussi le nom de l'État correspondant à chaque *quarter* de sorte
que si notre ami ne l'a pas, il puisse l'ajouter à sa collection.

<!--
In the match expression for this code, we add a variable called `state` to the
pattern that matches values of the variant `Coin::Quarter`. When a
`Coin::Quarter` matches, the `state` variable will bind to the value of that
quarter’s state. Then we can use `state` in the code for that arm, like so:
-->

Dans l'expression `match` de ce code, nous avons ajouté une variable `etat` au
motif qui correspond à la variante `PieceUs::Quarter`. Quand on aura une
correspondance `PieceUs::Quarter`, la variable `etat` sera liée à la valeur de
l'État de cette pièce. Ensuite, nous pourrons utiliser `etat` dans le code de
cette branche, comme ceci :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

<!--
If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin`
would be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each
of the match arms, none of them match until we reach `Coin::Quarter(state)`. At
that point, the binding for `state` will be the value `UsState::Alaska`. We can
then use that binding in the `println!` expression, thus getting the inner
state value out of the `Coin` enum variant for `Quarter`.
-->

Si nous appelons `valeur_en_centimes(PieceUs::Quarter(EtatUs::Alaska))`, `piece`
vaudra `PieceUs::Quarter(EtatUs::Alaska)`. Quand nous comparons cette valeur
avec toutes les branches du `match`, aucune d'entre elles ne correspondra
jusqu'à ce qu'on arrive à `PieceUs::Quarter(etat)`. À partir de ce moment, la
variable `etat` aura la valeur `EtatUs::Alaska`. Nous pouvons alors utiliser
cette variable dans l'expression `println!`, ce qui nous permet d'afficher la
valeur de l'État à l'intérieur de la variante `Quarter` de l'énumération
`PieceUs`.

<!--
### Matching with `Option<T>`
-->

### Utiliser `match` avec `Option<T>`

<!--
In the previous section, we wanted to get the inner `T` value out of the `Some`
case when using `Option<T>`; we can also handle `Option<T>` using `match` as we
did with the `Coin` enum! Instead of comparing coins, we’ll compare the
variants of `Option<T>`, but the way that the `match` expression works remains
the same.
-->

Dans la section précédente, nous voulions obtenir la valeur interne `T` dans le
cas de `Some` lorsqu'on utilisait `Option<T>` ; nous pouvons aussi gérer les
`Option<T>` en utilisant `match` comme nous l'avons fait avec l'énumération
`PieceUs` ! Au lieu de comparer des pièces, nous allons comparer les variantes
de `Option<T>`, mais la façon d'utiliser l'expression `match` reste la même.

<!--
Let’s say we want to write a function that takes an `Option<i32>` and, if
there’s a value inside, adds 1 to that value. If there isn’t a value inside,
the function should return the `None` value and not attempt to perform any
operations.
-->

Disons que nous voulons écrire une fonction qui prend une `Option<i32>` et qui,
s'il y a une valeur à l'intérieur, ajoute 1 à cette valeur. S'il n'y a pas de
valeur à l'intérieur, la fonction retournera la valeur `None` et ne va rien
faire de plus.

<!--
This function is very easy to write, thanks to `match`, and will look like
Listing 6-5.
-->

Cette fonction est très facile à écrire, grâce à `match`, et ressemblera à
l'encart 6-5.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-5: A function that uses a `match` expression on
an `Option<i32>`</span>
-->

<span class="caption">Encart 6-5 : Une fonction qui utilise une expression
`match` sur une `Option<i32>`</span>

<!--
Let’s examine the first execution of `plus_one` in more detail. When we call
`plus_one(five)`, the variable `x` in the body of `plus_one` will have the
value `Some(5)`. We then compare that against each match arm.
-->

Examinons la première exécution de `plus_un` en détail. Lorsque nous appelons
`plus_un(cinq)`, la variable `x` dans le corps de `plus_un` aura la valeur
`Some(5)`. Ensuite, nous comparons cela à chaque branche du `match`.

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

<!--
The `Some(5)` value doesn’t match the pattern `None`, so we continue to the
next arm.
-->

La valeur `Some(5)` ne correspond pas au motif `None`, donc nous continuons à la
branche suivante.

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

<!--
Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. The
`i` binds to the value contained in `Some`, so `i` takes the value `5`. The
code in the match arm is then executed, so we add 1 to the value of `i` and
create a new `Some` value with our total `6` inside.
-->

Est-ce que `Some(5)` correspond au motif `Some(i)` ? Bien sûr ! Nous avons la
même variante. Le `i` va prendre la valeur contenue dans le `Some`, donc `i`
prend la valeur `5`. Le code dans la branche du `match` est exécuté, donc nous
ajoutons 1 à la valeur de `i` et nous créons une nouvelle valeur `Some` avec
notre résultat `6` à l'intérieur.

<!--
Now let’s consider the second call of `plus_one` in Listing 6-5, where `x` is
`None`. We enter the `match` and compare to the first arm.
-->

Maintenant, regardons le second appel à `plus_un` dans l'encart 6-5, où `x` vaut
`None`. Nous entrons dans le `match` et nous le comparons à la première branche.

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

<!--
It matches! There’s no value to add to, so the program stops and returns the
`None` value on the right side of `=>`. Because the first arm matched, no other
arms are compared.
-->

Cela correspond ! Il n'y a pas de valeur à additionner, donc le programme
s'arrête et retourne la valeur `None` qui est dans le côté droit du `=>`. Comme
la première branche correspond, les autres branches ne sont pas comparées.

<!--
Combining `match` and enums is useful in many situations. You’ll see this
pattern a lot in Rust code: `match` against an enum, bind a variable to the
data inside, and then execute code based on it. It’s a bit tricky at first, but
once you get used to it, you’ll wish you had it in all languages. It’s
consistently a user favorite.
-->

La combinaison de `match` et des énumérations est utile dans de nombreuses
situations. Vous allez revoir de nombreuses fois ce schéma dans du code Rust :
utiliser `match` sur une énumération, récupérer la valeur qu'elle renferme, et
exécuter du code en fonction de sa valeur. C'est un peu délicat au début, mais
une fois que vous vous y êtes habitué, vous regretterez de ne pas l'avoir dans
les autres langages. Cela devient toujours l'outil préféré de ses utilisateurs.

<!--
### Matches Are Exhaustive
-->

### Les `match` sont toujours exhaustifs

<!--
There’s one other aspect of `match` we need to discuss. Consider this version
of our `plus_one` function that has a bug and won’t compile:
-->

Il y a un autre point de `match` que nous devons aborder. Examinez cette version
de notre fonction `plus_un` qui a un bogue et ne va pas se compiler :

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

<!--
We didn’t handle the `None` case, so this code will cause a bug. Luckily, it’s
a bug Rust knows how to catch. If we try to compile this code, we’ll get this
error:
-->

Nous n'avons pas géré le cas du `None`, donc ce code va générer un bogue.
Heureusement, c'est un bogue que Rust sait gérer. Si nous essayons de compiler
ce code, nous allons obtenir cette erreur :

<!--
```console
{{#include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```
-->

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

<!--
Rust knows that we didn’t cover every possible case and even knows which
pattern we forgot! Matches in Rust are *exhaustive*: we must exhaust every last
possibility in order for the code to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null, thus making the billion-dollar mistake discussed earlier impossible.
-->

Rust sait que nous n'avons pas couvert toutes les possibilités et sait même quel
motif nous avons oublié ! Les `match` de Rust sont *exhaustifs* : nous devons
traiter toutes les possibilités afin que le code soit valide. C'est notamment le
cas avec `Option<T>` : quand Rust nous empêche d'oublier de gérer explicitement
le cas de `None`, il nous protège d'une situation où nous supposons que nous
avons une valeur alors que nous pourrions avoir null, ce qui rend impossible
l'erreur à un milliard de dollars que nous avons vue précédemment.

<!--
### Catch-all Patterns and the `_` Placeholder
-->

### Les motifs génériques et le motif `_`

<!--
Using enums, we can also take special actions for a few particular values, but
for all other values take one default action. Imagine we’re implementing a game
where, if you roll a 3 on a dice roll, your player doesn’t move, but instead
gets a new fancy hat. If you roll a 7, your player loses a fancy hat. For all
other values, your player moves that number of spaces on the game board. Here’s
a `match` that implements that logic, with the result of the dice roll
hardcoded rather than a random value, and all other logic represented by
functions without bodies because actually implementing them is out of scope for
this example:
-->

En utilisant les énumérations, nous pouvons aussi appliquer des actions
spéciales pour certaines valeurs précises, mais une action par défaut pour
toutes les autres valeurs. Imaginons que nous implémentons un jeu dans lequel,
si vous obtenez une valeur de 3 sur un lancé de dé, votre joueur ne se déplace
pas, mais à la place il obtient un nouveau chapeau fantaisie. Si vous obtenez
un 7, votre joueur perd son chapeau fantaisie. Pour toutes les autres valeurs,
votre joueur se déplace de ce nombre de cases sur le plateau du jeu. Voici un
`match` qui implémente cette logique, avec le résultat du lancé de dé codé en
dur plutôt qu'issu d'une génération aléatoire, et toute la logique des autres
fonctions sont des corps vides car leur implémentation n'est pas le sujet de
cet exemple :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

<!--
For the first two arms, the patterns are the literal values 3 and 7. For the
last arm that covers every other possible value, the pattern is the variable
we’ve chosen to name `other`. The code that runs for the `other` arm uses the
variable by passing it to the `move_player` function.
-->

Dans les deux premières branches, les motifs sont les valeurs litérales 3 et 7.
La dernière branche couvre toutes les autres valeurs possibles, le motif est la
variable `autre`. Le code qui s'exécute pour la branche `autre` utilise la
variable en la passant dans la fonction `deplacer_joueur`.

<!--
This code compiles, even though we haven’t listed all the possible values a
`u8` can have, because the last pattern will match all values not specifically
listed. This catch-all pattern meets the requirement that `match` must be
exhaustive. Note that we have to put the catch-all arm last because the
patterns are evaluated in order. Rust will warn us if we add arms after a
catch-all because those later arms would never match!
-->

Ce code se compile, même si nous n'avons pas listé toutes les valeurs possibles
qu'un `u8` puisse avoir, car le dernier motif va correspondre à toutes les
valeurs qui ne sont pas spécifiquement listés. Ce motif générique répond à la
condition qu'un `match` doive être exhaustif. Notez que nous devons placer la
branche avec le motif générique en tout dernier, car les motifs sont évalués
dans l'ordre. Rust va nous prévenir si nous ajoutons des branches après un motif
générique car toutes ces autres branches ne seront jamais vérifiées !

<!--
Rust also has a pattern we can use when we don’t want to use the value in the
catch-all pattern: `_`, which is a special pattern that matches any value and
does not bind to that value. This tells Rust we aren’t going to use the value,
so Rust won’t warn us about an unused variable.
-->

Rust a aussi un motif que nous pouvons utiliser lorsque nous n'avons pas besoin
d'utiliser la valeur dans le motif générique : `_`, qui est un motif spécial
qui vérifie n'importe quelle valeur et ne récupère pas cette valeur. Ceci
indique à Rust que nous n'allons pas utiliser la valeur, donc Rust ne va pas
nous prévenir qu'il y a une variable non utilisée.

<!--
Let’s change the rules of the game to be that if you roll anything other than
a 3 or a 7, you must roll again. We don’t need to use the value in that case,
so we can change our code to use `_` instead of the variable named `other`:
-->

Changeons les règles du jeu pour que si nous obtenions autre chose qu'un 3 ou
un 7, nous jetions à nouveau le dé. Nous n'avons pas besoin d'utiliser la valeur
dans ce cas, donc nous pouvons changer notre code pour utiliser `_` au lieu de
la variable `autre` :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

<!--
This example also meets the exhaustiveness requirement because we’re explicitly
ignoring all other values in the last arm; we haven’t forgotten anything.
-->

Cet exemple répond bien aux critères d'exhaustivité car nous ignorons
explicitement toutes les autres valeurs dans la dernière branche ; nous n'avons
rien oublié.

<!--
If we change the rules of the game one more time, so that nothing else happens
on your turn if you roll anything other than a 3 or a 7, we can express that
by using the unit value (the empty tuple type we mentioned in [“The Tuple
Type”][tuples]<!-- ignore -- > section) as the code that goes with the `_` arm:
-->

Si nous changeons à nouveau les règles du jeu, afin que rien se passe si vous
obtenez autre chose qu'un 3 ou un 7, nous pouvons exprimer cela en utilisant la
valeur unité (le type tuple vide que nous avons cité dans [une section
précédente][tuples]<!-- ignore -->) dans le code de la branche `_` :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

<!--
Here, we’re telling Rust explicitly that we aren’t going to use any other value
that doesn’t match a pattern in an earlier arm, and we don’t want to run any
code in this case.
-->

Ici, nous indiquons explicitement à Rust que nous n'allons pas utiliser d'autres
valeurs qui ne correspondent pas à un motif des branches antérieures, et nous ne
voulons lancer aucun code dans ce cas.

<!--
There’s more about patterns and matching that we’ll cover in [Chapter
18][ch18-00-patterns]<!-- ignore -- >. For now, we’re going to move on to the
`if let` syntax, which can be useful in situations where the `match` expression
is a bit wordy.
-->

Il existe aussi d'autres motifs que nous allons voir dans le
[chapitre 18][ch18-00-patterns]<!-- ignore -->. Pour l'instant, nous allons voir
l'autre syntaxe `if let`, qui peut se rendre utile dans des cas où l'expression
`match` est trop verbeuse.

<!--
[tuples]: ch03-02-data-types.html#the-tuple-type
[ch18-00-patterns]: ch18-00-patterns.html
-->

[tuples]: ch03-02-data-types.html
[ch18-00-patterns]: ch18-00-patterns.html
