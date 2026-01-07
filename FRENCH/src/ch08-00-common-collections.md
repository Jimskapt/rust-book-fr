<!--
# Common Collections
-->

# Les collections standard

<!--
Rust’s standard library includes a number of very useful data structures called
*collections*. Most other data types represent one specific value, but
collections can contain multiple values. Unlike the built-in array and tuple
types, the data these collections point to is stored on the heap, which means
the amount of data does not need to be known at compile time and can grow or
shrink as the program runs. Each kind of collection has different capabilities
and costs, and choosing an appropriate one for your current situation is a
skill you’ll develop over time. In this chapter, we’ll discuss three
collections that are used very often in Rust programs:
-->

La bibliothèque standard de Rust apporte quelques structures de données très
utiles appelées *collections*. La plupart des autres types de données
représentent une seule valeur précise, mais les collections peuvent contenir
plusieurs valeurs. Contrairement aux tableaux et aux tuples, les données que ces
collections contiennent sont stockées sur le tas, ce qui veut dire que la
quantité de données n'a pas à être connue au moment de la compilation et peut
augmenter ou diminuer pendant l'exécution du programme. Chaque type de
collection a ses avantages et ses inconvénients, et en choisir un qui répond à
votre besoin sur le moment est une aptitude que vous allez développer avec le
temps. Dans ce chapitre, nous allons découvrir trois collections qui sont très
utilisées dans les programmes Rust :

<!--
* A *vector* allows you to store a variable number of values next to each other.
* A *string* is a collection of characters. We’ve mentioned the `String` type
  previously, but in this chapter we’ll talk about it in depth.
* A *hash map* allows you to associate a value with a particular key. It’s a
  particular implementation of the more general data structure called a *map*.
-->

* Le *vecteur* qui vous permet de stocker un nombre variable de valeurs les unes
  à côté des autres.
* La *String*, qui est une collection de caractères. Nous avons déjà aperçu le
  type `String` précédemment, mais dans ce chapitre, nous allons l'étudier en
  détail.
* La *table de hachage* qui vous permet d'associer une valeur à une clé précise.
  C'est une implémentation spécifique d'une structure de données plus
  générique : le *tableau associatif*.

<!--
To learn about the other kinds of collections provided by the standard library,
see [the documentation][collections].
-->

Pour en savoir plus sur les autres types de collections fournies par la
bibliothèque standard, allez voir [la documentation][collections].

<!--
We’ll discuss how to create and update vectors, strings, and hash maps, as well
as what makes each special.
-->

Nous allons voir comment créer et modifier les vecteurs, les Strings et les
tables de hachage, et étudier leurs différences.

<!--
[collections]: ../std/collections/index.html
-->

[collections]: https://doc.rust-lang.org/std/collections/index.html
