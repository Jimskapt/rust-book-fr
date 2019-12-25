<!--
# Managing Growing Projects with Packages, Crates, and Modules
-->

# Gérer des projets grandissants avec les paquets, crates et modules

<!--
As you write large programs, organizing your code will be important because
keeping track of your entire program in your head will become impossible. By
grouping related functionality and separating code with distinct features,
you’ll clarify where to find code that implements a particular feature and
where to go to change how a feature works.
-->

Lorsque vous commencez à écrire des gros programmes, organiser votre code va
devenir important car vous ne pourrez plus vous souvenir de l'intégralité de
votre programme. En regroupant des fonctionnalités qui ont des points
communs et en séparant des autres fonctionnalités, vous clarifiez l'endroit où
trouver le code qui implémente une fonctionnalité spécifique afin de pouvoir le
relire ou le modifier.

<!--
The programs we’ve written so far have been in one module in one file. As a
project grows, you can organize code by splitting it into multiple modules and
then multiple files. A package can contain multiple binary crates and
optionally one library crate. As a package grows, you can extract parts into
separate crates that become external dependencies. This chapter covers all
these techniques. For very large projects of a set of interrelated packages
that evolve together, Cargo provides workspaces, which we’ll cover in the
[“Cargo Workspaces”][workspaces]<!-- ignore -- > section in Chapter 14.
-->

Le programme que nous avons écrit précédemment était dans un module et dans un
seul fichier. Quand le projet grandit, vous pouvez organiser votre code en le
découpant en plusieurs modules et ensuite en plusieurs fichiers. Un paquet peut
contenir plusieurs crates binaires et accessoirement une crate de bibliothèque.
Lorsqu'un paquet grandit, vous pouvez en extraire des parties dans des crates
séparées qui deviennent des dépendances externes. Ce chapitre va aborder toutes
ces techniques. Pour un projet de très grande envergure qui a des paquets
interconnectés qui évoluent ensemble, Cargo propose les espaces de travail, que
nous allons découvrir dans une section du
[chapitre 14][workspaces]<!-- ignore -->.

<!--
In addition to grouping functionality, encapsulating implementation details
lets you reuse code at a higher level: once you’ve implemented an operation,
other code can call that code via the code’s public interface without knowing
how the implementation works. The way you write code defines which parts are
public for other code to use and which parts are private implementation details
that you reserve the right to change. This is another way to limit the amount
of detail you have to keep in your head.
-->

En plus de regrouper des fonctionnalités, les modules vous permettent
d'encapsuler les détails de l'implémentation d'une opération : vous pouvez
écrire du code puis l'utiliser comme une abstraction à travers l'interface de
programmation publique (API) du code sans se soucier de connaître les détails de
son implémentation. La façon dont vous écrivez le code définit quelles sont les
parties sont publiques pour qu'un autre code puisse l'utiliser et quelles
parties sont des détails d'implémentation privées dont vous vous réservez le
droit de modifier. C'est un autre moyen de limiter le nombre d'éléments de
l'API pour celui qui l'utilise.

<!--
A related concept is scope: the nested context in which code is written has a
set of names that are defined as “in scope.” When reading, writing, and
compiling code, programmers and compilers need to know whether a particular
name at a particular spot refers to a variable, function, struct, enum, module,
constant, or other item and what that item means. You can create scopes and
change which names are in or out of scope. You can’t have two items with the
same name in the same scope; tools are available to resolve name conflicts.
-->

Un concept qui lui est associé est la portée : le contexte dans lequel le code
est écrit a un jeu de noms qui sont définis comme “dans la portée”. Quand ils
lisent, écrivent et compilent du code, les développeurs et les compilateurs ont
besoin de savoir précisément ce que désigne une entité, que cette entité soit
une variable, une fonction, une structure, une énumération, un module, une
constante ... Vous pouvez créer des portées et décider si les entités sont dans
la portée ou non. Vous ne pouvez pas avoir deux entités avec le même nom dans la
même portée ; cependant des outils existent pour résoudre les conflits de nom.

<!--
Rust has a number of features that allow you to manage your code’s
organization, including which details are exposed, which details are private,
and what names are in each scope in your programs. These features, sometimes
collectively referred to as the *module system*, and include:
-->

Rust a de nombreuses fonctionnalités qui vous permettent de gérer l'organisation
de votre code, grâce à ce que la communauté Rust appelle le *système de
modules*.
Ce système définit quels sont les éléments qui sont accessibles depuis
l'extérieur de la bibliothèque (notion de privé ou public), ansi que leur
portée.
Ces fonctionnalités comprennent :

<!--
* **Packages:** A Cargo feature that lets you build, test, and share crates
* **Crates:** A tree of modules that produces a library or executable
* **Modules** and **use:** Let you control the organization, scope, and
  privacy of paths
* **Paths:** A way of naming an item, such as a struct, function, or module
-->

* **les paquets :** une fonctionnalité de Cargo qui vous permet de compiler,
  tester, et partager des crates.
* **les `Crates` :** une arborescence de modules qui fournit une bibliothèque ou
  un exécutable
* **les chemins :** une façon de nommer un élément, comme une structure, une
  fonction ou un module.

<!--
In this chapter, we’ll cover all these features, discuss how they interact, and
explain how to use them to manage scope. By the end, you should have a solid
understanding of the module system and be able to work with scopes like a pro!
-->

Dans ce chapitre, nous allons découvrir ces fonctionnalités, voir comment elles
intéragissent, et expliquer comment les utiliser pour gérer les portées. À
l'issue de ce chapitre, vous aurez de solides connaissances sur le système de
modules et vous pourrez travailler avec les portées comme un pro !

[workspaces]: ch14-03-cargo-workspaces.html
