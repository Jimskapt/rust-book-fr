<!--
## Paths for Referring to an Item in the Module Tree
-->

## Désigner un élément dans l'arborescence de modules

<!--
To show Rust where to find an item in a module tree, we use a path in the same
way we use a path when navigating a filesystem. If we want to call a function,
we need to know its path.
-->

Pour indiquer à Rust où trouver un élément dans l'arborescence de modules, nous
utilisons un chemin à l'instar des chemins que nous utilisons lorsque nous
naviguons dans un système de fichiers. Si nous voulons appeler une fonction,
nous avons besoin de connaître son chemin.

<!--
A path can take two forms:
-->

Il existe deux types de chemins :

<!--
* An *absolute path* starts from a crate root by using a crate name or a
  literal `crate`.
* A *relative path* starts from the current module and uses `self`, `super`, or
  an identifier in the current module.
-->

* Un *chemin absolu* qui commence à partir de la racine de la crate en utilisant
  le nom d'une crate, ou le mot `crate`.
* Un *chemin relatif* qui commence à partir du module courant et qui utilise
  `self`, `super`, ou un identificateur à l'intérieur du module.

<!--
Both absolute and relative paths are followed by one or more identifiers
separated by double colons (`::`).
-->

Les chemins absolus et relatifs sont suivis par un ou plusieurs identificateurs
séparés par `::`.

<!--
Let’s return to the example in Listing 7-1. How do we call the
`add_to_waitlist` function? This is the same as asking, what’s the path of the
`add_to_waitlist` function? In Listing 7-3, we simplified our code a bit by
removing some of the modules and functions. We’ll show two ways to call the
`add_to_waitlist` function from a new function `eat_at_restaurant` defined in
the crate root. The `eat_at_restaurant` function is part of our library crate’s
public API, so we mark it with the `pub` keyword. In the [”Exposing Paths with
the `pub` Keyword”][pub]<!-- ignore -- > section, we’ll go into more detail
about `pub`. Note that this example won’t compile just yet; we’ll explain why
in a bit.
-->

Reprenons notre exemple de l'encart 7-1. Comment pouvons-nous appeler la
fonction `ajouter_a_la_liste_attente` ? Cela revient à se demander : quel est le
chemin de la fonction `ajouter_a_la_liste_attente` ? Dans l'encart 7-3, nous
avons un peu simplifié notre code en enlevant quelques modules et quelques
fonctions. Nous allons voir deux façons d'appeler la fonction
`ajouter_a_la_liste_attente` à partir d'une nouvelle fonction
`manger_au_restaurant` définie à la racine de la crate. La fonction
`manger_au_restaurant` fait partie de l'API publique de notre crate de
bibliothèque, donc nous la marquons avec le mot-clé `pub`. Dans la section
[”Exposer les chemins avec le mot-clé `pub`”][pub]<!-- ignore -->, nous en
apprendrons plus sur `pub`. Notez que cet exemple ne se compile pas pour le
moment ; nous allons l'expliquer un peu plus tard.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-3: Calling the `add_to_waitlist` function using
absolute and relative paths</span>
-->

<span class="caption">Encart 7-3 : Appel à la fonction
`ajouter_a_la_liste_attente` en utilisant un chemin absolu et relatif</span>

<!--
The first time we call the `add_to_waitlist` function in `eat_at_restaurant`,
we use an absolute path. The `add_to_waitlist` function is defined in the same
crate as `eat_at_restaurant`, which means we can use the `crate` keyword to
start an absolute path.
-->

Au premier appel de la fonction `ajouter_a_la_liste_attente` dans
`manger_au_restaurant`, nous utilisons un chemin absolu. La fonction
`ajouter_a_la_liste_attente` est définie dans la même crate que
`manger_au_restaurant`, ce qui veut dire que nous pouvons utiliser le mot-clé
`crate` pour démarrer un chemin absolu.

<!--
After `crate`, we include each of the successive modules until we make our way
to `add_to_waitlist`. You can imagine a filesystem with the same structure, and
we’d specify the path `/front_of_house/hosting/add_to_waitlist` to run the
`add_to_waitlist` program; using the `crate` name to start from the crate root
is like using `/` to start from the filesystem root in your shell.
-->

Après `crate`, nous ajoutons chacun des modules successifs jusqu'à
`ajouter_a_la_liste_attente`. Nous pouvons faire l'analogie avec un système de
fichiers qui aurait la même structure, où nous pourrions utiliser le chemin
`/salle_a_manger/accueil/ajouter_a_la_liste_attente` pour lancer le programme
`ajouter_a_la_liste_attente` ; utiliser le nom `crate` pour partir de la racine
de la crate revient à utiliser `/` pour partir de la racine de votre système de
fichiers dans votre invite de commande.

<!--
The second time we call `add_to_waitlist` in `eat_at_restaurant`, we use a
relative path. The path starts with `front_of_house`, the name of the module
defined at the same level of the module tree as `eat_at_restaurant`. Here the
filesystem equivalent would be using the path
`front_of_house/hosting/add_to_waitlist`. Starting with a name means that the
path is relative.
-->

Lors du second appel à `ajouter_a_la_liste_attente` dans `manger_au_restaurant`,
nous utilisons un chemin relatif. Le chemin commence par `salle_a_manger`, le
nom du module qui est défini au même niveau que `manger_au_restaurant` dans
l'arborescence de modules. Ici, l'équivalent en terme de système de fichier
serait le chemin `salle_a_manger/accueil/ajouter_a_la_liste_attente`. Commencer
par un nom signifie que le chemin est relatif.

<!--
Choosing whether to use a relative or absolute path is a decision you’ll make
based on your project. The decision should depend on whether you’re more likely
to move item definition code separately from or together with the code that
uses the item. For example, if we move the `front_of_house` module and the
`eat_at_restaurant` function into a module named `customer_experience`, we’d
need to update the absolute path to `add_to_waitlist`, but the relative path
would still be valid. However, if we moved the `eat_at_restaurant` function
separately into a module named `dining`, the absolute path to the
`add_to_waitlist` call would stay the same, but the relative path would need to
be updated. Our preference is to specify absolute paths because it’s more
likely to move code definitions and item calls independently of each other.
-->

Choisir entre utiliser un chemin relatif ou absolu sera une décision que vous
ferez en fonction de votre projet. Le choix se fera en fonction de si vous êtes
susceptible de déplacer la définition de l'élément souhaité séparément ou en
même temps que le code qui l'utilise. Par exemple, si nous déplaçons le module
`salle_a_manger` ainsi que la fonction `manger_au_restaurant` dans un module qui
s'appelle `experience_client`, nous aurons besoin de mettre à jour le chemin
absolu vers `ajouter_a_la_liste_attente`, mais le chemin relatif restera valide.
Cependant, si nous avions déplacé uniquement la fonction `manger_au_restaurant`
dans un module `repas` séparé, le chemin absolu de l'appel à
`ajouter_a_la_liste_attente` restera le même, mais le chemin relatif aura besoin
d'être mis à jour. Notre préférence est d'utiliser un chemin absolu car il est
plus facile de déplacer les définitions de code et les appels aux éléments
indépendamment les uns des autres.

<!--
Let’s try to compile Listing 7-3 and find out why it won’t compile yet! The
error we get is shown in Listing 7-4.
-->

Essayons de compiler l'encart 7-3 et essayons de comprendre pourquoi il ne se
compile pas pour le moment ! L'erreur que nous obtenons est affichée dans
l'encart 7-4.

<!--
```console
{{#include ../listings-sources/ch07-managing-growing-projects/listing-07-03/output.txt}}
```
-->

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<!--
<span class="caption">Listing 7-4: Compiler errors from building the code in
Listing 7-3</span>
-->

<span class="caption">Encart 7-4 : Les erreurs de compilation du code de
l'encart 7-3</span>

<!--
The error messages say that module `hosting` is private. In other words, we
have the correct paths for the `hosting` module and the `add_to_waitlist`
function, but Rust won’t let us use them because it doesn’t have access to the
private sections.
-->

Le message d'erreur nous rappelle que ce module `accueil` est privé. Autrement
dit, nous avons des chemins corrects pour le module `accueil` et pour la
fonction `ajouter_a_la_liste_attente`, mais Rust ne nous laisse pas les utiliser
car il n'a pas accès aux sections privées.

<!--
Modules aren’t useful only for organizing your code. They also define Rust’s
*privacy boundary*: the line that encapsulates the implementation details
external code isn’t allowed to know about, call, or rely on. So, if you want to
make an item like a function or struct private, you put it in a module.
-->

Les modules ne servent pas uniquement à organiser votre code. Ils définissent
aussi les *limites de visibilité* de Rust : le code externe n'est pas autorisé
à connaître, à appeler ou à se fier à des éléments internes au module. Donc, si
vous voulez rendre un élément privé comme une fonction ou une structure, vous
devez le placer dans un module.

<!--
The way privacy works in Rust is that all items (functions, methods, structs,
enums, modules, and constants) are private by default. Items in a parent module
can’t use the private items inside child modules, but items in child modules
can use the items in their ancestor modules. The reason is that child modules
wrap and hide their implementation details, but the child modules can see the
context in which they’re defined. To continue with the restaurant metaphor,
think of the privacy rules as being like the back office of a restaurant: what
goes on in there is private to restaurant customers, but office managers can
see and do everything in the restaurant in which they operate.
-->

La visibilité en Rust fait en sorte que tous les éléments (fonctions,
méthodes, structures, énumérations, modules et constantes) sont privés par
défaut. Les éléments dans un module parent ne peuvent pas utiliser les éléments
privés dans les modules enfants, mais les éléments dans les modules enfants
peuvent utiliser les éléments dans les modules parents. C'est parce que les
modules enfants englobent et cachent les détails de leur implémentation, mais
les modules enfants peuvent voir dans quel contexte ils sont définis. Pour
continuer la métaphore du restaurant, considérez que les règles de visibilité de
Rust fonctionnent comme les cuisines d'un restaurant : ce qui s'y passe n'est
pas connu des clients, mais les gestionnaires peuvent tout voir et tout faire
dans le restaurant dans lequel ils travaillent.

<!--
Rust chose to have the module system function this way so that hiding inner
implementation details is the default. That way, you know which parts of the
inner code you can change without breaking outer code. But you can expose inner
parts of child modules’ code to outer ancestor modules by using the `pub`
keyword to make an item public.
-->

Rust a décidé de faire fonctionner le système de modules de façon à ce que les
détails d'implémentation interne sont cachés par défaut. Ainsi, vous savez
quelles parties du code interne vous pouvez changer sans casser le code externe.
Mais vous pouvez exposer aux parents des parties internes des modules enfants en
utilisant le mot-clé `pub` afin de les rendre publiques.

<!--
### Exposing Paths with the `pub` Keyword
-->

### Exposer des chemins avec le mot-clé `pub`

<!--
Let’s return to the error in Listing 7-4 that told us the `hosting` module is
private. We want the `eat_at_restaurant` function in the parent module to have
access to the `add_to_waitlist` function in the child module, so we mark the
`hosting` module with the `pub` keyword, as shown in Listing 7-5.
-->

Retournons à l'erreur de l'encart 7-4 qui nous informe que le module `accueil`
est privé. Nous voulons que la fonction `manger_au_restaurant` du module parent
ait accès à la fonction `ajouter_a_la_liste_attente` du module enfant, donc nous
utilisons le mot-clé `pub` sur le module `accueil`, comme dans l'encart 7-5.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-5: Declaring the `hosting` module as `pub` to
use it from `eat_at_restaurant`</span>
-->

<span class="caption">Encart 7-5 : Utiliser `pub` sur le module `accueil` permet
de l'utiliser dans `manger_au_restaurant`</span>

<!--
Unfortunately, the code in Listing 7-5 still results in an error, as shown in
Listing 7-6.
-->

Malheureusement, il reste une erreur dans le code de l'encart 7-5, la voici dans
l'encart 7-6.

<!--
```console
{{#include ../listings-sources/ch07-managing-growing-projects/listing-07-05/output.txt}}
```
-->

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<!--
<span class="caption">Listing 7-6: Compiler errors from building the code in
Listing 7-5</span>
-->

<span class="caption">Encart 7-6 : Erreurs de compilation du code de l'encart
7-5</span>

<!--
What happened? Adding the `pub` keyword in front of `mod hosting` makes the
module public. With this change, if we can access `front_of_house`, we can
access `hosting`. But the *contents* of `hosting` are still private; making the
module public doesn’t make its contents public. The `pub` keyword on a module
only lets code in its ancestor modules refer to it.
-->

Que s'est-il passé ? Ajouter le mot-clé `pub` devant `mod accueil` rend public
le module. Avec cette modification, si nous pouvons accéder à `salle_a_manger`,
alors nous pouvons accéder à `accueil`. Mais le *contenu* de `accueil` reste
privé ; rendre le module public ne rend pas son contenu public. Le mot-clé `pub`
sur un module permet uniquement au code de ses parents d'y faire référence.

<!--
The errors in Listing 7-6 say that the `add_to_waitlist` function is private.
The privacy rules apply to structs, enums, functions, and methods as well as
modules.
-->

Les erreurs dans l'encart 7-6 nous informent que la fonction
`ajouter_a_la_liste_attente` est privée. Les règles de visibilité s'appliquent
aussi bien aux modules qu'aux structures, énumérations, fonctions et méthodes.

<!--
Let’s also make the `add_to_waitlist` function public by adding the `pub`
keyword before its definition, as in Listing 7-7.
-->

Rendons publique la fonction `ajouter_a_la_liste_attente`, en ajoutant le
mot-clé `pub` devant sa définition, comme dans l'encart 7-7.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground,test_harness
{{#rustdoc_include ../listings-sources/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```
-->

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-7: Adding the `pub` keyword to `mod hosting`
and `fn add_to_waitlist` lets us call the function from
`eat_at_restaurant`</span>
-->

<span class="caption">Encart 7-7 : Ajout du mot-clé `pub` devant `mod accueil`
et `fn ajouter_a_la_liste_attente` pour nous permettre d'appeler la fonction à
partir de `manger_au_restaurant`</span>

<!--
Now the code will compile! Let’s look at the absolute and the relative path and
double-check why adding the `pub` keyword lets us use these paths in
`add_to_waitlist` with respect to the privacy rules.
-->

Maintenant, le code va compiler ! Analysons les chemins relatif et absolu et
vérifions pourquoi l'ajout du mot-clé `pub` nous permet d'utiliser ces chemins
dans `ajouter_a_la_liste_attente` tout en respectant les règles de visibilité.

<!--
In the absolute path, we start with `crate`, the root of our crate’s module
tree. Then the `front_of_house` module is defined in the crate root. The
`front_of_house` module isn’t public, but because the `eat_at_restaurant`
function is defined in the same module as `front_of_house` (that is,
`eat_at_restaurant` and `front_of_house` are siblings), we can refer to
`front_of_house` from `eat_at_restaurant`. Next is the `hosting` module marked
with `pub`. We can access the parent module of `hosting`, so we can access
`hosting`. Finally, the `add_to_waitlist` function is marked with `pub` and we
can access its parent module, so this function call works!
-->

Dans le chemin absolu, nous commençons avec `crate`, la racine de l'arborescence
de modules de notre crate. Ensuite, le module `salle_a_manger` est défini à la
racine de la crate. Le module `salle_a_manger` n'est pas public, mais comme la
fonction `manger_au_restaurant` est définie dans le même module que
`salle_a_manger` (car `manger_au_restaurant` et `salle_a_manger` sont frères),
nous pouvons utiliser `salle_a_manger` à partir de `manger_au_restaurant`.
Ensuite, nous avons le module `accueil`, défini avec `pub`. Nous pouvons accéder
au module parent de `accueil`, donc nous pouvons accéder à `accueil`. Enfin, la
fonction `ajouter_a_la_liste_attente` est elle aussi définie avec `pub` et nous
pouvons accéder à son module parent, donc au final cet appel à la fonction
fonctionne bien !

<!--
In the relative path, the logic is the same as the absolute path except for the
first step: rather than starting from the crate root, the path starts from
`front_of_house`. The `front_of_house` module is defined within the same module
as `eat_at_restaurant`, so the relative path starting from the module in which
`eat_at_restaurant` is defined works. Then, because `hosting` and
`add_to_waitlist` are marked with `pub`, the rest of the path works, and this
function call is valid!
-->

Dans le chemin relatif, le fonctionnement est le même que le chemin absolu sauf
pour la première étape : plutôt que de démarrer de la racine de la crate, le
chemin commence à partir de `salle_a_manger`. Le module `salle_a_manger` est
défini dans le même module que `manger_au_restaurant`, donc le chemin relatif
qui commence à partir du module où est défini `manger_au_restaurant` fonctionne
bien. Ensuite, comme `accueil` et `ajouter_a_la_liste_attente` sont définis avec
`pub`, le reste du chemin fonctionne, et cet appel à la fonction est donc
valide !

<!--
### Starting Relative Paths with `super`
-->

### Commencer les chemins relatifs avec `super`

<!--
We can also construct relative paths that begin in the parent module by using
`super` at the start of the path. This is like starting a filesystem path with
the `..` syntax. Why would we want to do this?
-->

Nous pouvons aussi créer des chemins relatifs qui commencent à partir du module
parent en utilisant `super` au début du chemin. C'est comme débuter un chemin
dans un système de fichiers avec la syntaxe `..`. Mais pourquoi voudrions-nous
faire cela ?

<!--
Consider the code in Listing 7-8 that models the situation in which a chef
fixes an incorrect order and personally brings it out to the customer. The
function `fix_incorrect_order` calls the function `serve_order` by specifying
the path to `serve_order` starting with `super`:
-->

Imaginons le code dans l'encart 7-8 qui représente le cas où le chef corrige une
commande erronée et l'apporte personnellement au client pour s'excuser. La
fonction `corriger_commande_erronee` appelle la fonction `servir_commande` en
commençant le chemin de `servir_commande` avec `super` :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground,test_harness
{{#rustdoc_include ../listings-sources/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```
-->

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-8: Calling a function using a relative path
starting with `super`</span>
-->

<span class="caption">Encart 7-8 : Appel d'une fonction en utilisant un chemin
relatif qui commence par `super`</span>

<!--
The `fix_incorrect_order` function is in the `back_of_house` module, so we can
use `super` to go to the parent module of `back_of_house`, which in this case
is `crate`, the root. From there, we look for `serve_order` and find it.
Success! We think the `back_of_house` module and the `serve_order` function are
likely to stay in the same relationship to each other and get moved together
should we decide to reorganize the crate’s module tree. Therefore, we used
`super` so we’ll have fewer places to update code in the future if this code
gets moved to a different module.
-->

La fonction `corriger_commande_erronee` est dans le module `cuisines`, donc nous
pouvons utiliser `super` pour nous rendre au module parent de `cuisines`, qui
dans notre cas est `crate`, la racine. De là, nous cherchons `servir_commande`
et nous la trouvons. Avec succès ! Nous pensons que le module `cuisines` et la
fonction `servir_commande` vont toujours garder la même relation et devrons être
déplacés ensemble si nous réorganisons l'arborescence de modules de la crate.
Ainsi, nous avons utilisé `super` pour avoir moins de code à mettre à jour à
l'avenir si ce code est déplacé dans un module différent.

<!--
### Making Structs and Enums Public
-->

### Rendre publiques des structures et des énumérations

<!--
We can also use `pub` to designate structs and enums as public, but there are a
few extra details. If we use `pub` before a struct definition, we make the
struct public, but the struct’s fields will still be private. We can make each
field public or not on a case-by-case basis. In Listing 7-9, we’ve defined a
public `back_of_house::Breakfast` struct with a public `toast` field but a
private `seasonal_fruit` field. This models the case in a restaurant where the
customer can pick the type of bread that comes with a meal, but the chef
decides which fruit accompanies the meal based on what’s in season and in
stock. The available fruit changes quickly, so customers can’t choose the fruit
or even see which fruit they’ll get.
-->

Nous pouvons aussi utiliser `pub` pour déclarer des structures et des
énumérations publiquement, mais il y a d'autres points à prendre en compte. Si
nous utilisons `pub` avant la définition d'une structure, nous rendons la
structure publique, mais les champs de la structure restent privés. Nous pouvons
rendre chaque champ public ou non au cas par cas. Dans l'encart 7-9, nous avons
défini une structure publique `cuisines::PetitDejeuner` avec un champ public
`tartine_grillee` mais avec un champ privé `fruit_de_saison`. Cela simule un
restaurant où le client peut choisir le type de pain qui accompagne le repas,
mais le chef décide des fruits qui accompagnent le repas en fonction de la
saison et ce qu'il y a en stock. Les fruits disponibles changent rapidement,
donc les clients ne peuvent pas choisir le fruit ou même voir quel fruit ils
obtiendront.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-9: A struct with some public fields and some
private fields</span>
-->

<span class="caption">Encart 7-9 : Une structure avec certains champs publics et
d'autres privés</span>

<!--
Because the `toast` field in the `back_of_house::Breakfast` struct is public,
in `eat_at_restaurant` we can write and read to the `toast` field using dot
notation. Notice that we can’t use the `seasonal_fruit` field in
`eat_at_restaurant` because `seasonal_fruit` is private. Try uncommenting the
line modifying the `seasonal_fruit` field value to see what error you get!
-->

Comme le champ `tartine_grillee` est public dans la structure
`cuisines::PetitDejeuner`, nous pouvons lire et écrire dans le champ
`tartine_grillee` à partir de `manger_au_restaurant` en utilisant `.`. Notez
aussi que nous ne pouvons pas utiliser le champ `fruit_de_saison` dans
`manger_au_restaurant` car `fruit_de_saison` est privé. Essayez de dé-commenter
la ligne qui tente de modifier la valeur du champ `fruit_de_saison` et voyez
l'erreur que vous obtenez !

<!--
Also, note that because `back_of_house::Breakfast` has a private field, the
struct needs to provide a public associated function that constructs an
instance of `Breakfast` (we’ve named it `summer` here). If `Breakfast` didn’t
have such a function, we couldn’t create an instance of `Breakfast` in
`eat_at_restaurant` because we couldn’t set the value of the private
`seasonal_fruit` field in `eat_at_restaurant`.
-->

Aussi, remarquez que comme `cuisines::PetitDejeuner` a un champ privé, la
structure a besoin de fournir une fonction associée publique qui construit une
instance de `PetitDejeuner` (que nous avons nommée `en_ete` ici). Si
`PetitDejeuner` n'avait pas une fonction comme celle-ci, nous ne pourrions pas
créer une instance de `PetitDejeuner` dans `manger_au_restaurant` car nous ne
pourrions pas donner une valeur au champ privé `fruit_de_saison` dans
`manger_au_restaurant`.

<!--
In contrast, if we make an enum public, all of its variants are then public. We
only need the `pub` before the `enum` keyword, as shown in Listing 7-10.
-->

Par contre, si nous rendons publique une énumération, toutes ses variantes
seront publiques. Nous avons simplement besoin d'un `pub` devant le mot-clé
`enum`, comme dans l'encart 7-10.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-10: Designating an enum as public makes all its
variants public</span>
-->

<span class="caption">Encart 7-10 : On rend publique une énumération et cela
rend aussi toutes ses variantes publiques</span>

<!--
Because we made the `Appetizer` enum public, we can use the `Soup` and `Salad`
variants in `eat_at_restaurant`. Enums aren’t very useful unless their variants
are public; it would be annoying to have to annotate all enum variants with
`pub` in every case, so the default for enum variants is to be public. Structs
are often useful without their fields being public, so struct fields follow the
general rule of everything being private by default unless annotated with `pub`.
-->

Comme nous rendons l'énumération `AmuseBouche` publique, nous pouvons utiliser
les variantes `Soupe` et `Salade` dans `manger_au_restaurant`. Les énumérations
ne sont pas très utiles si elles n'ont pas leurs variantes publiques ; et cela
serait pénible d'avoir à marquer toutes les variantes de l'énumération avec
`pub`, donc par défaut les variantes d'énumérations sont publiques. Les
structures sont souvent utiles sans avoir de champs publics, donc les champs des
structures sont tous privés par défaut, sauf si ces éléments sont marqués d'un
`pub`.

<!--
There’s one more situation involving `pub` that we haven’t covered, and that is
our last module system feature: the `use` keyword. We’ll cover `use` by itself
first, and then we’ll show how to combine `pub` and `use`.
-->

Il y a encore une chose que nous n'avons pas abordée concernant `pub`, et c'est
la dernière fonctionnalité du système de modules : le mot-clé `use`. Nous
commencerons par parler de l'utilisation de `use` de manière générale, puis nous
verrons comment combiner `pub` et `use`.

<!--
[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
-->

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposer-des-chemins-avec-le-mot-clé-pub
