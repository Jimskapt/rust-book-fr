<!--
## Controlling How Tests Are Run
-->

## Gérer l'exécution des tests

<!--
Just as `cargo run` compiles your code and then runs the resulting binary,
`cargo test` compiles your code in test mode and runs the resulting test
binary. You can specify command line options to change the default behavior of
`cargo test`. For example, the default behavior of the binary produced by
`cargo test` is to run all the tests in parallel and capture output generated
during test runs, preventing the output from being displayed and making it
easier to read the output related to the test results.
-->

Comme `cargo run` qui compile votre code et qui exécute ensuite le binaire qui
en résulte, `cargo test` compile votre code en mode test et lance le binaire de
tests qu'il produit. Vous pouvez rajouter des options en ligne de commande pour
changer le comportement par défaut de `cargo test`. Par exemple, le
comportement par défaut des binaires produits par `cargo test` est de lancer
tous les tests en parallèle et de capturer la sortie pendant l'exécution des
tests, ce qui lui évite d'être affichée sur l'écran pendant ce temps,
facilitant la lecture des messages relatifs aux résultats de l'exécution des
tests.

<!--
Some command line options go to `cargo test`, and some go to the resulting test
binary. To separate these two types of arguments, you list the arguments that
go to `cargo test` followed by the separator `--` and then the ones that go to
the test binary. Running `cargo test --help` displays the options you can use
with `cargo test`, and running `cargo test -- --help` displays the options you
can use after the separator `--`.
-->

Certaines options de la ligne de commande s'appliquent à `cargo test`, et
certaines au binaire de tests qui en résulte. Pour séparer ces types
d'arguments, il faut lister les arguments qui s'appliquent à `cargo test`,
suivis du séparateur `--`, puis ajouter ceux qui s'appliquent au binaire
de tests. L'exécution de `cargo test --help` affiche les options que vous
pouvez utiliser sur `cargo test`, et l'exécution de `cargo test -- --help`
affiche les options que vous pouvez utiliser après le séparateur `--`.

<!--
### Running Tests in Parallel or Consecutively
-->

### Lancer les tests en parallèle ou en séquence

<!--
When you run multiple tests, by default they run in parallel using threads.
This means the tests will finish running faster so you can get feedback quicker
on whether or not your code is working. Because the tests are running at the
same time, make sure your tests don’t depend on each other or on any shared
state, including a shared environment, such as the current working directory or
environment variables.
-->

Lorsque vous lancez de nombreux tests, par défaut ils s'exécutent en parallèle
dans des tâches. Cela veut dire que tous les tests vont finir de s'exécuter plus
rapidement afin que vous sachiez si votre code fonctionne ou non. Comme les
tests s'exécutent en même temps, il faut s'assurer qu'ils ne dépendent
pas les uns des autres ou d'un état partagé, y compris un environnement partagé,
comme le répertoire de travail actuel ou des variables d'environnement.

<!--
For example, say each of your tests runs some code that creates a file on disk
named *test-output.txt* and writes some data to that file. Then each test reads
the data in that file and asserts that the file contains a particular value,
which is different in each test. Because the tests run at the same time, one
test might overwrite the file between when another test writes and reads the
file. The second test will then fail, not because the code is incorrect but
because the tests have interfered with each other while running in parallel.
One solution is to make sure each test writes to a different file; another
solution is to run the tests one at a time.
-->

Par exemple, disons que chacun de vos tests exécute du code qui crée un fichier
*test-sortie.txt* sur le disque dur et qu'il écrit quelques données dans ce
fichier. Ensuite, chaque test lit les données de ce fichier et vérifie que le
fichier contient une valeur précise, qui est différente dans chaque test. Comme
les tests sont lancés en même temps, un test risque d'écraser le contenu du
fichier entre le moment où un autre test lit et écrit sur ce fichier. Le second
test va ensuite échouer, non pas parce que le code est incorrect mais parce
que les tests se sont perturbés mutuellement pendant qu'ils s'exécutaient en
parallèle. Une solution serait de s'assurer que chaque test écrit dans un
fichier différent ; une autre serait de lancer les tests les uns après les autres.

<!--
If you don’t want to run the tests in parallel or if you want more fine-grained
control over the number of threads used, you can send the `--test-threads` flag
and the number of threads you want to use to the test binary. Take a look at
the following example:
-->

Si vous ne souhaitez pas exécuter les tests en parallèle ou si vous voulez un
contrôle plus précis du nombre de tâches utilisées, vous pouvez utiliser l'option
 `--test-threads` suivie du nombre de tâches que vous souhaitez
que le binaire de test exécute en parallèle. Regardez cet exemple :

<!--
```console
$ cargo test -- --test-threads=1
```
-->

```console
$ cargo test -- --test-threads=1
```

<!--
We set the number of test threads to `1`, telling the program not to use any
parallelism. Running the tests using one thread will take longer than running
them in parallel, but the tests won’t interfere with each other if they share
state.
-->

Nous avons réglé le nombre de tâches à `1`, ce qui indique au programme de ne
pas utiliser le parallélisme. Exécuter ces tests en n'effectuant qu'une seule
tâche à la fois va prendre plus de temps que de les lancer en parallèle, mais
cela assure que les tests ne vont pas s'influencer mutuellement s'ils partagent
le même état.

<!--
### Showing Function Output
-->

### Afficher la sortie de la fonction

<!--
By default, if a test passes, Rust’s test library captures anything printed to
standard output. For example, if we call `println!` in a test and the test
passes, we won’t see the `println!` output in the terminal; we’ll see only the
line that indicates the test passed. If a test fails, we’ll see whatever was
printed to standard output with the rest of the failure message.
-->

Par défaut, si un test réussit, la bibliothèque de test de Rust récupère tout
ce qui est affiché sur la sortie standard. Par exemple, si nous appelons
`println!` dans un test et que le test réussit, nous ne verrons pas la sortie
correspondant au `println!` dans le terminal ; on verra seulement la ligne qui
indique que le test a réussi. Si un test échoue, nous verrons ce qui a été
affiché sur la sortie standard avec le reste des messages d'erreur.

<!--
As an example, Listing 11-10 has a silly function that prints the value of its
parameter and returns 10, as well as a test that passes and a test that fails.
-->

Par exemple, l'encart 11-10 a une fonction stupide qui affiche la valeur de ses
paramètres et retourne 10, ainsi qu'un test qui réussit et un test qui échoue.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```
-->

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-10: Tests for a function that calls
`println!`</span>
-->

<span class="caption">Encart 11-10 : Tests d'une fonction qui fait appel à
`println!`</span>

<!--
When we run these tests with `cargo test`, we’ll see the following output:
-->

Lorsque nous lançons ces tests avec `cargo test`, nous voyons cette sortie :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

<!--
Note that nowhere in this output do we see `I got the value 4`, which is what
is printed when the test that passes runs. That output has been captured. The
output from the test that failed, `I got the value 8`, appears in the section
of the test summary output, which also shows the cause of the test failure.
-->

Remarquez que nous n'avons jamais vu `J'ai obtenu la valeur 4` dans cette
sortie, qui est ce qui est affiché lors de l'exécution du test qui réussit.
Cette sortie a été capturée. La sortie pour le test qui a échoué,
`J'ai obtenu la valeur 8`, s'affiche dans la section de la sortie
correspondante au résumé des tests, qui affiche aussi les causes de l'échec
du test.

<!--
If we want to see printed values for passing tests as well, we can tell Rust
to also show the output of successful tests at the end with `--show-output`.
-->

Si nous voulons aussi voir les valeurs affichées pour les tests réussis, nous
pouvons demander à Rust d'afficher également la sortie des tests fructueux en
lui rajoutant à la fin `--show-output`.

<!--
```console
$ cargo test -- --show-output
```
-->

```console
$ cargo test -- --show-output
```

<!--
When we run the tests in Listing 11-10 again with the `--show-output` flag, we
see the following output:
-->

Lorsque nous lançons à nouveau les tests de l'encart 11-10 avec l'option
`--show-output`, nous voyons la sortie suivante :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

<!--
### Running a Subset of Tests by Name
-->

### Exécuter un sous-ensemble de tests en fonction de son nom

<!--
Sometimes, running a full test suite can take a long time. If you’re working on
code in a particular area, you might want to run only the tests pertaining to
that code. You can choose which tests to run by passing `cargo test` the name
or names of the test(s) you want to run as an argument.
-->

Parfois, lancer une suite de tests entière peut prendre beaucoup de temps. Si
vous travaillez sur du code d'un périmètre bien défini, vous pourriez avoir
besoin d'exécuter uniquement les tests relatifs à ce code. Vous pouvez choisir
quels tests exécuter en envoyant le ou les noms du ou des tests que vous souhaitez
exécuter en argument de `cargo test`.

<!--
To demonstrate how to run a subset of tests, we’ll create three tests for our
`add_two` function, as shown in Listing 11-11, and choose which ones to run.
-->

Dans le but de démontrer comment lancer un sous-ensemble de tests, nous allons
créer trois tests pour notre fonction `ajouter_deux` dans l'encart 11-11, et
choisir lesquels nous allons exécuter.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-11: Three tests with three different
names</span>
-->

<span class="caption">Encart 11-11 : Trois tests avec trois noms différents
</span>

<!--
If we run the tests without passing any arguments, as we saw earlier, all the
tests will run in parallel:
-->

Si nous exécutons les tests sans ajouter d'arguments, comme nous l'avons vu
précédemment, tous les tests vont s'exécuter en parallèle :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

<!--
#### Running Single Tests
-->

#### Exécuter des tests individuellement

<!--
We can pass the name of any test function to `cargo test` to run only that test:
-->

Nous pouvons donner le nom de n'importe quelle fonction de test à `cargo test`
afin d'exécuter uniquement ce test :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

<!--
Only the test with the name `one_hundred` ran; the other two tests didn’t match
that name. The test output lets us know we had more tests than what this
command ran by displaying `2 filtered out` at the end of the summary line.
-->

Le test avec le nom `cent` est le seul exécuté ; les deux autres tests ne
correspondent pas à ce nom. La sortie du test nous indique que nous avons
d'autres tests en plus de celui que cette commande a exécuté en affichant
`2 filtered out` à la fin de la ligne de résumé.

<!--
We can’t specify the names of multiple tests in this way; only the first value
given to `cargo test` will be used. But there is a way to run multiple tests.
-->

Nous ne pouvons pas renseigner plusieurs noms de tests de cette manière ; il
n'y a que la première valeur fournie à `cargo test` qui sera utilisée. Mais
il existe un moyen d'exécuter plusieurs tests.

<!--
#### Filtering to Run Multiple Tests
-->

#### Filtrer pour exécuter plusieurs tests

<!--
We can specify part of a test name, and any test whose name matches that value
will be run. For example, because two of our tests’ names contain `add`, we can
run those two by running `cargo test add`:
-->

Nous pouvons ne renseigner qu'une partie d'un nom de test, et tous les tests dont
les noms correspondent à cette valeur vont être exécutés. Par exemple, comme
deux de nos noms de tests contiennent `ajouter`, nous pouvons exécuter ces deux
en lançant `cargo test ajouter` :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

<!--
This command ran all tests with `add` in the name and filtered out the test
named `one_hundred`. Also note that the module in which a test appears becomes
part of the test’s name, so we can run all the tests in a module by filtering
on the module’s name.
-->

Cette commande a lancé tous les tests qui contiennent `ajouter` dans leur nom
et a filtré le test `cent`. Notez aussi que le module dans lequel un test est
présent fait partie du nom du test, ainsi nous pouvons exécuter tous les tests
d'un module en filtrant avec le nom du module.

<!--
### Ignoring Some Tests Unless Specifically Requested
-->

### Ignorer certains tests sauf s'ils sont demandés explicitement

<!--
Sometimes a few specific tests can be very time-consuming to execute, so you
might want to exclude them during most runs of `cargo test`. Rather than
listing as arguments all tests you do want to run, you can instead annotate the
time-consuming tests using the `ignore` attribute to exclude them, as shown
here:
-->

Parfois, certains tests spécifiques peuvent prendre beaucoup de temps à
s'exécuter, de sorte que vous voulez les exclure de la majorité des exécutions
de `cargo test`. Plutôt que de lister en argument tous les tests que vous
souhaitez exécuter, vous pouvez plutôt faire une annotation sur les tests qui
prennent du temps en utilisant l'attribut `ignore` pour les exclure, comme
ci-dessous :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs}}
```

<!--
After `#[test]` we add the `#[ignore]` line to the test we want to exclude. Now
when we run our tests, `it_works` runs, but `expensive_test` doesn’t:
-->

Après `#[test]`, nous avons ajouté la ligne `#[ignore]` pour le test que nous
souhaitons exclure. Maintenant lorsque nous exécutons nos tests, `it_works`
s'exécute, mais pas `test_long` :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

<!--
The `expensive_test` function is listed as `ignored`. If we want to run only
the ignored tests, we can use `cargo test -- --ignored`:
-->

La fonction `test_long` est listée comme `ignored`. Si nous voulons exécuter
uniquement les tests ignorés, nous pouvons utiliser `cargo test -- --ignored` :

<!--
```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```
-->

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

<!--
By controlling which tests run, you can make sure your `cargo test` results
will be fast. When you’re at a point where it makes sense to check the results
of the `ignored` tests and you have time to wait for the results, you can run
`cargo test -- --ignored` instead. If you want to run all tests whether they’re
ignored or not, you can run `cargo test -- --include-ignored`.
-->

En gérant quels tests sont exécutés, vous pouvez vous assurer que vos résultats
de `cargo test` seront rapides. Lorsque vous arrivez à un stade où il est
justifié de vérifier le résultat des tests `ignored` et que vous avez le temps
d'attendre ces résultats, vous pouvez lancer à la place
`cargo test -- --ignored`. Si vous voulez exécuter tous les tests, qu'ils soient
ignorés ou non, vous pouvez lancer `cargo test -- --include-ignored`.
