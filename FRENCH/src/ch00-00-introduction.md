<!--
# Introduction
-->

# Introduction

<!--
> Note: This edition of the book is the same as [The Rust Programming
> Language][nsprust] available in print and ebook format from [No Starch
> Press][nsp].
-->

> Note : la version anglaise de ce livre est disponible au format papier et
> ebook chez [No Starch Press][nsp] à cette adresse :
> [The Rust Programming Language][nsprust]

<!--
[nsprust]: https://nostarch.com/rust
[nsp]: https://nostarch.com/
-->

[nsprust]: https://nostarch.com/rust
[nsp]: https://nostarch.com/

<!--
Welcome to *The Rust Programming Language*, an introductory book about Rust.
The Rust programming language helps you write faster, more reliable software.
High-level ergonomics and low-level control are often at odds in programming
language design; Rust challenges that conflict. Through balancing powerful
technical capacity and a great developer experience, Rust gives you the option
to control low-level details (such as memory usage) without all the hassle
traditionally associated with such control.
-->

Bienvenue sur *Le langage de programmation Rust*, un livre d'initiation à Rust.
Le langage de programmation Rust vous aide à écrire plus rapidement des
logiciels plus fiables. L'ergonomie de haut-niveau et la maîtrise de bas-niveau
sont souvent en opposition dans la conception des langages de programmation ;
Rust remet en cause ce conflit. Grâce à l'équilibre entre ses puissantes
capacités techniques et une bonne ergonomie de développement, Rust vous donne
la possibilité de contrôler les détails de bas-niveau (comme l'utilisation de
la mémoire) sans tous les soucis traditionnellement associés à ce genre de
pratique.

<!--
## Who Rust Is For
-->

## À qui s'adresse Rust

<!--
Rust is ideal for many people for a variety of reasons. Let’s look at a few of
the most important groups.
-->

Rust est idéal pour de nombreuses personnes pour diverses raisons. Analysons
quelques-uns des groupes les plus importants.

<!--
### Teams of Developers
-->

### Équipes de développeurs

<!--
Rust is proving to be a productive tool for collaborating among large teams of
developers with varying levels of systems programming knowledge. Low-level code
is prone to a variety of subtle bugs, which in most other languages can be
caught only through extensive testing and careful code review by experienced
developers. In Rust, the compiler plays a gatekeeper role by refusing to
compile code with these elusive bugs, including concurrency bugs. By working
alongside the compiler, the team can spend their time focusing on the program’s
logic rather than chasing down bugs.
-->

Rust se révèle être un outil productif pour la collaboration entre de grandes
équipes de développeurs ayant différents niveaux de connaissances en
programmation système. Le code de bas-niveau est sujet à une multitude de bogues
subtils, qui, dans la plupart des autres langages, ne peuvent être prévenus
qu'au moyen de campagnes de test étendues et de minutieuses revues de
code menées par des développeurs chevronnés. Avec Rust, le compilateur joue le
rôle de gardien en refusant de compiler du code qui comprend ces bogues
discrets et vicieux, y compris les bogues de concurrence. En travaillant avec
le compilateur, l'équipe peut se concentrer sur la logique du programme plutôt
que de traquer les bogues.

<!--
Rust also brings contemporary developer tools to the systems programming world:
-->

Rust offre aussi des outils de développement modernes au monde de la
programmation système :

<!--
* Cargo, the included dependency manager and build tool, makes adding,
  compiling, and managing dependencies painless and consistent across the Rust
  ecosystem.
* Rustfmt ensures a consistent coding style across developers.
* The Rust Language Server powers Integrated Development Environment (IDE)
  integration for code completion and inline error messages.
-->

* Cargo, l'outil intégré de gestion de dépendances et de compilation, qui
  uniformise et facilite l'ajout, la compilation, et la gestion des dépendances
  dans l'écosystème Rust.
* Rustfmt, qui assure une cohérence de style de codage pour tous les
  développeurs.
* Le *Rust Langage Server* alimente les environnements de développement
  intégrés (IDE) pour la complétion du code et l'affichage direct des messages
  d'erreur.

<!--
By using these and other tools in the Rust ecosystem, developers can be
productive while writing systems-level code.
-->

En utilisant ces outils ainsi que d'autres dans l'écosystème Rust, les
développeurs peuvent être plus productifs quand ils écrivent du code système.

<!--
### Students
-->

### Étudiants

<!--
Rust is for students and those who are interested in learning about systems
concepts. Using Rust, many people have learned about topics like operating
systems development. The community is very welcoming and happy to answer
student questions. Through efforts such as this book, the Rust teams want to
make systems concepts more accessible to more people, especially those new to
programming.
-->

Rust est conçu pour les étudiants et ceux qui s'intéressent à l'apprentissage
des concepts système. En utilisant Rust, de nombreuses personnes ont appris
des domaines comme le développement de systèmes d'exploitation. La communauté
est très accueillante et répond volontiers aux questions des étudiants. Grâce à
des initiatives comme ce livre, les équipes de Rust veulent rendre les notions
système accessibles au plus grand nombre, particulièrement à ceux qui débutent
dans la programmation.

<!--
### Companies
-->

### Entreprises

<!--
Hundreds of companies, large and small, use Rust in production for a variety of
tasks. Those tasks include command line tools, web services, DevOps tooling,
embedded devices, audio and video analysis and transcoding, cryptocurrencies,
bioinformatics, search engines, Internet of Things applications, machine
learning, and even major parts of the Firefox web browser.
-->

Des centaines d'entreprises, petites et grosses, utilisent Rust en production
pour différentes missions. Ils l'utilisent pour des outils en ligne de commande,
des services web, des outils DevOps, des systèmes embarqués, de l'analyse et de
la conversion audio et vidéo, des cryptomonnaies, de la bio-informatique, des
moteurs de recherche, de l'internet des objets *(IoT)*, de l'apprentissage
automatique *(machine learning)*, et même des parties importantes du navigateur
internet Firefox.

<!--
### Open Source Developers
-->

### Développeurs de logiciel libre

<!--
Rust is for people who want to build the Rust programming language, community,
developer tools, and libraries. We’d love to have you contribute to the Rust
language.
-->

Rust est ouvert aux personnes qui veulent développer le langage de programmation
Rust, la communauté, les outils de développement et les bibliothèques. Nous
serions ravis que vous contribuiez au langage Rust.

<!--
### People Who Value Speed and Stability
-->

### Les personnes qui recherchent la rapidité et la stabilité

<!--
Rust is for people who crave speed and stability in a language. By speed, we
mean the speed of the programs that you can create with Rust and the speed at
which Rust lets you write them. The Rust compiler’s checks ensure stability
through feature additions and refactoring. This is in contrast to the brittle
legacy code in languages without these checks, which developers are often
afraid to modify. By striving for zero-cost abstractions, higher-level features
that compile to lower-level code as fast as code written manually, Rust
endeavors to make safe code be fast code as well.
-->

Rust est une solution pour les personnes qui chérissent la rapidité et la
stabilité dans un langage. Par rapidité, nous entendons la vitesse des
programmes que vous pouvez créer avec Rust et la rapidité avec laquelle Rust
vous permet de les écrire. Les vérifications du compilateur de Rust assurent la
stabilité durant l'ajout de fonctionnalités ou le remaniement du code. Cela
le démarque des langages qui ne font pas ces contrôles sur du code instable que
le programme a hérité avec le temps, et que bien souvent les développeurs ont
peur de modifier. En s'efforçant de mettre en place des abstractions sans coût,
des fonctionnalités de haut-niveau qui compilent vers du code bas-niveau aussi
rapide que s'il avait été écrit à la main, Rust fait en sorte que le
code sûr soit aussi du code rapide.

<!--
The Rust language hopes to support many other users as well; those mentioned
here are merely some of the biggest stakeholders. Overall, Rust’s greatest
ambition is to eliminate the trade-offs that programmers have accepted for
decades by providing safety *and* productivity, speed *and* ergonomics. Give
Rust a try and see if its choices work for you.
-->

Le langage Rust espère aider beaucoup d'autres utilisateurs ; ceux cités ici ne
font partie que d'un univers bien plus grand. Globalement, la plus grande
ambition de Rust est d'éradiquer les compromis auxquels les développeurs
se soumettaient depuis des décennies en leur apportant sécurité *et*
productivité, rapidité *et* ergonomie. Essayez Rust et vérifiez si ses décisions
vous conviennent.

<!--
## Who This Book Is For
-->

## À qui est destiné ce livre

<!--
This book assumes that you’ve written code in another programming language but
doesn’t make any assumptions about which one. We’ve tried to make the material
broadly accessible to those from a wide variety of programming backgrounds. We
don’t spend a lot of time talking about what programming *is* or how to think
about it. If you’re entirely new to programming, you would be better served by
reading a book that specifically provides an introduction to programming.
-->

Ce livre suppose que vous avez écrit du code dans un autre langage de
programmation mais ne suppose pas lequel. Nous avons essayé de rendre son
contenu le plus accessible au plus grand nombre d'expériences de programmation
possible. Nous ne nous évertuons pas à nous questionner sur *ce qu'est* la
programmation ou comment l'envisager. Si vous êtes débutant en programmation,
vous seriez mieux avisé en lisant un livre qui vous initie à la programmation.

<!--
## How to Use This Book
-->

## Comment utiliser ce livre

<!--
In general, this book assumes that you’re reading it in sequence from front to
back. Later chapters build on concepts in earlier chapters, and earlier
chapters might not delve into details on a topic; we typically revisit the
topic in a later chapter.
-->

Globalement, ce livre est prévu pour être lu dans l'ordre. Les chapitres
suivants s'appuient sur les notions abordées dans les chapitres précédents, et
lorsque les chapitres précédents ne peuvent pas approfondir un sujet, ce sera
généralement fait dans un chapitre suivant.

<!--
You’ll find two kinds of chapters in this book: concept chapters and project
chapters. In concept chapters, you’ll learn about an aspect of Rust. In project
chapters, we’ll build small programs together, applying what you’ve learned so
far. Chapters 2, 12, and 20 are project chapters; the rest are concept chapters.
-->

Vous allez rencontrer deux différents types de chapitres dans ce livre : les
chapitres théoriques et les chapitres de projet. Dans les chapitres théoriques,
vous allez apprendre un sujet à propos de Rust. Dans un chapitre de projet, nous
allons construire ensemble des petits programmes, pour appliquer ce que vous
avez appris précédemment. Les chapitres 2, 12 et 20 sont des chapitres de
projet ; les autres sont des chapitres théoriques.

<!--
Chapter 1 explains how to install Rust, how to write a “Hello, world!” program,
and how to use Cargo, Rust’s package manager and build tool. Chapter 2 is a
hands-on introduction to the Rust language. Here we cover concepts at a high
level, and later chapters will provide additional detail. If you want to get
your hands dirty right away, Chapter 2 is the place for that. At first, you
might even want to skip Chapter 3, which covers Rust features similar to those
of other programming languages, and head straight to Chapter 4 to learn about
Rust’s ownership system. However, if you’re a particularly meticulous learner
who prefers to learn every detail before moving on to the next, you might want
to skip Chapter 2 and go straight to Chapter 3, returning to Chapter 2 when
you’d like to work on a project applying the details you’ve learned.
-->

Le chapitre 1 explique comment installer Rust, comment écrire un programme
"Hello, world!" et comment utiliser Cargo, le gestionnaire de paquets et outil
de compilation. Le chapitre 2 est une initiation pratique au langage Rust. Nous
y aborderons des concepts de haut-niveau, et les chapitres suivants apporteront
plus de détails. Si vous voulez vous *salir les mains* tout de suite,
le chapitre 2 est l'endroit pour cela. Au début, vous pouvez même sauter le
chapitre 3, qui aborde les fonctionnalités de Rust semblables aux autres
langages de programmation, et passer directement au chapitre 4 pour en savoir
plus sur le système de possession *(ownership)* de Rust. Toutefois, si vous êtes
un apprenti particulièrement minutieux qui préfère apprendre chaque
particularité avant de passer à la suivante, vous pouvez sauter le chapitre 2 et
passer directement au chapitre 3, puis revenir au chapitre 2 lorsque vous
souhaitez travailler sur un projet en appliquant les notions que vous avez
apprises.

<!--
Chapter 5 discusses structs and methods, and Chapter 6 covers enums, `match`
expressions, and the `if let` control flow construct. You’ll use structs and
enums to make custom types in Rust.
-->

Le chapitre 5 traite des structures et des méthodes, et le chapitre 6 couvre les
énumérations, les expressions `match`, et la structure de contrôle `if let`.
Vous emploierez les structures et les énumérations pour créer des types
personnalisés avec Rust.

<!--
In Chapter 7, you’ll learn about Rust’s module system and about privacy rules
for organizing your code and its public Application Programming Interface
(API). Chapter 8 discusses some common collection data structures that the
standard library provides, such as vectors, strings, and hash maps. Chapter 9
explores Rust’s error-handling philosophy and techniques.
-->

Au chapitre 7, vous apprendrez le système de modules de Rust et les règles de
visibilité, afin d'organiser votre code et son interface de programmation
applicative (API) publique. Le chapitre 8 traitera des structures de collections
de données usuelles fournies par la bibliothèque standard, comme les vecteurs,
les chaînes de caractères et les tables de hachage *(hash maps)*. Le chapitre 9
explorera la philosophie et les techniques de gestion d'erreurs de Rust.

<!--
Chapter 10 digs into generics, traits, and lifetimes, which give you the power
to define code that applies to multiple types. Chapter 11 is all about testing,
which even with Rust’s safety guarantees is necessary to ensure your program’s
logic is correct. In Chapter 12, we’ll build our own implementation of a subset
of functionality from the `grep` command line tool that searches for text
within files. For this, we’ll use many of the concepts we discussed in the
previous chapters.
-->

Le chapitre 10 nous plongera dans la généricité, les *traits* et
les durées de vie, qui vous donneront la capacité de créer du code qui s'adapte
à différents types. Le chapitre 11 traitera des techniques de test, qui restent
nécessaires malgré les garanties de sécurité de Rust, pour s'assurer que
la logique de votre programme est valide. Au chapitre 12, nous écrirons
notre propre implémentation d'un sous-ensemble des fonctionnalités du programme
en ligne de commande `grep`, qui recherche du texte dans des fichiers.
Pour ce faire, nous utiliserons de nombreuses notions abordées dans les
chapitres précédents.

<!--
Chapter 13 explores closures and iterators: features of Rust that come from
functional programming languages. In Chapter 14, we’ll examine Cargo in more
depth and talk about best practices for sharing your libraries with others.
Chapter 15 discusses smart pointers that the standard library provides and the
traits that enable their functionality.
-->

Le chapitre 13 explorera les fermetures *(closures)* et itérateurs : ce sont les
fonctionnalités de Rust inspirées des langages de programmation fonctionnels.
Au chapitre 14, nous explorerons plus en profondeur Cargo et les bonnes
pratiques pour partager vos propres bibliothèques avec les autres. Le chapitre
15 parlera de pointeurs intelligents qu'apporte la bibliothèque standard et des
*traits* qui activent leurs fonctionnalités.

<!--
In Chapter 16, we’ll walk through different models of concurrent programming
and talk about how Rust helps you to program in multiple threads fearlessly.
Chapter 17 looks at how Rust idioms compare to object-oriented programming
principles you might be familiar with.
-->

Au chapitre 16, nous passerons en revue les différents modes de programmation
concurrente et comment Rust nous aide à développer dans des tâches parallèles
sans crainte. Le chapitre 17 comparera les fonctionnalités de Rust aux
principes de programmation orientée objet, que vous connaissez peut-être.

<!--
Chapter 18 is a reference on patterns and pattern matching, which are powerful
ways of expressing ideas throughout Rust programs. Chapter 19 contains a
smorgasbord of advanced topics of interest, including unsafe Rust, macros, and
more about lifetimes, traits, types, functions, and closures.
-->

Le chapitre 18 est une référence sur les motifs et le filtrage de motif
*(pattern matching)*, qui sont des moyens puissants permettant de communiquer
des idées dans les programmes Rust. Le chapitre 19 contient une foultitude de
sujets avancés intéressants, comme le code Rust non sécurisé *(unsafe)*, les
macros et plus de détails sur les durées de vie, les *traits*, les types, les
fonctions et les fermetures *(closures)*.

<!--
In Chapter 20, we’ll complete a project in which we’ll implement a low-level
multithreaded web server!
-->

Au chapitre 20, nous terminerons un projet dans lequel nous allons
implémenter en bas-niveau un serveur web multitâches !

<!--
Finally, some appendices contain useful information about the language in a
more reference-like format. Appendix A covers Rust’s keywords, Appendix B
covers Rust’s operators and symbols, Appendix C covers derivable traits
provided by the standard library, Appendix D covers some useful development
tools, and Appendix E explains Rust editions.
-->

Et finalement, quelques annexes qui contiennent des informations utiles sur le
langage sous forme de référentiels qui renvoient à d'autres documents. L'annexe
A liste les mots-clés de Rust, l'annexe B couvre les opérateurs et symboles de
Rust, l'annexe C parle des *traits* dérivables qu'apporte la bibliothèque
standard, l'annexe D référence certains outils de développement utiles, et
l'annexe E explique les différentes éditions de Rust.

<!--
There is no wrong way to read this book: if you want to skip ahead, go for it!
You might have to jump back to earlier chapters if you experience any
confusion. But do whatever works for you.
-->

Il n'y a pas de mauvaise manière de lire ce livre : si vous voulez sauter des
étapes, allez-y !
Vous devrez alors peut-être revenir sur les chapitres précédents si vous
éprouvez des difficultés. Mais faites comme bon vous semble.

<!--
<span id="ferris"></span>
-->

<span id="ferris"></span>

<!--
An important part of the process of learning Rust is learning how to read the
error messages the compiler displays: these will guide you toward working code.
As such, we’ll provide many examples that don’t compile along with the error
message the compiler will show you in each situation. Know that if you enter
and run a random example, it may not compile! Make sure you read the
surrounding text to see whether the example you’re trying to run is meant to
error. Ferris will also help you distinguish code that isn’t meant to work:
-->

Une composante importante du processus d'apprentissage de Rust est de comprendre
comment lire les messages d'erreur qu'affiche le compilateur : ils vous
guideront vers du code correct.
Ainsi, nous citerons de nombreux exemples qui ne compilent pas, avec le message
d'erreur que le compilateur devrait vous afficher dans chaque cas. C'est donc
normal que dans certains cas, si vous copiez et exécutez un exemple au hasard,
il ne compile pas ! Assurez-vous d'avoir lu le texte autour pour savoir si
l'exemple que vous tentez de compiler doit échouer. Ferris va aussi vous aider
à identifier du code qui ne devrait pas fonctionner :

<!-- markdownlint-disable -->
<!--
| Ferris                                                                                                           | Meaning                                          |
|------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>            | This code does not compile!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>                   | This code panics!                                |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | This code does not produce the desired behavior. |
-->
<!-- markdownlint-restore -->

| Ferris                                                                                                                             | Signification                           |
|------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris avec un point d'interrogation"/>                     | Ce code ne compile pas !                |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris qui lève ses bras"/>                                           | Ce code panique !                       |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris avec une pince en l'air, haussant les épaules"/> | Ce code ne se comporte pas comme voulu. |

<!--
In most situations, we’ll lead you to the correct version of any code that
doesn’t compile.
-->

Dans la plupart des cas, nous vous guiderons vers la version du code qui devrait
fonctionner.

<!--
## Source Code
-->

## Code source

<!--
The source files from which this book is generated can be found on
[GitHub][book].
-->

Les fichiers du code source qui a généré ce livre en anglais sont disponibles
sur [GitHub][book].

La version française est aussi disponible sur [GitHub][book-fr].

<!--
[book]: https://github.com/rust-lang/book/tree/main/src
-->

[book]: https://github.com/rust-lang/book/tree/main/src
[book-fr]: https://github.com/Jimskapt/rust-book-fr
