<!--
## Hello, Cargo!
-->

## Hello, Cargo!

<!--
Cargo is Rust’s build system and package manager. Most Rustaceans use this tool
to manage their Rust projects because Cargo handles a lot of tasks for you,
such as building your code, downloading the libraries your code depends on, and
building those libraries. (We call libraries your code needs *dependencies*.)
-->

Cargo est le système de compilation et de gestion de paquets de Rust. La plupart
des Rustacés utilisent cet outil pour gérer les projets Rust, car Cargo
s'occupe de nombreuses tâches pour vous, comme compiler votre code, télécharger
les bibliothèques dont votre code dépend, et compiler ces bibliothèques. (On
appelle *dépendance* une bibliothèque nécessaire pour votre code.)

<!--
The simplest Rust programs, like the one we’ve written so far, don’t have any
dependencies. So if we had built the “Hello, world!” project with Cargo, it
would only use the part of Cargo that handles building your code. As you write
more complex Rust programs, you’ll add dependencies, and if you start a project
using Cargo, adding dependencies will be much easier to do.
-->

Des programmes Rust très simples, comme le petit que nous avons précédemment,
n'ont pas de dépendance. Donc si nous avions compilé le projet “Hello, world!”
avec Cargo, cela n'aurait fait appel qu'à la fonctionnalité de Cargo qui
s'occupe de la compilation de votre code. Quand vous écrirez des programmes Rust
plus complexes, vous ajouterez des dépendances, et si vous créez un projet en
utilisant Cargo, l'ajout des dépendances sera plus facile à faire.

<!--
Because the vast majority of Rust projects use Cargo, the rest of this book
assumes that you’re using Cargo too. Cargo comes installed with Rust if you
used the official installers discussed in the
[“Installation”][installation]<!-- ignore -- > section. If you installed Rust
through some other means, check whether Cargo is installed by entering the
following into your terminal:
-->

Comme la large majorité des projets Rust utilisent Cargo, la suite de ce livre
va supposer que vous utilisez aussi Cargo. Cargo s'installe avec Rust si vous
avez utilisé l'installateur officiel présenté dans la section
[“Installation”][installation]<!-- ignore -->. Si vous avez installé Rust
autrement, vérifiez que Cargo est installé en utilisant la commande suivante
dans votre terminal :

```text
$ cargo --version
```

<!--
If you see a version number, you have it! If you see an error, such as `command
not found`, look at the documentation for your method of installation to
determine how to install Cargo separately.
-->

Si vous voyez un numéro de version, c'est qu'il est installé ! Si vous voyez une
erreur comme `Commande non trouvée` (ou `command not found`), alors consultez la
documentation de votre méthode d'installation pour savoir comment installer
séparément Cargo.

<!--
### Creating a Project with Cargo
-->

### Créer un projet avec Cargo

<!--
Let’s create a new project using Cargo and look at how it differs from our
original “Hello, world!” project. Navigate back to your *projects* directory (or
wherever you decided to store your code). Then, on any operating system, run
the following:
-->

Créons un nouveau projet en utilisant Cargo et analysons les différences avec
notre projet initial “Hello, world!”. Retournez dans votre dossier *projects*
(ou là où vous avez décidé d'enregistrer votre code). Ensuite, sur n'importe
quel système d'exploitation, lancez les commandes suivantes :

```text
$ cargo new hello_cargo
$ cd hello_cargo
```

<!--
The first command creates a new directory called *hello_cargo*. We’ve named
our project *hello_cargo*, and Cargo creates its files in a directory of the
same name.
-->

La première commande crée un nouveau dossier appelé *hello_cargo*. Nous avons
appelé notre projet *hello_cargo*, et Cargo crée ses fichiers dans un dossier
avec le même nom.

<!--
Go into the *hello_cargo* directory and list the files. You’ll see that Cargo
has generated two files and one directory for us: a *Cargo.toml* file and a
*src* directory with a *main.rs* file inside. It has also initialized a new Git
repository along with a *.gitignore* file.
-->

Rendez-vous dans le dossier *hello_cargo* et afficher la liste des fichiers.
Vous constaterez que Cargo a généré deux fichiers et un dossier pour nous : un
fichier *Cargo.toml* et un dossier *src* avec un fichier *main.rs* à
l'intérieur. Il a aussi créé un nouveau dépôt Git ainsi qu'un fichier
*.gitignore*.

<!--
> Note: Git is a common version control system. You can change `cargo new` to
> use a different version control system or no version control system by using
> the `--vcs` flag. Run `cargo new --help` to see the available options.
-->

> Note : Git est un système de gestion de versions très répandu. Vous pouvez
> changer `cargo new` pour utiliser un autre système de gestion de versions
> ou ne pas en utiliser du tout en écrivant le drapeau `--vcs`.
> Lancez `cargo new --help` pour en savoir plus sur les options disponibles.

<!--
Open *Cargo.toml* in your text editor of choice. It should look similar to the
code in Listing 1-2.
-->

Ouvrez *Cargo.toml* dans votre éditeur de texte favori. Son contenu devrait
être similaire au code dans l'encart 1-2.

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

<!--
<span class="caption">Listing 1-2: Contents of *Cargo.toml* generated by `cargo
new`</span>
-->

<span class="caption">Encart 1-2 : Contenu de *Cargo.toml* généré par `cargo
new`</span>

<!--
This file is in the [*TOML*][toml]<!-- ignore -- > (*Tom’s Obvious, Minimal
Language*) format, which is Cargo’s configuration format.
-->

Ce fichier est au format [*TOML*][toml]<!-- ignore --> (*Tom’s Obvious, Minimal
Language*), qui est le format de configuration de Cargo.

[toml]: https://github.com/toml-lang/toml

<!--
The first line, `[package]`, is a section heading that indicates that the
following statements are configuring a package. As we add more information to
this file, we’ll add other sections.
-->

La première ligne, `[package]`, est un en-tête de section qui indique que les
instructions suivantes configurent un paquet. Au fur et à mesure que nous
ajouterons plus de détails à ce fichier, nous ajouterons des sections
supplémentaires.

<!--
The next four lines set the configuration information Cargo needs to compile
your program: the name, the version, who wrote it, and the edition of Rust to
use. Cargo gets your name and email information from your environment, so if
that information is not correct, fix the information now and then save the
file. We’ll talk about the `edition` key in Appendix E.
-->

Les quatre lignes suivantes définissent les informations de configuration dont
Cargo a besoin pour compiler votre programme : le nom, la version, l'auteur et
l'édition de Rust à utiliser. Cargo récupère votre nom et votre courriel à
partir de votre environnement, donc si ces informations sont erronées, vous
pouvez les corriger dès maintenant et ensuite enregistrer le fichier. Nous
aborderons la clé `edition` dans l'annexe E.

<!--
The last line, `[dependencies]`, is the start of a section for you to list any
of your project’s dependencies. In Rust, packages of code are referred to as
*crates*. We won’t need any other crates for this project, but we will in the
first project in Chapter 2, so we’ll use this dependencies section then.
-->

La dernière ligne, `[dependencies]`, est le début d'une section qui vous permet
de lister les dépendances de votre projet. Dans Rust, les paquets de code sont
désignés sous le nom de *crates*. Nous n'allons pas utiliser de *crate* pour ce
projet, mais nous le ferons pour le premier projet au chapitre 2 ; nous
utiliserons alors cette section à ce moment-là.

<!--
Now open *src/main.rs* and take a look:
-->

Maintenant, ouvrez *src/main.rs* et jetez-y un coup d'œil :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

<!--
Cargo has generated a “Hello, world!” program for you, just like the one we
wrote in Listing 1-1! So far, the differences between our previous project and
the project Cargo generates are that Cargo placed the code in the *src*
directory, and we have a *Cargo.toml* configuration file in the top directory.
-->

Cargo a généré un programme “Hello, world!” pour vous, exactement comme celui
que nous avons écrit dans l'encart 1-1 ! Pour le moment, les seules différences
entre notre projet précédent et le projet que Cargo a généré sont que Cargo a
placé le code dans le dossier *src*, et que nous avons un fichier de
configuration *Cargo.toml* à la racine du dossier projet.

<!--
Cargo expects your source files to live inside the *src* directory. The
top-level project directory is just for README files, license information,
configuration files, and anything else not related to your code. Using Cargo
helps you organize your projects. There’s a place for everything, and
everything is in its place.
-->

Cargo prévoit de stocker vos fichiers sources dans le dossier *src*. Le dossier
parent est là uniquement pour les fichiers README, pour les informations à
propos de la licence, pour les fichiers de configuration et tout ce qui n'est
pas directement relié à votre code. Utiliser Cargo vous aide à structurer vos
projets. Il y a un endroit pour tout, et tout est à sa place.

<!--
If you started a project that doesn’t use Cargo, as we did with the “Hello,
world!” project, you can convert it to a project that does use Cargo. Move the
project code into the *src* directory and create an appropriate *Cargo.toml*
file.
-->

Si vous commencez un projet sans utiliser Cargo, comme nous l'avons fait avec
le projet “Hello, world!”, vous pouvez le transformer en projet qui
utilise Cargo. Déplacez le code de votre projet dans un dossier *src* et créez
un fichier *Cargo.toml* adéquat.

<!--
### Building and Running a Cargo Project
-->

### Compiler et exécuter un projet Cargo

<!--
Now let’s look at what’s different when we build and run the “Hello, world!”
program with Cargo! From your *hello_cargo* directory, build your project by
entering the following command:
-->

Maintenant, regardons ce qu'il y a de différent quand nous compilons et
exécutons le programme “Hello, world!” avec Cargo ! À l'intérieur de votre
dossier *hello_cargo*, compilez votre projet en utilisant la commande suivante :

```text
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

<!--
This command creates an executable file in *target/debug/hello_cargo* (or
*target\debug\hello_cargo.exe* on Windows) rather than in your current
directory. You can run the executable with this command:
-->

Cette commande crée un fichier exécutable dans *target/debug/hello_cargo* (ou
*target\debug\hello_cargo.exe* sous Windows) plutôt que de le déposer dans votre
dossier courant. Vous pouvez lancer l'exécutable avec cette commande :

<!--
```text
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```
-->

```text
$ ./target/debug/hello_cargo # ou .\target\debug\hello_cargo.exe sous Windows
Hello, world!
```

<!--
If all goes well, `Hello, world!` should print to the terminal. Running `cargo
build` for the first time also causes Cargo to create a new file at the top
level: *Cargo.lock*. This file keeps track of the exact versions of
dependencies in your project. This project doesn’t have dependencies, so the
file is a bit sparse. You won’t ever need to change this file manually; Cargo
manages its contents for you.
-->

Si tout s'est bien passé, `Hello, world!` devrait s'afficher dans le terminal.
Lancer `cargo build` pour la première fois devrait aussi mener Cargo à créer
un nouveau fichier à la racine du dossier projet : *Cargo.lock*. Ce fichier
garde une trace des versions exactes des dépendances de votre
projet. Ce projet n'a pas de dépendance, donc le fichier est un peu vide. Vous
n'aurez jamais besoin de changer ce fichier manuellement ; Cargo va gérer son
contenu pour vous.

<!--
We just built a project with `cargo build` and ran it with
`./target/debug/hello_cargo`, but we can also use `cargo run` to compile the
code and then run the resulting executable all in one command:
-->

Nous venons de compiler un projet avec `cargo build` avant de l'exécuter avec
`./target/debug/hello_cargo`, mais nous pouvons aussi utiliser `cargo run` pour
compiler le code et ensuite lancer l'exécutable dans une seule et même
commande :

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

<!--
Notice that this time we didn’t see output indicating that Cargo was compiling
`hello_cargo`. Cargo figured out that the files hadn’t changed, so it just ran
the binary. If you had modified your source code, Cargo would have rebuilt the
project before running it, and you would have seen this output:
-->

Notez que cette fois-ci, nous ne voyons pas de messages indiquant que Cargo a
compilé `hello_cargo`. Cargo a détecté que les fichiers n'avaient pas changé,
donc il a juste exécuté le binaire. Si vous aviez modifié votre code source,
Cargo aurait recompilé le projet avant de le lancer, et vous auriez eu les
messages suivants :

```text
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

<!--
Cargo also provides a command called `cargo check`. This command quickly checks
your code to make sure it compiles but doesn’t produce an executable:
-->

Cargo fournit aussi une commande appelée `cargo check`. Elle vérifie rapidement
votre code pour s'assurer qu'il est compilable, mais ne produit pas
d'exécutable :

```text
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

<!--
Why would you not want an executable? Often, `cargo check` is much faster than
`cargo build`, because it skips the step of producing an executable. If you’re
continually checking your work while writing the code, using `cargo check` will
speed up the process! As such, many Rustaceans run `cargo check` periodically
as they write their program to make sure it compiles. Then they run `cargo
build` when they’re ready to use the executable.
-->

Dans quel cas n'aurions-nous pas besoin d'un exécutable ? Parfois, `cargo check`
est bien plus rapide que `cargo build`, car il saute l'étape de création de
l'exécutable. Si vous vérifiez votre travail continuellement pendant que vous
écrivez votre code, utiliser `cargo check` accélèrera le processus ! C'est
pourquoi de nombreux Rustacés utilisent périodiquement `cargo check` quand ils
écrivent leur programme afin de s'assurer qu'il compile. Ensuite, ils lancent
`cargo build` quand ils sont prêts à utiliser l'exécutable.

<!--
Let’s recap what we’ve learned so far about Cargo:
-->

Récapitulons ce que nous avons appris sur Cargo :

<!--
* We can build a project using `cargo build` or `cargo check`.
* We can build and run a project in one step using `cargo run`.
* Instead of saving the result of the build in the same directory as our code,
  Cargo stores it in the *target/debug* directory.
-->

* Nous pouvons compiler un projet en utilisant `cargo build` ou `cargo check`
* Nous pouvons compiler puis exécuter un projet en une seule fois en utilisant
  `cargo run`.
* Au lieu d'enregistrer le résultat de la compilation dans le même dossier que
  votre code, Cargo l'enregistre dans le dossier *target/debug*.

<!--
An additional advantage of using Cargo is that the commands are the same no
matter which operating system you’re working on. So, at this point, we’ll no
longer provide specific instructions for Linux and macOS versus Windows.
-->

Un autre avantage d'utiliser Cargo est que les commandes sont les mêmes peu
importe le système d'exploitation que vous utilisez.
Donc à partir de maintenant, nous n'allons plus faire d'opérations spécifiques
à Linux et macOS par rapport à Windows.

<!--
### Building for Release
-->

### Compiler pour diffuser

<!--
When your project is finally ready for release, you can use `cargo build
--release` to compile it with optimizations. This command will create an
executable in *target/release* instead of *target/debug*. The optimizations
make your Rust code run faster, but turning them on lengthens the time it takes
for your program to compile. This is why there are two different profiles: one
for development, when you want to rebuild quickly and often, and another for
building the final program you’ll give to a user that won’t be rebuilt
repeatedly and that will run as fast as possible. If you’re benchmarking your
code’s running time, be sure to run `cargo build --release` and benchmark with
the executable in *target/release*.
-->

Quand votre projet est finalement prêt à être diffusé, vous pouvez utiliser
`cargo build --release` pour le compiler en l'optimisant. Cette commande va
créer un exécutable dans *target/release* au lieu de *target/debug*. Ces
optimisations rendent votre code Rust plus rapide à exécuter, mais l'utiliser
rallonge le temps de compilation de votre programme. C'est pourquoi il y a deux
différents profils : un pour le développement, quand vous voulez recompiler
rapidement et souvent, et un autre pour compiler le programme final qui sera
livré à un utilisateur, qui n'aura pas besoin d'être recompilé à plusieurs
reprises et qui s'exécutera aussi vite que possible. Si vous évaluez le temps
d'exécution de votre code, assurez-vous de lancer `cargo build --release` et
d'utiliser l'exécutable dans *target/release* pour vos bancs de test.

<!--
### Cargo as Convention
-->

### Cargo comme convention

<!--
With simple projects, Cargo doesn’t provide a lot of value over just using
`rustc`, but it will prove its worth as your programs become more intricate.
With complex projects composed of multiple crates, it’s much easier to let
Cargo coordinate the build.
-->

Pour des projets simples, Cargo n'apporte pas grand-chose par rapport à `rustc`,
mais il vous montrera son intérêt au fur et à mesure
que vos programmes deviendront plus complexes. Avec des projets complexes
composés de plusieurs *crates*, il est plus facile de laisser Cargo prendre en
charge la coordination de la compilation.

<!--
Even though the `hello_cargo` project is simple, it now uses much of the real
tooling you’ll use in the rest of your Rust career. In fact, to work on any
existing projects, you can use the following commands to check out the code
using Git, change to that project’s directory, and build:
-->

Même si le projet `hello_cargo` est simple, il utilise maintenant une grande
partie de l'outillage que vous rencontrerez dans votre carrière avec Rust. En
effet, pour travailler sur n'importe quel projet Rust existant, vous n'avez
qu'à saisir les commandes suivantes pour télécharger le code avec Git, vous
déplacer dans le dossier projet et compiler :

<!--
```text
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```
-->

```text
$ git clone example.com/projet_quelconque
$ cd projet_quelconque
$ cargo build
```

<!--
For more information about Cargo, check out [its documentation].
-->

Pour plus d'informations à propos de Cargo, vous pouvez consulter [sa
documentation][its documentation].

[its documentation]: https://doc.rust-lang.org/cargo/

<!--
## Summary
-->

## Résumé

<!--
You’re already off to a great start on your Rust journey! In this chapter,
you’ve learned how to:
-->

Vous êtes déjà bien lancé dans votre périple avec Rust ! Dans ce chapitre, vous
avez appris comment :

<!--
* Install the latest stable version of Rust using `rustup`
* Update to a newer Rust version
* Open locally installed documentation
* Write and run a “Hello, world!” program using `rustc` directly
* Create and run a new project using the conventions of Cargo
-->

* Installer la dernière version stable de Rust en utilisant `rustup`
* Mettre à jour Rust vers une nouvelle version
* Ouvrir la documentation installée en local
* Écrire et exécuter un programme “Hello, world!” en utilisant directement
  `rustc`
* Créer et exécuter un nouveau projet en utilisant les conventions de Cargo

<!--
This is a great time to build a more substantial program to get used to reading
and writing Rust code. So, in Chapter 2, we’ll build a guessing game program.
If you would rather start by learning how common programming concepts work in
Rust, see Chapter 3 and then return to Chapter 2.
-->

C'est le moment idéal pour construire un programme plus ambitieux pour
s'habituer à lire et écrire du code Rust. Donc, au chapitre 2, nous allons
écrire un programme de *jeu de devinettes*.
Si vous préférez commencer par apprendre comment les principes de programmation
de base fonctionnent avec Rust, rendez-vous au chapitre 3, puis revenez au
chapitre 2.

[installation]: ch01-01-installation.html#installation
