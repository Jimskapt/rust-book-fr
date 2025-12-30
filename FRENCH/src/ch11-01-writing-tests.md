<!--
## How to Write Tests
-->

## Comment écrire des tests

<!--
Tests are Rust functions that verify that the non-test code is functioning in
the expected manner. The bodies of test functions typically perform these three
actions:
-->

Les tests sont des fonctions Rust qui vérifient que le code qui n'est pas un
test se comporte bien de la manière attendue. Les corps des fonctions de test
effectuent généralement ces trois actions :

<!--
1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.
-->

1. Initialiser toutes les données ou les états,
2. Lancer le code que vous voulez tester,
3. Vérifier que les résultats correspondent bien à ce que vous souhaitez.

<!--
Let’s look at the features Rust provides specifically for writing tests that
take these actions, which include the `test` attribute, a few macros, and the
`should_panic` attribute.
-->

Découvrons les fonctionnalités spécifiques qu'offre Rust pour écrire des tests
qui font ces actions, dont l'attribut `test`, quelques
macros et l'attribut `should_panic`.

<!--
### The Anatomy of a Test Function
-->

### L'anatomie d'une fonction de test

<!--
At its simplest, a test in Rust is a function that’s annotated with the `test`
attribute. Attributes are metadata about pieces of Rust code; one example is
the `derive` attribute we used with structs in Chapter 5. To change a function
into a test function, add `#[test]` on the line before `fn`. When you run your
tests with the `cargo test` command, Rust builds a test runner binary that runs
the functions annotated with the `test` attribute and reports on whether each
test function passes or fails.
-->

Dans la forme la plus simple, un test en Rust est une fonction qui est marquée
avec l'attribut `test`. Les attributs sont des métadonnées sur des parties de
code Rust ; un exemple est l'attribut `derive` que nous avons utilisé sur les
structures au chapitre 5. Pour transformer une fonction en une fonction de test,
il faut ajouter `#[test]` dans la ligne avant le `fn`. Lorsque vous lancez vos
tests avec la commande `cargo test`, Rust construit un binaire d'exécution de tests
qui exécute les fonctions marquées avec l'attribut `test` et fait un rapport sur
quelles fonctions ont réussi ou échoué.

<!--
When we make a new library project with Cargo, a test module with a test
function in it is automatically generated for us. This module helps you start
writing your tests so you don’t have to look up the exact structure and syntax
of test functions every time you start a new project. You can add as many
additional test functions and as many test modules as you want!
-->

Lorsque nous créons une nouvelle bibliothèque avec Cargo, un module de tests
qui contient une fonction de test est automatiquement créé pour nous. Ce module
vous aide à démarrer l'écriture de vos tests afin que vous n'ayez pas à chercher
la structure et la syntaxe exacte d'une fonction de test à chaque fois que vous
débutez un nouveau projet. Vous pouvez ajouter autant de fonctions de test et
autant de modules de tests que vous le souhaitez !

<!--
We’ll explore some aspects of how tests work by experimenting with the template
test generated for us without actually testing any code. Then we’ll write some
real-world tests that call some code that we’ve written and assert that its
behavior is correct.
-->

Nous allons découvrir quelques aspects du fonctionnement des tests en
expérimentant avec le modèle de tests généré pour nous, mais qui ne teste aucun
code pour le moment. Ensuite, nous écrirons quelques tests plus proches de la
réalité, qui utiliseront du code que nous avons écrit et qui valideront son bon
comportement.

<!--
Let’s create a new library project called `adder`:
-->

Commençons par créer un nouveau projet de bibliothèque que nous appellerons
`addition` :

<!--
```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```
-->

```console
$ cargo new addition --lib
     Created library `addition` project
$ cd addition
```

<!--
The contents of the *src/lib.rs* file in your `adder` library should look like
Listing 11-1.
-->

Le contenu de votre fichier *src/lib.rs* dans votre bibliothèque `addition`
devrait ressembler à l'encart 11-1.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new --lib listing-11-01 --name adder
cd listing-11-01
cargo test
git co output.txt
cd ../../..
-- >
-->

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-1: The test module and function generated
automatically by `cargo new`</span>
-->

<span class="caption">Encart 11-1 : Le module de test et la fonction générés
automatiquement par `cargo new`</span>

<!--
For now, let’s ignore the top two lines and focus on the function to see how it
works. Note the `#[test]` annotation before the `fn` line: this attribute
indicates this is a test function, so the test runner knows to treat this
function as a test. We could also have non-test functions in the `tests` module
to help set up common scenarios or perform common operations, so we need to
indicate which functions are tests by using the `#[test]` attribute.
-->

Pour l'instant, ignorons les deux premières lignes et concentrons-nous sur la
fonction pour voir comment elle fonctionne. Remarquez l'annotation `#[test]`
avant la ligne `fn` : cet attribut indique que c'est une fonction de test, donc
l'exécuteur de tests sait qu'il doit considérer cette fonction comme étant un
test. Nous pouvons aussi avoir des fonctions qui ne font pas de tests dans le
module `tests` afin de configurer des scénarios communs ou exécuter des
opérations communes, c'est pourquoi nous devons indiquer quelles fonctions sont
des tests en utilisant l'attribut `#[test]`.

<!--
The function body uses the `assert_eq!` macro to assert that 2 + 2 equals 4.
This assertion serves as an example of the format for a typical test. Let’s run
it to see that this test passes.
-->

Le corps de la fonction utilise la macro `assert_eq!` pour vérifier que 2 + 2
vaut bien 4. Cette vérification sert d'exemple pour expliquer le format d'un
test classique. Lançons-le pour vérifier si ce test est validé.

<!--
The `cargo test` command runs all tests in our project, as shown in Listing
11-2.
-->

La commande `cargo test` lance tous les tests présents dans votre projet, comme
le montre l'encart 11-2.

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

<!--
<span class="caption">Listing 11-2: The output from running the automatically
generated test</span>
-->

<span class="caption">Encart 11-2 : Le résultat du lancement des tests sur le
test généré automatiquement</span>

<!--
Cargo compiled and ran the test. After the `Compiling`, `Finished`, and
`Running` lines is the line `running 1 test`. The next line shows the name
of the generated test function, called `it_works`, and the result of running
that test, `ok`. The overall summary of running the tests appears next. The
text `test result: ok.` means that all the tests passed, and the portion that
reads `1 passed; 0 failed` totals the number of tests that passed or failed.
-->

Cargo a compilé et lancé le test. Après les lignes `Compiling`, `Finished`, et
`Running`, on trouve la ligne `running 1 test`. La ligne suivante montre le nom
de la fonction de test `it_works`, qui a été générée précédemment, et le
résultat de l'exécution de ce test, `ok`. Le résumé général de l'exécution des
tests s'affiche ensuite. Le texte `test result: ok.` signifie que tous les tests
ont réussi, et la partie `1 passed; 0 failed` compte le nombre total de tests
qui ont réussi ou échoué.

<!--
Because we don’t have any tests we’ve marked as ignored, the summary shows `0
ignored`. We also haven’t filtered the tests being run, so the end of the
summary shows `0 filtered out`. We’ll talk about ignoring and filtering out
tests in the next section, [“Controlling How Tests Are
Run.”][controlling-how-tests-are-run]<!-- ignore -- >
-->

Comme nous n'avons aucun test que nous avons marqué comme ignoré, le résumé
affiche `0 ignored`. Nous n'avons pas non plus filtré les tests qui ont été
exécutés, donc la fin du résumé affiche `0 filtered out`. Nous verrons comment
ignorer et filtrer les tests dans la prochaine section, [“Contrôler comment les
tests sont exécutés”][controlling-how-tests-are-run]<!-- ignore -->.

<!--
The `0 measured` statistic is for benchmark tests that measure performance.
Benchmark tests are, as of this writing, only available in nightly Rust. See
[the documentation about benchmark tests][bench] to learn more.
-->

La statistique `0 measured` sert pour des tests de benchmark qui mesurent les
performances. Les tests de benchmark ne sont disponibles pour le moment que dans
la version expérimentale de Rust (nightly), au moment de la rédaction.
Rendez-vous sur [la documentation sur les tests de benchmark][bench] pour en
savoir plus.

<!--
[bench]: ../unstable-book/library-features/test.html
-->

[bench]: https://doc.rust-lang.org/unstable-book/library-features/test.html

<!--
The next part of the test output, which starts with `Doc-tests adder`, is for
the results of any documentation tests. We don’t have any documentation tests
yet, but Rust can compile any code examples that appear in our API
documentation. This feature helps us keep our docs and our code in sync! We’ll
discuss how to write documentation tests in the [“Documentation Comments as
Tests”][doc-comments]<!-- ignore -- > section of Chapter 14. For now, we’ll
ignore the `Doc-tests` output.
-->

La partie suivante du résultat des tests, qui commence par `Doc-tests addition`,
concerne les résultats de tous les tests présents dans la documentation. Nous
n'avons pas de tests dans la documentation pour le moment, mais Rust peut
compiler tous les exemples de code qui sont présents dans la documentation de
notre API. Cette fonctionnalité nous aide à garder synchronisés notre
documentation et notre code ! Nous verrons comment écrire nos tests dans la
documentation dans une section du chapitre 14. Pour le moment, nous allons
ignorer la partie `Doc-tests` du résultat.

<!--
Let’s change the name of our test to see how that changes the test output.
Change the `it_works` function to a different name, such as `exploration`, like
so:
-->

Changeons le nom de notre test pour voir comment cela change le résultat du
test. Changeons le nom de la fonction `it_works` pour un nom différent, comme
`exploration` ci-dessous :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

<!--
Then run `cargo test` again. The output now shows `exploration` instead of
`it_works`:
-->

Lancez ensuite à nouveau `cargo test`. Le résultat affiche désormais
`exploration` plutôt que `it_works` :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

<!--
Let’s add another test, but this time we’ll make a test that fails! Tests fail
when something in the test function panics. Each test is run in a new thread,
and when the main thread sees that a test thread has died, the test is marked
as failed. We talked about the simplest way to cause a panic in Chapter 9,
which is to call the `panic!` macro. Enter the new test, `another`, so your
*src/lib.rs* file looks like Listing 11-3.
-->

Ajoutons un autre test, mais cette fois nous allons construire un test qui
échoue ! Les tests échouent lorsque quelque chose dans la fonction de test
panique. Chaque test est lancé dans une nouvelle tâche, et lorsque la tâche
principale voit qu'une tâche de test a été interrompue par panique, le test est considéré
comme ayant échoué. Nous avons vu la façon la plus simple de faire paniquer au
chapitre 9, qui consiste à appeler la macro `panic!`. Ecrivez ce nouveau test,
`un_autre`, de sorte que votre fichier `src/lib.rs` ressemble à ceci :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs:here}}
```
-->

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 11-3: Adding a second test that will fail because
we call the `panic!` macro</span>
-->

<span class="caption">Encart 11-3 : Ajout d'un second test qui va échouer car
nous appelons la macro `panic!`</span>

<!--
Run the tests again using `cargo test`. The output should look like Listing
11-4, which shows that our `exploration` test passed and `another` failed.
-->

Lancez à nouveau les tests en utilisant `cargo test`. Le résultat devrait
ressembler à l'encart 11-4, qui va afficher que notre test `exploration` a
réussi et que `un_autre` a échoué.

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

<!--
<span class="caption">Listing 11-4: Test results when one test passes and one
test fails</span>
-->

<span class="caption">Encart 11-4 : Les résultats de tests lorsque un test
réussit et un autre test échoue</span>

<!--
Instead of `ok`, the line `test tests::another` shows `FAILED`. Two new
sections appear between the individual results and the summary: the first
section displays the detailed reason for each test failure. In this case,
`another` failed because it `panicked at 'Make this test fail'`, which happened
on line 10 in the *src/lib.rs* file. The next section lists just the names of
all the failing tests, which is useful when there are lots of tests and lots of
detailed failing test output. We can use the name of a failing test to run just
that test to more easily debug it; we’ll talk more about ways to run tests in
the [“Controlling How Tests Are Run”][controlling-how-tests-are-run]<!-- ignore
-- > section.
-->

À la place du `ok`, la ligne `test tests:un_autre` affiche `FAILED`. Deux
nouvelles sections apparaissent entre la liste des tests et le résumé : la
première section affiche les raisons détaillées de chaque échec de test. Dans
notre cas, `un_autre` a échoué car il a paniqué à 'Fait échouer ce test', qui
est placé à la ligne 10 du fichier *src/lib.rs*. La partie suivante liste
simplement les noms de tous les tests qui ont échoué, ce qui est utile lorsqu'il
y a de nombreux tests et beaucoup de détails provenant des tests qui échouent.
Nous pouvons utiliser le nom d'un test qui échoue pour lancer uniquement ce test
afin de déboguer plus facilement ; nous allons voir plus de façons de lancer
des tests dans la [section suivante][controlling-how-tests-are-run]<!-- ignore
-->.

<!--
The summary line displays at the end: overall, our test result is `FAILED`.
We had one test pass and one test fail.
-->

La ligne de résumé s'affiche à la fin : au final, le résultat de nos tests est
au statut `FAILED` (échoué). Nous avons un test réussi et un test échoué.

<!--
Now that you’ve seen what the test results look like in different scenarios,
let’s look at some macros other than `panic!` that are useful in tests.
-->

Maintenant que vous avez vu à quoi ressemblent les résultats de tests dans
différents scénarios, voyons d'autres macros que `panic!` qui nous seront utiles
pour les tests.

<!--
### Checking Results with the `assert!` Macro
-->

### Vérifier les résultats avec la macro `assert!`

<!--
The `assert!` macro, provided by the standard library, is useful when you want
to ensure that some condition in a test evaluates to `true`. We give the
`assert!` macro an argument that evaluates to a Boolean. If the value is
`true`, `assert!` does nothing and the test passes. If the value is `false`,
the `assert!` macro calls the `panic!` macro, which causes the test to fail.
Using the `assert!` macro helps us check that our code is functioning in the
way we intend.
-->

La macro `assert!`, fournie par la bibliothèque standard, est utile lorsque vous
voulez vous assurer qu'une condition dans un test vaut `true`. Nous fournissons
à la macro `assert!` un argument qui donne un Booléen une fois interprété. Si la
valeur est `true`, `assert!` ne fait rien et le test est réussi. Si la valeur
est `false`, la macro `assert!` appelle la macro `panic!`, qui fait échouer le
test. L'utilisation de la macro `assert!` nous aide à vérifier que notre code
fonctionne bien comme nous le souhaitions.

<!--
In Chapter 5, Listing 5-15, we used a `Rectangle` struct and a `can_hold`
method, which are repeated here in Listing 11-5. Let’s put this code in the
*src/lib.rs* file and write some tests for it using the `assert!` macro.
-->

Dans le chapitre 5, dans l'encart 5-15, nous avons utilisé une structure
`Rectangle` et une méthode `peut_contenir`, qui sont recopiés dans l'encart 11-5
ci-dessous. Ajoutons ce code dans le fichier *src/lib.rs* et écrivons quelques
tests en utilisant la macro `assert!`.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 11-5: Using the `Rectangle` struct and its
`can_hold` method from Chapter 5</span>
-->

<span class="caption">Encart 11-5 : Utilisation de la structure `Rectangle` et de
sa méthode `peut_contenir` du chapitre 5</span>

<!--
The `can_hold` method returns a Boolean, which means it’s a perfect use case
for the `assert!` macro. In Listing 11-6, we write a test that exercises the
`can_hold` method by creating a `Rectangle` instance that has a width of 8 and
a height of 7 and asserting that it can hold another `Rectangle` instance that
has a width of 5 and a height of 1.
-->

La méthode `peut_contenir` retourne un Booléen, ce qui veut dire que c'est un
cas parfait pour tester la macro `assert!`. Dans l'encart 11-6, nous écrivons un
test qui s'applique sur la méthode `peut_contenir` en créant une instance de
`Rectangle` qui a une largeur de 8 et une hauteur de 7, et qui vérifie qu'il
peut contenir une autre instance de `Rectangle` qui a une largeur de 6 et une
hauteur de 1.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 11-6: A test for `can_hold` that checks whether a
larger rectangle can indeed hold a smaller rectangle</span>
-->

<span class="caption">Encart 11-6 : Un test pour `peut_contenir` qui vérifie le
cas où un grand rectangle peut contenir un plus petit rectangle</span>

<!--
Note that we’ve added a new line inside the `tests` module: `use super::*;`.
The `tests` module is a regular module that follows the usual visibility rules
we covered in Chapter 7 in the [“Paths for Referring to an Item in the Module
Tree”][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore -- >
section. Because the `tests` module is an inner module, we need to bring the
code under test in the outer module into the scope of the inner module. We use
a glob here so anything we define in the outer module is available to this
`tests` module.
-->

Remarquez que nous avons ajouté une nouvelle ligne à l'intérieur du module
`test` : `use super::*;`. Le module `tests` est un module classique qui suit les
règles de visibilité que nous avons vues au chapitre 7 dans la section [“Les
chemins pour désigner un élément dans l'arborescence de
module”][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore -->.
Comme le module `tests` est un module interne, nous avons besoin de ramener le
code à tester qui se trouve dans son module parent dans la portée interne du
module. Nous utilisons ici un opérateur global afin que tout ce que nous
avons défini dans le module parent soit disponible dans le module `tests`.

<!--
We’ve named our test `larger_can_hold_smaller`, and we’ve created the two
`Rectangle` instances that we need. Then we called the `assert!` macro and
passed it the result of calling `larger.can_hold(&smaller)`. This expression
is supposed to return `true`, so our test should pass. Let’s find out!
-->

Nous avons nommé notre test `un_grand_peut_contenir_un_petit`, et nous avons
créé les deux instances `Rectangle` dont nous avons besoin. Ensuite, nous avons
appelé la macro `assert!` et nous lui avons passé le résultat de l'appel à
`le_grand.peut_contenir(&le_petit)`. Cette expression est censée retourner
`true`, donc notre test devrait réussir. Vérifions cela !

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

<!--
It does pass! Let’s add another test, this time asserting that a smaller
rectangle cannot hold a larger rectangle:
-->

Il a réussi ! Ajoutons maintenant un autre test, qui vérifie cette fois qu'un
petit rectangle ne peut contenir un rectangle plus grand :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

<!--
Because the correct result of the `can_hold` function in this case is `false`,
we need to negate that result before we pass it to the `assert!` macro. As a
result, our test will pass if `can_hold` returns `false`:
-->

Comme le résultat correct de la fonction `peut_contenir` dans ce cas doit être
`false`, nous devons faire un négatif de cette fonction avant de l'envoyer à la
macro `assert!`. Cela aura pour effet de faire réussir notre test si
`peut_contenir` retourne `false` :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

<!--
Two tests that pass! Now let’s see what happens to our test results when we
introduce a bug in our code. Let’s change the implementation of the `can_hold`
method by replacing the greater than sign with a less than sign when it
compares the widths:
-->

Voilà deux tests qui réussissent ! Maintenant, voyons ce qui se passe dans les
résultats de nos tests lorsque nous introduisons un bogue dans notre code.
Changeons l'implémentation de la méthode `peut_contenir` en remplaçant
l'opérateur *plus grand que* par un *plus petit que* au moment de la comparaison
des largeurs :

<!--
```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```
-->

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

<!--
Running the tests now produces the following:
-->

Le lancement des tests donne maintenant le résultat suivant :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

<!--
Our tests caught the bug! Because `larger.width` is 8 and `smaller.width` is
5, the comparison of the widths in `can_hold` now returns `false`: 8 is not
less than 5.
-->

Nos tests ont repéré le bogue ! Comme `le_grand.largeur` est 8 et
`le_petit.largeur` est 5, la comparaison des largeurs dans `peut_contenir`
retourne maintenant `false` : 8 n'est pas plus petit que 5.

<!--
### Testing Equality with the `assert_eq!` and `assert_ne!` Macros
-->

### Tester l'égalité avec les macros `assert_eq!` et `assert_ne!`

<!--
A common way to test functionality is to compare the result of the code under
test to the value you expect the code to return to make sure they’re equal. You
could do this using the `assert!` macro and passing it an expression using the
`==` operator. However, this is such a common test that the standard library
provides a pair of macros—`assert_eq!` and `assert_ne!`—to perform this test
more conveniently. These macros compare two arguments for equality or
inequality, respectively. They’ll also print the two values if the assertion
fails, which makes it easier to see *why* the test failed; conversely, the
`assert!` macro only indicates that it got a `false` value for the `==`
expression, not the values that led to the `false` value.
-->

Une façon courante de tester des fonctionnalités est de comparer le résultat du
code à tester par rapport à une valeur que vous souhaitez que le code retourne,
afin de vous assurer qu'elles soient bien égales. Vous pouvez faire cela avec la
macro `assert!`, en lui passant une expression qui utilise l'opérateur `==`.
Cependant, c'est un test si courant que la bibliothèque standard fournit une
paire de macros (`assert_eq!` et `assert_ne!`) pour procéder à ce test plus
facilement. Les macros comparent respectivement l'égalité ou la non égalité de
deux arguments. Elles vont aussi afficher les deux valeurs si la vérification
échoue, ce qui va nous aider à comprendre *pourquoi* le test a échoué ;
paradoxalement, la macro `assert!` indique seulement qu'elle a obtenu une valeur
`false` de l'expression avec le `==`, mais n'affiche pas les valeurs qui l'ont
menée à la valeur `false`.

<!--
In Listing 11-7, we write a function named `add_two` that adds `2` to its
parameter and returns the result. Then we test this function using the
`assert_eq!` macro.
-->

Dans l'encart 11-7, nous écrivons une fonction `ajouter_deux` qui ajoute `2` à
son paramètre et retourne le résultat. Ensuite, nous testons cette fonction en
utilisant la macro `assert_eq!`.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-7: Testing the function `add_two` using the
`assert_eq!` macro</span>
-->

<span class="caption">Encart 11-7 : Test de la fonction `ajouter_deux` en
utilisant la macro `assert_eq!`.</span>

<!--
Let’s check that it passes!
-->

Vérifions si cela fonctionne !

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

<!--
The first argument we gave to the `assert_eq!` macro, `4`, is equal to the
result of calling `add_two(2)`. The line for this test is `test
tests::it_adds_two ... ok`, and the `ok` text indicates that our test passed!
-->

Le premier argument que nous avons donné à la macro `assert_eq!`, `4`, est bien
égal au résultat de l'appel à `ajouter_deux`. La ligne correspondant à ce test
est `test tests::cela_ajoute_deux ... ok`, et le texte `ok` indique que notre
test a réussi !

<!--
Let’s introduce a bug into our code to see what it looks like when a test that
uses `assert_eq!` fails. Change the implementation of the `add_two` function to
instead add `3`:
-->

Ajoutons un bogue dans notre code pour voir ce qui se passe lorsqu'un test
qui utilise `assert_eq!` échoue. Changez l'implémentation de la fonction
`ajouter_deux` pour ajouter plutôt `3` :

<!--
```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```
-->

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

<!--
Run the tests again:
-->

Lancez à nouveau les tests :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

<!--
Our test caught the bug! The `it_adds_two` test failed, displaying the message
`` assertion failed: `(left == right)` `` and showing that `left` was `4` and
`right` was `5`. This message is useful and helps us start debugging: it means
the `left` argument to `assert_eq!` was `4` but the `right` argument, where we
had `add_two(2)`, was `5`.
-->

Notre test a détecté le bogue ! Le test `cela_ajoute_deux` a échoué, ce qui a
affiché le message `` assertion failed: `(left == right)` `` qui nous explique
qu'à gauche nous avions `4` et qu'à droite nous avions `5`. Ce message utile
nous aide au débogage : cela veut dire que l'argument de gauche de `assert_eq!`
valait `4` mais que l'argument de droite, où nous avions `ajouter_deux(2)`,
valait `5`.

<!--
Note that in some languages and test frameworks, the parameters to the
functions that assert two values are equal are called `expected` and `actual`,
and the order in which we specify the arguments matters. However, in Rust,
they’re called `left` and `right`, and the order in which we specify the value
we expect and the value that the code under test produces doesn’t matter. We
could write the assertion in this test as `assert_eq!(add_two(2), 4)`, which
would result in a failure message that displays `` assertion failed: `(left ==
right)` `` and that `left` was `5` and `right` was `4`.
-->

Notez que dans certains langages et environnements de test, les paramètres des
fonctions qui vérifient que deux valeurs soient égales sont appelés `attendu` et
`effectif`, et l'ordre dans lesquels nous renseignons les arguments est
important. Cependant, dans Rust, on les appelle `gauche` et `droite`, et l'ordre
dans lesquels nous renseignons la valeur que nous attendons et la valeur que
produit le code à tester n'est pas important. Nous pouvons écrire la
vérification de ce test dans la forme `assert_eq!(ajouter_deux(2), 4)`, ce qui
donnera un message d'échec qui affichera `` assertion failed: `(left == right)`
`` et que gauche vaudra `5` et droit vaudra `4`.

<!--
The `assert_ne!` macro will pass if the two values we give it are not equal and
fail if they’re equal. This macro is most useful for cases when we’re not sure
what a value *will* be, but we know what the value definitely *won’t* be if our
code is functioning as we intend. For example, if we’re testing a function that
is guaranteed to change its input in some way, but the way in which the input
is changed depends on the day of the week that we run our tests, the best thing
to assert might be that the output of the function is not equal to the input.
-->

La macro `assert_ne!` va réussir si les deux valeurs que nous lui donnons ne
sont pas égales et va échouer si elles sont égales. Cette macro est utile dans
les cas où nous ne sommes pas sûrs de ce que *devrait* valoir une valeur, mais
que nous savons ce que la valeur ne devrait surtout *pas* être si notre code
fonctionne comme nous le souhaitons. Par exemple, si nous testons une fonction
qui doit transformer sa valeur d'entrée de manière à ce qu'elle dépende du jour
de la semaine où nous lançons nos tests, la meilleure façon de vérifier serait
que la sortie de la fonction ne soit pas égale à son entrée.

<!--
Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators
`==` and `!=`, respectively. When the assertions fail, these macros print their
arguments using debug formatting, which means the values being compared must
implement the `PartialEq` and `Debug` traits. All the primitive types and most
of the standard library types implement these traits. For structs and enums
that you define, you’ll need to implement `PartialEq` to assert that values of
those types are equal or not equal. You’ll need to implement `Debug` to print
the values when the assertion fails. Because both traits are derivable traits,
as mentioned in Listing 5-12 in Chapter 5, this is usually as straightforward
as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum
definition. See Appendix C, [“Derivable Traits,”][derivable-traits]<!-- ignore
-- > for more details about these and other derivable traits.
-->

Sous la surface, les macros `assert_eq!` et `assert_ne!` utilisent
respectivement les opérateurs `==` et `!=`. Lorsque les vérifications échouent,
ces macros affichent leurs arguments en utilisant le formatage de débogage, ce
qui veut dire que les valeurs comparées doivent implémenter les traits
`PartialEq` et `Debug`. Tous les types primitifs et la plupart des types de
la bibliothèque standard implémentent ces traits. Concernant les structures et
les énumérations que vous définissez, vous allez avoir besoin de leur
implémenter `Debug` pour afficher les valeurs lorsque les vérifications
échouent. Du fait que ces traits sont des traits dérivables, comme nous l'avons évoqué
dans l'encart 5-12 du chapitre 5, il suffit généralement de simplement ajouter
l'annotation `#[derive(PartialEq, Debug)]` sur les définitions de vos structures
ou énumérations. Rendez-vous à [l'annexe C][derivable-traits]<!-- ignore -->
pour en savoir plus sur ces derniers et les autres traits dérivables.

<!--
### Adding Custom Failure Messages
-->

### Ajouter des messages d'échec personnalisés

<!--
You can also add a custom message to be printed with the failure message as
optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros. Any
arguments specified after the one required argument to `assert!` or the two
required arguments to `assert_eq!` and `assert_ne!` are passed along to the
`format!` macro (discussed in Chapter 8 in the [“Concatenation with the `+`
Operator or the `format!`
Macro”][concatenation-with-the--operator-or-the-format-macro]<!-- ignore -- >
section), so you can pass a format string that contains `{}` placeholders and
values to go in those placeholders. Custom messages are useful to document
what an assertion means; when a test fails, you’ll have a better idea of what
the problem is with the code.
-->

Vous pouvez aussi ajouter un message personnalisé qui peut être affiché avec le
message d'échec comme un argument optionnel aux macros `assert!`, `assert_eq!`,
et `assert_ne!`. Tous les arguments renseignés après celui qui est obligatoire
dans `assert!` ou les deux arguments obligatoires de `assert_eq!` et
`assert_ne!` sont envoyés à la macro `format!` (que nous avons vue dans une
section du
[chapitre
8][concatenation-with-the--operator-or-the-format-macro]<!-- ignore -->), ainsi
vous pouvez passer une chaine de caractères de formatage qui contient des espaces
réservés `{}` et les valeurs iront dans ces espaces réservés. Les messages
personnalisés sont utiles pour documenter ce que fait une vérification ;
lorsqu'un test échoue, vous aurez une idée plus précise du problème avec ce
code.

<!--
For example, let’s say we have a function that greets people by name and we
want to test that the name we pass into the function appears in the output:
-->

Par exemple, disons que nous avons une fonction qui accueille les gens par leur
nom et que nous voulons tester que le nom que nous envoyons à la fonction
apparaît dans le résultat :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

<!--
The requirements for this program haven’t been agreed upon yet, and we’re
pretty sure the `Hello` text at the beginning of the greeting will change. We
decided we don’t want to have to update the test when the requirements change,
so instead of checking for exact equality to the value returned from the
`greeting` function, we’ll just assert that the output contains the text of the
input parameter.
-->

Les spécifications de ce programme n'ont pas été validées entièrement pour le
moment, et on est quasiment sûr que le texte `Salut` au début va changer. Nous
avons décidé que nous ne devrions pas à avoir à changer le test si les
spécifications changent, donc plutôt que de vérifier l'égalité exacte de la
valeur retournée par la fonction `accueil`, nous allons uniquement vérifier que
le résultat contient le texte correspondant au paramètre d'entrée de la
fonction.

<!--
Let’s introduce a bug into this code by changing `greeting` to not include
`name` to see what this test failure looks like:
-->

Introduisons un bogue dans ce code en changeant `accueil` pour ne pas
ajouter `nom` afin de voir ce que donne l'échec de ce test :

<!--
```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```
-->

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

<!--
Running this test produces the following:
-->

L'exécution du test va donner ceci :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

<!--
This result just indicates that the assertion failed and which line the
assertion is on. A more useful failure message in this case would print the
value we got from the `greeting` function. Let’s change the test function,
giving it a custom failure message made from a format string with a placeholder
filled in with the actual value we got from the `greeting` function:
-->

Ce résultat indique simplement que la vérification a échoué, et à quel endroit.
Le message d'échec serait plus utile dans notre cas s'il affichait la valeur
que nous obtenons de la fonction `accueil`. Changeons la fonction de test, pour
lui donner un message d'erreur personnalisé, qui est une chaîne de caractères
de formatage avec un espace réservé qui contiendra la valeur que
nous avons obtenue de la fonction `accueil` :

<!--
```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

<!--
Now when we run the test, we’ll get a more informative error message:
-->

Maintenant, lorsque nous lançons à nouveau le test, nous obtenons un message
d'échec plus explicite :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

<!--
We can see the value we actually got in the test output, which would help us
debug what happened instead of what we were expecting to happen.
-->

Nous pouvons voir la valeur que nous avons obtenue lors de la lecture du
résultat du test, ce qui va nous aider à déboguer ce qui s'est passé à la
place de ce que nous voulions qu'il se passe.

<!--
### Checking for Panics with `should_panic`
-->

### Vérifier le fonctionnement des paniques avec `should_panic`

<!--
In addition to checking that our code returns the correct values we expect,
it’s also important to check that our code handles error conditions as we
expect. For example, consider the `Guess` type that we created in Chapter 9,
Listing 9-13. Other code that uses `Guess` depends on the guarantee that `Guess`
instances will contain only values between 1 and 100. We can write a test that
ensures that attempting to create a `Guess` instance with a value outside that
range panics.
-->

En plus de vérifier que notre code retourne bien les valeurs que nous
souhaitons, il est aussi important de vérifier que notre code gère bien les cas
d'erreurs comme nous le souhaitons. Par exemple, utilisons le type `Supposition`
que nous avons créé au chapitre 9, dans l'encart 9-13. Les autres codes qui
utilisent `Supposition` reposent sur la garantie que les instances de
`Supposition` contiennent uniquement des valeurs entre 1 et 100. Nous pouvons
écrire un test qui s'assure que la création d'une instance de `Supposition`
avec une valeur en dehors de cette intervalle va faire paniquer le programme.

<!--
We do this by adding another attribute, `should_panic`, to our test function.
This attribute makes a test pass if the code inside the function panics; the
test will fail if the code inside the function doesn’t panic.
-->

Nous allons vérifier cela en ajoutant un autre attribut, `should_panic`, à notre
fonction de test. Cet attribut fait réussir le test si le code à l'intérieur
de la fonction fait paniquer ; le test va échouer si le code à l'intérieur de
la fonction ne panique pas.

<!--
Listing 11-8 shows a test that checks that the error conditions of `Guess::new`
happen when we expect them to.
-->

L'encart 11-8 nous montre un test qui vérifie que les conditions d'erreur de
`Supposition::new` fonctionne bien comme nous l'avons prévu.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-8: Testing that a condition will cause a
`panic!`</span>
-->

<span class="caption">Encart 11-8 : Tester qu'une condition va faire un `panic`
</span>

<!--
We place the `#[should_panic]` attribute after the `#[test]` attribute and
before the test function it applies to. Let’s look at the result when this test
passes:
-->

Nous plaçons l'attribut `#[should_panic]` après l'attribut `#[test]` et avant
la fonction de test sur laquelle il s'applique. Voyons le résultat lorsque ce
test réussit :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

<!--
Looks good! Now let’s introduce a bug in our code by removing the condition
that the `new` function will panic if the value is greater than 100:
-->

Ca fonctionne ! Maintenant, ajoutons un bogue dans notre code en enlevant
la condition dans laquelle la fonction `new` panique lorsque la valeur est
plus grande que 100 :

<!--
```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```
-->

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

<!--
When we run the test in Listing 11-8, it will fail:
-->

Lorsque nous lançons le test de l'encart 11-8, il va échouer :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

<!--
We don’t get a very helpful message in this case, but when we look at the test
function, we see that it’s annotated with `#[should_panic]`. The failure we got
means that the code in the test function did not cause a panic.
-->

Dans ce cas, nous n'obtenons pas de message très utile, mais lorsque nous
regardons la fonction de test, nous constatons qu'elle est marquée avec
`#[should_panic]`. L'échec que nous obtenons signifie que le code dans la
fonction de test n'a pas fait paniquer.

<!--
Tests that use `should_panic` can be imprecise because they only indicate that
the code has caused some panic. A `should_panic` test would pass even if the
test panics for a different reason from the one we were expecting to happen. To
make `should_panic` tests more precise, we can add an optional `expected`
parameter to the `should_panic` attribute. The test harness will make sure that
the failure message contains the provided text. For example, consider the
modified code for `Guess` in Listing 11-9 where the `new` function panics with
different messages depending on whether the value is too small or too large.
-->

Les tests qui utilisent `should_panic` ne sont parfois pas assez explicites car
ils indiquent seulement que le code a paniqué. Un test `should_panic` peut
réussir, même si le test panique pour une raison différente de celle que nous
attendions. Pour rendre les tests `should_panic` plus précis, nous pouvons
ajouter un paramètre optionnel `expected` à l'attribut `should_panic`. Le
système de test va s'assurer que le message d'échec contient bien le texte
renseigné. Par exemple, imaginons le code modifié de `Supposition` dans
l'encart 11-9 où la fonction `new` panique avec des messages différents si la
valeur est trop petite ou trop grande.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 11-9: Testing that a condition will cause a
`panic!` with a particular panic message</span>
-->

<span class="caption">Encart 11-9 : On vérifie qu'une situation va provoquer un
`panic!` avec un message de panique bien précis</span>

<!--
This test will pass because the value we put in the `should_panic` attribute’s
`expected` parameter is a substring of the message that the `Guess::new`
function panics with. We could have specified the entire panic message that we
expect, which in this case would be `Guess value must be less than or equal to
100, got 200.` What you choose to specify in the expected parameter for
`should_panic` depends on how much of the panic message is unique or dynamic
and how precise you want your test to be. In this case, a substring of the
panic message is enough to ensure that the code in the test function executes
the `else if value > 100` case.
-->

Ce test va réussir car la valeur que nous insérons dans l'attribut `expected`
de `should_panic` est une partie du message de panique de la fonction
`Supposition::new`. Nous aurions pu renseigner le message de panique en entier
que nous attendions, qui dans ce cas est `La supposition doit être plus petite
ou égale à 100, et nous avons 200.`. Ce que vous choisissez de renseigner dans
le paramètre `expected` de `should_panic` dépend de la mesure dans laquelle le
message de panique est unique ou dynamique et de la précision de votre test que
vous souhaitez appliquer. Dans ce cas, un extrait du message de panique est
suffisant pour s'assurer que le code de la fonction de test s'exécute dans le
cas du `else if valeur > 100`.

<!--
To see what happens when a `should_panic` test with an `expected` message
fails, let’s again introduce a bug into our code by swapping the bodies of the
`if value < 1` and the `else if value > 100` blocks:
-->

Pour voir ce qui se passe lorsqu'un test `should_panic` qui a un message
`expected` qui échoue, essayons à nouveau d'introduire un bogue dans notre code
en permutant les corps des blocs de `if valeur < 1` et de
`else if valeur > 100` :

<!--
```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```
-->

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

<!--
This time when we run the `should_panic` test, it will fail:
-->

Cette fois, lorsque nous lançons le test avec `should_panic`, il devrait
échouer :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

<!--
The failure message indicates that this test did indeed panic as we expected,
but the panic message did not include the expected string `'Guess value must be
less than or equal to 100'`. The panic message that we did get in this case was
`Guess value must be greater than or equal to 1, got 200.` Now we can start
figuring out where our bug is!
-->

Le message d'échec nous informe que ce test a paniqué comme prévu, mais que le
message de panique n'inclut pas la chaîne de caractères prévue `'La supposition
doit être plus petite ou égale à 100'`. Le message de panique que nous avons
obtenu dans ce cas était `La supposition doit être plus grande ou égale à 1, et
nous avons 200.`. Maintenant, on comprend mieux où est le bogue !

<!--
### Using `Result<T, E>` in Tests
-->

### Utiliser `Result<T, E>` dans les tests

<!--
So far, we’ve written tests that panic when they fail. We can also write tests
that use `Result<T, E>`! Here’s the test from Listing 11-1, rewritten to use
`Result<T, E>` and return an `Err` instead of panicking:
-->

Précédemment, nous avons écrit des tests qui paniquent lorsqu'ils échouent.
Nous pouvons également écrire des tests qui utilisent `Result<T, E>` ! Voici
le test de l'encart 11-1, réécrit pour utiliser `Result<T, E>` et retourner
une `Err` au lieu de paniquer :

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs}}
```

<!--
The `it_works` function now has a return type, `Result<(), String>`. In the
body of the function, rather than calling the `assert_eq!` macro, we return
`Ok(())` when the test passes and an `Err` with a `String` inside when the test
fails.
-->

La fonction `it_works` a maintenant un type de retour, `Result<(), String>`.
Dans le corps de la fonction, plutôt que d'appeler la macro `assert_eq!`, nous
retournons `Ok(())` lorsque le test réussit et une `Err` avec une `String` à
l'intérieur lorsque le test échoue.

<!--
Writing tests so they return a `Result<T, E>` enables you to use the question
mark operator in the body of tests, which can be a convenient way to write
tests that should fail if any operation within them returns an `Err` variant.
-->

Écrire vos tests afin qu'ils retournent un `Result<T, E>` vous permet
d'utiliser l'opérateur *point d'interrogation* dans le corps des tests, ce
qui est un outil facile à utiliser pour écrire des tests qui peuvent échouer
si n'importe quelle opération en son sein retourne une variante de `Err`.

<!--
You can’t use the `#[should_panic]` annotation on tests that use `Result<T,
E>`. To assert that an operation returns an `Err` variant, *don’t* use the
question mark operator on the `Result<T, E>` value. Instead, use
`assert!(value.is_err())`.
-->

Vous ne pouvez pas utiliser l'annotation `#[should_panic]` sur les tests qui
utilisent `Result<T, E>`. Pour vérifier qu'une opération retourne une variante
`Err`, *n'utilisez pas* l'opérateur "point d'interrogation" sur la valeur de
type `Result<T, E>`. À la place, utilisez plutôt `assert!(valeur.is_err())`.

<!--
Now that you know several ways to write tests, let’s look at what is happening
when we run our tests and explore the different options we can use with `cargo
test`.
-->

Maintenant que vous avez appris différentes manières d'écrire des tests, voyons
ce qui se passe lorsque nous lançons nos tests et explorons les différentes
options que nous pouvons utiliser avec `cargo test`.

<!--
[concatenation-with-the--operator-or-the-format-macro]:
ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro
[controlling-how-tests-are-run]:
ch11-02-running-tests.html#controlling-how-tests-are-run
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
-->

[concatenation-with-the--operator-or-the-format-macro]: ch08-02-strings.html
[controlling-how-tests-are-run]: ch11-02-running-tests.html
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html
[paths-for-referring-to-an-item-in-the-module-tree]:
ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
