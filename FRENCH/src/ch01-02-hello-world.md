<!--
## Hello, World!
-->

## Hello, World!

<!--
Now that you’ve installed Rust, let’s write your first Rust program. It’s
traditional when learning a new language to write a little program that prints
the text `Hello, world!` to the screen, so we’ll do the same here!
-->

Maintenant que vous avez installé Rust, écrivons notre premier programme Rust.
Lorsqu'on apprend un nouveau langage, il est de tradition d'écrire un petit
programme qui écrit le texte "Hello, world!" à l'écran, donc c'est ce que nous
allons faire !

<!--
> Note: This book assumes basic familiarity with the command line. Rust makes
> no specific demands about your editing or tooling or where your code lives, so
> if you prefer to use an integrated development environment (IDE) instead of
> the command line, feel free to use your favorite IDE. Many IDEs now have some
> degree of Rust support; check the IDE’s documentation for details.
> The Rust team has been focusing on enabling great
> IDE support via `rust-analyzer`. See [Appendix D][devtools]
> for more details.
-->

> Note : ce livre part du principe que vous êtes familier avec la ligne de
> commande. Rust n'impose pas d'exigences sur votre éditeur, vos outils ou
> l'endroit où vous mettez votre code, donc si vous préférez utiliser un
> environnement de développement intégré (IDE) au lieu de la ligne de commande,
> vous êtes libre d'utiliser votre IDE favori. De nombreux IDE prennent en
> charge Rust à des degrés divers ; consultez la documentation de
> l'IDE pour plus d'informations. L'équipe Rust s'est attelée à
> améliorer l'intégration dans les IDE via `rust-analyzer`. Voir
> l'[annexe D][devtools] pour plus de détails.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-project-directory"></a>

<!--
### Project Directory Setup
-->

### Créer un répertoire projet

<!--
You’ll start by making a directory to store your Rust code. It doesn’t matter
to Rust where your code lives, but for the exercises and projects in this book,
we suggest making a *projects* directory in your home directory and keeping all
your projects there.
-->

Nous allons commencer par créer un répertoire pour y ranger le code Rust. Là où
vous mettez votre code n'est pas important pour Rust, mais pour les exercices et
projets de ce livre, nous vous suggérons de créer un répertoire _projects_ dans
votre répertoire utilisateur et de ranger tous vos projets là-dedans.

<!--
Open a terminal and enter the following commands to make a *projects* directory
and a directory for the “Hello, world!” project within the *projects* directory.
-->

Ouvrez un terminal et écrivez les commandes suivantes pour créer un
répertoire _projects_ et un répertoire pour le projet “Hello, world!” à l'intérieur
de ce répertoire _projects_.

<!--
For Linux, macOS, and PowerShell on Windows, enter this:
-->

Sous Linux, macOS et PowerShell sous Windows, écrivez ceci :

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

<!--
For Windows CMD, enter this:
-->

Avec CMD sous Windows, écrivez ceci :

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-and-running-a-rust-program"></a>

<!--
### Rust Program Basics
-->

### Les bases d'un programme Rust

<!--
Next, make a new source file and call it *main.rs*. Rust files always end with
the *.rs* extension. If you’re using more than one word in your filename, the
convention is to use an underscore to separate them. For example, use
_hello_world.rs_ rather than _helloworld.rs_.
-->

Ensuite, créez un nouveau fichier source et appelez-le _main.rs_. Les fichiers
Rust se terminent toujours par l'extension _.rs_. Si vous utilisez plusieurs
mots dans votre nom de fichier, la convention est d'utiliser un tiret bas (`_`)
pour les séparer. Par exemple, vous devriez utiliser _hello_world.rs_ au lieu de
_helloworld.rs_.

<!--
Now open the *main.rs* file you just created and enter the code in Listing 1-1.
-->

Maintenant, ouvrez le fichier _main.rs_ que vous venez de créer et entrez le
code de l'encart 1-1.

<!--
<span class="filename">Filename: main.rs</span>
-->

<span class="filename">Fichier : main.rs</span>

<!--
```rust
fn main() {
    println!("Hello, world!");
}
```
-->

```rust
fn main() {
    println!("Hello, world!");
}
```

<!--
<span class="caption">Listing 1-1: A program that prints `Hello, world!`</span>
-->

<span class="caption">Encart 1-1 : Un programme qui affiche `Hello, world!`
</span>

<!--
Save the file and go back to your terminal window in the
_~/projects/hello_world_ directory. On Linux or macOS, enter the following
commands to compile and run the file:
-->

Enregistrez le fichier et retournez dans votre terminal dans le répertoire
_~/projects/hello_world_. Sur Linux ou macOS, écrivez les commandes suivantes
pour compiler et exécuter le fichier :

```console
$ rustc main.rs
$ ./main
Hello, world!
```

<!--
TODO there's a misteack in the upstream English version: make a PR to correct it
On Windows, enter the command `.\main` instead of `./main`:
=> add missing .exe extension:
On Windows, enter the command `.\main.exe` instead of `./main`:

TODO Also correct the powershell terminal copy below in the same PR
-->

Sur Windows, écrivez la commande `.\main.exe` à la place de `./main` :

```powershell
> rustc main.rs
> .\main.exe
Hello, world!
```

<!--
Regardless of your operating system, the string `Hello, world!` should print to
the terminal. If you don’t see this output, refer back to the
[“Troubleshooting”][troubleshooting]<!-- ignore -- > part of the Installation
section for ways to get help.
-->

Peu importe votre système d'exploitation, la chaîne de caractères
`Hello, world!` devrait s'écrire dans votre terminal. Si cela ne s'affiche pas,
référez-vous à la partie ["dépannage"][troubleshooting]<!-- ignore --> du
chapitre d'installation pour vous aider.

<!--
If `Hello, world!` did print, congratulations! You’ve officially written a Rust
program. That makes you a Rust programmer—welcome!
-->

Si `Hello, world!` s'affiche, félicitations ! Vous avez officiellement écrit un
programme Rust. Cela fait de vous un développeur Rust — bienvenue !

<!-- Old headings. Do not remove or links may break. -->

<a id="anatomy-of-a-rust-program"></a>

<!--
### The Anatomy of a Rust Program
-->

### Structure d'un programme Rust

<!--
Let’s review this “Hello, world!” program in detail. Here’s the first piece of
the puzzle:
-->

Regardons en détail ce qui s'est passé dans votre programme “Hello, world!”.
Voici le premier morceau du puzzle :

```rust
fn main() {

}
```

<!--
These lines define a function named `main`. The `main` function is special: It
is always the first code that runs in every executable Rust program. Here, the
first line declares a function named `main` that has no parameters and returns
nothing. If there were parameters, they would go inside the parentheses (`()`).
-->

Ces lignes définissent une fonction dans Rust. La fonction `main` est
spéciale : c'est toujours le premier code qui est exécuté dans tous les
programmes en Rust. Ici, la première ligne déclare une fonction qui s'appelle
`main`, qui n'a pas de paramètre et qui ne retourne aucune valeur. S'il y avait
des paramètres, ils seraient placés entre les parenthèses (`()`).

<!--
The function body is wrapped in `{}`. Rust requires curly brackets around all
function bodies. It’s good style to place the opening curly bracket on the same
line as the function declaration, adding one space in between.
-->

Le corps de la fonction est placé entre accolades `{}`. Rust a besoin d'avoir
ces accolades autour du corps de chaque fonction. C'est une
bonne pratique d'insérer l'accolade ouvrante sur la même ligne que la
déclaration de la fonction, en ajoutant une espace entre les deux.

<!--
> Note: If you want to stick to a standard style across Rust projects, you can
> use an automatic formatter tool called `rustfmt` to format your code in a
> particular style (more on `rustfmt` in
> [Appendix D][devtools]<!-- ignore -- >). The Rust team has included this tool
> with the standard Rust distribution, as `rustc` is, so it should already be
> installed on your computer!
-->

> Note : si vous souhaitez formater le code de tous vos projets Rust de manière
> standardisée, vous pouvez utiliser un outil de formatage automatique tel que
> `rustfmt` (pour en savoir plus sur `rustfmt`, voir 
> l'[annexe D][devtools]<!-- ignore -->). L'équipe de Rust a intégré cet outil
> dans la distribution standard de Rust, comme pour `rustc` par exemple, donc 
> il est probablement déjà installé sur votre ordinateur ! Consultez la
> documentation en ligne pour en savoir plus.

<!--
The body of the `main` function holds the following code:
-->

Le corps de la fonction `main` contient le code suivant :

```rust
    println!("Hello, world!");
```

<!--
This line does all the work in this little program: It prints text to the
screen. There are three important details to notice here.
-->

Cette ligne fait tout le travail dans ce petit programme : il écrit le texte à
l'écran. Il y a trois détails importants à noter ici.

<!--
# NOTE_FR_TRANSLATION_UPDATE:
# Update from the English version: paragraph suppressed, commented out:

First, Rust style is to indent with four spaces, not a tab.

Premièrement, le style de Rust est d'indenter avec quatre espaces, et non pas
avec une tabulation.
-->

<!--

First, `println!` calls a Rust macro. If it had called a function instead, it
would be entered as `println` (without the `!`). Rust macros are a way to write
code that generates code to extend Rust syntax, and we’ll discuss them in more
detail in [Chapter 20][ch20-macros]. For now, you just need to
know that using a `!` means that you’re calling a macro instead of a normal
function and that macros don’t always follow the same rules as functions.
-->

Premièmement, `println!` fait appel à une macro Rust. S'il avait appelé une
fonction à la place, cela aurait été écrit `println` (sans le `!`). Nous
aborderons les macros Rust plus en détail dans le chapitre 19. Pour l'instant,
vous avez juste à savoir qu'utiliser un `!` signifie que vous utilisez une
macro plutôt qu'une fonction classique. Les macros ne suivent pas toujours les
mêmes règles que les fonctions.

<!--
Second, you see the `"Hello, world!"` string. We pass this string as an argument
to `println!`, and the string is printed to the screen.
-->

Deuxièmement, vous voyez la chaîne de caractères `"Hello, world!"`. Nous
envoyons cette chaîne en argument à `println!` et cette chaîne est affichée
à l'écran.

<!--
Third, we end the line with a semicolon (`;`), which indicates that this
expression is over, and the next one is ready to begin. Most lines of Rust code
end with a semicolon.
-->

Troisièmement, nous terminons la ligne avec un point-virgule (`;`), qui indique
que cette expression est terminée et que la suivante est prête à commencer. La
plupart des lignes de Rust se terminent avec un point-virgule.

<!-- Old headings. Do not remove or links may break. -->
<a id="compiling-and-running-are-separate-steps"></a>

<!--
### Compilation and Execution
-->

### Compilation et exécution

<!--
You’ve just run a newly created program, so let’s examine each step in the
process.
-->

Vous venez de lancer un nouveau programme fraîchement créé, donc penchons-nous
sur chaque étape du processus.

<!--
Before running a Rust program, you must compile it using the Rust compiler by
entering the `rustc` command and passing it the name of your source file, like
this:
-->

Avant de lancer un programme Rust, vous devez le compiler en utilisant le
compilateur Rust en entrant la commande `rustc` et en lui passant le nom de
votre fichier source, comme ceci :

```console
$ rustc main.rs
```

<!--
If you have a C or C++ background, you’ll notice that this is similar to `gcc`
or `clang`. After compiling successfully, Rust outputs a binary executable.
-->

Si vous avez de l'expérience en C ou en C++, vous observerez des similarités
avec `gcc` ou `clang`.
Après avoir compilé avec succès, Rust produit un binaire exécutable.

<!--
On Linux, macOS, and PowerShell on Windows, you can see the executable by
entering the `ls` command in your shell:
-->

Avec Linux, macOS et PowerShell sous Windows, vous pouvez voir l'exécutable en
utilisant la commande `ls` dans votre terminal :

```console
$ ls
main  main.rs
```

<!--
On Linux and macOS, you’ll see two files. With PowerShell on Windows, you’ll
see the same three files that you would see using CMD.  With CMD on Windows, you
would enter the following:
-->

Avec Linux et macOS, vous devriez voir deux fichiers. Avec PowerShell sous
Windows, vous devriez voir les trois mêmes fichiers que vous verriez en
utilisant CMD.  Avec CMD sous Windows, vous devez saisir la commande suivante :

<!--
```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```
-->

```cmd
> dir /B %= l'option /B demande à n'afficher que les noms de fichiers =%
main.exe
main.pdb
main.rs
```

<!--
This shows the source code file with the *.rs* extension, the executable file
(*main.exe* on Windows, but *main* on all other platforms), and, when using
Windows, a file containing debugging information with the *.pdb* extension.
From here, you run the *main* or *main.exe* file, like this:
-->

Ceci affiche le fichier de code source avec l'extension _.rs_, le fichier
exécutable (_main.exe_ sous Windows, mais _main_ sur toutes les autres
plateformes) et, quand on utilise Windows, un fichier qui contient des
informations de débogage avec l'extension _.pdb_. Dans ce répertoire, vous pouvez
exécuter le fichier _main_ ou _main.exe_ comme ceci :

<!--
```console
$ ./main # or .\main.exe on Windows
```
-->

```console
$ ./main # ou .\main.exe sous Windows
```

<!--
If your _main.rs_ is your “Hello, world!” program, this line prints `Hello,
world!` to your terminal.
-->

Si _main.rs_ est votre programme “Hello, world!”, cette ligne affiche
`Hello, world!` dans votre terminal.

<!--
If you’re more familiar with a dynamic language, such as Ruby, Python, or
JavaScript, you might not be used to compiling and running a program as
separate steps. Rust is an _ahead-of-time compiled_ language, meaning you can
compile a program and give the executable to someone else, and they can run it
even without having Rust installed. If you give someone a _.rb_, _.py_, or
_.js_ file, they need to have a Ruby, Python, or JavaScript implementation
installed (respectively). But in those languages, you only need one command to
compile and run your program. Everything is a trade-off in language design.
-->

Si vous connaissez un langage dynamique, comme Ruby, Python, ou JavaScript, vous
n'avez peut-être pas l'habitude de compiler puis lancer votre programme dans des
étapes séparées.
Rust est un langage à _compilation anticipée_, ce qui veut dire que
vous pouvez compiler le programme et le donner à quelqu'un d'autre, et il peut
l'exécuter sans avoir Rust d'installé. Si vous donnez à quelqu'un un fichier
_.rb_, _.py_ ou _.js_, il a besoin d'avoir respectivement un interpréteur Ruby,
Python, ou Javascript d'installé. Cependant, avec ces langages, vous n'avez
besoin que d'une seule commande pour compiler et exécuter votre programme.
Dans la conception d'un langage, tout est une question de compromis.

<!--
Just compiling with `rustc` is fine for simple programs, but as your project
grows, you’ll want to manage all the options and make it easy to share your
code. Next, we’ll introduce you to the Cargo tool, which will help you write
real-world Rust programs.
-->

Compiler avec `rustc` peut suffire pour de petits programmes, mais au fur et à
mesure que votre programme grandit, vous allez avoir besoin de régler plus
d'options et faciliter le partage de votre code. À la page suivante, nous allons
découvrir l'outil Cargo, qui va vous aider à écrire des programmes Rust à
l'épreuve de la réalité.

<!--
[troubleshooting]: ch01-01-installation.html#troubleshooting
-->

[troubleshooting]: ch01-01-installation.html#dépannage
[devtools]: appendix-04-useful-development-tools.html
