<!--
## Packages and Crates
-->

## Les paquets et les crates

<!--
The first parts of the module system we’ll cover are packages and crates. A
crate is a binary or library. The *crate root* is a source file that the Rust
compiler starts from and makes up the root module of your crate (we’ll explain
modules in depth in the [“Defining Modules to Control Scope and
Privacy”][modules]<!-- ignore -- > section). A *package* is one or more crates
that provide a set of functionality. A package contains a *Cargo.toml* file
that describes how to build those crates.
-->

La première partie du système de modules que nous allons aborder concerne les
paquets et les *crates*. Une crate est un binaire ou une bibliothèque. Pour la
compiler, le compilateur Rust part d'un fichier source, la racine de la *crate*,
à partir duquel est alors créé le *module racine* de votre *crate* (nous verrons
les modules plus en détail dans la [section suivante][modules]<!-- ignore -->).
Un *paquet* se compose d'une ou plusieurs crates qui fournissent un ensemble de
fonctionnalités. Un paquet contient un fichier *Cargo.toml* qui décrit comment
construire ces crates.

<!--
Several rules determine what a package can contain. A package can contain
at most one library crate. It can contain as many binary crates
as you’d like, but it must contain at least one crate (either library or
binary).
-->

Il y a plusieurs règles qui déterminent ce qu'un paquet peut contenir. Il *doit*
contenir au maximum une seule crate de bibliothèque. Il peut contenir autant de
crates binaires que vous le souhaitez, mais il doit contenir au moins une crate
(que ce soit une bibliothèque ou un binaire).

<!--
Let’s walk through what happens when we create a package. First, we enter the
command `cargo new`:
-->

Découvrons ce qui se passe quand nous créons un paquet. D'abord, nous utilisons
la commande `cargo new` :

<!--
```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```
-->

```console
$ cargo new mon-projet
     Created binary (application) `mon-projet` package
$ ls mon-projet
Cargo.toml
src
$ ls mon-projet/src
main.rs
```

<!--
When we entered the command, Cargo created a *Cargo.toml* file, giving us a
package. Looking at the contents of *Cargo.toml*, there’s no mention of
*src/main.rs* because Cargo follows a convention that *src/main.rs* is the
crate root of a binary crate with the same name as the package. Likewise, Cargo
knows that if the package directory contains *src/lib.rs*, the package contains
a library crate with the same name as the package, and *src/lib.rs* is its
crate root. Cargo passes the crate root files to `rustc` to build the library
or binary.
-->

Lorsque nous avons saisi la commande, Cargo a créé un fichier *Cargo.toml*, qui
définit un paquet. Si on regarde le contenu de *Cargo.toml*, le fichier
*src/main.rs* n'est pas mentionné car Cargo obéit à une convention selon
laquelle *src/main.rs* est la racine de la crate binaire portant le même
nom que le paquet. De la même façon, Cargo sait que si le répertoire du paquet
contient *src/lib.rs*, alors le paquet contient une crate de bibliothèque qui a
le même nom que le paquet, et que *src/lib.rs* est sa racine. Cargo transmet les
fichiers de la crate racine à `rustc` pour compiler la bibliothèque ou le
binaire.

<!--
Here, we have a package that only contains *src/main.rs*, meaning it only
contains a binary crate named `my-project`. If a package contains *src/main.rs*
and *src/lib.rs*, it has two crates: a binary and a library, both with the same
name as the package. A package can have multiple binary crates by placing files
in the *src/bin* directory: each file will be a separate binary crate.
-->

Dans notre cas, nous avons un paquet qui contient uniquement *src/main.rs*, ce
qui veut dire qu'il contient uniquement une crate binaire qui s'appelle
`mon-projet`. Si un paquet contient *src/main.rs* et *src/lib.rs*, il a deux
crates : une binaire et une bibliothèque, chacune avec le même nom que le
paquet. Un paquet peut avoir plusieurs crates binaires en ajoutant des fichiers
dans le répertoire *src/bin* : chaque fichier sera une crate séparée.

<!--
A crate will group related functionality together in a scope so the
functionality is easy to share between multiple projects. For example, the
`rand` crate we used in [Chapter 2][rand]<!-- ignore -- > provides functionality
that generates random numbers. We can use that functionality in our own
projects by bringing the `rand` crate into our project’s scope. All the
functionality provided by the `rand` crate is accessible through the crate’s
name, `rand`.
-->

Une crate regroupe plusieurs fonctionnalités associées ensemble dans une portée
afin que les fonctionnalités soient faciles à partager entre plusieurs projets.
Par exemple, la crate `rand` que nous avons utilisée dans
[le chapitre 2][rand]<!-- ignore --> nous permet de générer des nombres
aléatoires. Nous pouvons utiliser cette fonctionnalité dans notre propre projet
en important la crate `rand` dans la portée de notre projet. Toutes les
fonctionnalités fournies par la crate `rand` sont accessibles via le nom de la
crate, `rand`.

<!--
Keeping a crate’s functionality in its own scope clarifies whether particular
functionality is defined in our crate or the `rand` crate and prevents
potential conflicts. For example, the `rand` crate provides a trait named
`Rng`. We can also define a `struct` named `Rng` in our own crate. Because a
crate’s functionality is namespaced in its own scope, when we add `rand` as a
dependency, the compiler isn’t confused about what the name `Rng` refers to. In
our crate, it refers to the `struct Rng` that we defined. We would access the
`Rng` trait from the `rand` crate as `rand::Rng`.
-->

Ranger une fonctionnalité d'une crate dans sa propre portée clarifie si une
fonctionnalité précise est définie dans notre crate ou dans la crate `rand` et
évite ainsi de potentiels conflits. Par exemple, la crate `rand` fournit un
*trait* qui s'appelle `Rng`. Nous pouvons nous aussi définir une structure qui
s'appelle `Rng` dans notre propre crate. Comme les fonctionnalités des crates
sont dans la portée de leur propre espace de nom, quand nous ajoutons `rand` en
dépendance, il n'y a pas d'ambiguïté pour le compilateur sur le nom `Rng`. Dans
notre crate, il se réfère au `struct Rng` que nous avons défini. Nous accédons
au *trait* `Rng` de la crate `rand` via `rand::Rng`.

<!--
Let’s move on and talk about the module system!
-->

Poursuivons et parlons maintenant du système de modules !

<!--
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
-->

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#générer-le-nombre-secret
