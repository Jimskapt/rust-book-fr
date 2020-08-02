<!--
## Refutability: Whether a Pattern Might Fail to Match
-->

## La réfutabilité : lorsqu'un motif peut échouer à correspondre

<!--
Patterns come in two forms: refutable and irrefutable. Patterns that will match
for any possible value passed are *irrefutable*. An example would be `x` in the
statement `let x = 5;` because `x` matches anything and therefore cannot fail
to match. Patterns that can fail to match for some possible value are
*refutable*. An example would be `Some(x)` in the expression `if let Some(x) =
a_value` because if the value in the `a_value` variable is `None` rather than
`Some`, the `Some(x)` pattern will not match.
-->

Les motifs se divisent en deux catégories : réfutables et irréfutables. Les
motifs qui vont correspondre à n'importe quelle valeur qu'on lui passe sont
*irréfutables*. Un exemple serait le `x` dans l'instruction `let x = 5;` car
`x` correspond à tout ce qui est possible et ainsi ne peut pas échouer à la
correspondance. Les motifs qui peuvent échouer à correspondre à quelques valeurs
sont *réfutables*. Un exemple serait `Some(x)` dans l'expression
`if let Some(x) = une_valeur` car si la valeur dans la variable `une_valeur` est
`None` au lieu de `Some`, le motif `Some(x)` ne correspondra pas.

<!--
Function parameters, `let` statements, and `for` loops can only accept
irrefutable patterns, because the program cannot do anything meaningful when
values don’t match. The `if let` and `while let` expressions accept
refutable and irrefutable patterns, but the compiler warns against
irrefutable patterns because by definition they’re intended to handle possible
failure: the functionality of a conditional is in its ability to perform
differently depending on success or failure.
-->

Les paramètres de fonctions, les instructions `let`, et les boucles `for`
peuvent seulement accepter des motifs irréfutables, car le programme ne peut
rien faire d'autre lorsque les valeurs ne correspondent pas. Les expressions
`if let` et `while let` acceptent les motifs réfutables et irréfutables, mais le
compilateur met en garde contre l'utilisation des motifs irréfutables dans ce
cas car par définition ces expressions sont prévues pour gérer un problème
éventuel : le but des conditions est de se comporter différemment en fonction de
la réussite ou de l'échec.

<!--
In general, you shouldn’t have to worry about the distinction between refutable
and irrefutable patterns; however, you do need to be familiar with the concept
of refutability so you can respond when you see it in an error message. In
those cases, you’ll need to change either the pattern or the construct you’re
using the pattern with, depending on the intended behavior of the code.
-->

De manière générale, vous ne devriez pas avoir à vous soucier des différences
entre les motifs réfutables et irréfutables ; en revanche, vous devez vous
familiariser avec le concept de réfutabilité afin que vous puissiez comprendre
lorsque vous le verrez dans un message d'erreur. Dans ce cas, vous allez avoir
besoin de changer soit le motif, soit la construction avec laquelle vous
utilisez, en fonction du comportement prévu du code.

<!--
Let’s look at an example of what happens when we try to use a refutable pattern
where Rust requires an irrefutable pattern and vice versa. Listing 18-8 shows a
`let` statement, but for the pattern we’ve specified `Some(x)`, a refutable
pattern. As you might expect, this code will not compile.
-->

Examinons un exemple de ce qu'il se passe lorsque nous essayons d'utiliser un
motif réfutable lorsque Rust prévoit d'utiliser un motif irréfutable, et
vice-versa. L'encart 18-8 montre une instruction `let`, mais pour le motif nous
avons renseigné `Some(x)`, un motif réfutable. Comme vous pouvez vous en douter,
ce code ne va pas se compiler.

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-08/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-08/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-8: Attempting to use a refutable pattern with
`let`</span>
-->

<span class="caption">Encart 18-8 : tentative d'utilisation d'un motif
réfutable avec `let`</span>

<!--
If `some_option_value` was a `None` value, it would fail to match the pattern
`Some(x)`, meaning the pattern is refutable. However, the `let` statement can
only accept an irrefutable pattern because there is nothing valid the code can
do with a `None` value. At compile time, Rust will complain that we’ve tried to
use a refutable pattern where an irrefutable pattern is required:
-->

Si `une_option_quelconque` était une valeur `None`, cela ferait échouer le motif
`Some(x)`, ce qui signifie que le motif est réfutable. Cependant, l'instruction
`let` ne peut accepter qu'un motif irréfutable car il n'y a pas d'instructions à
suivre dans le cas d'une valeur `None`. A la compilation, Rust s'y opposera en
expliquant que nous avons essayé d'utiliser un motif réfutable là où un motif
irréfutable est nécessaire :

<!--
```console
{{#include ../listings-sources/ch18-patterns-and-matching/listing-18-08/output.txt}}
```
-->

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-08/output.txt}}
```

<!--
Because we didn’t cover (and couldn’t cover!) every valid value with the
pattern `Some(x)`, Rust rightfully produces a compiler error.
-->

Comme nous n'avons pas couvert (et nous ne pouvons pas le faire !) chaque
valeur possible avec le motif `Some(x)`, Rust génère une erreur de compilation,
à juste titre.

<!--
To fix the problem where we have a refutable pattern where an irrefutable
pattern is needed, we can change the code that uses the pattern: instead of
using `let`, we can use `if let`. Then if the pattern doesn’t match, the code
will just skip the code in the curly brackets, giving it a way to continue
validly. Listing 18-9 shows how to fix the code in Listing 18-8.
-->

Pour corriger le problème lorsque nous avons un motif réfutable où un motif
irréfutable est nécessaire, nous pouvons changer le code qui utilise ce motif :
au lieu d'utiliser `let`, nous pouvons utiliser `if let`. Ensuite, si le motif
ne correspond pas, le code va simplement sauter le code entre les accolades,
nous offrant la possibilité de continuer correctement. L'encart 18-9 montre
comment corriger le code de l'encart 18-8.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-09/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-9: Using `if let` and a block with refutable
patterns instead of `let`</span>
-->

<span class="caption">Encart 18-9 : utilisation de `if let` et d'un bloc avec
un motif réfutable plutôt qu'un `let`</span>

<!--
We’ve given the code an out! This code is perfectly valid, although it means we
cannot use an irrefutable pattern without receiving an error. If we give `if
let` a pattern that will always match, such as `x`, as shown in Listing 18-10,
the compiler will give a warning.
-->

Ce code est parfaitement valide, bien que cela signifie que nous ne pouvons pas
utiliser un motif irréfutable sans avoir d'erreur. Si nous donnons au `if let`
un motif qui correspond toujours, comme pour `x` montré dans l'encart 18-10, le
compilateur va lever un avertissement.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-10/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-10/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-10: Attempting to use an irrefutable pattern
with `if let`</span>
-->

<span class="caption">Encart 18-10 : tentative d'utiliser un motif irréfutable
avec `if let`</span>

<!--
Rust complains that it doesn’t make sense to use `if let` with an irrefutable
pattern:
-->

Rust explique que cela ne fait aucun sens d'utiliser `if let` avec un motif
irréfutable :

<!--
```console
{{#include ../listings-sources/ch18-patterns-and-matching/listing-18-10/output.txt}}
```
-->

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-10/output.txt}}
```

<!--
For this reason, match arms must use refutable patterns, except for the last
arm, which should match any remaining values with an irrefutable pattern. Rust
allows us to use an irrefutable pattern in a `match` with only one arm, but
this syntax isn’t particularly useful and could be replaced with a simpler
`let` statement.
-->

C'est pourquoi les branches de `match` doivent utiliser des motifs réfutables,
sauf pour la dernière branche, qui devrait correspondre à n'importe quelle
valeur grâce à son motif irréfutable. Rust nous permet d'utiliser un motif
irréfutable dans un `match` avec une seule branche, mais cette syntaxe n'est
pas particulièrement utile et devrait être remplacée par une instruction `let`
plus simple.

<!--
Now that you know where to use patterns and the difference between refutable
and irrefutable patterns, let’s cover all the syntax we can use to create
patterns.
-->

Maintenant que vous savez où utiliser les motifs et les différences entre les
motifs réfutables et irréfutables, voyons les syntaxes que nous pouvons
utiliser pour créer des motifs.
