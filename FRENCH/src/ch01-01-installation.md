<!--
## Installation
-->

## Installation

<!--
The first step is to install Rust. We’ll download Rust through `rustup`, a
command line tool for managing Rust versions and associated tools. You’ll need
an internet connection for the download.
-->

La première étape consiste à installer Rust. Nous allons télécharger Rust via
`rustup`, un outil en ligne de commande conçu pour gérer les versions de Rust et
les outils qui leur sont associés. Vous allez avoir besoin d'une connexion
Internet pour le téléchargement.

<!--
> Note: If you prefer not to use `rustup` for some reason, please see [the Rust
> installation page](https://www.rust-lang.org/tools/install) for other options.
-->

> Note : si vous préférez ne pas utiliser `rustup` pour une raison ou une autre,
> vous pouvez vous référer à
> [la page d'installation de Rust][rust-installation-page]
> pour d'autres méthodes d'installation.

[rust-installation-page]: https://www.rust-lang.org/tools/install

<!--
The following steps install the latest stable version of the Rust compiler.
Rust’s stability guarantees ensure that all the examples in the book that
compile will continue to compile with newer Rust versions. The output might
differ slightly between versions, because Rust often improves error messages
and warnings. In other words, any newer, stable version of Rust you install
using these steps should work as expected with the content of this book.
-->

L'étape suivante est d'installer la dernière version stable du compilateur Rust.
La garantie de stabilité de Rust assurera que tous les exemples dans le livre
qui se compilent bien vont continuer à se compiler avec les nouvelles versions
de Rust. La sortie peut varier légèrement d'une version à une autre, car Rust
améliore souvent les messages d'erreur et les avertissements. En résumé, toute
nouvelle version stable de Rust que vous installez de cette manière devrait
fonctionner en cohérence avec le contenu de ce livre.

<!--
> ### Command Line Notation
>
> In this chapter and throughout the book, we’ll show some commands used in the
> terminal. Lines that you should enter in a terminal all start with `$`. You
> don’t need to type in the `$` character; it indicates the start of each
> command. Lines that don’t start with `$` typically show the output of the
> previous command. Additionally, PowerShell-specific examples will use `>`
> rather than `$`.
-->

> ### La notation en ligne de commande
>
> Dans ce chapitre et les suivants dans le livre, nous allons montrer quelques
> commandes tapées dans le terminal. Les lignes que vous devrez écrire dans le
> terminal commencent toutes par `$`. Vous n'avez pas besoin d'écrire le
> caractère `$`; il marque le début de chaque commande. Les lignes qui ne
> commencent pas par `$` montrent généralement le résultat de la commande
> précédente. De plus, les exemples propres à PowerShell utiliseront
> `>` plutôt que `$`.

<!--
### Installing `rustup` on Linux or macOS
-->

### Installer `rustup` sur Linux ou macOS

<!--
If you’re using Linux or macOS, open a terminal and enter the following command:
-->

Si vous utilisez Linux ou macOS, ouvrez un terminal et écrivez la commande
suivante :

```text
$ curl https://sh.rustup.rs -sSf | sh
```

<!--
The command downloads a script and starts the installation of the `rustup`
tool, which installs the latest stable version of Rust. You might be prompted
for your password. If the install is successful, the following line will appear:
-->

Cette commande télécharge un script et lance l'installation de l'outil `rustup`,
qui va installer la dernière version stable de Rust. Il est possible que l'on
vous demande votre mot de passe. Si l'installation se déroule bien, vous
devriez voir la ligne suivante s'afficher :

```text
Rust is installed now. Great!
```

<!--
If you prefer, feel free to download the script and inspect it before running
it.
-->

Si vous préférez, vous pouvez télécharger le script et l'examiner avant de le
lancer.

<!--
The installation script automatically adds Rust to your system PATH after your
next login. If you want to start using Rust right away instead of restarting
your terminal, run the following command in your shell to add Rust to your
system PATH manually:
-->

Le script d'installation va ajouter automatiquement Rust dans la variable
*PATH* de votre système après votre prochaine connexion utilisateur.
Si vous préférez utiliser Rust tout de suite au lieu de redémarrer votre
terminal, lancez la commande suivante dans votre terminal pour ajouter Rust dans
le *PATH* de votre système manuellement :

```text
$ source $HOME/.cargo/env
```

<!--
Alternatively, you can add the following line to your *~/.bash_profile*:
-->

Vous pouvez aussi ajouter la ligne suivante dans votre *~/.bash_profile* :

```text
$ export PATH="$HOME/.cargo/bin:$PATH"
```

<!--
Additionally, you’ll need a linker of some kind. It’s likely one is already
installed, but when you try to compile a Rust program and get errors indicating
that a linker could not execute, that means a linker isn’t installed on your
system and you’ll need to install one manually. C compilers usually come with
the correct linker. Check your platform’s documentation for how to install a C
compiler. Also, some common Rust packages depend on C code and will need a C
compiler. Therefore, it might be worth installing one now.
-->

De plus, vous aurez peut-être besoin d'un *linker* quelconque. Il est probable
qu'il y en ait déjà un d'installé, mais si vous essayez de compiler un
programme Rust et que vous avez des erreurs qui disent que le *linker* ne peut
pas être exécuté, cela veut dire qu'il n'y a pas de *linker* d'installé sur
votre système et que vous devrez en installer un manuellement. Les
compilateurs C sont généralement livrés avec un bon *linker*. Référez-vous à la
documentation de votre plateforme pour savoir comment installer un compilateur
C. De plus, certains paquets Rust communs nécessitent du code C et auront
besoin d'un compilateur C. C'est pourquoi cela vaut la peine d'en installer un
dès maintenant.

<!--
### Installing `rustup` on Windows
-->

### Installer `rustup` sous Windows

<!--
On Windows, go to [https://www.rust-lang.org/tools/install][install] and follow
the instructions for installing Rust. At some point in the installation, you’ll
receive a message explaining that you’ll also need the C++ build tools for
Visual Studio 2013 or later. The easiest way to acquire the build tools is to
install [Build Tools for Visual Studio 2019][visualstudio]. The tools are in
the Other Tools and Frameworks section.
-->

Sous Windows, il faut aller sur
[https://www.rust-lang.org/tools/install][install] et suivre les instructions
pour installer Rust. À un moment donné durant l'installation, vous aurez un
message vous expliquant qu'il va vous falloir l'outil de compilation C++
pour Visual Studio 2013 ou plus récent. La méthode la plus facile pour obtenir
les outils de compilation est d'installer
[Build Tools pour Visual Studio 2019][visualstudio].

<!-- markdownlint-disable -->
<!--
[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://www.visualstudio.com/downloads/#build-tools-for-visual-studio-2019
FR translation: the website redirects to French version, we lose the "id" link.
Fix is directly below:
-->
<!-- markdownlint-restore -->

[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://www.visualstudio.com/fr/downloads/#build-tools-for-visual-studio-2019

<!--
The rest of this book uses commands that work in both *cmd.exe* and PowerShell.
If there are specific differences, we’ll explain which to use.
-->

La suite de ce livre utilisera des commandes qui fonctionnent à la fois dans
*cmd.exe* et PowerShell. S'il y a des différences particulières, nous vous
expliquerons lesquelles utiliser.

<!--
### Updating and Uninstalling
-->

### Mettre à jour et désinstaller

<!--
After you’ve installed Rust via `rustup`, updating to the latest version is
easy. From your shell, run the following update script:
-->

Après avoir installé Rust avec `rustup`, la mise à jour vers la dernière version
est facile. Dans votre terminal, lancez le script de mise à jour suivant :

```text
$ rustup update
```

<!--
To uninstall Rust and `rustup`, run the following uninstall script from your
shell:
-->

Pour désinstaller Rust et `rustup`, exécutez le script de désinstallation
suivant dans votre terminal :

```text
$ rustup self uninstall
```

<!--
### Troubleshooting
-->

### Dépannage

<!--
To check whether you have Rust installed correctly, open a shell and enter this
line:
-->

Pour vérifier si Rust est correctement installé, ouvrez un terminal et entrez
cette ligne :

```text
$ rustc --version
```

<!--
You should see the version number, commit hash, and commit date for the latest
stable version that has been released in the following format:
-->

Vous devriez voir le numéro de version, le *hash* de *commit*,
et la date de *commit* de la dernière version stable qui a été publiée,
au format suivant :

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

<!--
If you see this information, you have installed Rust successfully! If you don’t
see this information and you’re on Windows, check that Rust is in your `%PATH%`
system variable. If that’s all correct and Rust still isn’t working, there are
a number of places you can get help. The easiest is the #beginners channel on
[the official Rust Discord][discord]. There, you can chat with other Rustaceans
(a silly nickname we call ourselves) who can help you out. Other great
resources include [the Users forum][users] and [Stack Overflow][stackoverflow].
-->

Si vous voyez cette information, c'est que vous avez installé Rust avec succès !
Si vous ne voyez pas cette information et que vous êtes sous Windows, vérifiez
que Rust est présent dans votre variable d'environnement système `%PATH%`. Si
tout est correct et que Rust ne fonctionne toujours pas, il y a quelques
endroits où vous pourrez trouver de l'aide. Le plus accessible est le
canal #beginners sur le [Discord officiel de Rust][discord]. Là-bas, vous pouvez
dialoguer en ligne avec d'autres *Rustacés* (un surnom ridicule que nous nous
donnons entre nous) qui pourront vous aider. D'autres bonnes sources de données
sont [le forum d'utilisateurs][users] et [Stack Overflow][stackoverflow].

[discord]: https://discord.gg/rust-lang
[users]: https://users.rust-lang.org/
[stackoverflow]: http://stackoverflow.com/questions/tagged/rust

<!--
### Local Documentation
-->

### Documentation en local

<!--
The installation of Rust also includes a copy of the documentation locally, so
you can read it offline. Run `rustup doc` to open the local documentation in
your browser.
-->

L'installation de Rust embarque aussi une copie de la documentation en local
pour que vous puissiez la lire hors ligne. Lancez `rustup doc` afin d'ouvrir la
documentation locale dans votre navigateur.

<!--
Any time a type or function is provided by the standard library and you’re not
sure what it does or how to use it, use the application programming interface
(API) documentation to find out!
-->

À chaque fois que vous n'êtes pas sûr de ce que fait un type ou une fonction
fournie par la bibliothèque standard ou que vous ne savez pas comment
l'utiliser, utilisez cette documentation de l'interface de programmation
applicative *(API)* pour le savoir !
