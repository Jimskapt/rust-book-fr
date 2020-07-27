<!--
## Comparing Performance: Loops vs. Iterators
-->

## Comparaison des performances : les boucles et les itérateurs

<!--
To determine whether to use loops or iterators, you need to know which version
of our `search` functions is faster: the version with an explicit `for` loop or
the version with iterators.
-->

Pour déterminer s'il faut utiliser des boucles ou des itérateurs, nous devons
savoir quelle version de notre fonction `rechercher` est la plus rapide : la
version avec une boucle `for` explicite ou la version avec des itérateurs ?

<!--
We ran a benchmark by loading the entire contents of *The Adventures of
Sherlock Holmes* by Sir Arthur Conan Doyle into a `String` and looking for the
word *the* in the contents. Here are the results of the benchmark on the
version of `search` using the `for` loop and the version using iterators:
-->

Nous avons lancé un benchmark en chargeant tout le contenu de *The Adventures
of Sherlock Holmes* de Sir Arthur Conan Doyle dans une `String` et en cherchant
le mot "the" dans le contenu. Voici les résultats du benchmark sur la version
de `rechercher` avec une boucle `for` et avec un itérateur :

<!--
```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```
-->

```text
test benchmark_rechercher_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test benchmark_rechercher_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

<!--
The iterator version was slightly faster! We won’t explain the benchmark code
here, because the point is not to prove that the two versions are equivalent
but to get a general sense of how these two implementations compare
performance-wise.
-->

La version avec l'itérateur était un peu plus rapide ! Nous n'expliquerons pas
le code du benchmark ici, car il ne s'agit pas de prouver que les deux versions
sont équivalentes, mais d'avoir une idée générale de la différence de
performances entre les deux.

<!--
For a more comprehensive benchmark, you should check using various texts of
various sizes as the `contents`, different words and words of different lengths
as the `query`, and all kinds of other variations. The point is this:
iterators, although a high-level abstraction, get compiled down to roughly the
same code as if you’d written the lower-level code yourself. Iterators are one
of Rust’s *zero-cost abstractions*, by which we mean using the abstraction
imposes no additional runtime overhead. This is analogous to how Bjarne
Stroustrup, the original designer and implementor of C++, defines
*zero-overhead* in “Foundations of C++” (2012):
-->

Pour un benchmark plus complet, nous vous conseillons d'utiliser des textes de
différentes tailles pour `contenu`, des mots différents et de différentes
longueurs pour `recherche`, ainsi que tout autre type de variation que vous
pourriez trouver. Le point important est le suivant : les itérateurs, bien qu'il
s'agisse d'une abstraction de haut niveau, sont compilés à peu près comme si
vous aviez écrit vous-même le code un niveau plus bas. Les itérateurs sont l'une
des abstractions à *coût zéro* de Rust, c'est-à-dire que l'utilisation de
l'abstraction n'impose aucun surcoût lors de l'exécution. C'est la même notion
que Bjarne Stroustrup, le concepteur et développeur original de C++, définit le
*coût zéro* dans “Foundations of C++” (2012) :

<!--
> In general, C++ implementations obey the zero-overhead principle: What you
> don’t use, you don’t pay for. And further: What you do use, you couldn’t hand
> code any better.
-->

> En général, les implémentations de C++ obéissent au principe du coût zéro :
> ce que vous n'utilisez pas, vous ne payez pas. Et encore plus loin : ce que
> vous utilisez, vous ne pouvez pas mieux le coder.

<!--
As another example, the following code is taken from an audio decoder. The
decoding algorithm uses the linear prediction mathematical operation to
estimate future values based on a linear function of the previous samples. This
code uses an iterator chain to do some math on three variables in scope: a
`buffer` slice of data, an array of 12 `coefficients`, and an amount by which
to shift data in `qlp_shift`. We’ve declared the variables within this example
but not given them any values; although this code doesn’t have much meaning
outside of its context, it’s still a concise, real-world example of how Rust
translates high-level ideas to low-level code.
-->

Comme autre exemple, le code suivant est tiré d'un décodeur audio. L'algorithme
de décodage utilise l'opération mathématique de prédiction linéaire pour
estimer les valeurs futures à partir d'une fonction linéaire des échantillons
précédents. Ce code utilise une chaîne d'itérateurs pour faire quelques calculs
sur trois variables dans la portée : une slice de données `tampon`, un tableau
de 12 `coefficients`, et une quantité de décalage des données dans `decalage`.
Nous avons déclaré les variables dans cet exemple, mais nous ne leur avons pas
donné de valeurs ; bien que ce code n'ait pas beaucoup de signification en
dehors de son contexte, c'est toutefois un exemple concis et concret de la façon
dont Rust traduit des idées de haut niveau en code de plus bas niveau.

<!--
```rust,ignore
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```
-->

```rust,ignore
let tampon: &mut [i32];
let coefficients: [i64; 12];
let decalage: i16;

for i in 12..tampon.len() {
    let prediction = coefficients.iter()
                                 .zip(&tampon[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> decalage;
    let delta = tampon[i];
    tampon[i] = prediction as i32 + delta;
}
```

<!--
To calculate the value of `prediction`, this code iterates through each of the
12 values in `coefficients` and uses the `zip` method to pair the coefficient
values with the previous 12 values in `buffer`. Then, for each pair, we
multiply the values together, sum all the results, and shift the bits in the
sum `qlp_shift` bits to the right.
-->

Pour calculer la valeur de `prediction`, ce code itère sur chacune des 12
valeurs dans `coefficients` et utilise la méthode `zip` pour appairer la
valeur de coefficient avec les 12 valeurs précédentes, présentes dans `tampon`.
Ensuite, pour chaque paire, nous multiplions les valeurs ensemble, nous
additionnons tous les résultats et nous décalons les bits de l'addition de la
valeur de `decalage` vers la droite.

<!--
Calculations in applications like audio decoders often prioritize performance
most highly. Here, we’re creating an iterator, using two adaptors, and then
consuming the value. What assembly code would this Rust code compile to? Well,
as of this writing, it compiles down to the same assembly you’d write by hand.
There’s no loop at all corresponding to the iteration over the values in
`coefficients`: Rust knows that there are 12 iterations, so it “unrolls” the
loop. *Unrolling* is an optimization that removes the overhead of the loop
controlling code and instead generates repetitive code for each iteration of
the loop.
-->

Les calculs dans des applications comme les décodeurs audio donnent souvent la
priorité aux performances. Ici, nous créons un itérateur à l'aide de deux
adaptateurs, puis nous en consommons la valeur. A quel code d'assemblage
ressemblera ce code Rust ressemblera-t-il une fois compilé ? Et bien, à l'heure
où nous écrivons ces lignes, il est équivalent même code assembleur que vous
écririez à la main. Il n'y a pas du tout de boucle correspondant à l'itération
sur les valeurs dans `coefficients` : Rust sait qu'il y a 12 itérations, donc il
“déroule” la boucle. Le *déroulage* est une optimisation qui supprime la
surcharge du code de contrôle de boucle et génère à la place du code répété pour
chaque itération de la boucle.

<!--
All of the coefficients get stored in registers, which means accessing the
values is very fast. There are no bounds checks on the array access at runtime.
All these optimizations that Rust is able to apply make the resulting code
extremely efficient. Now that you know this, you can use iterators and closures
without fear! They make code seem like it’s higher level but don’t impose a
runtime performance penalty for doing so.
-->

Tous les coefficients sont stockés dans des registres, ce qui signifie qu'il
est très rapide d'accéder à ces valeurs. Il n'y a pas de vérification des bornes
sur les accès au tableau à l'exécution. Toutes ces optimisations que Rust est
capable d'appliquer rendent le code produit extrêmement efficace. Maintenant
que vous savez cela, vous pouvez utiliser des itérateurs et des closures
sans crainte ! Ils font en sorte que le code est de haut niveau, mais ils n'ont
pas de pénalité de performance à l'exécution.

<!--
## Summary
-->

## Résumé

<!--
Closures and iterators are Rust features inspired by functional programming
language ideas. They contribute to Rust’s capability to clearly express
high-level ideas at low-level performance. The implementations of closures and
iterators are such that runtime performance is not affected. This is part of
Rust’s goal to strive to provide zero-cost abstractions.
-->

Les closures et les itérateurs sont des fonctionnalités de Rust inspirées par
des idées des langages de programmation fonctionnels. Ils contribuent à la
capacité de Rust d'exprimer clairement des idées de haut niveau avec des
performances dignes d'un langage de bas niveau. Les implémentations des
closures et des itérateurs sont telles que les performances à l'exécution ne
sont pas affectées. Cela fait partie de l'objectif de Rust de s'efforcer à
fournir des abstractions à coût zéro.

<!--
Now that we’ve improved the expressiveness of our I/O project, let’s look at
some more features of `cargo` that will help us share the project with the
world.
-->

Maintenant que nous avons amélioré l'expressivité de notre projet
d'entrée/sortie, regardons d'autres fonctionnalités fournies par `cargo` qui
nous aideront à partager notre projet avec le monde entier.
