<!--
# Functional Language Features: Iterators and Closures
-->

# Les fonctionnalités des langages fonctionnels : les itérateurs et les fermetures

<!--
Rust’s design has taken inspiration from many existing languages and
techniques, and one significant influence is *functional programming*.
Programming in a functional style often includes using functions as values by
passing them in arguments, returning them from other functions, assigning them
to variables for later execution, and so forth.
-->

La conception de Rust s'est inspirée de nombreux langages et technologies
existantes, et une de ses influences la plus marquante est la *programmation
fonctionnelle*. La programmation fonctionnelle consiste souvent à utiliser une
fonction comme une valeur en la passant en argument d'une autre fonction, la
retourner en résultat d'une autre fonction, l'assigner à une variable pour
l'exécuter plus tard, par exemple.

<!--
In this chapter, we won’t debate the issue of what functional programming is or
isn’t but will instead discuss some features of Rust that are similar to
features in many languages often referred to as functional.
-->

Dans ce chapitre, nous n'allons pas débattre sur ce qu'est ou non la
programmation fonctionnelle, mais nous allons plutôt voir quelques
fonctionnalités de Rust qui sont similaires à celles des autres langages souvent
considérés comme fonctionnels.

<!--
More specifically, we’ll cover:
-->

Plus précisément, nous allons voir :

<!--
* *Closures*, a function-like construct you can store in a variable
* *Iterators*, a way of processing a series of elements
* How to use these two features to improve the I/O project in Chapter 12
* The performance of these two features (Spoiler alert: they’re faster than you
  might think!)
-->

* *les fermetures*, une construction qui ressemble à une fonction que vous
  pouvez stocker dans une variable
* *les itérateurs*, une façon de travailler sur une série d'éléments
* Comment utiliser ces deux fonctionnalités pour améliorer le projet
  d'entrée/sortie du chapitre 12
* Etudier la performance de ces deux fonctionnalités (divulgâchage : elles sont
  probablement plus rapides que ce que vous pensez !)

<!--
Other Rust features, such as pattern matching and enums, which we’ve covered in
other chapters, are influenced by the functional style as well. Mastering
closures and iterators is an important part of writing idiomatic, fast Rust
code, so we’ll devote this entire chapter to them.
-->

Les autres fonctionnalités de Rust, comme le filtrage par motif et les
énumérations que nous avons vu dans les chapitres précédents, sont influencés
par la programmation fonctionnelle. La maîtrise des fermetures et des itérateurs
est une étape importante pour écrire du code Rust performant, c'est pourquoi
nous allons leur dédier ce chapitre entier.
