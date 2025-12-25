<!--
# Generic Types, Traits, and Lifetimes
-->

# Les types génériques, les traits et les durées de vie

<!--
Every programming language has tools for effectively handling the duplication
of concepts. In Rust, one such tool is *generics*. Generics are abstract
stand-ins for concrete types or other properties. When we’re writing code, we
can express the behavior of generics or how they relate to other generics
without knowing what will be in their place when compiling and running the code.
-->

Tous les langages de programmation ont des outils pour gérer la duplication des
concepts. En Rust, un de ces outils est la *généricité*. La généricité permet
de remplacer des types concrets ou d'autres propriétés par des paramètres
abstraits appelés *génériques*. Lorsque nous écrivons du code, nous pouvons
exprimer le comportement des génériques, ou comment ils interagissent avec
d'autres génériques, sans savoir ce qu'il y aura à leur place lors de la
compilation et de l'exécution du code.

<!--
Similar to the way a function takes parameters with unknown values to run the
same code on multiple concrete values, functions can take parameters of some
generic type instead of a concrete type, like `i32` or `String`. In fact, we’ve
already used generics in Chapter 6 with `Option<T>`, Chapter 8 with `Vec<T>`
and `HashMap<K, V>`, and Chapter 9 with `Result<T, E>`. In this chapter, you’ll
explore how to define your own types, functions, and methods with generics!
-->

De la même manière qu'une fonction prend des paramètres avec des valeurs
inconnues pour exécuter le même code sur plusieurs valeurs concrètes, les
fonctions peuvent prendre des paramètres d'un type générique plutôt que d'un
type concret comme `i32` ou `String`. En fait, nous avons déjà utilisé des types
génériques au chapitre 6 avec `Option<T>`, au chapitre 8 avec `Vec<T>` et
`HashMap<K, V>`, et au chapitre 9 avec `Result<T, E>`. Dans ce chapitre, nous
allons voir comment définir nos propres types, fonctions et méthodes utilisant
des types génériques !

<!--
First, we’ll review how to extract a function to reduce code duplication. Next,
we’ll use the same technique to make a generic function from two functions that
differ only in the types of their parameters. We’ll also explain how to use
generic types in struct and enum definitions.
-->

Pour commencer, nous allons examiner comment construire une fonction pour
réduire la duplication de code. Ensuite, nous utiliserons la même technique pour
construire une fonction générique à partir de deux fonctions qui se distinguent
uniquement par le type de leurs paramètres. Nous expliquerons aussi comment
utiliser les types génériques dans les définitions de structures et
d'énumérations.

<!--
Then you’ll learn how to use *traits* to define behavior in a generic way. You
can combine traits with generic types to constrain a generic type to only
those types that have a particular behavior, as opposed to just any type.
-->

Ensuite, vous apprendrez comment utiliser les *traits* pour définir un
comportement de manière générique. Vous pouvez combiner les traits avec des
types génériques pour contraindre un type générique uniquement à des types qui
ont un comportement particulier, et non pas accepter n'importe quel type.

<!--
Finally, we’ll discuss *lifetimes*, a variety of generics that give the
compiler information about how references relate to each other. Lifetimes allow
us to borrow values in many situations while still enabling the compiler to
check that the references are valid.
-->

Enfin, nous verrons les *durées de vie*, un genre de générique qui indique au
compilateur comment les références s'articulent entre elles. Les durées de vie
nous permettent d'emprunter des valeurs dans différentes situations tout en
donnant les éléments au compilateur pour vérifier que les références sont
toujours valides.

<!--
## Removing Duplication by Extracting a Function
-->

## Supprimer les doublons en construisant une fonction

<!--
Before diving into generics syntax, let’s first look at how to remove
duplication that doesn’t involve generic types by extracting a function. Then
we’ll apply this technique to extract a generic function! In the same way that
you recognize duplicated code to extract into a function, you’ll start to
recognize duplicated code that can use generics.
-->

Avant de plonger dans la syntaxe des génériques, nous allons regarder comment
supprimer les doublons, sans utiliser de types génériques, en construisant une
fonction. Ensuite, nous allons appliquer cette technique pour construire une
fonction générique ! De la même manière que vous détectez du code dupliqué pour
l'extraire dans une fonction, vous allez commencer à reconnaître du code
dupliqué qui peut utiliser la généricité.

<!--
Consider a short program that finds the largest number in a list, as shown in
Listing 10-1.
-->

Imaginons un petit programme qui trouve le nombre le plus grand dans une liste,
comme dans l'encart 10-1.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier: src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-1: Code to find the largest number in a list
of numbers</span>
-->

<span class="caption">Encart 10-1 : Le code pour trouver le nombre le plus grand
dans une liste de nombres</span>

<!--
This code stores a list of integers in the variable `number_list` and places
the first number in the list in a variable named `largest`. Then it iterates
through all the numbers in the list, and if the current number is greater than
the number stored in `largest`, it replaces the number in that variable.
However, if the current number is less than or equal to the largest number seen
so far, the variable doesn’t change, and the code moves on to the next number
in the list. After considering all the numbers in the list, `largest` should
hold the largest number, which in this case is 100.
-->

Ce code stocke une liste de nombres entiers dans la variable `liste_de_nombres`
et place le premier nombre de la liste dans une variable qui s'appelle
`le_plus_grand`. Ensuite, il parcourt tous les nombres dans la liste, et si le
nombre courant est plus grand que le nombre stocké dans `le_plus_grand`, il
remplace le nombre dans cette variable. Cependant, si le nombre courant est
plus petit ou égal au nombre le plus grand trouvé précédemment, la variable ne
change pas, et le code passe au nombre suivant de la liste. Après avoir parcouru
tous les nombres de la liste, `le_plus_grand` devrait stocker le plus grand
nombre, qui est 100 dans notre cas.

<!--
To find the largest number in two different lists of numbers, we can duplicate
the code in Listing 10-1 and use the same logic at two different places in the
program, as shown in Listing 10-2.
-->

Pour trouver le nombre le plus grand dans deux listes de nombres différentes,
nous pourrions dupliquer le code de l'encart 10-1 et suivre la même logique à
deux endroits différents du programme, comme dans l'encart 10-2.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

<!--
<span class="caption">Listing 10-2: Code to find the largest number in *two*
lists of numbers</span>
-->

<span class="caption">Encart 10-2 : Le code pour trouver le plus grand nombre
dans *deux* listes de nombres</span>

<!--
Although this code works, duplicating code is tedious and error prone. We also
have to update the code in multiple places when we want to change it.
-->

Bien que ce code fonctionne, la duplication de code est fastidieuse et source
d'erreurs. Nous devons aussi mettre à jour le code à plusieurs endroits si nous
souhaitons le modifier.

<!--
To eliminate this duplication, we can create an abstraction by defining a
function that operates on any list of integers given to it in a parameter. This
solution makes our code clearer and lets us express the concept of finding the
largest number in a list abstractly.
-->

Pour éviter cette duplication, nous pouvons créer un niveau d'abstraction en
définissant une fonction qui travaille avec n'importe quelle liste de nombres
entiers qu'on lui donne en paramètre. Cette solution rend notre code plus clair
et nous permet d'exprimer le concept de trouver le nombre le plus grand dans une
liste de manière abstraite.

<!--
In Listing 10-3, we extracted the code that finds the largest number into a
function named `largest`. Unlike the code in Listing 10-1, which can find the
largest number in only one particular list, this program can find the largest
number in two different lists.
-->

Dans l'encart 10-3, nous avons extrait le code qui trouve le nombre le plus
grand dans une fonction qui s'appelle `le_plus_grand`. Contrairement au code de
l'encart 10-1, qui pouvait trouver le nombre le plus grand dans une seule liste
en particulier, ce programme peut trouver le nombre le plus grand dans deux
listes différentes.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-3: Abstracted code to find the largest number
in two lists</span>
-->

<span class="caption">Encart 10-3 : du code abstrait qui trouve le plus grand
nombre dans deux listes</span>

<!--
The `largest` function has a parameter called `list`, which represents any
concrete slice of `i32` values that we might pass into the function. As a
result, when we call the function, the code runs on the specific values that we
pass in. Don’t worry about the syntax of the `for` loop for now. We aren’t
referencing a reference to an `i32` here; we’re pattern matching and
destructuring each `&i32` that the `for` loop gets so that `item` will be an
`i32` inside the loop body. We’ll cover pattern matching in detail in [Chapter
18][ch18]<!-- ignore -- >.
-->

La fonction `le_plus_grand` a un paramètre qui s'appelle `liste`, qui représente
n'importe quelle slice concrète de valeurs `i32` que nous pouvons passer à la
fonction. Au final, lorsque nous appelons la fonction, le code s'exécute sur les
valeurs précises que nous lui avons fournies. Mais ne nous préoccupons pas de
la syntaxe de la boucle `for` pour l'instant. Ici, nous n'utilisons pas une
référence vers un `i32`, nous destructurons via le filtrage par motif chaque
`&i32` afin que la boucle `for` utilise cet `element` en tant que `i32` dans le
corps de la boucle. Nous parlerons plus en détails du filtrage par motif au
[chapitre 18][ch18]<!-- ignore -->.

<!--
In sum, here are the steps we took to change the code from Listing 10-2 to
Listing 10-3:
-->

En résumé, voici les étapes que nous avons suivies pour changer le code de
l'encart 10-2 pour obtenir celui de l'encart 10-3 :

<!--
1. Identify duplicate code.
2. Extract the duplicate code into the body of the function and specify the
   inputs and return values of that code in the function signature.
3. Update the two instances of duplicated code to call the function instead.
-->

1. Identification du code dupliqué.
2. Extraction du code dupliqué dans le corps de la fonction et ajout de
   précisions sur les entrées et les valeurs de retour de ce code dans la
   signature de la fonction.
3. Remplacement des deux instances du code dupliqué par des appels à la
   fonction.

<!--
Next, we’ll use these same steps with generics to reduce code duplication in
different ways. In the same way that the function body can operate on an
abstract `list` instead of specific values, generics allow code to operate on
abstract types.
-->

Ensuite, nous allons utiliser les mêmes étapes avec la généricité pour réduire
la duplication de code de différentes façons. De la même manière que le corps
d'une fonction peut opérer sur une `liste` abstraite plutôt que sur des valeurs
spécifiques, la généricité permet de travailler sur des types abstraits.

<!--
For example, say we had two functions: one that finds the largest item in a
slice of `i32` values and one that finds the largest item in a slice of `char`
values. How would we eliminate that duplication? Let’s find out!
-->

Par exemple, imaginons que nous ayons deux fonctions : une qui trouve l'élément
le plus grand dans une slice de valeurs `i32` et une qui trouve l'élément le
plus grand dans une slice de valeurs `char`. Comment pourrions-nous éviter la
duplication ? Voyons cela dès maintenant !

<!--
[ch18]: ch18-00-patterns.html
-->

[ch18]: ch18-00-patterns.html
