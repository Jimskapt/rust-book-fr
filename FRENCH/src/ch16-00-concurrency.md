<!--
# Fearless Concurrency
-->

# La concurrence sans craintes

<!--
Handling concurrent programming safely and efficiently is another of Rust’s
major goals. *Concurrent programming*, where different parts of a program
execute independently, and *parallel programming*, where different parts of a
program execute at the same time, are becoming increasingly important as more
computers take advantage of their multiple processors. Historically,
programming in these contexts has been difficult and error prone: Rust hopes to
change that.
-->

Le développement sécurisé et efficace dans des contextes de concurrence est un
autre objectif majeur de Rust. La *programmation concurrente*, dans laquelle
différentes parties d'un programme s'exécutent de manière indépendente, et le
*parallélisme*, dans lequel différentes parties d'un programme s'exécutent en
même temps, sont devenus des pratiques de plus en plus importantes au fur et à
mesure que les ordinateurs tirent parti de leurs processeurs multiples.
Historiquement, le développement dans ces contextes est difficile et favorise
les erreurs : Rust compte bien changer la donne.

<!--
Initially, the Rust team thought that ensuring memory safety and preventing
concurrency problems were two separate challenges to be solved with different
methods. Over time, the team discovered that the ownership and type systems are
a powerful set of tools to help manage memory safety *and* concurrency
problems! By leveraging ownership and type checking, many concurrency errors
are compile-time errors in Rust rather than runtime errors. Therefore, rather
than making you spend lots of time trying to reproduce the exact circumstances
under which a runtime concurrency bug occurs, incorrect code will refuse to
compile and present an error explaining the problem. As a result, you can fix
your code while you’re working on it rather than potentially after it has been
shipped to production. We’ve nicknamed this aspect of Rust *fearless*
*concurrency*. Fearless concurrency allows you to write code that is free of
subtle bugs and is easy to refactor without introducing new bugs.
-->

Au début, l'équipe de Rust pensait que garantir la sécurité de la mémoire et
éviter les problèmes de concurrence étaient deux challenges distincts qui
devaient être résolus de manière différentes. Avec le temps, l'équipe a
découvert que les systèmes de possession et de type sont des jeux d'outils
puissants qui aident à sécuriser la mémoire *et* à régler des problèmes de
concurrence ! En exploitant la possession et la vérification de type, de
nombreuses erreurs de concurrence deviennent des erreurs à la compilation en
Rust plutôt que des erreurs à l'exécution. Ainsi, plutôt que d'avoir à passer
beaucoup de votre temps à tenter de reproduire les circonstances exactes dans
lesquelles un bogue de concurrence s'est produit à l'exécution, le code
incorrecte va refuser de se compiler et va vous afficher une erreur expliquant
le problème. Au final, vous pouvez corriger votre code pendant que vous
travaillez dessus plutôt que d'avoir à le faire à posteriori après qu'il ai
potentiellement été livré en production. Nous avons surnommé cet aspect de Rust
la *concurrence sans craintes*. La concurrence sans craintes vous permet
d'écrire du code dépourvu de bogues subtils et il sera facile de le remanier
sans risquer d'introduire de nouveaux bogues.

<!--
> Note: For simplicity’s sake, we’ll refer to many of the problems as
> *concurrent* rather than being more precise by saying *concurrent and/or
> parallel*. If this book were about concurrency and/or parallelism, we’d be
> more specific. For this chapter, please mentally substitute *concurrent
> and/or parallel* whenever we use *concurrent*.
-->

> Remarque : pour des raisons de simplicité, nous allons désigner la plupart
> des problèmes par *des problèmes de concurrence* plutôt que d'être trop
> précis en disant *des problèmes de concurrence et/ou de parallélisme*. Si ce
> livre traitait spécifiquement de concurrence et/ou de parallélisme, nous
> serions plus précis. Pour ce chapitre, veuillez garder à l'esprit que nous
> parlons de *concurrence et/ou de parallélisme* à chaque fois que nous
> parlerons de *concurrence*.

<!--
Many languages are dogmatic about the solutions they offer for handling
concurrent problems. For example, Erlang has elegant functionality for
message-passing concurrency but has only obscure ways to share state between
threads. Supporting only a subset of possible solutions is a reasonable
strategy for higher-level languages, because a higher-level language promises
benefits from giving up some control to gain abstractions. However, lower-level
languages are expected to provide the solution with the best performance in any
given situation and have fewer abstractions over the hardware. Therefore, Rust
offers a variety of tools for modeling problems in whatever way is appropriate
for your situation and requirements.
-->

De nombreux langages sont dogmatiques sur les solutions qu'ils offrent pour
gérer les problèmes de concurrence. Par exemple, Erlang a une fonctionnalité
élégante de passage de messages pour la concurrence mais a une façon étrange
de partager un état entre les tâches. Ne proposer qu'un sous-ensemble de
solutions possibles est une stratégie acceptable pour les langages de haut
niveau, car un langage de haut niveau offre des avantages en sacrifiant
certains contrôles pour être plus accessibles. Cependant, les langages de bas
niveau sont censés fournir la solution la plus performante dans n'importe
quelle situation donnée et proposer moins de facilités sur le matériel.
Cependant, Rust offre une gamme d'outils répondre aux problèmes de toutes les
manières possibles pour vos besoins du moment.

<!--
Here are the topics we’ll cover in this chapter:
-->

Voici les sujets que nous allons aborder dans ce chapitre :

<!--
* How to create threads to run multiple pieces of code at the same time
* *Message-passing* concurrency, where channels send messages between threads
* *Shared-state* concurrency, where multiple threads have access to some piece
  of data
* The `Sync` and `Send` traits, which extend Rust’s concurrency guarantees to
  user-defined types as well as types provided by the standard library
-->

* Comment créer des tâches pour exécuter plusieurs parties de code en même temps
* Le *passage de message* en concurrence, qui permet à plusieurs tâches
  d'accéder à la même donnée
* Les traits `Sync` et `Send`, qui étendent les garanties de Rust sur la
  concurrence autant à des types définis par les utilisateurs que par ceux qui
  sont fournis par la bibliothèque standard
