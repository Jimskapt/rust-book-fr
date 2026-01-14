<!--
## Defining Modules to Control Scope and Privacy
-->

## Définir des modules pour gérer la portée et la visibilité

<!--
In this section, we’ll talk about modules and other parts of the module system,
namely *paths* that allow you to name items; the `use` keyword that brings a
path into scope; and the `pub` keyword to make items public. We’ll also discuss
the `as` keyword, external packages, and the glob operator. For now, let’s
focus on modules!
-->

Dans cette section, nous allons aborder les modules et les autres outils du
système de modules, à savoir les *chemins* qui nous permettent de nommer les
éléments ; l'utilisation du mot-clé `use` qui importe un chemin dans la portée ;
et le mot-clé `pub` qui rend publics les éléments. Nous verrons aussi le mot-clé
`as`, les paquets externes, et l'opérateur glob. Pour commencer, penchons-nous
sur les modules !

<!--
*Modules* let us organize code within a crate into groups for readability and
easy reuse. Modules also control the *privacy* of items, which is whether an
item can be used by outside code (*public*) or is an internal implementation
detail and not available for outside use (*private*).
-->

Les *modules* nous permettent de regrouper le code d'une crate pour une
meilleure lisibilité et pour la facilité de réutilisation. Les modules
permettent aussi de gérer la *visibilité* des éléments, qui précise si un
élément peut être utilisé à l'extérieur du module (*c'est public*) ou s'il est
un constituant interne et n'est pas disponible pour une utilisation externe
(*c'est privé*).

<!--
As an example, let’s write a library crate that provides the functionality of a
restaurant. We’ll define the signatures of functions but leave their bodies
empty to concentrate on the organization of the code, rather than actually
implement a restaurant in code.
-->

Voici un exemple : écrivons une crate de bibliothèque qui permet de simuler un
restaurant. Nous allons définir les signatures des fonctions mais nous allons
laisser leurs corps vides pour nous concentrer sur l'organisation du code,
plutôt que de coder pour de vrai un restaurant.

<!--
In the restaurant industry, some parts of a restaurant are referred to as
*front of house* and others as *back of house*. Front of house is where
customers are; this is where hosts seat customers, servers take orders and
payment, and bartenders make drinks. Back of house is where the chefs and cooks
work in the kitchen, dishwashers clean up, and managers do administrative work.
-->

Dans le secteur de la restauration, certaines parties d'un restaurant sont
assimilées à la *salle à manger* et d'autres *aux cuisines*. La partie salle à
manger est l'endroit où se trouvent les clients ; c'est l'endroit où les hôtes
installent les clients, où les serveurs prennent les commandes et encaissent les
clients, et où les barmans préparent des boissons. Dans la partie cuisines, nous
retrouvons les chefs et les cuisiniers qui travaillent dans la cuisine, mais
aussi les plongeurs qui nettoient la vaisselle et les gestionnaires qui
s'occupent des tâches administratives.

<!--
To structure our crate in the same way that a real restaurant works, we can
organize the functions into nested modules. Create a new library named
`restaurant` by running `cargo new --lib restaurant`; then put the code in
Listing 7-1 into *src/lib.rs* to define some modules and function signatures.
-->

Pour organiser notre crate de la même manière qu'un vrai restaurant, nous
pouvons organiser les fonctions avec des modules imbriqués. Créez une nouvelle
bibliothèque qui s'appelle `restaurant` en utilisant
`cargo new --lib restaurant` ; puis écrivez le code de l'encart 7-1 dans
*src/lib.rs* afin de définir quelques modules et quelques signatures de
fonctions.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-1: A `front_of_house` module containing other
modules that then contain functions</span>
-->

<span class="caption">Encart 7-1 : Un module `salle_a_manger` qui contient
d'autres modules qui contiennent eux-mêmes des fonctions</span>

<!--
We define a module by starting with the `mod` keyword and then specify the
name of the module (in this case, `front_of_house`) and place curly brackets
around the body of the module. Inside modules, we can have other modules, as in
this case with the modules `hosting` and `serving`. Modules can also hold
definitions for other items, such as structs, enums, constants, traits, or—as
in Listing 7-1—functions.
-->

Nous définissons un module en commençant avec le mot-clé `mod` et nous précisons
ensuite le nom du module (dans notre cas, `salle_a_manger`) et nous ajoutons des
accolades autour du corps du module. Dans les modules, nous pouvons avoir
d'autres modules, comme dans notre cas avec les modules `accueil` et `service`.
Les modules peuvent aussi contenir des définitions pour d'autres éléments, comme
des structures, des énumérations, des constantes, des traits, ou des fonctions
(comme c'est le cas dans l'encart 7-1).

<!--
By using modules, we can group related definitions together and name why
they’re related. Programmers using this code would have an easier time finding
the definitions they wanted to use because they could navigate the code based
on the groups rather than having to read through all the definitions.
Programmers adding new functionality to this code would know where to place the
code to keep the program organized.
-->

Grâce aux modules, nous pouvons regrouper ensemble des définitions qui sont
liées et donner un nom à ce lien. Les développeurs qui utiliseront ce code
pourront plus facilement trouver les définitions dont ils ont besoin car ils
peuvent parcourir le code en fonction des groupes plutôt que d'avoir à lire
toutes les définitions. Les développeurs qui veulent rajouter des nouvelles
fonctionnalités à ce code sauront maintenant où placer le code tout en gardant
le programme organisé.

<!--
Earlier, we mentioned that *src/main.rs* and *src/lib.rs* are called crate
roots. The reason for their name is that the contents of either of these two
files form a module named `crate` at the root of the crate’s module structure,
known as the *module tree*.
-->

Précédemment, nous avons dit que *src/main.rs* et *src/lib.rs* étaient des
racines de crates. Nous les appelons ainsi car le contenu de chacun de ces
deux fichiers constituent un module qui s'appelle `crate` à la racine de
*l'arborescence du module*.

<!--
Listing 7-2 shows the module tree for the structure in Listing 7-1.
-->

L'encart 7-2 présente l'arborescence du module pour la structure de
l'encart 7-1.

<!--
```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
-->

```text
crate
 └── salle_a_manger
     ├── accueil
     │   ├── ajouter_a_la_liste_attente
     │   └── installer_a_une_table
     └── service
         ├── prendre_commande
         ├── servir_commande
         └── encaisser
```

<!--
<span class="caption">Listing 7-2: The module tree for the code in Listing
7-1</span>
-->

<span class="caption">Encart 7-2 : L'arborescence des modules pour le code de
l'encart 7-1</span>

<!--
This tree shows how some of the modules nest inside one another (for example,
`hosting` nests inside `front_of_house`). The tree also shows that some modules
are *siblings* to each other, meaning they’re defined in the same module
(`hosting` and `serving` are defined within `front_of_house`). To continue the
family metaphor, if module A is contained inside module B, we say that module A
is the *child* of module B and that module B is the *parent* of module A.
Notice that the entire module tree is rooted under the implicit module named
`crate`.
-->

Cette arborescence montre comment les modules sont imbriqués entre eux (par
exemple, `accueil` est imbriqué dans `salle_a_manger`). L'arborescence montre
aussi que certains modules sont les *frères* d'autres modules, ce qui veut dire
qu'ils sont définis dans le même module (`accueil` et `service` sont définis
dans `salle_a_manger`). Pour prolonger la métaphore familiale, si le module A
est contenu dans le module B, on dit que le module A est *l'enfant* du module B
et que ce module B est le *parent* du module A. Notez aussi que le module
implicite nommé `crate` est le parent de toute cette arborescence.

<!--
The module tree might remind you of the filesystem’s directory tree on your
computer; this is a very apt comparison! Just like directories in a filesystem,
you use modules to organize your code. And just like files in a directory, we
need a way to find our modules.
-->

L'arborescence des modules peut rappeler les répertoires du système de fichiers de
votre ordinateur ; et c'est une excellente comparaison ! Comme les répertoires dans
un système de fichiers, vous utilisez les modules pour organiser votre code. Et
comme pour les fichiers dans un répertoire, nous avons besoin d'un moyen de trouver
nos modules.
