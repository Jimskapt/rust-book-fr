<!--
# Error Handling
-->

# La gestion des erreurs

<!--
Errors are a fact of life in software, so Rust has a number of features for
handling situations in which something goes wrong. In many cases, Rust requires
you to acknowledge the possibility of an error and take some action before your
code will compile. This requirement makes your program more robust by ensuring
that you’ll discover errors and handle them appropriately before you’ve
deployed your code to production!
-->

Les erreurs font partie de la vie des programmes informatiques, c'est pourquoi
Rust a des fonctionnalités pour gérer les situations dans lesquelles quelque
chose dérape. Dans de nombreux cas, Rust exige que vous anticipiez les erreurs
possibles et que vous preniez des dispositions avant de pouvoir compiler votre
code. Cette exigence rend votre programme plus résilient en s'assurant que vous
détectez et gérez les erreurs correctement avant même que vous ne déployiez
votre code en production !

<!--
Rust groups errors into two major categories: *recoverable* and *unrecoverable*
errors. For a recoverable error, such as a *file not found* error, we most
likely just want to report the problem to the user and retry the operation.
Unrecoverable errors are always symptoms of bugs, like trying to access a
location beyond the end of an array, and so we want to immediately stop the
program.
-->

Rust classe les erreurs dans deux catégories principales : les erreurs
*récupérables* et *irrécupérables*. Pour les erreurs récupérables, comme
l'erreur *le fichier n'a pas été trouvé*, nous préférons probablement signaler
le problème à l'utilisateur et relancer l'opération. Les erreurs irrécupérables
sont toujours des symptômes de bogues, comme par exemple essayer d'accéder à un
élément en dehors de l'intervalle de données d'un tableau : dans un tel cas,
nous voulons arrêter immédiatement l'exécution du programme.

<!--
Most languages don’t distinguish between these two kinds of errors and handle
both in the same way, using mechanisms such as exceptions. Rust doesn’t have
exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and
the `panic!` macro that stops execution when the program encounters an
unrecoverable error. This chapter covers calling `panic!` first and then talks
about returning `Result<T, E>` values. Additionally, we’ll explore
considerations when deciding whether to try to recover from an error or to stop
execution.
-->

La plupart des langages de programmation ne font pas de distinction entre ces
deux types d'erreurs et les gèrent de la même manière, en utilisant des
fonctionnalités comme les exceptions. Rust n'a pas d'exception. À la place, il
a les types `Result<T, E>` pour les erreurs récupérables, et la macro `panic!`
qui arrête l'exécution quand le programme se heurte à des erreurs
irrécupérables. Nous allons commencer ce chapitre par expliquer l'utilisation de
`panic!`, puis nous allons voir les valeurs de retour `Result<T, E>`. Enfin,
nous allons voir les éléments à prendre en compte pour décider si nous devons
essayer de rattraper une erreur ou alors arrêter l'exécution.
