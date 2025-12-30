<!--
## Test Organization
-->

## L'organisation des tests

<!--
As mentioned at the start of the chapter, testing is a complex discipline, and
different people use different terminology and organization. The Rust community
thinks about tests in terms of two main categories: *unit tests* and
*integration tests*. Unit tests are small and more focused, testing one module
in isolation at a time, and can test private interfaces. Integration tests are
entirely external to your library and use your code in the same way any other
external code would, using only the public interface and potentially exercising
multiple modules per test.
-->

Comme nous l'avons évoqué au début du chapitre, le test est une discipline
complexe, et différentes personnes utilisent des terminologies et organisations
différentes. La communauté Rust a conçu les tests dans deux catégories
principales : *les tests unitaires* et *les tests d'intégration*. Les tests
unitaires sont petits et plus précis, testent un module isolé à la fois, et
peuvent tester les interfaces privées. Les tests d'intégration sont uniquement
externes à notre bibliothèque et consomment notre code exactement de la même
manière que tout autre code externe le ferait, en utilisant uniquement
l'interface publique et éventuellement en utilisant plusieurs modules dans un
test.

<!--
Writing both kinds of tests is important to ensure that the pieces of your
library are doing what you expect them to, separately and together.
-->

L'écriture de ces deux types de tests est importante pour s'assurer que chaque
élément de notre bibliothèque fait bien ce que vous attendez d'eux, de
manière isolée et conjuguée avec d'autres.

<!--
### Unit Tests
-->

### Les tests unitaires

<!--
The purpose of unit tests is to test each unit of code in isolation from the
rest of the code to quickly pinpoint where code is and isn’t working as
expected. You’ll put unit tests in the *src* directory in each file with the
code that they’re testing. The convention is to create a module named `tests`
in each file to contain the test functions and to annotate the module with
`cfg(test)`.
-->

Le but des tests unitaires est de tester chaque élément du code de manière
séparée du reste du code pour identifier rapidement où le code fonctionne
ou non comme prévu. Vous devriez insérer les tests unitaires dans le
répertoire *src* dans chaque fichier, à côté du code qu'ils testent. La convention
est de créer un module `tests` dans chaque fichier qui contient les fonctions
de test et de marquer le module avec `cfg(test)`.

<!--
#### The Tests Module and `#[cfg(test)]`
-->

#### Les modules de tests et `#[cfg(test)]`

<!--
The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run
the test code only when you run `cargo test`, not when you run `cargo build`.
This saves compile time when you only want to build the library and saves space
in the resulting compiled artifact because the tests are not included. You’ll
see that because integration tests go in a different directory, they don’t need
the `#[cfg(test)]` annotation. However, because unit tests go in the same files
as the code, you’ll use `#[cfg(test)]` to specify that they shouldn’t be
included in the compiled result.
-->

L'annotation `#[cfg(test)]` sur les modules de tests indique à Rust de
compiler et d'exécuter le code de test seulement lorsque vous lancez
`cargo test`, et non pas lorsque vous lancez `cargo build`. Cela diminue la
durée de compilation lorsque vous souhaitez uniquement compiler la bibliothèque
et cela réduit la taille dans l'artéfact compilé qui en résulte car les tests
n'y sont pas intégrés. Vous verrez plus tard que comme les tests d'intégration
se placent dans un répertoire différent, ils n'ont pas besoin de l'annotation
`#[cfg(test)]`. Cependant, comme les tests unitaires vont dans les mêmes
fichiers que le code, vous devriez utiliser `#[cfg(test)]` pour marquer qu'ils
ne devraient pas être inclus dans les résultats de compilation.

<!--
Recall that when we generated the new `adder` project in the first section of
this chapter, Cargo generated this code for us:
-->

Souvenez-vous, lorsque nous avons généré le nouveau projet `addition` dans la
première section de ce chapitre, Cargo a généré ce code pour nous :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

<!--
This code is the automatically generated test module. The attribute `cfg`
stands for *configuration* and tells Rust that the following item should only
be included given a certain configuration option. In this case, the
configuration option is `test`, which is provided by Rust for compiling and
running tests. By using the `cfg` attribute, Cargo compiles our test code only
if we actively run the tests with `cargo test`. This includes any helper
functions that might be within this module, in addition to the functions
annotated with `#[test]`.
-->

Ce code est le module de test généré automatiquement. L'attribut `cfg` est
l'abréviation de *configuration* et indique à Rust que l'élément suivant ne
doit être intégré que lorsqu'une certaine option de configuration est donnée. Dans
ce cas, l'option de configuration est `test`, qui est fournie par Rust pour la
compilation et l'exécution des tests. En utilisant l'attribut `cfg`, Cargo
compile notre code de tests uniquement si nous avons exécuté les tests avec
`cargo test`. Cela inclut toutes les fonctions auxiliaires qui pourraient se
trouver dans ce module, en plus des fonctions marquées d'un `#[test]`.

<!--
#### Testing Private Functions
-->

#### Tester des fonctions privées

<!--
There’s debate within the testing community about whether or not private
functions should be tested directly, and other languages make it difficult or
impossible to test private functions. Regardless of which testing ideology you
adhere to, Rust’s privacy rules do allow you to test private functions.
Consider the code in Listing 11-12 with the private function `internal_adder`.
-->

Il existe un débat dans la communauté des testeurs au sujet de la nécessité ou non
de tester directement les fonctions privées, et d'autres langages rendent
difficile, voire impossible, de tester les fonctions privées. Quelle que soit
votre approche des tests, les règles de protection de Rust vous permettent de
tester des fonctions privées. Imaginons le code de l'encart 11-12 qui contient
la fonction privée `addition_interne`.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-12: Testing a private function</span>
-->

<span class="caption">Encart 11-12 : Test d'une fonction privée</span>

<!--
Note that the `internal_adder` function is not marked as `pub`. Tests are just
Rust code, and the `tests` module is just another module. As we discussed in
the [“Paths for Referring to an Item in the Module Tree”][paths]<!-- ignore -- >
section, items in child modules can use the items in their ancestor modules. In
this test, we bring all of the `test` module’s parent’s items into scope with
`use super::*`, and then the test can call `internal_adder`. If you don’t think
private functions should be tested, there’s nothing in Rust that will compel
you to do so.
-->

Remarquez que la fonction `addition_interne` n'est pas marquée comme `pub`. Les
tests sont uniquement du code Rust, et le module `test` est simplement un autre
module. Comme nous l'avons vu dans [la section "Désigner un élément dans
l'arborescence de modules"][paths]<!-- ignore -->, les
éléments dans les modules enfants peuvent utiliser les éléments dans leurs
modules parents. Dans ce test, nous importons dans la portée tous les éléments
du parent du module `test` grâce à `use super::*;`, permettant ensuite au test
de faire appel à `addition_interne`. Si vous pensez qu'une fonction privée ne
doit pas être testée, il n'y a rien qui vous y force avec Rust.

<!--
### Integration Tests
-->

### Les tests d'intégration

<!--
In Rust, integration tests are entirely external to your library. They use your
library in the same way any other code would, which means they can only call
functions that are part of your library’s public API. Their purpose is to test
whether many parts of your library work together correctly. Units of code that
work correctly on their own could have problems when integrated, so test
coverage of the integrated code is important as well. To create integration
tests, you first need a *tests* directory.
-->

En Rust, les tests d'intégration sont exclusivement externes à votre
bibliothèque. Ils consomment votre bibliothèque de la même manière que n'importe
quel autre code, ce qui signifie qu'ils ne peuvent appeler que les
fonctions qui font partie de l'interface de programmation applicative (API)
publique de votre bibliothèque. Leur but est de tester si les multiples parties
de votre bibliothèque fonctionnent correctement ensemble. Les portions de code
qui fonctionnent bien toutes seules pourraient rencontrer des problèmes une fois
imbriquées avec d'autres, donc les tests qui couvrent l'intégration du code sont
tout aussi importants. Pour créer des tests d'intégration, vous avez d'abord besoin
d'un répertoire *tests*.

<!--
#### The *tests* Directory
-->

#### Le répertoire *tests*

<!--
We create a *tests* directory at the top level of our project directory, next
to *src*. Cargo knows to look for integration test files in this directory. We
can then make as many test files as we want to in this directory, and Cargo
will compile each of the files as an individual crate.
-->

Nous créons un répertoire *tests* au niveau le plus haut de notre répertoire projet,
juste à côté de *src*. Cargo sait qu'il doit rechercher les fichiers de test
d'intégration dans ce répertoire. Nous pouvons ensuite construire autant de
fichiers de test que nous le souhaitons dans ce répertoire, et Cargo va compiler
chacun de ces fichiers comme une crate individuelle.

<!--
Let’s create an integration test. With the code in Listing 11-12 still in the
*src/lib.rs* file, make a *tests* directory, create a new file named
*tests/integration_test.rs*, and enter the code in Listing 11-13.
-->

Commençons à créer un test d'intégration. Avec le code de l'encart 11-12
toujours présent dans le fichier *src/lib.rs*, créez un répertoire *tests*, puis
un nouveau fichier *tests/test_integration.rs* et insérez-y le code de l'encart
11-13.

<!--
<span class="filename">Filename: tests/integration_test.rs</span>
-->

<span class="filename">Fichier : tests/test_integration.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

<!--
<span class="caption">Listing 11-13: An integration test of a function in the
`adder` crate</span>
-->

<span class="caption">Encart 11-13 : Un test d'intégration d'une fonction
présente dans la crate `addition`</span>

<!--
We’ve added `use adder` at the top of the code, which we didn’t need in the
unit tests. The reason is that each file in the `tests` directory is a separate
crate, so we need to bring our library into each test crate’s scope.
-->

Nous avons ajouté `use addition` en haut du code, ce que nous n'avions pas
besoin de faire dans les tests unitaires. La raison à cela est que chaque
fichier dans le répertoire `tests` est une crate séparée, donc nous devons importer
notre bibliothèque dans la portée de chaque crate de test.

<!--
We don’t need to annotate any code in *tests/integration_test.rs* with
`#[cfg(test)]`. Cargo treats the `tests` directory specially and compiles files
in this directory only when we run `cargo test`. Run `cargo test` now:
-->

Nous n'avons pas besoin de marquer du code avec `#[cfg(test)]` dans
*tests/test_integration.rs*. Cargo traite le répertoire `tests` de manière
particulière et compile les fichiers présents dans ce répertoire uniquement si nous
lançons `cargo test`. Lancez dès maintenant `cargo test` :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

<!--
The three sections of output include the unit tests, the integration test, and
the doc tests. The first section for the unit tests is the same as we’ve been
seeing: one line for each unit test (one named `internal` that we added in
Listing 11-12) and then a summary line for the unit tests.
-->

Les trois sections de la sortie concernent les tests unitaires, les tests
d'intégration et les tests de documentation. La première section relative aux
tests unitaires est la même que celle que nous avons déjà vue : une ligne pour
chaque test unitaire (celui qui s'appelle `interne` que nous avons inséré dans
l'encart 11-12) suivie d'une ligne de résumé des tests unitaires.

<!--
The integration tests section starts with the line `Running
target/debug/deps/integration_test-1082c4b063a8fbe6` (the hash at the end of
your output will be different). Next, there is a line for each test function in
that integration test and a summary line for the results of the integration
test just before the `Doc-tests adder` section starts.
-->

La section des tests d'intégration commence avec la ligne `Running
target/debug/deps/test_integration-1082c4b063a8fbe6` (le hachage à la fin de
votre sortie pourrait être différent). Ensuite, il y a une ligne pour chaque
fonction de test présente dans ce test d'intégration et une ligne de résumé pour
les résultats des tests d'intégration, juste avant que la section
`Doc-tests addition` ne commence.

<!--
Similarly to how adding more unit test functions adds more result lines to the
unit tests section, adding more test functions to the integration test file
adds more result lines to this integration test file’s section. Each
integration test file has its own section, so if we add more files in the
*tests* directory, there will be more integration test sections.
-->

De la même façon que plus vous ajoutiez de fonctions de tests unitaires et plus
vous aviez de lignes de résultats dans la section des tests unitaires, plus
vous ajoutez des fonctions de tests aux fichiers de tests d'intégration et plus
vous obtenez de lignes de résultat dans la section correspondant aux fichiers
des tests d'intégration. Chaque fichier de test d'intégration a sa propre section,
donc si nous ajoutons plus de fichiers dans le répertoire *tests*, il y aura plus
de sections de tests d'intégration.

<!--
We can still run a particular integration test function by specifying the test
function’s name as an argument to `cargo test`. To run all the tests in a
particular integration test file, use the `--test` argument of `cargo test`
followed by the name of the file:
-->

Nous pouvons aussi exécuter une fonction de test d'intégration précise en
utilisant le nom de la fonction de test comme argument à `cargo test`. Pour
exécuter tous les tests d'un fichier de tests d'intégration précis, utilisez
l'argument `--test` de `cargo test` suivi du nom du fichier :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

<!--
This command runs only the tests in the *tests/integration_test.rs* file.
-->

Cette commande exécute seulement les tests dans le fichier
*tests/test_integration.rs*.

<!--
#### Submodules in Integration Tests
-->

#### Les sous-modules des tests d'intégration

<!--
As you add more integration tests, you might want to make more than one file in
the *tests* directory to help organize them; for example, you can group the
test functions by the functionality they’re testing. As mentioned earlier, each
file in the *tests* directory is compiled as its own separate crate.
-->

Au fur et à mesure que vous ajouterez des tests d'intégration, vous pourriez
avoir besoin de les diviser en plusieurs fichiers dans le répertoire *tests* pour
vous aider à les organiser ; par exemple, vous pouvez regrouper les fonctions
de test par fonctionnalités qu'elles testent. Comme mentionné précédemment,
chaque fichier dans le répertoire *tests* est compilé comme étant sa propre crate
séparée de toutes les autres.

<!--
Treating each integration test file as its own crate is useful to create
separate scopes that are more like the way end users will be using your crate.
However, this means files in the *tests* directory don’t share the same
behavior as files in *src* do, as you learned in Chapter 7 regarding how to
separate code into modules and files.
-->

Le fait que chaque fichier de test d'intégration soit sa propre crate est utile
pour créer des portées séparées qui ressemblent à la manière dont les
développeurs vont consommer votre crate. Cependant, cela veut aussi dire que
les fichiers dans le répertoire *tests* ne partagent pas le même comportement que
les fichiers dans *src*, comme vous l'avez appris au chapitre 7 à
propos de la manière de séparer le code dans des modules et des fichiers.

<!--
The different behavior of files in the *tests* directory is most noticeable
when you have a set of helper functions that would be useful in multiple
integration test files and you try to follow the steps in the [“Separating
Modules into Different Files”][separating-modules-into-files]<!-- ignore -- >
section of Chapter 7 to extract them into a common module. For example, if we
create *tests/common.rs* and place a function named `setup` in it, we can add
some code to `setup` that we want to call from multiple test functions in
multiple test files:
-->

Ce comportement différent des fichiers dans le répertoire *tests* est encore plus
notable lorsque vous avez un jeu de fonctions d'aide qui s'avèrent utiles
pour plusieurs fichiers de test d'intégration et que vous essayez de suivre les
étapes de la section [“Séparer les modules dans différents
fichiers”][separating-modules-into-files]<!-- ignore --> du chapitre 7 afin de
les extraire dans un module en commun. Par exemple, si nous créons
*tests/commun.rs* et que nous y plaçons une fonction `parametrage` à
l'intérieur, nous pourrions ajouter du code à `parametrage` que nous voudrions
appeler à partir de différentes fonctions de test dans différents fichiers de
test :

<!--
<span class="filename">Filename: tests/common.rs</span>
-->

<span class="filename">Fichier : tests/commun.rs</span>

<!--
```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

<!--
When we run the tests again, we’ll see a new section in the test output for the
*common.rs* file, even though this file doesn’t contain any test functions nor
did we call the `setup` function from anywhere:
-->

Lorsque nous lançons les tests à nouveau, nous allons voir une nouvelle section
dans la sortie des tests, correspondant au fichier *commun.rs*, même si ce
fichier ne contient aucune fonction de test et que nous n'avons utilisé nulle
part la fonction `parametrage` :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

<!--
Having `common` appear in the test results with `running 0 tests` displayed for
it is not what we wanted. We just wanted to share some code with the other
integration test files.
-->

Nous ne voulons pas que `commun` apparaisse dans les résultats, ni que cela
affiche `running 0 tests`. Nous voulons juste partager du code avec les autres
fichiers de test d'intégration.

<!--
To avoid having `common` appear in the test output, instead of creating
*tests/common.rs*, we’ll create *tests/common/mod.rs*. This is an alternate
naming convention that Rust also understands. Naming the file this way tells
Rust not to treat the `common` module as an integration test file. When we move
the `setup` function code into *tests/common/mod.rs* and delete the
*tests/common.rs* file, the section in the test output will no longer appear.
Files in subdirectories of the *tests* directory don’t get compiled as separate
crates or have sections in the test output.
-->

Pour éviter que `commun` s'affiche sur la sortie de test, au lieu de créer le
fichier *tests/commun.rs*, nous allons créer *tests/commun/mod.rs*. C'est
une convention de nommage alternative que Rust comprend aussi. Nommer le
fichier ainsi indique à Rust de ne pas traiter le module `commun` comme un
fichier de test d'intégration. Lorsque nous déplaçons le code de la fonction
`parametrage` dans *tests/commun/mod.rs* et que nous supprimons le fichier
*tests/commun.rs*, la section dans la sortie des tests ne va plus s'afficher.
Les fichiers dans les sous-répertoires du répertoire *tests* ne seront pas
compilés comme étant une crate séparée et n'auront pas de sections dans la
sortie des tests.

<!--
After we’ve created *tests/common/mod.rs*, we can use it from any of the
integration test files as a module. Here’s an example of calling the `setup`
function from the `it_adds_two` test in *tests/integration_test.rs*:
-->

Après avoir créé *tests/commun/mod.rs*, nous pouvons l'utiliser à partir de
n'importe quel fichier de test d'intégration comme un module. Voici un
exemple d'appel à la fonction `parametrage` à partir du test
`cela_ajoute_deux` dans *tests/test_integration.rs* :

<!--
<span class="filename">Filename: tests/integration_test.rs</span>
-->

<span class="filename">Fichier : tests/integration_test.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

<!--
Note that the `mod common;` declaration is the same as the module declaration
we demonstrated in Listing 7-21. Then in the test function, we can call the
`common::setup()` function.
-->

Remarquez que la déclaration `mod commun;` est la même que la déclaration d'un
module que nous avons montrée dans l'encart 7-21. Ensuite, dans la fonction
de tests, nous pouvons appeler la fonction `commun::parametrage`.

<!--
#### Integration Tests for Binary Crates
-->

#### Tests d'intégration pour les crates binaires

<!--
If our project is a binary crate that only contains a *src/main.rs* file and
doesn’t have a *src/lib.rs* file, we can’t create integration tests in the
*tests* directory and bring functions defined in the *src/main.rs* file into
scope with a `use` statement. Only library crates expose functions that other
crates can use; binary crates are meant to be run on their own.
-->

Si notre projet est une crate binaire qui contient uniquement un fichier
*src/main.rs* et n'a pas de fichier *src/lib.rs*, nous ne pouvons pas créer
de tests d'intégration dans le répertoire *tests* et importer les fonctions
définies dans le fichier *src/main.rs* dans notre portée avec une instruction
`use`. Seules les crates de bibliothèque exposent des fonctions que les autres
crates peuvent utiliser ; les crates binaires sont conçues pour être exécutées
de manière isolée.

<!--
This is one of the reasons Rust projects that provide a binary have a
straightforward *src/main.rs* file that calls logic that lives in the
*src/lib.rs* file. Using that structure, integration tests *can* test the
library crate with `use` to make the important functionality available.
If the important functionality works, the small amount of code in the
*src/main.rs* file will work as well, and that small amount of code doesn’t
need to be tested.
-->

C'est une des raisons pour lesquelles les projets Rust qui fournissent un
binaire ont un simple fichier *src/main.rs* qui fait appel à la logique
présente dans le fichier *src/lib.rs*. En utilisant cette structure, les tests
d'intégration *peuvent* tester la crate de bibliothèque avec le `use` pour
pouvoir accéder à la fonctionnalité importante. Si la fonctionnalité
importante fonctionne, la petite portion de code dans le fichier
*src/main.rs* va fonctionner, et cette petite partie de code n'a pas besoin
d'être testée.

<!--
## Summary
-->

## Résumé

<!--
Rust’s testing features provide a way to specify how code should function to
ensure it continues to work as you expect, even as you make changes. Unit tests
exercise different parts of a library separately and can test private
implementation details. Integration tests check that many parts of the library
work together correctly, and they use the library’s public API to test the code
in the same way external code will use it. Even though Rust’s type system and
ownership rules help prevent some kinds of bugs, tests are still important to
reduce logic bugs having to do with how your code is expected to behave.
-->

Les fonctionnalités de test de Rust permettent de spécifier comment le code
doit fonctionner pour garantir qu'il va continuer à fonctionner comme vous le
souhaitez, même si vous faites des changements. Les tests unitaires permettent
de tester séparément différentes parties d'une bibliothèque et peuvent tester
l'implémentation des éléments privés. Les tests d'intégration vérifient que de
nombreuses parties de la bibliothèque fonctionnent correctement ensemble, et
ils utilisent l'API publique de la bibliothèque pour tester le code, de la même
manière  que le ferait du code externe qui l'utiliserait. Même si le système de
type de Rust et les règles de possession aident à empêcher certains types de
bogues, les tests restent toujours importants pour réduire les bogues de
logique concernant le comportement attendu de votre code.

<!--
Let’s combine the knowledge you learned in this chapter and in previous
chapters to work on a project!
-->

Et maintenant, combinons le savoir que vous avez accumulé dans ce chapitre et
dans les chapitres précédents en travaillant sur un nouveau projet !

<!--
[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]:
ch07-05-separating-modules-into-different-files.html
-->

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]:
ch07-05-separating-modules-into-different-files.html
