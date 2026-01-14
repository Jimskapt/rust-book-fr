<!--
## Concise Control Flow with `if let`
-->

## Une structure de contrôle concise : `if let`

<!--
The `if let` syntax lets you combine `if` and `let` into a less verbose way to
handle values that match one pattern while ignoring the rest. Consider the
program in Listing 6-6 that matches on an `Option<u8>` value in the `config_max`
variable but only wants to execute code if the value is the `Some` variant.
-->

La syntaxe `if let` vous permet de combiner `if` et `let` afin de gérer les
valeurs qui correspondent à un motif donné, tout en ignorant les autres.
Imaginons le programme dans l'encart 6-6 qui fait un `match` sur la valeur
`Option<u8>` de la variable `une_valeur_u8` mais n'a besoin d'exécuter du code que
si la valeur est la variante `Some`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-6: A `match` that only cares about executing
code when the value is `Some`</span>
-->

<span class="caption">Encart 6-6 : Un `match` qui n'exécute du code que si la
valeur est `Some`</span>

<!--
If the value is `Some`, we print out the value in the `Some` variant by binding
the value to the variable `max` in the pattern. We don’t want to do anything
with the `None` value. To satisfy the `match` expression, we have to add `_ =>
()` after processing just one variant, which is annoying boilerplate code to
add.
-->

Si la valeur est un `Some`, nous affichons la valeur dans la variante `Some` en
associant la valeur à la variable `max` dans le motif. Nous ne voulons rien
faire avec la valeur `None`. Pour satisfaire l'expression `match`, nous devons
ajouter `_ => ()` après avoir géré une seule variante, ce qui est du code
inutile.

<!--
Instead, we could write this in a shorter way using `if let`. The following
code behaves the same as the `match` in Listing 6-6:
-->

À la place, nous pourrions écrire le même programme de manière plus concise en
utilisant `if let`. Le code suivant se comporte comme le `match` de l'encart
6-6 :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

<!--
The syntax `if let` takes a pattern and an expression separated by an equal
sign. It works the same way as a `match`, where the expression is given to the
`match` and the pattern is its first arm. In this case, the pattern is
`Some(max)`, and the `max` binds to the value inside the `Some`. We can then
use `max` in the body of the `if let` block in the same way as we used `max` in
the corresponding `match` arm. The code in the `if let` block isn’t run if the
value doesn’t match the pattern.
-->

La syntaxe `if let` prend un motif et une expression séparés par un signe égal.
Elle fonctionne de la même manière qu'un `match` où l'expression est donnée au
`match` et où le motif est sa première branche. Dans ce cas, le motif est
`Some(max)`, et le `max` est associé à la valeur dans le `Some`. Nous pouvons
ensuite utiliser `max` dans le corps du bloc `if let` de la même manière que
nous avons utilisé `max` dans la branche correspondante au `match`. Le code dans
le bloc `if let` n'est pas exécuté si la valeur ne correspond pas au motif.

<!--
Using `if let` means less typing, less indentation, and less boilerplate code.
However, you lose the exhaustive checking that `match` enforces. Choosing
between `match` and `if let` depends on what you’re doing in your particular
situation and whether gaining conciseness is an appropriate trade-off for
losing exhaustive checking.
-->

Utiliser `if let` permet d'écrire moins de code, et de moins l'indenter.
Cependant, vous perdez la vérification de l'exhaustivité qu'assure le `match`.
Choisir entre `match` et `if let` dépend de la situation : à vous de choisir
s'il vaut mieux être concis ou appliquer une vérification exhaustive.

<!--
In other words, you can think of `if let` as syntax sugar for a `match` that
runs code when the value matches one pattern and then ignores all other values.
-->

Autrement dit, vous pouvez considérer le `if let` comme du sucre syntaxique pour
un `match` qui exécute du code uniquement quand la valeur correspond à un motif
donné et ignore toutes les autres valeurs.

<!--
We can include an `else` with an `if let`. The block of code that goes with the
`else` is the same as the block of code that would go with the `_` case in the
`match` expression that is equivalent to the `if let` and `else`. Recall the
`Coin` enum definition in Listing 6-4, where the `Quarter` variant also held a
`UsState` value. If we wanted to count all non-quarter coins we see while also
announcing the state of the quarters, we could do that with a `match`
expression like this:
-->

Nous pouvons joindre un `else` à un `if let`. Le bloc de code qui va dans le
`else` est le même que le bloc de code qui va dans le cas `_` avec l'expression
`match`. Souvenez-vous de la définition de l'énumération `PieceUs` de l'encart
6-4, où la variante `Quarter` stockait aussi une valeur `EtatUs`. Si nous
voulions compter toutes les pièces qui ne sont pas des *quarters* que nous
voyons passer, tout en affichant l'État des *quarters*, nous pourrions le faire
avec une expression `match` comme ceci :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

<!--
Or we could use an `if let` and `else` expression like this:
-->

Ou nous pourrions utiliser une expression `if let`/`else` comme ceci :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

<!--
If you have a situation in which your program has logic that is too verbose to
express using a `match`, remember that `if let` is in your Rust toolbox as well.
-->

Si vous trouvez que votre programme est alourdi par l'utilisation d'un `match`,
souvenez-vous que `if let` est aussi présent dans votre boîte à outils Rust.

<!--
## Summary
-->

## Résumé

<!--
We’ve now covered how to use enums to create custom types that can be one of a
set of enumerated values. We’ve shown how the standard library’s `Option<T>`
type helps you use the type system to prevent errors. When enum values have
data inside them, you can use `match` or `if let` to extract and use those
values, depending on how many cases you need to handle.
-->

Nous avons désormais appris comment utiliser les énumérations pour créer des
types personnalisés qui peuvent faire partie d'un jeu de valeurs recensées. Nous
avons montré comment le type `Option<T>` de la bibliothèque standard vous aide
à utiliser le système de types pour éviter les erreurs. Lorsque les valeurs
d'énumération contiennent des données, vous pouvez utiliser `match` ou `if let`
pour extraire et utiliser ces valeurs, à choisir en fonction du nombre de cas
que vous voulez gérer.

<!--
Your Rust programs can now express concepts in your domain using structs and
enums. Creating custom types to use in your API ensures type safety: the
compiler will make certain your functions get only values of the type each
function expects.
-->

Vos programmes Rust peuvent maintenant décrire des concepts métier à l'aide de
structures et d'énumérations. Créer des types personnalisés à utiliser dans
votre API assure la sécurité des types : le compilateur s'assurera que vos
fonctions ne reçoivent que des valeurs du type attendu.

<!--
In order to provide a well-organized API to your users that is straightforward
to use and only exposes exactly what your users will need, let’s now turn to
Rust’s modules.
-->

Afin de fournir une API bien organisée, simple à utiliser et qui n'expose que ce
dont vos utilisateurs auront besoin, découvrons maintenant les modules de Rust.
