<!--
## Reference Cycles Can Leak Memory
-->

## Les boucles de références qui peuvent provoquer des fuites de mémoire

<!--
Rust’s memory safety guarantees make it difficult, but not impossible, to
accidentally create memory that is never cleaned up (known as a *memory leak*).
Preventing memory leaks entirely is not one of Rust’s guarantees in the same
way that disallowing data races at compile time is, meaning memory leaks are
memory safe in Rust. We can see that Rust allows memory leaks by using `Rc<T>`
and `RefCell<T>`: it’s possible to create references where items refer to each
other in a cycle. This creates memory leaks because the reference count of each
item in the cycle will never reach 0, and the values will never be dropped.
-->

Les garanties de sécurité de la mémoire de Rust rendent difficile, mais pas
impossible, la création accidentelle de mémoire qui n'est jamais nettoyée
(aussi appelée *fuite de mémoire*). Eviter absolument les fuites de mémoire
n'est pas une des garanties de Rust comme c'est le cas pour l'accès
concurrent à la compilation, ce qui signifie que les fuites de mémoire ne
sont dangereuses, pour Rust. Nous pouvons constater que Rust permet les fuites
de mémoire en utilisant `Rc<T>` et `RefCell<T>` : il est possible de créer des
références où les éléments se réfèrent entre eux de manière cyclique. Cela
crée des fuites de mémoire car le compteur de références de chaque élément
dans la boucle de références ne vaudra jamais 0, et les valeurs ne seront
jamais libérées.

<!--
### Creating a Reference Cycle
-->

### Créer une boucle de références

<!--
Let’s look at how a reference cycle might happen and how to prevent it,
starting with the definition of the `List` enum and a `tail` method in Listing
15-25:
-->

Voyons comment une boucle de références peut exister et comment l'éviter, en
commençant par la définition de l'énumération `List` et la méthode `parcourir`
de l'encart 15-25 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-25/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-25/src/main.rs}}
```

<!--
<span class="caption">Listing 15-25: A cons list definition that holds a
`RefCell<T>` so we can modify what a `Cons` variant is referring to</span>
-->

<span class="caption">Encart 15-25 : une liste de construction qui stocke une
`RefCell<T>` pour que nous puissions modifier ce sur quoi une variante `Cons`
pointe</span>

<!--
We’re using another variation of the `List` definition from Listing 15-5. The
second element in the `Cons` variant is now `RefCell<Rc<List>>`, meaning that
instead of having the ability to modify the `i32` value as we did in Listing
15-24, we want to modify which `List` value a `Cons` variant is pointing to.
We’re also adding a `tail` method to make it convenient for us to access the
second item if we have a `Cons` variant.
-->

Nous utilisons une autre variation de la définition de `List` de l'encart 15-5.
Le second élément dans la variante `Cons` est maintenant un
`RefCell<Rc<List>>`, ce qui signifie qu'au lieu de pouvoir modifier la valeur
`i32` comme nous l'avions fait dans l'encart 15-24, nous modifions ce sur quoi
une variante `Cons` pointe (qui reste une valeur `List`). Nous ajoutons
également une méthode `parcourir` pour nous faciliter l'accès au second élément
si nous avons une variante `Cons`.

<!--
In Listing 15-26, we’re adding a `main` function that uses the definitions in
Listing 15-25. This code creates a list in `a` and a list in `b` that points to
the list in `a`. Then it modifies the list in `a` to point to `b`, creating a
reference cycle. There are `println!` statements along the way to show what the
reference counts are at various points in this process.
-->

Dans l'encart 15-26, nous ajoutons une fonction `main` qui utilise les
définitions de l'encart 15-25. Ce code crée une liste dans `a` et une liste
dans `b` qui pointe sur la liste de `a`. Ensuite, on modifie la liste de `a`
pour pointer sur `b`, ce qui crée une boucle de références. Il y a aussi des
instructions `println!` tout du long pour montrer la valeur des compteurs de
références à différents endroits du processus.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-26: Creating a reference cycle of two `List`
values pointing to each other</span>
-->

<span class="caption">Encart 15-26 : création d'une boucle de références de
deux valeurs `List` qui se pointent mutuellement dessus</span>

<!--
We create an `Rc<List>` instance holding a `List` value in the variable `a`
with an initial list of `5, Nil`. We then create an `Rc<List>` instance
holding another `List` value in the variable `b` that contains the value 10 and
points to the list in `a`.
-->

Nous créons une instance `Rc<List>` qui stocke une valeur `List` dans la
variable `a` avec une valeur initiale de `5, Nil`. Nous créons ensuite une
instance `Rc<List>` qui stocke une autre valeur `List` dans la variable `b`
qui contient la valeur 10 et pointe vers la liste dans `a`.

<!--
We modify `a` so it points to `b` instead of `Nil`, creating a cycle. We
do that by using the `tail` method to get a reference to the
`RefCell<Rc<List>>` in `a`, which we put in the variable `link`. Then we use
the `borrow_mut` method on the `RefCell<Rc<List>>` to change the value inside
from an `Rc<List>` that holds a `Nil` value to the `Rc<List>` in `b`.
-->

Nous modifions `a` afin qu'elle pointe sur `b` au lieu de `Nil`, ce qui crée
une boucle. Nous faisons ceci en utilisant la méthode `parcourir` pour obtenir
une référence au `RefCell<Rc<List>>` présent dans `a`, que nous plaçons dans la
variable `link`. Ensuite nous utilisons la méthode `borrow_mut` sur le
`RefCell<Rc<List>>` pour changer la valeur présente en son sein par une
`Rc<List>` qui stocke une valeur `Nil` vers le `Rc<List>` présent dans `b`.

<!--
When we run this code, keeping the last `println!` commented out for the
moment, we’ll get this output:
-->

Lorsque nous exécutons ce code, en gardant le dernier `println!` commenté
pour le moment, nous obtenons ceci :

<!--
```console
{{#include ../listings-sources/ch15-smart-pointers/listing-15-26/output.txt}}
```
-->

```console
{{#include ../listings/ch15-smart-pointers/listing-15-26/output.txt}}
```

<!--
The reference count of the `Rc<List>` instances in both `a` and `b` are 2
after we change the list in `a` to point to `b`. At the end of `main`, Rust
will try to drop `b` first, which will decrease the count of the `Rc<List>`
instance in `b` by 1.
-->

Les compteurs de références des instances de `Rc<List>` valent tous les deux 2
pour `a` et `b` après avoir modifié `a` pour qu'elle pointe sur `b`. A la fin
du `main`, Rust va nettoyer d'abord `b`, ce qui va réduire le compteur de
l'instance `Rc<List>` de `b` de 1.

<!--
However, because `a` is still referencing the `Rc<List>` that was in `b`, that
`Rc<List>` has a count of 1 rather than 0, so the memory the `Rc<List>` has on
the heap won’t be dropped. The memory will just sit there with a count of 1,
forever. To visualize this reference cycle, we’ve created a diagram in Figure
15-4.
-->

Cependant, comme `a` pointe toujours sur le `Rc<List>` qui était sur `b`, le
compteur de son `Rc<List>` vaudra toujours 1 au lieu de 0, donc la mémoire de
sur le tas de ce `Rc<List>` ne sera pas nettoyé. La mémoire va juste rester ici
avec un compteur à 1, pour toujours. Pour représenter la boucle de références,
nous avons créé un diagramme dans l'illustration 15-4.

<!--
<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" />
-->

<img
    alt="Une boucle de références de listes"
    src="img/trpl15-04.svg"
    class="center" />

<!--
<span class="caption">Figure 15-4: A reference cycle of lists `a` and `b`
pointing to each other</span>
-->

<span class="caption">Illustration 15-4 : une boucle de références entre les
listes `a` et `b` qui se pointent mutuellement dessus</span>

<!--
If you uncomment the last `println!` and run the program, Rust will try to
print this cycle with `a` pointing to `b` pointing to `a` and so forth until it
overflows the stack.
-->

Si vous décommentez le dernier `println!` et que vous exécutez le programme,
Rust va essayer d'afficher cette boucle avec `a` qui pointe sur `b` qui pointe
sur `a` ... et ainsi de suite jusqu'à ce que cela fasse déborder la pile.

<!--
In this case, right after we create the reference cycle, the program ends. The
consequences of this cycle aren’t very dire. However, if a more complex program
allocated lots of memory in a cycle and held onto it for a long time, the
program would use more memory than it needed and might overwhelm the system,
causing it to run out of available memory.
-->

Dans ce cas, juste après que nous ayons créé la boucle de références, le
programme se termine. Les conséquences de cette boucle ne sont désastreuses.
Cependant, si un programme plus complexe alloue beaucoup de mémoire dans une
boucle de références et la garde pendant longtemps, le programme va utiliser
bien plus de mémoire qu'il a besoin et pourrait surcharger le système, qui
devrait épuiser la mémoire disponible.

<!--
Creating reference cycles is not easily done, but it’s not impossible either.
If you have `RefCell<T>` values that contain `Rc<T>` values or similar nested
combinations of types with interior mutability and reference counting, you must
ensure that you don’t create cycles; you can’t rely on Rust to catch them.
Creating a reference cycle would be a logic bug in your program that you should
use automated tests, code reviews, and other software development practices to
minimize.
-->

La création de boucles de références n'est pas facile à réaliser, mais n'est pas
non plus impossible. Si vous avez des valeurs `RefCell<T>` qui contiennent des
valeurs `Rc<T>` ou combinaisons similaires de types emboîtées avec de la
mutabilité interne et du comptage de références, vous devez vous assurer que
vous ne créez pas de boucles ; vous ne pouvez pas compter sur Rust pour les
détecter. La création de boucle de références devrait être un bogue de logique
de votre programme dont vous devriez réduire le risque en pratiquant des tests
automatisés, des revues de code, ainsi que d'autres pratiques de développement.

<!--
Another solution for avoiding reference cycles is reorganizing your data
structures so that some references express ownership and some references don’t.
As a result, you can have cycles made up of some ownership relationships and
some non-ownership relationships, and only the ownership relationships affect
whether or not a value can be dropped. In Listing 15-25, we always want `Cons`
variants to own their list, so reorganizing the data structure isn’t possible.
Let’s look at an example using graphs made up of parent nodes and child nodes
to see when non-ownership relationships are an appropriate way to prevent
reference cycles.
-->

Une autre solution pour éviter les boucles de références est de réorganiser vos
structures de données afin que certaines références prennent possession et
d'autres non. Par conséquent, vous pouvez obtenir des boucles de certaines
références qui prennent possession ou d'autres références qui ne prennent pas
possession, et seules celles qui prennent possession décident si oui ou non une
valeur peut être libérée. Dans l'encart 15-25, nous voulons toujours que les
variantes `Cons` possèdent leur propre liste, donc il est impossible de
réorganiser la structure des données. Voyons maintenant un exemple qui utilise
des branches constituées de nœuds parents et enfants pour voir lorsque les
liens qui ne prennent pas possession sont appropriés pour éviter les boucles de
références.

<!--
### Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`
-->

### Eviter les boucles de références : transformer un `Rc<T>` en `Weak<T>`

<!--
So far, we’ve demonstrated that calling `Rc::clone` increases the
`strong_count` of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned
up if its `strong_count` is 0. You can also create a *weak reference* to the
value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a
reference to the `Rc<T>`. When you call `Rc::downgrade`, you get a smart
pointer of type `Weak<T>`. Instead of increasing the `strong_count` in the
`Rc<T>` instance by 1, calling `Rc::downgrade` increases the `weak_count` by 1.
The `Rc<T>` type uses `weak_count` to keep track of how many `Weak<T>`
references exist, similar to `strong_count`. The difference is the `weak_count`
doesn’t need to be 0 for the `Rc<T>` instance to be cleaned up.
-->

Précédemment, nous avons démontré que l'appel à `Rc::clone` augmente le
`strong_count` d'une instance de `Rc<T>`, et une instance `Rc<T>` est nettoyée
seulement si son `strong_count` est à 0. Vous pouvez aussi créer un *pointeur
faible* (NdT : d'où le `weak`) vers la valeur présente dans une instance `Rc<T>`
en appelant `Rc::downgrade` et en lui passant une référence vers le `Rc<T>`.
Lorsque vous faites appel à `Rc::downgrade`, vous obtenez un pointeur
intelligent du type `Weak<T>`. Plutôt que d'augmenter le `strong_count` de
l'instance de 1, l'appel à `Rc::downgrade` augmente le `weak_count` de 1. Le
type `Rc<T>` utilise le `weak_count` pour compter combien de références
`Weak<T>` existent, de la même manière que `strong_count`. La différence réside
dans le fait que `weak_count` n'a pas besoin d'être 0 pour que l'instance
`Rc<T>` soit nettoyée.

<!--
Strong references are how you can share ownership of an `Rc<T>` instance. Weak
references don’t express an ownership relationship. They won’t cause a
reference cycle because any cycle involving some weak references will be broken
once the strong reference count of values involved is 0.
-->

Les références fortes désignent la manière de partager la propriété d'une
instance `Rc<T>`. Les pointeurs faibles ne désignent pas le lien qui prend
possession. Ils ne vont provoquer de boucle de références car n'importe quelle
boucle qui implique des pointeurs faibles se terminera lorsque le compteur de
références fortes lié vaudra 0.

<!--
Because the value that `Weak<T>` references might have been dropped, to do
anything with the value that a `Weak<T>` is pointing to, you must make sure the
value still exists. Do this by calling the `upgrade` method on a `Weak<T>`
instance, which will return an `Option<Rc<T>>`. You’ll get a result of `Some`
if the `Rc<T>` value has not been dropped yet and a result of `None` if the
`Rc<T>` value has been dropped. Because `upgrade` returns an `Option<Rc<T>>`,
Rust will ensure that the `Some` case and the `None` case are handled, and
there won’t be an invalid pointer.
-->

Comme la valeur contenue dans une référence `Weak<T>` peut être libérée, pour
pouvoir faire quelque chose avec cette valeur, vous devez vous assurer qu'elle
existe toujours. Vous pouvez faire ceci en appelant la méthode `upgrade` sur
une instance `Weak<T>`, qui va retourner une `Option<Rc<T>>`. Ce résultat
retournera `Some` si la valeur `Rc<T>` n'a pas encore été libérée, et un `None`
si la valeur `Rc<T>` a été libérée. Comme `upgrade` retourne une
`Option<Rc<T>>`, Rust va s'assurer que les cas de `Some` et de `None` sont bien
gérés, et qu'il n'existe pas de pointeur invalide.

<!--
As an example, rather than using a list whose items know only about the next
item, we’ll create a tree whose items know about their children items *and*
their parent items.
-->

Par exemple, plutôt que d'utiliser une liste dont les éléments ne connaissent
que les éléments suivants, nous allons créer un arbre dont les éléments
connaissent les éléments enfants *et* leurs éléments parents.

<!--
#### Creating a Tree Data Structure: a `Node` with Child Nodes
-->

#### Créer une structure d'arbre de données : un `Noeud` avec des nœuds enfants

<!--
To start, we’ll build a tree with nodes that know about their child nodes.
We’ll create a struct named `Node` that holds its own `i32` value as well as
references to its children `Node` values:
-->

Pour commencer, nous allons créer un arbre avec des nœuds qui connaissent
leurs nœuds enfants. Nous allons créer une structure `Noeud` qui contient sa
propre valeur ainsi que les références vers ses `Noeud` enfants :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```

<!--
We want a `Node` to own its children, and we want to share that ownership with
variables so we can access each `Node` in the tree directly. To do this, we
define the `Vec<T>` items to be values of type `Rc<Node>`. We also want to
modify which nodes are children of another node, so we have a `RefCell<T>` in
`children` around the `Vec<Rc<Node>>`.
-->

Nous souhaitons qu'un `Noeud` prenne possession de ses enfants, et nous
souhaitons partager la possession avec des variables afin d'accéder directement
à chaque `Noeud` de l'arbre. Pour pouvoir faire ceci, nous définissons les
éléments du `Vec<T>` pour être des valeurs du type `Rc<Noeud>`. Nous souhaitons
également modifier tel nœud est enfant de tel autre nœud, donc nous plaçons
un `RefCell<T>` dans `enfants` qui contient ce `Vec<Rc<Noeud>>`.

<!--
Next, we’ll use our struct definition and create one `Node` instance named
`leaf` with the value 3 and no children, and another instance named `branch`
with the value 5 and `leaf` as one of its children, as shown in Listing 15-27:
-->

Ensuite, nous allons utiliser notre définition de structure et créer une
instance de `Noeud` qui s'appellera `feuille` avec la valeur `3` et sans
enfant, comme dans l'encart 15-27 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Filename : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```

<!--
<span class="caption">Listing 15-27: Creating a `leaf` node with no children
and a `branch` node with `leaf` as one of its children</span>
-->

<span class="caption">Encart 15-27 : création d'un nœud `feuille` sans aucun
enfant et un nœud `branche` avec `feuille` comme enfant</span>

<!--
We clone the `Rc<Node>` in `leaf` and store that in `branch`, meaning the
`Node` in `leaf` now has two owners: `leaf` and `branch`. We can get from
`branch` to `leaf` through `branch.children`, but there’s no way to get from
`leaf` to `branch`. The reason is that `leaf` has no reference to `branch` and
doesn’t know they’re related. We want `leaf` to know that `branch` is its
parent. We’ll do that next.
-->

Nous créons un clone du `Rc<Noeud>` dans `feuille` et nous le stockons dans
`branche`, ce qui signifie que le `Noeud` dans `feuille` a maintenant deux
propriétaires : `feuille` et `branche`. Nous pouvons obtenir `feuille` à partir
de `branche` en utilisant `branche.feuille`, mais il n'y a pas de moyen
d'obtenir `branche` à partir de `feuille`. La raison est que `feuille` n'a pas
de référence vers `branche` et ne sait pas s'ils sont liés. Nous voulons que
`feuille` sache quelle `branche` est son parent. C'est ce que nous allons faire
dès maintenant.

<!--
#### Adding a Reference from a Child to Its Parent
-->

#### Ajouter une référence à un enfant vers son parent

<!--
To make the child node aware of its parent, we need to add a `parent` field to
our `Node` struct definition. The trouble is in deciding what the type of
`parent` should be. We know it can’t contain an `Rc<T>`, because that would
create a reference cycle with `leaf.parent` pointing to `branch` and
`branch.children` pointing to `leaf`, which would cause their `strong_count`
values to never be 0.
-->

Pour que le nœud enfant connaisse son parent, nous devons ajouter un champ
`parent` vers notre définition de structure `Noeud`. La difficulté ici est de
choisir quel sera le type de `parent`. Nous savons qu'il ne peut pas contenir
de `Rc<T>`, car cela créera une boucle de référence avec `feuille.parent` qui
pointe sur `branche` et `branche.enfant` qui pointe sur `feuille`, ce qui va
faire que leurs valeurs `strong_count` ne sera jamais à 0.

<!--
Thinking about the relationships another way, a parent node should own its
children: if a parent node is dropped, its child nodes should be dropped as
well. However, a child should not own its parent: if we drop a child node, the
parent should still exist. This is a case for weak references!
-->

En concevant le lien d'une autre manière, un nœud parent devrait prendre
possession de ses enfants : si un nœud parent est libéré, ses nœuds enfants
devraient aussi être libérés. Cependant, un enfant ne devrait pas prendre
possession de son parent : si nous libérons un nœud enfant, le parent doit
toujours exister. C'est donc un cas d'emploi pour les pointeurs faibles !

<!--
So instead of `Rc<T>`, we’ll make the type of `parent` use `Weak<T>`,
specifically a `RefCell<Weak<Node>>`. Now our `Node` struct definition looks
like this:
-->

Donc, plutôt qu'un `Rc<T>`, nous allons faire en sorte que le type de `parent`
soit un `Weak<T>`, plus précisément un `RefCell<Weak<Noeud>>`. Maintenant,
la définition de notre structure `Noeud` devrait ressembler à ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```

<!--
A node will be able to refer to its parent node but doesn’t own its parent.
In Listing 15-28, we update `main` to use this new definition so the `leaf`
node will have a way to refer to its parent, `branch`:
-->

Un nœud devrait pouvoir avoir une référence vers son nœud parent, mais il ne
devrait pas prendre possession de son parent. Dans l'encart 15-28, nous mettons
à jour cette nouvelle définition pour que le nœud `feuille` puisse avoir un
moyen de pointer vers son parent, `branche` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```

<!--
<span class="caption">Listing 15-28: A `leaf` node with a weak reference to its
parent node `branch`</span>
-->

<span class="caption">Encart 15-28 : un nœud `feuille` avec un pointeur faible
vers son nœud parent, `branche`</span>

<!--
Creating the `leaf` node looks similar to how creating the `leaf` node looked
in Listing 15-27 with the exception of the `parent` field: `leaf` starts out
without a parent, so we create a new, empty `Weak<Node>` reference instance.
-->

La création du nœud `feuille` semble être identique à la création du nœud
`feuille` de l'encart 15-27, sauf pour le champ `parent` : `feuille` commence
sans parent, donc nous créons une nouvelle instance de référence de type
`Weak<Noeud>`, qui est vide.

<!--
At this point, when we try to get a reference to the parent of `leaf` by using
the `upgrade` method, we get a `None` value. We see this in the output from the
first `println!` statement:
-->

A ce moment-là, lorsque nous essayons d'obtenir une référence vers le parent de
`feuille` en utilisant la méthode `upgrade`, nous obtenons une valeur `None`.
Nous constatons cela dans la première instruction `println!` sur la sortie :

<!--
```text
leaf parent = None
```
-->

```text
parent de la feuille = None
```

<!--
When we create the `branch` node, it will also have a new `Weak<Node>`
reference in the `parent` field, because `branch` doesn’t have a parent node.
We still have `leaf` as one of the children of `branch`. Once we have the
`Node` instance in `branch`, we can modify `leaf` to give it a `Weak<Node>`
reference to its parent. We use the `borrow_mut` method on the
`RefCell<Weak<Node>>` in the `parent` field of `leaf`, and then we use the
`Rc::downgrade` function to create a `Weak<Node>` reference to `branch` from
the `Rc<Node>` in `branch.`
-->

Lorsque nous créons le nœud `branche`, il va aussi avoir une nouvelle
référence `Weak<Noeud>` dans le champ `parent`, car `branche` n'a pas de nœud
parent. Nous avons néanmoins `feuille` dans `enfants` de `branche`. Une fois
que nous avons l'instance de `Noeud` dans `branche`, nous pouvons modifier
`feuille` pour lui donner une référence `Weak<Noeud>` vers son parent. Nous
utilisons la méthode `borrow_mut` sur la `RefCell<Weak<Noeud>>` du champ
`parent` de `feuille`, et ensuite nous utilisons la fonction `Rc::downgrade`
pour créer une référence de type `Weak<Node>` vers `branche` à partir du
`Rc<Noeud>` présent dans `branche`.

<!--
When we print the parent of `leaf` again, this time we’ll get a `Some` variant
holding `branch`: now `leaf` can access its parent! When we print `leaf`, we
also avoid the cycle that eventually ended in a stack overflow like we had in
Listing 15-26; the `Weak<Node>` references are printed as `(Weak)`:
-->

Lorsque nous affichons à nouveau le parent de `feuille`, cette fois nous
obtenons la variante `Some` qui contient `branche` : désormais, `feuille` peut
accéder à son parent ! Lorsque nous affichons `feuille`, nous avons aussi évité
la boucle qui aurait probablement fini en débordement de pile comme nous
l'avions expérimenté dans l'encart 15-26 ; les références `Weak<Noeud>`
s'écrivent `(Weak)` :

<!--
```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```
-->

```text
parent de la feuille = Some(Noeud { valeur: 5, parent: RefCell { value: (Weak) },
enfants: RefCell { value: [Noeud { valeur: 3, parent: RefCell { value: (Weak) },
enfants: RefCell { value: [] } }] } })
```

<!--
The lack of infinite output indicates that this code didn’t create a reference
cycle. We can also tell this by looking at the values we get from calling
`Rc::strong_count` and `Rc::weak_count`.
-->

L'absence d'une sortie infinie nous confirme que ce code ne crée pas de boucle
de références. Nous pouvons aussi le constater en affichant les valeurs que
nous pouvons obtenir en faisant appel à `Rc::strong_count` et `Rc::weak_count`.

<!--
#### Visualizing Changes to `strong_count` and `weak_count`
-->

#### Visualiser les modifications de `strong_count` et `weak_count`

<!--
Let’s look at how the `strong_count` and `weak_count` values of the `Rc<Node>`
instances change by creating a new inner scope and moving the creation of
`branch` into that scope. By doing so, we can see what happens when `branch` is
created and then dropped when it goes out of scope. The modifications are shown
in Listing 15-29:
-->

Regardons comment changent les valeurs `strong_count` et `weak_count` des
instances de `Rc<Noeud>` en créant une portée interne et en déplaçant la
création de `branche` dans cette portée. En faisant ceci, nous pourrons
constater ce qui se passe lorsque `branche` est créée et lorsqu'elle sera
libérée lorsqu'elle sortira de la portée. Ces modifications sont présentées
dans l'encart 15-29 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-29: Creating `branch` in an inner scope and
examining strong and weak reference counts</span>
-->

<span class="caption">Encart 15-29 : création de `branche` dans une portée
interne et vérification des compteurs de références strong et weak</span>

<!--
After `leaf` is created, its `Rc<Node>` has a strong count of 1 and a weak
count of 0. In the inner scope, we create `branch` and associate it with
`leaf`, at which point when we print the counts, the `Rc<Node>` in `branch`
will have a strong count of 1 and a weak count of 1 (for `leaf.parent` pointing
to `branch` with a `Weak<Node>`). When we print the counts in `leaf`, we’ll see
it will have a strong count of 2, because `branch` now has a clone of the
`Rc<Node>` of `leaf` stored in `branch.children`, but will still have a weak
count of 0.
-->

Après la création de `feuille`, son `Rc<Noeud>` a le compteur strong à 1 et le
compteur weak à 0. Dans la portée interne, nous créons `branche` et l'associons
à `feuille`, et à partir de là, lorsque nous affichons les compteurs, le
`Rc<Noeud>` dans `branche` aura le compteur strong à 1 et le compteur weak à 1
(pour que `feuille.parent` pointe sur `branche` avec un `Weak<Noeud>`). Lorsque
nous affichons les compteurs dans `feuille` nous constatons qu'il a le compteur
strong à 2, car `branche` a maintenant un clone du `Rc<Noeud>` de `feuille`
stocké dans `branche.enfants`, mais a toujours le compteur weak à 0.

<!--
When the inner scope ends, `branch` goes out of scope and the strong count of
the `Rc<Node>` decreases to 0, so its `Node` is dropped. The weak count of 1
from `leaf.parent` has no bearing on whether or not `Node` is dropped, so we
don’t get any memory leaks!
-->

Lorsque la portée interne se termine, `branche` sort de la portée et le
compteur strong de `Rc<Noeud>` décroît à 0, donc son `Noeud` est libéré. Le
compteur weak à 1 de `feuille.parent` n'a aucune répercussion suite à la
libération ou non du `Noeud`, donc nous ne sommes pas dans une situation de
fuite de mémoire !

<!--
If we try to access the parent of `leaf` after the end of the scope, we’ll get
`None` again. At the end of the program, the `Rc<Node>` in `leaf` has a strong
count of 1 and a weak count of 0, because the variable `leaf` is now the only
reference to the `Rc<Node>` again.
-->

Si nous essayons d'accéder au parent de `feuille` après la fin de la portée,
nous allons à nouveau obtenir `None`. A la fin du programme, le `Rc<Noeud>`
dans `feuille` a son compteur strong à 1 et son compteur weak à 0, car la
variable `feuille` est à nouveau la seule référence au `Rc<Noeud>`.

<!--
All of the logic that manages the counts and value dropping is built into
`Rc<T>` and `Weak<T>` and their implementations of the `Drop` trait. By
specifying that the relationship from a child to its parent should be a
`Weak<T>` reference in the definition of `Node`, you’re able to have parent
nodes point to child nodes and vice versa without creating a reference cycle
and memory leaks.
-->

Toute cette logique qui gère les compteurs et les libérations des valeurs est
intégrée dans `Rc<T>` et `Weak<T>` et leurs implémentations du trait `Drop`. En
précisant dans la définition de `Noeud` que le lien entre un enfant et son
parent doit être une référence `Weak<T>`, vous pouvez avoir des nœuds parents
qui pointent sur des nœuds enfants et vice versa sans risquer de créer des
boucles de références et des fuites de mémoire.

<!--
## Summary
-->

## Résumé

<!--
This chapter covered how to use smart pointers to make different guarantees and
trade-offs from those Rust makes by default with regular references. The
`Box<T>` type has a known size and points to data allocated on the heap. The
`Rc<T>` type keeps track of the number of references to data on the heap so
that data can have multiple owners. The `RefCell<T>` type with its interior
mutability gives us a type that we can use when we need an immutable type but
need to change an inner value of that type; it also enforces the borrowing
rules at runtime instead of at compile time.
-->

Ce chapitre a expliqué l'utilisation des pointeurs intelligents pour appliquer
différentes garanties et compromis que celles qu'applique Rust par défaut avec
les références classiques. Le type `Box<T>` a une taille connue et pointe sur
une donnée allouée sur le tas. Le type `Rc<T>` compte le nombre de références
vers une donnée présente sur le tas afin que cette donnée puisse avoir
plusieurs propriétaires. Le type `RefCell<T>` nous permet de l'utiliser lorsque
nous avons besoin d'un type immuable mais que nous avons besoin de changer une
valeur interne à ce type, grâce à sa fonctionnalité de mutabilité interne ;
elle nous permet aussi d'appliquer les règles d'emprunt à l'exécution plutôt
qu'à la compilation.

<!--
Also discussed were the `Deref` and `Drop` traits, which enable a lot of the
functionality of smart pointers. We explored reference cycles that can cause
memory leaks and how to prevent them using `Weak<T>`.
-->

Nous avons aussi vu les traits `Deref` et `Drop`, qui offrent des
fonctionnalités très importantes aux pointeurs intelligents. Nous avons
expérimenté les boucles de références qui peuvent causer des fuites de mémoire
et nous avons vu comment les éviter en utilisant `Weak<T>`.

<!--
If this chapter has piqued your interest and you want to implement your own
smart pointers, check out [“The Rustonomicon”][nomicon] for more useful
information.
-->

Si ce chapitre a éveillé votre curiosité et que vous souhaitez mettre en œuvre
vos propres pointeurs intelligents, visitez [“The Rustonomicon”][nomicon] pour
en savoir plus.

<!--
Next, we’ll talk about concurrency in Rust. You’ll even learn about a few new
smart pointers.
-->

Au chapitre suivant, nous allons parler de concurrence en Rust. Vous
découvrirez peut-être même quelques nouveaux pointeurs intelligents ...

<!--
[nomicon]: ../nomicon/index.html
-->

[nomicon]: https://doc.rust-lang.org/nomicon/index.html
