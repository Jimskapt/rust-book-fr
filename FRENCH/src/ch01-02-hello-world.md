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
allons faire !

<!--
> Note: This book assumes basic familiarity with the command line. Rust makes
> no specific demands about your editing or tooling or where your code lives, so
> if you prefer to use an integrated development environment (IDE) instead of
> the command line, feel free to use your favorite IDE. Many IDEs now have some
> degree of Rust support; check the IDE’s documentation for details. Recently,
> the Rust team has been focusing on enabling great IDE support, and progress
> has been made rapidly on that front!
-->

> Note : ce livre part du principe que vous êtes familier avec la ligne de
> commande. Rust n'impose pas d'exigences sur votre éditeur, vos outils ou
> l'endroit où vous mettez votre code, donc si vous préférez utiliser un
> environnement de développement intégré (IDE) au lieu de la ligne de commande,
> vous êtes libre d'utiliser votre IDE favori. De nombreux IDE prennent en
> charge Rust à des degrés divers ; consultez la documentation de
> l'IDE pour plus d'informations. Récemment, l'équipe Rust s'est attelée à
> améliorer l'intégration dans les IDE et des progrès ont rapidement été faits
> dans ce domaine !

<!--
### Creating a Project Directory
-->

### Créer un dossier projet

<!--
You’ll start by making a directory to store your Rust code. It doesn’t matter
to Rust where your code lives, but for the exercises and projects in this book,
we suggest making a *projects* directory in your home directory and keeping all
your projects there.
-->

Nous allons commencer par créer un dossier pour y ranger le code Rust. Là où
vous mettez votre code n'est pas important pour Rust, mais pour les exercices et
projets de ce livre, nous vous suggérons de créer un dossier *projects* dans
votre dossier utilisateur et de ranger tous vos projets là-dedans.

<!--
Open a terminal and enter the following commands to make a *projects* directory
and a directory for the “Hello, world!” project within the *projects* directory.
-->

Ouvrez un terminal et écrivez les commandes suivantes pour créer un
dossier *projects* et un dossier pour le projet “Hello, world!” à l'intérieur
de ce dossier *projects*.

<!--
For Linux, macOS, and PowerShell on Windows, enter this:
-->

Sous Linux, macOS et PowerShell sous Windows, écrivez ceci :

```text
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

<!--
### Writing and Running a Rust Program
-->

### Écrire et exécuter un programme Rust

<!--
Next, make a new source file and call it *main.rs*. Rust files always end with
the *.rs* extension. If you’re using more than one word in your filename, use
an underscore to separate them. For example, use *hello_world.rs* rather than
*helloworld.rs*.
-->

Ensuite, créez un nouveau fichier source et appelez-le *main.rs*. Les fichiers
Rust se terminent toujours par l'extension *.rs*. Si vous utilisez plusieurs
mots dans votre nom de fichier, utilisez un tiret bas (`_`) pour
les séparer. Par exemple, vous devriez utiliser *hello_world.rs* au lieu de
*helloworld.rs*.

<!--
Now open the *main.rs* file you just created and enter the code in Listing 1-1.
-->

Maintenant, ouvrez le fichier *main.rs* que vous venez de créer et entrez le
code de l'encart 1-1.

<!--
<span class="filename">Filename: main.rs</span>
-->

<span class="filename">Fichier : main.rs</span>

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
Save the file and go back to your terminal window. On Linux or macOS, enter
the following commands to compile and run the file:
-->

Enregistrez le fichier et retournez dans votre terminal. Sur Linux
ou macOS, écrivez les commandes suivantes pour compiler et exécuter le fichier :

```text
$ rustc main.rs
$ ./main
Hello, world!
```

<!--
On Windows, enter the command `.\main.exe` instead of `./main`:
-->

Sur Windows, écrivez la commande `.\main.exe` à la place de `.\main` :

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
référez-vous à la partie ["Dépannage"][troubleshooting]<!-- ignore --> du
chapitre d'installation pour vous aider.

<!--
If `Hello, world!` did print, congratulations! You’ve officially written a Rust
program. That makes you a Rust programmer—welcome!
-->

Si `Hello, world!` s'affiche, félicitations ! Vous avez officiellement écrit un
programme Rust. Cela fait de vous un développeur Rust — bienvenue !

<!--
### Anatomy of a Rust Program
-->

### Structure d'un programme Rust

<!--
Let’s review in detail what just happened in your “Hello, world!” program.
Here’s the first piece of the puzzle:
-->

Regardons en détail ce qui s'est passé dans votre programme “Hello, world!”.
Voici le premier morceau du puzzle :

```rust
fn main() {

}
```

<!--
These lines define a function in Rust. The `main` function is special: it is
always the first code that runs in every executable Rust program. The first
line declares a function named `main` that has no parameters and returns
nothing. If there were parameters, they would go inside the parentheses, `()`.
-->

Ces lignes définissent une fonction dans Rust. La fonction `main` est spéciale :
c'est toujours le premier code qui est exécuté dans tous les programmes
en Rust. La première ligne déclare une fonction qui s'appelle `main`, qui n'a
pas de paramètre et qui ne retourne aucune valeur. S'il y avait des paramètres,
ils seraient placés entre les parenthèses `()`.

<!--
Also, note that the function body is wrapped in curly brackets, `{}`. Rust
requires these around all function bodies. It’s good style to place the opening
curly bracket on the same line as the function declaration, adding one space in
between.
-->

À noter en outre que le corps de la fonction est placé entre des accolades
`{}`. Rust en a besoin autour du corps de chaque fonction. C'est une
bonne pratique d'insérer l'accolade ouvrante sur la même ligne que la
déclaration de la fonction, en ajoutant une espace entre les deux.

<!--
At the time of this writing, an automatic formatter tool called `rustfmt` is
under development. If you want to stick to a standard style across Rust
projects, `rustfmt` will format your code in a particular style. The Rust team
plans to eventually include this tool with the standard Rust distribution, like
`rustc`. So depending on when you read this book, it might already be installed
on your computer! Check the online documentation for more details.
-->

Pendant que nous écrivons ces lignes, un outil de formatage automatique de code
appelé `rustfmt` est en cours de développement.
Si vous voulez standardiser le style de votre code Rust dans vos projets,
`rustfmt` va formater votre code de manière bien définie.
L'équipe Rust envisage d'intégrer cet outil dans la distribution
standard de Rust, comme `rustc`. Donc, selon le moment où vous lirez ce livre,
il se peut qu'il soit déjà installé sur votre ordinateur ! Consultez la
documentation en ligne pour en savoir plus.

<!--
Inside the `main` function is the following code:
-->

À l'intérieur de la fonction `main`, nous avons le code suivant :

```rust
    println!("Hello, world!");
```

<!--
This line does all the work in this little program: it prints text to the
screen. There are four important details to notice here. First, Rust style is
to indent with four spaces, not a tab.
-->

Cette ligne fait tout le travail dans ce petit programme : il écrit le texte à
l'écran. Il y a quatre détails importants à noter ici. Premièrement, le style de
Rust est d'indenter avec quatre espaces, et non pas avec une tabulation.

<!--
Second, `println!` calls a Rust macro. If it called a function instead, it
would be entered as `println` (without the `!`). We’ll discuss Rust macros in
more detail in Chapter 19. For now, you just need to know that using a `!`
means that you’re calling a macro instead of a normal function.
-->

Deuxièmement, `println!` fait appel à une macro Rust. S'il appelait une
fonction à la place, cela serait écrit `println` (sans le `!`). Nous aborderons
les macros Rust plus en détail dans le chapitre 19. Pour l'instant, vous avez
juste à savoir qu'utiliser un `!` signifie que vous utilisez une macro plutôt
qu'une fonction classique.

<!--
Third, you see the `"Hello, world!"` string. We pass this string as an argument
to `println!`, and the string is printed to the screen.
-->

Troisièmement, vous voyez la chaîne de caractères `"Hello, world!"`. Nous
envoyons cette chaîne en argument à `println!` et cette chaîne est affichée
à l'écran.

<!--
Fourth, we end the line with a semicolon (`;`), which indicates that this
expression is over and the next one is ready to begin. Most lines of Rust code
end with a semicolon.
-->

Quatrièmement, nous terminons la ligne avec un point-virgule (`;`), qui indique
que cette expression est terminée et que la suivante est prête à commencer. La
plupart des lignes de Rust se terminent avec un point-virgule.

<!--
### Compiling and Running Are Separate Steps
-->

### La compilation et l'exécution sont des étapes séparées

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

```text
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
entering the `ls` command in your shell. On Linux and macOS, you’ll see two
files. With PowerShell on Windows, you’ll see the same three files that you
would see using CMD.
-->

Avec Linux, macOS et PowerShell sous Windows, vous pouvez voir l'exécutable en
utilisant la commande `ls` dans votre terminal. Avec Linux et macOS,
vous devriez voir deux fichiers. Avec PowerShell sous Windows, vous devriez voir
les trois mêmes fichiers que vous verriez en utilisant CMD.

```text
$ ls
main  main.rs
```

<!--
With CMD on Windows, you would enter the following:
-->

Avec CMD sous Windows, vous devez saisir la commande suivante :

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

Ceci affiche le fichier de code source avec l'extension *.rs*, le fichier
exécutable (*main.exe* sous Windows, mais *main* sur toutes les autres
plateformes) et, quand on utilise Windows, un fichier qui contient des
informations de débogage avec l'extension *.pdb*. Dans ce dossier, vous pouvez
exécuter le fichier *main* ou *main.exe* comme ceci :

<!--
```text
$ ./main # or .\main.exe on Windows
```
-->

```text
$ ./main # ou .\main.exe sous Windows
```

<!--
If *main.rs* was your “Hello, world!” program, this line would print `Hello,
world!` to your terminal.
-->

Si *main.rs* était votre programme “Hello, world!”, cette ligne devrait afficher
`Hello, world!` dans votre terminal.

<!--
If you’re more familiar with a dynamic language, such as Ruby, Python, or
JavaScript, you might not be used to compiling and running a program as
separate steps. Rust is an *ahead-of-time compiled* language, meaning you can
compile a program and give the executable to someone else, and they can run it
even without having Rust installed. If you give someone a *.rb*, *.py*, or
*.js* file, they need to have a Ruby, Python, or JavaScript implementation
installed (respectively). But in those languages, you only need one command to
compile and run your program. Everything is a trade-off in language design.
-->

Si vous connaissez un langage dynamique, comme Ruby, Python, ou JavaScript, vous
n'avez peut-être pas l'habitude de compiler puis lancer votre programme dans des
étapes séparées.
Rust est un langage à *compilation anticipée*, ce qui veut dire que
vous pouvez compiler le programme et le donner à quelqu'un d'autre, et il peut
l'exécuter sans avoir Rust d'installé. Si vous donnez à quelqu'un un fichier
*.rb*, *.py* ou *.js*, il a besoin d'avoir respectivement un interpréteur Ruby,
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

[troubleshooting]: ch01-01-installation.html#troubleshooting
