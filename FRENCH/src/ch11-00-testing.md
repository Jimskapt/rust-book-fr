<!--
# Writing Automated Tests
-->

# Écrire des tests automatisés

<!--
In his 1972 essay “The Humble Programmer,” Edsger W. Dijkstra said that
“Program testing can be a very effective way to show the presence of bugs, but
it is hopelessly inadequate for showing their absence.” That doesn’t mean we
shouldn’t try to test as much as we can!
-->

Dans son essai de 1972 “The Humble Programmer”, Edsger W. Dijkstra a dit qu'un
“test de programme peut être une manière très efficace de prouver la présence de
bogues, mais qu'il est totalement inadéquat pour prouver leur absence”. Mais
cela ne veut pas dire que nous ne devrions pas tester notre programme autant que
faire se peut !

<!--
Correctness in our programs is the extent to which our code does what we intend
it to do. Rust is designed with a high degree of concern about the correctness
of programs, but correctness is complex and not easy to prove. Rust’s type
system shoulders a huge part of this burden, but the type system cannot catch
every kind of incorrectness. As such, Rust includes support for writing
automated software tests within the language.
-->

L'exactitude de nos programmes est le niveau de conformité de notre code par
rapport à ce que nous voulons qu'il fasse. Rust est conçu dans un grand souci
d'exactitude des programmes, mais l'exactitude est complexe et difficile à
confirmer. Le système de type de Rust endosse une grande partie de cette charge,
mais le système de type ne peut pas détecter tous les genres d'erreurs. Ainsi,
Rust embarque des fonctionnalités pour écrire des tests automatisés de logiciels
à l'intérieur du langage.

<!--
As an example, say we write a function called `add_two` that adds 2 to whatever
number is passed to it. This function’s signature accepts an integer as a
parameter and returns an integer as a result. When we implement and compile
that function, Rust does all the type checking and borrow checking that you’ve
learned so far to ensure that, for instance, we aren’t passing a `String` value
or an invalid reference to this function. But Rust *can’t* check that this
function will do precisely what we intend, which is return the parameter plus 2
rather than, say, the parameter plus 10 or the parameter minus 50! That’s where
tests come in.
-->

Par exemple, imaginons que nous écrivons une fonction `ajouter_deux` qui ajoute
2 à n'importe quel nombre qu'on lui envoie. La signature de cette fonction
prend un entier en paramètre et retourne un entier comme résultat. Lorsque nous
implémentons et compilons cette fonction, Rust fait toutes les vérifications de
type et d'emprunt que vous avez apprises précédemment afin de s'assurer que, par
exemple, nous ne passions pas une valeur de type `String` ou une référence
invalide à cette fonction. Mais Rust *ne peut pas* vérifier que cette fonction
va faire précisément ce que nous avions prévu qu'elle fasse, qui en l'occurence
est de retourner le paramètre incrémenté de 2 plutôt que d'ajouter 10 ou
d'enlever 50, par exemple ! C'est pour cette situation que les tests sont
utiles.

<!--
We can write tests that assert, for example, that when we pass `3` to the
`add_two` function, the returned value is `5`. We can run these tests whenever
we make changes to our code to make sure any existing correct behavior has not
changed.
-->

Nous pouvons écrire des tests qui vérifient, par exemple, que lorsque nous
donnons `3` à la fonction `ajouter_deux`, elle retourne bien `5`. Nous pouvons
lancer ces tests à chaque fois que nous modifions notre code pour nous assurer
qu'aucun comportement existant et satisfaisant n'a changé.

<!--
Testing is a complex skill: although we can’t cover every detail about how to
write good tests in one chapter, we’ll discuss the mechanics of Rust’s testing
facilities. We’ll talk about the annotations and macros available to you when
writing your tests, the default behavior and options provided for running your
tests, and how to organize tests into unit tests and integration tests.
-->

Les tests restent une discipline complexe : bien que nous ne puissions couvrir
chaque détail sur l'écriture de bons tests en un seul chapitre, nous allons
découvrir les  mécanismes des moyens de test de Rust. Nous allons voir les
annotations et les macros que vous pourrez utiliser lorsque vous écrirez vos
tests, le comportement par défaut et les options disponibles pour lancer vos
tests, et comment organiser les tests en tests unitaires et tests d'intégration.
