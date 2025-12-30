<!--
## Cargo Workspaces
-->

## Les espaces de travail de cargo

<!--
In Chapter 12, we built a package that included a binary crate and a library
crate. As your project develops, you might find that the library crate
continues to get bigger and you want to split up your package further into
multiple library crates. In this situation, Cargo offers a feature called
*workspaces* that can help manage multiple related packages that are developed
in tandem.
-->

Dans le chapitre 12, nous avons construit un paquet qui comprenait une crate
binaire et une crate de bibliothèque. Au fur et à mesure que votre projet se
développe, vous pourrez constater que la crate de bibliothèque continue de
s'agrandir et vous voudriez alors peut-être diviser votre paquet en plusieurs
crates de bibliothèque. Pour cette situation, cargo a une fonctionnalité qui
s'appelle *les espaces de travail* qui peuvent aider à gérer plusieurs paquets
liés qui sont développés en tandem.

<!--
### Creating a Workspace
-->

### Créer un espace de travail

<!--
A *workspace* is a set of packages that share the same *Cargo.lock* and output
directory. Let’s make a project using a workspace—we’ll use trivial code so we
can concentrate on the structure of the workspace. There are multiple ways to
structure a workspace; we’re going to show one common way. We’ll have a
workspace containing a binary and two libraries. The binary, which will provide
the main functionality, will depend on the two libraries. One library will
provide an `add_one` function, and a second library an `add_two` function.
These three crates will be part of the same workspace. We’ll start by creating
a new directory for the workspace:
-->

Un *espace de travail* est un jeu de paquets qui partagent tous le même
*Cargo.lock* et le même dossier de sortie. Créons donc un projet en utilisant un
espace de travail — nous allons utiliser du code trivial afin de nous concentrer
sur la structure de l'espace de travail. Il existe plusieurs façons de
structurer un espace de travail ; nous allons vous montrer une manière commune
d'organisation. Nous allons avoir un espace de travail contenant un binaire et
deux bibliothèques. Le binaire, qui devrait fournir les fonctionnalités
principales, va dépendre des deux bibliothèques. Une bibliothèque va fournir une
fonction `ajouter_un`, et la seconde bibliothèque, une fonction `ajouter_deux`.
Ces trois crates feront partie du même espace de travail. Nous allons commencer
par créer un nouveau dossier pour cet espace de travail :

<!--
```console
$ mkdir add
$ cd add
```
-->

```console
$ mkdir ajout
$ cd ajout
```

<!--
Next, in the *add* directory, we create the *Cargo.toml* file that will
configure the entire workspace. This file won’t have a `[package]` section or
the metadata we’ve seen in other *Cargo.toml* files. Instead, it will start
with a `[workspace]` section that will allow us to add members to the workspace
by specifying the path to the package with our binary crate; in this case,
that path is *adder*:
-->

Ensuite, dans le dossier *ajout*, nous créons le fichier *Cargo.toml* qui va
configurer l'intégralité de l'espace de travail. Ce fichier n'aura pas de
section `[package]` ou les métadonnées que nous avons vues dans les autres
fichiers *Cargo.toml*. À la place, il commencera par une section `[workspace]`
qui va nous permettre d'ajouter des membres à l'espace de travail en
renseignant le chemin vers le paquet qui contient notre crate binaire ; dans ce
cas, ce chemin est *additioneur* :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

<!--
```toml
{{#include ../listings-sources/ch14-more-about-cargo/no-listing-01-workspace-with-adder-crate/add/Cargo.toml}}
```
-->

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace-with-adder-crate/ajout/Cargo.toml}}
```

<!--
Next, we’ll create the `adder` binary crate by running `cargo new` within the
*add* directory:
-->

Ensuite, nous allons créer la crate binaire `additioneur` en lançant `cargo new`
dans le dossier *ajout* :

<!--
<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
rm -rf adder
cargo new adder
copy output below
-- >
-->

<!--
```console
$ cargo new adder
     Created binary (application) `adder` package
```
-->

```console
$ cargo new additioneur
     Created binary (application) `additioneur` package
```

<!--
At this point, we can build the workspace by running `cargo build`. The files
in your *add* directory should look like this:
-->

À partir de ce moment, nous pouvons compiler l'espace de travail en lançant
`cargo build`. Les fichiers dans votre dossier *ajout* devraient ressembler à
ceci :

<!--
```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```
-->

```text
├── Cargo.lock
├── Cargo.toml
├── additioneur
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

<!--
The workspace has one *target* directory at the top level for the compiled
artifacts to be placed into; the `adder` package doesn’t have its own *target*
directory. Even if we were to run `cargo build` from inside the *adder*
directory, the compiled artifacts would still end up in *add/target* rather
than *add/adder/target*. Cargo structures the *target* directory in a workspace
like this because the crates in a workspace are meant to depend on each other.
If each crate had its own *target* directory, each crate would have to
recompile each of the other crates in the workspace to have the artifacts in
its own *target* directory. By sharing one *target* directory, the crates can
avoid unnecessary rebuilding.
-->

L'espace de travail a un dossier *target* au niveau le plus haut pour y placer
les artefacts compilés ; le paquet `additioneur` n'a pas son propre dossier
*target*. Même si nous lancions `cargo build` à l'intérieur du dossier
*additioneur*, les artefacts compilés finirons toujours dans *ajout/target*
plutôt que dans *ajout/additioneur/target*. Cargo organise ainsi le dossier
*target* car les crates d'un espace de travail sont censés dépendre l'une de
l'autre. Si chaque crate avait son propre dossier *target*, chaque crate
devrait recompiler chacune des autres crates présentes dans l'espace de
travail pour avoir les artefacts dans son propre dossier *target*. En
partageant un seul dossier *target*, les crates peuvent éviter des
re-compilations inutiles.

<!--
### Creating the Second Package in the Workspace
-->

### Créer le second paquet dans l'espace de travail

<!--
Next, let’s create another member package in the workspace and call it `add_one`.
Change the top-level *Cargo.toml* to specify the *add_one* path in the
`members` list:
-->

Ensuite, créons un autre paquet, membre de l'espace de travail et appelons-le
`ajouter_un`. Changeons le *Cargo.toml* du niveau le plus haut pour renseigner
le chemin vers *ajouter_un* dans la liste `members` :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

<!--
```toml
{{#include ../listings-sources/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```
-->

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/ajout/Cargo.toml}}
```

<!--
Then generate a new library crate named `add_one`:
-->

Ensuite, générons une nouvelle crate de bibliothèque `ajouter_un` :

<!--
<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
rm -rf add_one
cargo new add_one --lib
copy output below
-- >
-->

<!--
```console
$ cargo new add_one --lib
     Created library `add_one` package
```
-->

```console
$ cargo new ajouter_un --lib
     Created library `ajouter_un` package
```

<!--
Your *add* directory should now have these directories and files:
-->

Votre dossier *ajout* devrait maintenant avoir ces dossiers et fichiers :

<!--
```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```
-->

```text
├── Cargo.lock
├── Cargo.toml
├── ajouter_un
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── additioneur
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

<!--
In the *add_one/src/lib.rs* file, let’s add an `add_one` function:
-->

Dans le fichier *ajouter_un/src/lib.rs*, ajoutons une fonction `ajouter_un` :

<!--
<span class="filename">Filename: add_one/src/lib.rs</span>
-->

<span class="filename">Fichier : ajouter_un/src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/ajout/ajouter_un/src/lib.rs}}
```

<!--
Now that we have another package in the workspace, we can have the `adder`
package with our binary depend on the `add_one` package, that has our
library. First, we’ll need to add a path dependency on `add_one` to
*adder/Cargo.toml*.
-->

Maintenant que nous avons un autre paquet dans l'espace de travail, nous pouvons
faire en sorte que le paquet `additioneur` qui contient notre binaire dépende du
paquet `ajouter_un`, qui contient notre bibliothèque. D'abord, nous devons
ajouter un chemin de dépendance à `ajouter_un` dans *additioneur/Cargo.toml*.

<!--
<span class="filename">Filename: adder/Cargo.toml</span>
-->

<span class="filename">Fichier : additioneur/Cargo.toml</span>

<!--
```toml
{{#include ../listings-sources/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```
-->

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/ajout/additioneur/Cargo.toml:6:7}}
```

<!--
Cargo doesn’t assume that crates in a workspace will depend on each other, so
we need to be explicit about the dependency relationships between the crates.
-->

Cargo ne fait pas la supposition que les crates d'un espace de travail
dépendent l'une de l'autre, donc vous devez être explicites sur les relations
de dépendance entre les crates.

<!--
Next, let’s use the `add_one` function from the `add_one` crate in the `adder`
crate. Open the *adder/src/main.rs* file and add a `use` line at the top to
bring the new `add_one` library crate into scope. Then change the `main`
function to call the `add_one` function, as in Listing 14-7.
-->

Ensuite, utilisons la fonction `ajouter_un` de la crate `ajouter_un` dans la
crate `additioneur`. Ouvrez le fichier *additioneur/src/main.rs* et ajoutez une
ligne `use` tout en haut pour importer la bibliothèque `ajouter_un` dans la
portée. Changez ensuite la fonction `main` pour appeler la fonction
`ajouter_un`, comme dans l'encart 14-7.

<!--
<span class="filename">Filename: adder/src/main.rs</span>
-->

<span class="filename">Fichier : additioneur/src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/ajout/additioneur/src/main.rs}}
```

<!--
<span class="caption">Listing 14-7: Using the `add_one` library crate from the
 `adder` crate</span>
-->

<span class="caption">Encart 14-7 : utilisation de la bibliothèque `ajouter_un`
dans la crate `additioneur`</span>

<!--
Let’s build the workspace by running `cargo build` in the top-level *add*
directory!
-->

Compilons l'espace de travail en lançant `cargo build` dans le niveau le plus
haut du dossier *ajout* !

<!--
<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-- >
-->

<!--
```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```
-->

```console
$ cargo build
   Compiling ajouter_un v0.1.0 (file:///projects/ajout/ajouter_un)
   Compiling additioneur v0.1.0 (file:///projects/ajout/additioneur)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```

<!--
To run the binary crate from the *add* directory, we can specify which
package in the workspace we want to run by using the `-p` argument and the
package name with `cargo run`:
-->

Pour lancer la crate binaire à partir du dossier *ajout*, nous pouvons
préciser quel paquet nous souhaitons exécuter dans l'espace de travail en
utilisant l'argument `-p` suivi du nom du paquet avec `cargo run` :

<!--
<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-- >
-->

<!--
```console
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```
-->

```console
$ cargo run -p additioneur
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/additioneur`
Hello, world ! 10 plus un vaut 11 !
```

<!--
This runs the code in *adder/src/main.rs*, which depends on the `add_one` crate.
-->

Cela exécute le code de *additioneur/src/main.rs*, qui dépend de la crate
`ajouter_un`.

<!--
#### Depending on an External Package in a Workspace
-->

#### Dépendre d'un paquet externe dans un espace de travail

<!--
Notice that the workspace has only one *Cargo.lock* file at the top level of
the workspace rather than having a *Cargo.lock* in each crate’s directory. This
ensures that all crates are using the same version of all dependencies. If we
add the `rand` package to the *adder/Cargo.toml* and *add_one/Cargo.toml*
files, Cargo will resolve both of those to one version of `rand` and record
that in the one *Cargo.lock*. Making all crates in the workspace use the same
dependencies means the crates in the workspace will always be compatible with
each other. Let’s add the `rand` crate to the `[dependencies]` section in the
*add_one/Cargo.toml* file to be able to use the `rand` crate in the `add_one`
crate:
-->

Notez que l'espace de travail a un seul fichier *Cargo.lock* dans le niveau le
plus haut de l'espace de travail plutôt que d'avoir un *Cargo.lock* dans chaque
dossier de chaque crate. Cela garantit que toutes les crates utilisent la même
version de toutes les dépendances. Si nous ajoutons le paquet `rand` aux
fichiers *additioneur/Cargo.toml* et *ajouter_un/Cargo.toml*, cargo va réunir
les deux en une seule version de `rand` et enregistrer cela dans un seul
*Cargo.lock*. Faire en sorte que toutes les crates de l'espace de travail
utilisent la même dépendance signifie que les crates dans l'espace de travail
seront toujours compatibles l'une avec l'autre. Ajoutons la crate `rand` à la
section `[dependencies]` du fichier *ajouter_un/Cargo.toml* pour pouvoir
utiliser la crate `rand` dans la crate `ajouter_un` :

<!--
<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-- >
-->

<!--
<span class="filename">Filename: add_one/Cargo.toml</span>
-->

<span class="filename">Fichier : ajouter_un/Cargo.toml</span>

<!--
```toml
{{#include ../listings-sources/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```
-->

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/ajout/ajouter_un/Cargo.toml:6:7}}
```

<!--
We can now add `use rand;` to the *add_one/src/lib.rs* file, and building the
whole workspace by running `cargo build` in the *add* directory will bring in
and compile the `rand` crate. We will get one warning because we aren’t
referring to the `rand` we brought into scope:
-->

Nous pouvons maintenant ajouter `use rand;` au fichier *ajouter_un/src/lib.rs*
et compiler l'ensemble de l'espace de travail en lançant `cargo build` dans le
dossier *ajout*, ce qui va importer et compiler la crate `rand`. Nous devrions
avoir un avertissement car nous n'avons pas utilisé le `rand` que nous avons
introduit dans la portée :

<!--
<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-- >
-->

<!--
```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
   --snip--
   Compiling rand v0.8.3
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 -- > add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: 1 warning emitted

   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s
```
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
   -- partie masquée ici --
   Compiling rand v0.8.3
   Compiling ajouter_un v0.1.0 (file:///projects/ajout/ajouter_un)
warning: unused import: `rand`
 --> ajouter_un/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: 1 warning emitted

   Compiling additioneur v0.1.0 (file:///projects/ajout/additioneur)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s
```

<!--
The top-level *Cargo.lock* now contains information about the dependency of
`add_one` on `rand`. However, even though `rand` is used somewhere in the
workspace, we can’t use it in other crates in the workspace unless we add
`rand` to their *Cargo.toml* files as well. For example, if we add `use rand;`
to the *adder/src/main.rs* file for the `adder` package, we’ll get an error:
-->

Le *Cargo.lock* du niveau le plus haut contient maintenant les informations
de dépendance à `rand` pour `ajouter_un`. Cependant, même si `rand` est
utilisé quelque part dans l'espace de travail, nous ne pouvons pas l'utiliser
dans d'autres crates de l'espace de travail tant que nous n'ajoutons pas
`rand` dans leurs fichiers *Cargo.toml*. Par exemple, si nous ajoutons
`use rand;` dans le fichier *additioneur/src/main.rs* pour le paquet
`additioneur`, nous allons avoir une erreur :

<!--
<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-- >
-->

<!--
```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 -- > adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```
-->

```console
$ cargo build
  -- partie masquée ici --
   Compiling additioneur v0.1.0 (file:///projects/ajout/additioneur)
error[E0432]: unresolved import `rand`
 --> additioneur/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

<!--
To fix this, edit the *Cargo.toml* file for the `adder` package and indicate
that `rand` is a dependency for it as well. Building the `adder` package will
add `rand` to the list of dependencies for `adder` in *Cargo.lock*, but no
additional copies of `rand` will be downloaded. Cargo has ensured that every
crate in every package in the workspace using the `rand` package will be
using the same version. Using the same version of `rand` across the workspace
saves space because we won’t have multiple copies and ensures that the crates
in the workspace will be compatible with each other.
-->

Pour corriger cela, modifiez le fichier *Cargo.toml* pour le paquet
`additioneur` et indiquez que `rand` est une dépendance de cette crate aussi.
La compilation du paquet `additioneur` va rajouter `rand` à la liste des
dépendances pour `additioneur` dans *Cargo.lock*, mais aucune copie
supplémentaire de `rand` ne sera téléchargée. Cargo s'est assuré que toutes les
crates de chaque paquet de l'espace de travail qui utilise le paquet `rand`
seraient de la même version. Utiliser la même version de `rand` dans les
espaces de travail économise de l'espace car nous n'avons pas à multiplier les
copies, ni à nous assurer que les crates dans l'espace de travail sont
compatibles les unes avec les autres.

<!--
#### Adding a Test to a Workspace
-->

#### Ajouter un test à l'espace de travail

<!--
For another enhancement, let’s add a test of the `add_one::add_one` function
within the `add_one` crate:
-->

Afin de procéder à une autre amélioration, ajoutons un test de la fonction
`ajouter_un::ajouter_un` dans la crate `ajouter_un` :

<!--
<span class="filename">Filename: add_one/src/lib.rs</span>
-->

<span class="filename">Fichier : add_one/src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/ajout/ajouter_un/src/lib.rs}}
```

<!--
Now run `cargo test` in the top-level *add* directory:
-->

Lancez maintenant `cargo test` dans le niveau le plus haut du
dossier *ajout* :

<!--
<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in paths properly
-- >
-->

<!--
```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running target/debug/deps/add_one-f0253159197f7841

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/debug/deps/adder-49979ff40686fa8e

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
-->

```console
$ cargo test
   Compiling ajouter_un v0.1.0 (file:///projects/ajout/ajouter_un)
   Compiling additioneur v0.1.0 (file:///projects/ajout/additioneur)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running target/debug/deps/ajouter_un-f0253159197f7841

running 1 test
test tests::cela_fonctionne ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/debug/deps/additioneur-49979ff40686fa8e

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ajouter_un

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

<!--
The first section of the output shows that the `it_works` test in the `add_one`
crate passed. The next section shows that zero tests were found in the `adder`
crate, and then the last section shows zero documentation tests were found in
the `add_one` crate. Running `cargo test` in a workspace structured like this
one will run the tests for all the crates in the workspace.
-->

La première section de la sortie indique que le test `cela_fonctionne` de la
crate `ajouter_un` a réussi. La section suivante indique qu'aucun test n'a été
trouvé dans la crate `additioneur`, puis la dernière section indique elle
aussi qu'aucun test de documentation n'a été trouvé dans la crate `ajouter_un`.
Lancer `cargo test` dans un espace de travail structuré comme celui-ci va
exécuter les tests pour toutes les crates de cet espace de travail.

<!--
We can also run tests for one particular crate in a workspace from the
top-level directory by using the `-p` flag and specifying the name of the crate
we want to test:
-->

Nous pouvons aussi lancer des tests pour une crate en particulier dans un
espace de travail à partir du dossier du plus haut niveau en utilisant le
drapeau `-p` et en renseignant le nom de la crate que nous voulons tester :

<!--
<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-- >
-->

<!--
```console
$ cargo test -p add_one
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running target/debug/deps/add_one-b3235fea9a156f74

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
-->

```console
$ cargo test -p ajouter_un
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running target/debug/deps/ajouter_un-b3235fea9a156f74

running 1 test
test tests::cela_fonctionne ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ajouter_un

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

<!--
This output shows `cargo test` only ran the tests for the `add_one` crate and
didn’t run the `adder` crate tests.
-->

Cette sortie montre que `cargo test` a lancé les tests uniquement pour la
crate `ajouter_un` et n'a pas lancé les tests de la crate `additioneur`.

<!--
If you publish the crates in the workspace to [crates.io](https://crates.io/),
each crate in the workspace will need to be published separately. The `cargo
publish` command does not have an `--all` flag or a `-p` flag, so you must
change to each crate’s directory and run `cargo publish` on each crate in the
workspace to publish the crates.
-->

Si vous publiez les crates présentes dans l'espace de travail sur
[crates.io](https://crates.io/), chaque crate de l'espace de travail va avoir
besoin d'être publiée de manière séparée. La commande `cargo publish` n'a pas
de drapeau `--all` ou `-p`, donc vous devrez vous rendre dans chaque dossier de
chaque crate et lancer `cargo publish` sur chaque crate présente dans l'espace
de travail pour publier les crates.

<!--
For additional practice, add an `add_two` crate to this workspace in a similar
way as the `add_one` crate!
-->

En guise d'entrainement supplémentaire, ajoutez une crate `ajouter_deux` dans
cet espace de travail de la même manière que nous l'avons fait pour la crate
`ajouter_un` !

<!--
As your project grows, consider using a workspace: it’s easier to understand
smaller, individual components than one big blob of code. Furthermore, keeping
the crates in a workspace can make coordination between them easier if they are
often changed at the same time.
-->

Au fur et à mesure que votre projet se développe, pensez à utiliser un espace
de travail : il est plus facile de comprendre des composants individuels, plus
petits, plutôt qu'un gros tas de code. De plus, garder les crates dans un
espace de travail peut améliorer la coordination entre elles si elles sont souvent
modifiées ensemble.
