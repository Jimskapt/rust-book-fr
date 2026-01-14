<!--
## Unsafe Rust
-->

## Le Rust non sécurisé (`unsafe`)

<!--
All the code we’ve discussed so far has had Rust’s memory safety guarantees
enforced at compile time. However, Rust has a second language hidden inside it
that doesn’t enforce these memory safety guarantees: it’s called *unsafe Rust*
and works just like regular Rust, but gives us extra superpowers.
-->

Tout le code Rust que nous avons abordé jusqu'à présent a bénéficié des
garanties de sécurité de la mémoire, vérifiées à la compilation. Cependant Rust
possède un second langage caché en son sein qui n'applique pas ces vérifications
de sécurité de la mémoire : il s'appelle le *Rust non sécurisé* et
fonctionne comme le Rust habituel, mais fournit quelques super-pouvoirs
supplémentaires.

<!--
Unsafe Rust exists because, by nature, static analysis is conservative. When
the compiler tries to determine whether or not code upholds the guarantees,
it’s better for it to reject some valid programs rather than accept some
invalid programs. Although the code *might* be okay, if the Rust compiler
doesn’t have enough information to be confident, it will reject the code. In
these cases, you can use unsafe code to tell the compiler, “Trust me, I know
what I’m doing.” The downside is that you use it at your own risk: if you use
unsafe code incorrectly, problems due to memory unsafety, such as null pointer
dereferencing, can occur.
-->

Le Rust non sécurisé existe car, par nature, l'analyse statique est conservative.
Lorsque le compilateur essaye de déterminer si le code respecte ou non les
garanties, il vaut mieux rejeter quelques programmes valides plutôt que
d'accepter quelques programmes invalides. Bien que le code *puisse* être
correct, si le compilateur Rust n'a pas assez d'information pour être sûr, il
va refuser ce code. Dans ce cas, vous pouvez utiliser du code non sécurisé pour
dire au compilateur “fais-moi confiance, je sais ce que je fait”. Le prix à
payer pour cela est que vous l'utilisez à vos risques et périls : si
vous écrivez du code non sécurisé de manière incorrecte, des problèmes liés à
la sécurité de la mémoire peuvent se produire, tel qu'un déréférencement d'un
pointeur vide.

<!--
Another reason Rust has an unsafe alter ego is that the underlying computer
hardware is inherently unsafe. If Rust didn’t let you do unsafe operations, you
couldn’t do certain tasks. Rust needs to allow you to do low-level systems
programming, such as directly interacting with the operating system or even
writing your own operating system. Working with low-level systems programming
is one of the goals of the language. Let’s explore what we can do with unsafe
Rust and how to do it.
-->

Une autre raison pour laquelle Rust embarque son alter-ego non sécurisé est que
le matériel des ordinateurs sur lequel il repose n'est pas sécurisé par essence.
Si Rust ne vous laissait pas procéder à des opérations non sécurisées, vous ne
pourriez pas faire certaines choses. Rust doit pouvoir vous permettre de
développer du code bas-niveau, comme pouvoir interagir directement avec le
système d'exploitation ou même écrire votre propre système d'exploitation.
Pouvoir travailler avec des systèmes bas-niveau est un des objectifs du langage.
Voyons ce que nous pouvons faire avec le Rust non sécurisé et comment le faire.

<!--
### Unsafe Superpowers
-->

### Les super-pouvoirs du code non sécurisé

<!--
To switch to unsafe Rust, use the `unsafe` keyword and then start a new block
that holds the unsafe code. You can take five actions in unsafe Rust, called
*unsafe superpowers*, that you can’t in safe Rust. Those superpowers include
the ability to:
-->

Pour pouvoir utiliser le Rust non sécurisé, il faut utiliser le mot-clé `unsafe`
et ensuite créer un nouveau bloc qui contient le code non sécurisé. Vous pouvez
faire cinq actions en Rust non sécurisé, qui s'appellent *les super-pouvoirs du
non sécurisé*, actions que vous ne pourriez pas faire en Rust sécurisé. Ces super-pouvoirs
permettent de :

<!--
* Dereference a raw pointer
* Call an unsafe function or method
* Access or modify a mutable static variable
* Implement an unsafe trait
* Access fields of `union`s
-->

* Déréférencer un pointeur brut
* Faire appel à une fonction ou une méthode non sécurisée
* Lire ou modifier une variable statique mutable
* Implémenter un trait non sécurisé
* Accéder aux champs des `union`

<!--
It’s important to understand that `unsafe` doesn’t turn off the borrow checker
or disable any other of Rust’s safety checks: if you use a reference in unsafe
code, it will still be checked. The `unsafe` keyword only gives you access to
these five features that are then not checked by the compiler for memory
safety. You’ll still get some degree of safety inside of an unsafe block.
-->

Il est important de comprendre que `unsafe` ne désactive pas le vérificateur
d'emprunt et ne désactive pas les autres vérifications de sécurité de Rust : si
vous utilisez une référence dans du code non sécurisé, elle sera toujours
vérifiée. Le mot-clé `unsafe` vous donne seulement accès à ces cinq
fonctionnalités qui ne sont alors pas vérifiées par le compilateur en vue de veiller
à la sécurité de la mémoire. Vous conservez donc un certain niveau de sécurité à
l'intérieur d'un bloc `unsafe`.

<!--
In addition, `unsafe` does not mean the code inside the block is necessarily
dangerous or that it will definitely have memory safety problems: the intent is
that as the programmer, you’ll ensure the code inside an `unsafe` block will
access memory in a valid way.
-->

De plus, `unsafe` ne signifie pas que le code à l'intérieur du bloc est
obligatoirement dangereux ou qu'il va forcément présenter des problèmes de sécurité
mémoire : l'idée étant qu'en tant que développeur, vous vous assuriez que le code
à l'intérieur d'un bloc `unsafe` va accéder correctement à la mémoire.

<!--
People are fallible, and mistakes will happen, but by requiring these five
unsafe operations to be inside blocks annotated with `unsafe` you’ll know that
any errors related to memory safety must be within an `unsafe` block. Keep
`unsafe` blocks small; you’ll be thankful later when you investigate memory
bugs.
-->

Personne n'est parfait, les erreurs arrivent, et en imposant que ces cinq
opérations non sécurisés se trouvent dans des blocs marqués d'un `unsafe`, Rust
vous permet de savoir que ces éventuelles erreurs liées à la sécurité de la
mémoire se trouveront dans un bloc `unsafe`. Vous devez donc essayer de
minimiser la taille des blocs `unsafe` ; vous ne le regretterez pas lorsque
vous rechercherez des bogues de mémoire.

<!--
To isolate unsafe code as much as possible, it’s best to enclose unsafe code
within a safe abstraction and provide a safe API, which we’ll discuss later in
the chapter when we examine unsafe functions and methods. Parts of the standard
library are implemented as safe abstractions over unsafe code that has been
audited. Wrapping unsafe code in a safe abstraction prevents uses of `unsafe`
from leaking out into all the places that you or your users might want to use
the functionality implemented with `unsafe` code, because using a safe
abstraction is safe.
-->

Pour isoler autant que possible le code non sécurisé, il vaut mieux intégrer du
code non sécurisé dans une abstraction et fournir ainsi une API sécurisée,
comme nous le verrons plus tard dans ce chapitre lorsque nous examinerons les
fonctions et méthodes non sécurisées. Certaines parties de la bibliothèque
standard sont implémentées comme étant des abstractions sécurisées et basées
sur du code non sécurisé qui a été audité. Encapsuler du code non sécurisé dans
une abstraction sécurisée évite que l'utilisation de `unsafe` ne se propage
dans des endroits où vous ou vos utilisateurs souhaiteraient éviter d'utiliser
les fonctionnalités du code `unsafe`, car au final utiliser une abstraction
sécurisée doit rester sûr.

<!--
Let’s look at each of the five unsafe superpowers in turn. We’ll also look at
some abstractions that provide a safe interface to unsafe code.
-->

Analysons ces cinq super-pouvoirs à tour de rôle. Nous allons aussi
découvrir quelques abstractions qui fournissent une interface sécurisée pour
faire fonctionner du code non sécurisé.

<!--
### Dereferencing a Raw Pointer
-->

### Déréférencer un pointeur brut

<!--
In Chapter 4, in the [“Dangling References”][dangling-references]<!-- ignore
-- > section, we mentioned that the compiler ensures references are always
valid. Unsafe Rust has two new types called *raw pointers* that are similar to
references. As with references, raw pointers can be immutable or mutable and
are written as `*const T` and `*mut T`, respectively. The asterisk isn’t the
dereference operator; it’s part of the type name. In the context of raw
pointers, *immutable* means that the pointer can’t be directly assigned to
after being dereferenced.
-->

Au chapitre 4, dans la section
[“Les références pendouillantes”][dangling-references]<!-- ignore -->, nous
avions mentionné que le compilateur s'assure que les références sont toujours
valides. Le Rust non sécurisé offre deux nouveaux types qui s'appellent les
*pointeurs brut* et qui ressemblent aux références. Comme les références, les
pointeurs bruts peuvent être immuables ou mutables et s'écrivent respectivement
`*const T` et `*mut T`. L'astérisque n'est pas l'opérateur de déréférencement ;
il fait partie du nom du type. Dans un contexte de pointeur brut, *immuable*
signifie que le pointeur ne peut pas être affecté directement après avoir été
déréférencé.

<!--
Different from references and smart pointers, raw pointers:
-->

Par rapport aux références et aux pointeurs intelligents, les pointeurs
bruts peuvent :

<!--
* Are allowed to ignore the borrowing rules by having both immutable and
  mutable pointers or multiple mutable pointers to the same location
* Aren’t guaranteed to point to valid memory
* Are allowed to be null
* Don’t implement any automatic cleanup
-->

* ignorer les règles d'emprunt en ayant plusieurs pointeurs tant immuables que
  mutables ou en ayant plusieurs pointeurs mutables qui pointent vers le même
  endroit.
* ne pas être obligés de pointer sur un emplacement mémoire valide
* être autorisés à avoir la valeur nulle
* ne pas implémenter de fonctionnalité de nettoyage automatique

<!--
By opting out of having Rust enforce these guarantees, you can give up
guaranteed safety in exchange for greater performance or the ability to
interface with another language or hardware where Rust’s guarantees don’t apply.
-->

En renonçant à ce que Rust fasse respecter ces garanties, vous pouvez sacrifier
la sécurité garantie pour obtenir de meilleures performances ou avoir la
possibilité de vous interfacer avec un autre langage ou matériel pour lesquels
les garanties de Rust ne s'appliquent pas.

<!--
Listing 19-1 shows how to create an immutable and a mutable raw pointer from
references.
-->

L'encart 19-1 montre comment créer un pointeur brut immuable et mutable à partir
de références.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-01/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-01/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-1: Creating raw pointers from references</span>
-->

<span class="caption">Encart 19-1 : création de pointeurs bruts à partir de
références</span>

<!--
Notice that we don’t include the `unsafe` keyword in this code. We can create
raw pointers in safe code; we just can’t dereference raw pointers outside an
unsafe block, as you’ll see in a bit.
-->

Remarquez que nous n'incorporons pas le mot-clé `unsafe` dans ce code. Nous
pouvons créer des pointeurs bruts dans du code sécurisé ; nous ne pouvons
simplement pas déréférencer les pointeurs bruts à l'extérieur d'un bloc non
sécurisé, comme vous allez le constater d'ici peu.

<!--
We’ve created raw pointers by using `as` to cast an immutable and a mutable
reference into their corresponding raw pointer types. Because we created them
directly from references guaranteed to be valid, we know these particular raw
pointers are valid, but we can’t make that assumption about just any raw
pointer.
-->

Nous avons créé des pointeurs bruts en utilisant `as` pour transformer les
références immuables et mutables en leur type de pointeur brut correspondant.
Comme nous les avons créés directement à partir de références qui sont garanties
d'être valides, nous savons que ces pointeurs bruts seront valides, mais nous ne
pouvons pas faire cette supposition sur tous les pointeurs bruts.

<!--
Next, we’ll create a raw pointer whose validity we can’t be so certain of.
Listing 19-2 shows how to create a raw pointer to an arbitrary location in
memory. Trying to use arbitrary memory is undefined: there might be data at
that address or there might not, the compiler might optimize the code so there
is no memory access, or the program might error with a segmentation fault.
Usually, there is no good reason to write code like this, but it is possible.
-->

Ensuite, nous allons créer un pointeur brut dont la validité n'est pas
certaine. L'encart 19-2 montre comment créer un pointeur brut vers un
emplacement arbitraire de la mémoire. Essayer d'utiliser de la mémoire
arbitraire va engendrer un comportement incertain : il peut y avoir des données
à cette adresse comme il peut ne pas y en avoir, le compilateur pourrait
optimiser le code de tel sorte qu'aucun accès mémoire n'aura lieu ou bien le
programme pourrait déclencher une erreur de segmentation. Habituellement, il
n'y a pas de bonne raison d'écrire du code comme celui-ci, mais c'est possible.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-02/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-02/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-2: Creating a raw pointer to an arbitrary
memory address</span>
-->

<span class="caption">Encart 19-2 : création d'un pointeur brut vers une
adresse mémoire arbitraire</span>

<!--
Recall that we can create raw pointers in safe code, but we can’t *dereference*
raw pointers and read the data being pointed to. In Listing 19-3, we use the
dereference operator `*` on a raw pointer that requires an `unsafe` block.
-->

Souvenez-vous que nous pouvons créer des pointeurs bruts dans du code sécurisé,
mais que nous ne pouvons pas y *déréférencer* les pointeurs bruts et lire les
données sur lesquelles ils pointent. Dans l'encart 19-3, nous utilisons
l'opérateur de déréférencement `*` sur un pointeur brut qui nécessite un bloc
`unsafe`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-03/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-3: Dereferencing raw pointers within an
`unsafe` block</span>
-->

<span class="caption">Encart 19-3 : déréférencement d'un pointeur brut à
l'intérieur d'un bloc `unsafe`</span>

<!--
Creating a pointer does no harm; it’s only when we try to access the value that
it points at that we might end up dealing with an invalid value.
-->

La création de pointeur ne pose pas de problèmes ; c'est seulement lorsque nous
essayons d'accéder aux valeurs sur lesquelles ils pointent qu'on risque
d'obtenir une valeur invalide.

<!--
Note also that in Listing 19-1 and 19-3, we created `*const i32` and `*mut i32`
raw pointers that both pointed to the same memory location, where `num` is
stored. If we instead tried to create an immutable and a mutable reference to
`num`, the code would not have compiled because Rust’s ownership rules don’t
allow a mutable reference at the same time as any immutable references. With
raw pointers, we can create a mutable pointer and an immutable pointer to the
same location and change data through the mutable pointer, potentially creating
a data race. Be careful!
-->

Remarquez aussi que dans les encarts 19-1 et 19-3, nous avons créé les
pointeurs bruts `*const i32` et `*mut i32` qui pointent tous les deux au même
endroit de la mémoire, où `nombre` est stocké. Si nous avions plutôt tenté de
créer une référence immuable et une mutable vers `nombre`, le code n'aurait pas
compilé à cause des règles de possession de Rust qui ne permettent pas d'avoir
une référence mutable en même temps qu'une ou plusieurs références immuables.
Avec les pointeurs bruts, nous pouvons créer un pointeur mutable et un pointeur
immuable vers le même endroit et changer la donnée via le pointeur mutable, en
risquant un accès concurrent. Soyez vigilant !

<!--
With all of these dangers, why would you ever use raw pointers? One major use
case is when interfacing with C code, as you’ll see in the next section,
[“Calling an Unsafe Function or
Method.”](#calling-an-unsafe-function-or-method)<!-- ignore -- > Another case is
when building up safe abstractions that the borrow checker doesn’t understand.
We’ll introduce unsafe functions and then look at an example of a safe
abstraction that uses unsafe code.
-->

Avec tous ces dangers, pourquoi vous risquer à utiliser les pointeurs bruts ?
Une des utilisations principale consiste à s'interfacer avec du code
C, comme vous allez le découvrir dans la section suivante. Une autre
utilisation est de nous permettre de créer une abstraction sécurisée que le
vérificateur d'emprunt ne comprend pas. Nous allons découvrir les fonctions
non sécurisées puis voir un exemple d'une abstraction sécurisée qui
utilise du code non sécurisé.

<!--
### Calling an Unsafe Function or Method
-->

### Faire appel à une fonction ou une méthode non sécurisée

<!--
The second type of operation that requires an unsafe block is calls to unsafe
functions. Unsafe functions and methods look exactly like regular functions and
methods, but they have an extra `unsafe` before the rest of the definition. The
`unsafe` keyword in this context indicates the function has requirements we
need to uphold when we call this function, because Rust can’t guarantee we’ve
met these requirements. By calling an unsafe function within an `unsafe` block,
we’re saying that we’ve read this function’s documentation and take
responsibility for upholding the function’s contracts.
-->

Le deuxième type d'opération qui nécessite un bloc `unsafe` est l'appel à des
fonctions non sécurisées. Les fonctions et méthodes non sécurisées ressemblent
exactement aux méthodes et fonctions habituelles, mais ont un `unsafe` en plus
devant le reste de leur définition. Le mot-clé `unsafe` dans ce cas signifie
que la fonction a des exigences que nous devons respecter pour pouvoir y faire
appel, car Rust ne pourra pas garantir de son côté que nous les ayons remplies.
En faisant appel à une fonction non sécurisée dans un bloc `unsafe`, nous
reconnaissons que nous avons lu la documentation de cette fonction et pris la
responsabilité de respecter les conditions d'utilisation de la fonction.

<!--
Here is an unsafe function named `dangerous` that doesn’t do anything in its
body:
-->

Voici une fonction non sécurisée `dangereux`, qui ne fait rien dans son corps :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

<!--
We must call the `dangerous` function within a separate `unsafe` block. If we
try to call `dangerous` without the `unsafe` block, we’ll get an error:
-->

Nous devons faire appel à la fonction `dangereux` dans un bloc `unsafe` séparé.
Si nous essayons d'appeler `dangereux` sans le bloc `unsafe`, nous obtenons une
erreur :

<!--
```console
{{#include ../listings-sources/ch19-advanced-features/output-only-01-missing-unsafe/output.txt}}
```
-->

```console
{{#include ../listings/ch19-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

<!--
By inserting the `unsafe` block around our call to `dangerous`, we’re asserting
to Rust that we’ve read the function’s documentation, we understand how to use
it properly, and we’ve verified that we’re fulfilling the contract of the
function.
-->

En ajoutant le bloc `unsafe` autour de notre appel à `dangereux`, nous
déclarons à Rust que nous avons lu la documentation de la fonction, que nous
comprenons comment l'utiliser correctement et que nous avons vérifié que nous
répondons bien aux exigences de la fonction.

<!--
Bodies of unsafe functions are effectively `unsafe` blocks, so to perform other
unsafe operations within an unsafe function, we don’t need to add another
`unsafe` block.
-->

Les corps des fonctions non sécurisées sont bel et bien des blocs `unsafe`,
donc pour pouvoir procéder à d'autres opérations non sécurisées dans une
fonction non sécurisée, nous n'avons pas besoin d'ajouter un autre bloc
`unsafe`.

<!--
#### Creating a Safe Abstraction over Unsafe Code
-->

#### Créer une abstraction sécurisée sur du code non sécurisé

<!--
Just because a function contains unsafe code doesn’t mean we need to mark the
entire function as unsafe. In fact, wrapping unsafe code in a safe function is
a common abstraction. As an example, let’s study a function from the standard
library, `split_at_mut`, that requires some unsafe code and explore how we
might implement it. This safe method is defined on mutable slices: it takes one
slice and makes it two by splitting the slice at the index given as an
argument. Listing 19-4 shows how to use `split_at_mut`.
-->

Ce n'est pas parce qu'une fonction contient du code non sécurisé que nous devons
forcément marquer l'intégralité de cette fonction comme non sécurisée. En fait,
envelopper du code non sécurisé dans une fonction sécurisée est une abstraction
courante. Par exemple, étudions une fonction de la bibliothèque standard,
`split_at_mut`, qui nécessite du code non sécurisé, et étudions comment nous
devrions l'implémenter. Cette méthode sécurisée est définie sur des slices
mutables : elle prend une slice en paramètre et en créée deux autres en divisant
la slice à l'indice donné en argument. L'encart 19-4 montre comment utiliser
`split_at_mut`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-04/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-4: Using the safe `split_at_mut`
function</span>
-->

<span class="caption">Encart 19-4 : utilisation de la fonction sécurisée
`split_at_mut`</span>

<!--
We can’t implement this function using only safe Rust. An attempt might look
something like Listing 19-5, which won’t compile. For simplicity, we’ll
implement `split_at_mut` as a function rather than a method and only for slices
of `i32` values rather than for a generic type `T`.
-->

Nous ne pouvons pas implémenter cette fonction en utilisant uniquement du Rust
sécurisé. Une tentative en ce sens ressemblerait à l'encart 19-5, qui ne se
compilera pas. Par simplicité, nous allons implémenter `split_at_mut` comme une
fonction plutôt qu'une méthode et seulement pour des slices de valeurs `i32` au
lieu d'un type générique `T`.

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-05/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-5: An attempted implementation of
`split_at_mut` using only safe Rust</span>
-->

<span class="caption">Encart 19-5 : une tentative d'implémentation de
`split_at_mut` en utilisant uniquement du Rust sécurisé</span>

<!--
This function first gets the total length of the slice. Then it asserts that
the index given as a parameter is within the slice by checking whether it’s
less than or equal to the length. The assertion means that if we pass an index
that is greater than the length to split the slice at, the function will panic
before it attempts to use that index.
-->

Cette fonction commence par obtenir la longueur totale de la slice. Elle
vérifie ensuite que l'indice donné en paramètre est bien à l'intérieur de la
slice en vérifiant s'il est inférieur ou égal à la longueur. La vérification
implique que si nous envoyons un indice qui est plus grand que la longueur de
la slice à découper, la fonction va paniquer avant d'essayer d'utiliser cet
indice.

<!--
Then we return two mutable slices in a tuple: one from the start of the
original slice to the `mid` index and another from `mid` to the end of the
slice.
-->

Ensuite, nous retournons deux slices mutables dans un tuple : une à partir du
début de la slice initiale jusqu'à l'indice `mid` et une autre à partir de
l'indice jusqu'à la fin de la slice.

<!--
When we try to compile the code in Listing 19-5, we’ll get an error.
-->

Lorsque nous essayons de compiler le code de l'encart 19-5, nous allons obtenir
une erreur.

<!--
```console
{{#include ../listings-sources/ch19-advanced-features/listing-19-05/output.txt}}
```
-->

```console
{{#include ../listings/ch19-advanced-features/listing-19-05/output.txt}}
```

<!--
Rust’s borrow checker can’t understand that we’re borrowing different parts of
the slice; it only knows that we’re borrowing from the same slice twice.
Borrowing different parts of a slice is fundamentally okay because the two
slices aren’t overlapping, but Rust isn’t smart enough to know this. When we
know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.
-->

Le vérificateur d'emprunt de Rust ne comprend pas que nous empruntons
différentes parties de la slice ; il comprend seulement que nous empruntons la
même slice à deux reprises. L'emprunt de différentes parties d'une slice ne
pose fondamentalement pas de problèmes car les deux slices ne se chevauchent
pas, mais Rust n'est pas suffisamment intelligent pour comprendre ceci. Lorsque
nous savons que ce code est correct, mais que Rust ne le sait pas, il est
approprié d'utiliser du code non sécurisé.

<!--
Listing 19-6 shows how to use an `unsafe` block, a raw pointer, and some calls
to unsafe functions to make the implementation of `split_at_mut` work.
-->

L'encart 19-6 montre comment utiliser un bloc `unsafe`, un pointeur brut, et
quelques appels à des fonctions non sécurisées pour construire une
implémentation de `split_at_mut` qui fonctionne.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-06/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-6: Using unsafe code in the implementation of
the `split_at_mut` function</span>
-->

<span class="caption">Encart 19-6 : utilisation de code non sécurisé dans
l'implémentation de la fonction `split_at_mut`</span>

<!--
Recall from [“The Slice Type”][the-slice-type]<!-- ignore -- > section in
Chapter 4 that slices are a pointer to some data and the length of the slice.
We use the `len` method to get the length of a slice and the `as_mut_ptr`
method to access the raw pointer of a slice. In this case, because we have a
mutable slice to `i32` values, `as_mut_ptr` returns a raw pointer with the type
`*mut i32`, which we’ve stored in the variable `ptr`.
-->

Souvenez-vous de la section [“Le type slice”][the-slice-type]<!-- ignore --> du
chapitre 4 dans laquelle nous avions dit qu'une slice est définie par un pointeur
vers une donnée ainsi qu'une longueur de la slice. Nous avons utilisé la méthode
`len` pour obtenir la longueur d'une slice ainsi que la méthode `as_mut_ptr`
pour accéder au pointeur brut d'une slice. Dans ce cas, comme nous avons une
slice mutable de valeurs `i32`, `as_mut_ptr` retourne un pointeur brut avec le
type `*mut i32` que nous stockons dans la variable `ptr`.

<!--
We keep the assertion that the `mid` index is within the slice. Then we get to
the unsafe code: the `slice::from_raw_parts_mut` function takes a raw pointer
and a length, and it creates a slice. We use this function to create a slice
that starts from `ptr` and is `mid` items long. Then we call the `add`
method on `ptr` with `mid` as an argument to get a raw pointer that starts at
`mid`, and we create a slice using that pointer and the remaining number of
items after `mid` as the length.
-->

Nous avons conservé la vérification que l'indice `mid` soit dans la slice.
Ensuite, nous utilisons le code non sécurisé : la fonction
`slice::from_raw_parts_mut` prend en paramètre un pointeur brut et une longueur,
et elle créée une slice. Nous utilisons cette fonction pour créer une slice qui
débute à `ptr` et qui est longue de `mid` éléments. Ensuite nous faisons appel à
la méthode `add` sur `ptr` avec `mid` en argument pour obtenir un pointeur
brut qui démarre à `mid`, et nous créons une slice qui utilise ce pointeur et
le nombre restant d'éléments après `mid` comme longueur.

<!--
The function `slice::from_raw_parts_mut` is unsafe because it takes a raw
pointer and must trust that this pointer is valid. The `add` method on raw
pointers is also unsafe, because it must trust that the offset location is also
a valid pointer. Therefore, we had to put an `unsafe` block around our calls to
`slice::from_raw_parts_mut` and `add` so we could call them. By looking at
the code and by adding the assertion that `mid` must be less than or equal to
`len`, we can tell that all the raw pointers used within the `unsafe` block
will be valid pointers to data within the slice. This is an acceptable and
appropriate use of `unsafe`.
-->

La fonction `slice::from_raw_parts_mut` est non sécurisée car elle prend en
argument un pointeur brut et doit avoir confiance en la validité de ce pointeur.
La méthode `add` sur les pointeurs bruts est aussi non sécurisée, car elle
doit croire que l'emplacement décalé est aussi un pointeur valide. Voilà
pourquoi nous avons placé un bloc `unsafe` autour de nos appels à
`slice::from_raw_parts_mut` et `add` afin que nous puissions les effectuer. En
analysant le code et en ayant ajouté la vérification que `mid` doit être
inférieur ou égal à `len`, nous pouvons affirmer que tous les pointeurs bruts
utilisés dans le bloc `unsafe` sont des pointeurs valides vers les données de la
slice. C'est une utilisation acceptable et appropriée de `unsafe`.

<!--
Note that we don’t need to mark the resulting `split_at_mut` function as
`unsafe`, and we can call this function from safe Rust. We’ve created a safe
abstraction to the unsafe code with an implementation of the function that uses
`unsafe` code in a safe way, because it creates only valid pointers from the
data this function has access to.
-->

Remarquez que nous n'avons pas eu besoin de marquer la fonction résultante
`split_at_mut` comme étant `unsafe`, et que nous pouvons faire appel à cette
fonction dans du code Rust sécurisé. Nous avons créé une abstraction sécurisée
du code non sécurisé avec une implémentation de la fonction qui utilise de
manière sécurisée du code non sécurisé, car elle créée uniquement des pointeurs
valides à partir des données auxquelles cette fonction a accès.

<!--
In contrast, the use of `slice::from_raw_parts_mut` in Listing 19-7 would
likely crash when the slice is used. This code takes an arbitrary memory
location and creates a slice 10,000 items long.
-->

En contre-partie, l'utilisation de `slice::from_raw_parts_mut` dans l'encart
19-7 peut planter lorsque la slice sera utilisée. Ce code prend un emplacement
arbitraire dans la mémoire et crée un slice de 10 000 éléments.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-07/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-07/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-7: Creating a slice from an arbitrary memory
location</span>
-->

<span class="caption">Encart 19-7 : création d'une slice à partir d'un
emplacement mémoire arbitraire</span>

<!--
We don’t own the memory at this arbitrary location, and there is no guarantee
that the slice this code creates contains valid `i32` values. Attempting to use
`values` as though it’s a valid slice results in undefined behavior.
-->

Nous ne possédons pas la mémoire à cet emplacement arbitraire, et il n'y a
aucune garantie que la slice créée par ce code contiennent des valeurs `i32`
valides. Toute tentative d'utilisation de `valeurs` aura un comportement
imprévisible bien qu'il s'agisse d'une slice valide.

<!--
#### Using `extern` Functions to Call External Code
-->

#### Utiliser des fonctions `extern` pour faire appel à du code externe

<!--
Sometimes, your Rust code might need to interact with code written in another
language. For this, Rust has a keyword, `extern`, that facilitates the creation
and use of a *Foreign Function Interface (FFI)*. An FFI is a way for a
programming language to define functions and enable a different (foreign)
programming language to call those functions.
-->

Parfois, votre code Rust peut avoir besoin d'interagir avec du code écrit dans
d'autres langages. Dans ce cas, Rust propose un mot-clé, `extern`, qui facilite
la création et l'utilisation du *Foreign Function Interface (FFI)*. Le FFI est
un outil permettant à un langage de programmation de définir des fonctions auxquelles
d'autres langages de programmation pourront faire appel.

<!--
Listing 19-8 demonstrates how to set up an integration with the `abs` function
from the C standard library. Functions declared within `extern` blocks are
always unsafe to call from Rust code. The reason is that other languages don’t
enforce Rust’s rules and guarantees, and Rust can’t check them, so
responsibility falls on the programmer to ensure safety.
-->

L'encart 19-8 montre comment configurer l'intégration de la fonction `abs` de la
bibliothèque standard du C. Les fonctions déclarées dans des blocs `extern`
sont toujours non sécurisées lorsqu'on les utilise dans du code Rust. La raison
à cela est que les autres langages n'appliquent pas les règles et garanties de
Rust, Rust ne peut donc pas les vérifier, si bien que la responsabilité de s'assurer
de la sécurité revient au développeur.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-08/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-08/src/main.rs}}
```

<!--
<span class="caption">Listing 19-8: Declaring and calling an `extern` function
defined in another language</span>
-->

<span class="caption">Encart 19-8 : déclaration et appel à une fonction externe
qui est définie dans un autre langage</span>

<!--
Within the `extern "C"` block, we list the names and signatures of external
functions from another language we want to call. The `"C"` part defines which
*application binary interface (ABI)* the external function uses: the ABI
defines how to call the function at the assembly level. The `"C"` ABI is the
most common and follows the C programming language’s ABI.
-->

Au sein du bloc `extern "C"`, nous listons les noms et les signatures des
fonctions externes de l'autre langage que nous souhaitons solliciter. La partie
"C" définit quelle est l'*application binary interface (ABI)* que la fonction
doit utiliser : l'ABI définit comment faire appel à la fonction au niveau
assembleur. L'ABI `"C"` est la plus courante et respecte l'ABI du langage de
programmation C.

<!--
> #### Calling Rust Functions from Other Languages
>
> We can also use `extern` to create an interface that allows other languages
> to call Rust functions. Instead of an `extern` block, we add the `extern`
> keyword and specify the ABI to use just before the `fn` keyword. We also need
> to add a `#[no_mangle]` annotation to tell the Rust compiler not to mangle
> the name of this function. *Mangling* is when a compiler changes the name
> we’ve given a function to a different name that contains more information for
> other parts of the compilation process to consume but is less human readable.
> Every programming language compiler mangles names slightly differently, so
> for a Rust function to be nameable by other languages, we must disable the
> Rust compiler’s name mangling.
>
> In the following example, we make the `call_from_c` function accessible from
> C code, after it’s compiled to a shared library and linked from C:
>
> ```rust
> #[no_mangle]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
> ```
>
> This usage of `extern` does not require `unsafe`.
-->

> #### Faire appel à des fonctions Rust dans d'autres langages
>
> Nous pouvons aussi utiliser `extern` pour créer une interface qui permet à
> d'autres langages de faire appel à des fonctions Rust. Au lieu d'avoir un
> bloc `extern`, nous ajoutons le mot-clé `extern` et nous renseignons l'ABI
> à utiliser juste avant le mot-clé `fn`. Nous avons aussi besoin d'ajouter
> l'annotation `#[no_mangle]` pour dire au compilateur Rust de ne pas déformer
> le nom de cette fonction. La *déformation* s'effectue lorsqu'un compilateur
> change le nom que nous avons donné à une fonction pour un nom qui contient
> plus d'informations pour d'autres étapes du processus de compilation, mais
> qui est moins lisible par l'humain. Tous les compilateurs de langages de
> programmation déforment les noms de façon légèrement différente, donc pour
> que le nom d'une fonction Rust soit utilisable par d'autres langages, nous
> devons désactiver la déformation du nom par le compilateur de Rust.

<!--
### Accessing or Modifying a Mutable Static Variable
-->

### Lire ou modifier une variable statique mutable

<!--
Until now, we’ve not talked about *global variables*, which Rust does support
but can be problematic with Rust’s ownership rules. If two threads are
accessing the same mutable global variable, it can cause a data race.
-->

Jusqu'à présent, nous n'avons pas parlé des *variables globales*, que Rust
accepte mais qui peuvent poser des problèmes avec les règles de possession de
Rust. Si deux tâches accèdent en même temps à la même variable globale, cela
peut causer un accès concurrent.

<!--
In Rust, global variables are called *static* variables. Listing 19-9 shows an
example declaration and use of a static variable with a string slice as a
value.
-->

En Rust, les variables globales s'appellent des variables *statiques*. L'encart
19-9 montre un exemple de déclaration et d'utilisation d'une variable statique
avec une slice de chaîne de caractères comme valeur.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-09/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-09/src/main.rs}}
```

<!--
<span class="caption">Listing 19-9: Defining and using an immutable static
variable</span>
-->

<span class="caption">Encart 19-9 : définition et utilisation d'une variable
statique immuable</span>

<!--
Static variables are similar to constants, which we discussed in the
[“Differences Between Variables and
Constants”][differences-between-variables-and-constants]<!-- ignore -- >
section in Chapter 3. The names of static variables are in
`SCREAMING_SNAKE_CASE` by convention. Static variables can only store
references with the `'static` lifetime, which means the Rust compiler can
figure out the lifetime and we aren’t required to annotate it explicitly.
Accessing an immutable static variable is safe.
-->

Les variables statiques ressemblent aux constantes, que nous avons vues dans la
section
[“Différences entre les variables et les constantes”][differences-between-variables-and-constants]<!-- ignore -->
du chapitre 3. Les noms des variables statiques sont par convention en
`SCREAMING_SNAKE_CASE`. Les variables statiques
peuvent uniquement stocker des références ayant la durée de vie `'static`, de façon
à ce que le compilateur Rust puisse la déterminer tout seul et que nous
n'ayons pas besoin de la renseigner explicitement. L'accès à une variable
statique immuable est sécurisé.

<!--
Constants and immutable static variables might seem similar, but a subtle
difference is that values in a static variable have a fixed address in memory.
Using the value will always access the same data. Constants, on the other hand,
are allowed to duplicate their data whenever they’re used.
-->

Les constantes et les variables statiques immuables se ressemblent, mais leur
différence subtile est que les valeurs dans les variables statiques ont une
adresse fixe en mémoire. L'utilisation de sa valeur va toujours accéder à la
même donnée. Les constantes en revanche, peuvent reproduire leurs données à
chaque fois qu'elles sont utilisées.

<!--
Another difference between constants and static variables is that static
variables can be mutable. Accessing and modifying mutable static variables is
*unsafe*. Listing 19-10 shows how to declare, access, and modify a mutable
static variable named `COUNTER`.
-->

Une autre différence entre les constantes et les variables statiques est que
les variables statiques peuvent être mutables. Lire et modifier des variables
statiques mutables est *non sécurisé*. L'encart 19-10 montre comment déclarer,
lire et modifier la variable statique mutable `COMPTEUR`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-10/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-10/src/main.rs}}
```

<!--
<span class="caption">Listing 19-10: Reading from or writing to a mutable
static variable is unsafe</span>
-->

<span class="caption">Encart 19-10 : la lecture et l'écriture d'une variable
statique mutable est non sécurisé</span>

<!--
As with regular variables, we specify mutability using the `mut` keyword. Any
code that reads or writes from `COUNTER` must be within an `unsafe` block. This
code compiles and prints `COUNTER: 3` as we would expect because it’s single
threaded. Having multiple threads access `COUNTER` would likely result in data
races.
-->

Comme avec les variables classiques, nous renseignons la mutabilité en
utilisant le mot-clé `mut`. Tout code qui lit ou modifie `COMPTEUR` doit se
trouver dans un bloc `unsafe`. Ce code se compile et affiche `COMPTEUR : 3`
comme nous l'espérions car nous n'avons qu'une seule tâche. Si nous avions
plusieurs tâches qui accèdent à `COMPTEUR`, nous pourrions avoir un accès
concurrent.

<!--
With mutable data that is globally accessible, it’s difficult to ensure there
are no data races, which is why Rust considers mutable static variables to be
unsafe. Where possible, it’s preferable to use the concurrency techniques and
thread-safe smart pointers we discussed in Chapter 16 so the compiler checks
that data accessed from different threads is done safely.
-->

Avec les données mutables qui sont accessibles globalement, il devient
difficile de s'assurer qu'il n'y a pas d'accès concurrent, c'est pourquoi Rust
considère les variables statiques mutables comme étant non sécurisées. Lorsque
c'est possible, il vaut mieux utiliser les techniques de concurrence et les
pointeurs intelligents adaptés au multitâche que nous avons vus au chapitre 16,
afin que le compilateur puisse vérifier que les données qu'utilisent les
différentes tâches sont sécurisées.

<!--
### Implementing an Unsafe Trait
-->

### Implémenter un trait non sécurisé

<!--
Another use case for `unsafe` is implementing an unsafe trait. A trait is
unsafe when at least one of its methods has some invariant that the compiler
can’t verify. We can declare that a trait is `unsafe` by adding the `unsafe`
keyword before `trait` and marking the implementation of the trait as `unsafe`
too, as shown in Listing 19-11.
-->

Un autre cas d'usage de `unsafe` est l'implémentation d'un trait non sécurisé.
Un trait n'est pas sécurisé lorsque au moins une de ses méthodes contient une
invariante que le compilateur ne peut pas vérifier. Nous pouvons déclarer un
trait qui n'est pas sécurisé en ajoutant le mot-clé `unsafe` devant `trait` et
en marquant aussi l'implémentation du trait comme `unsafe`, comme dans
l'encart 19-11.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-11/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-11/src/main.rs}}
```

<!--
<span class="caption">Listing 19-11: Defining and implementing an unsafe
trait</span>
-->

<span class="caption">Encart 19-11 : définition et implémentation d'un trait non
sécurisé</span>

<!--
By using `unsafe impl`, we’re promising that we’ll uphold the invariants that
the compiler can’t verify.
-->

En utilisant `unsafe impl`, nous promettons que nous veillons aux invariantes
que le compilateur ne peut pas vérifier.

<!--
As an example, recall the `Sync` and `Send` marker traits we discussed in the
[“Extensible Concurrency with the `Sync` and `Send`
Traits”][extensible-concurrency-with-the-sync-and-send-traits]<!-- ignore -- >
section in Chapter 16: the compiler implements these traits automatically if
our types are composed entirely of `Send` and `Sync` types. If we implement a
type that contains a type that is not `Send` or `Sync`, such as raw pointers,
and we want to mark that type as `Send` or `Sync`, we must use `unsafe`. Rust
can’t verify that our type upholds the guarantees that it can be safely sent
across threads or accessed from multiple threads; therefore, we need to do
those checks manually and indicate as such with `unsafe`.
-->

Par exemple, souvenez-vous des traits `Sync` et `Send` que nous avions découverts
dans une section du
[chapitre 16][extensible-concurrency-with-the-sync-and-send-traits]<!--
ignore --> : le compilateur implémente automatiquement ces traits si nos types
sont entièrement composés des types `Send` et `Sync`. Si nous implémentions un
type qui contenait un type qui n'était pas `Send` ou `Sync`, tel que les
pointeurs bruts, et nous souhaitions marquer ce type comme étant `Send` ou
`Sync`, nous aurions dû utiliser `unsafe`. Rust ne peut pas vérifier que notre
type respecte les garanties pour que ce type puisse être envoyé en toute
sécurité entre des tâches ou qu'il puisse être utilisé par plusieurs tâches ;
en conséquence, nous avons besoin de faire ces vérifications manuellement et le
signaler avec `unsafe`.

<!--
### Accessing Fields of a Union
-->

### Utiliser des champs d'un Union

<!--
The final action that works only with `unsafe` is accessing fields of a
*union*. A `union` is similar to a `struct`, but only one declared field is
used in a particular instance at one time. Unions are primarily used to
interface with unions in C code. Accessing union fields is unsafe because Rust
can’t guarantee the type of the data currently being stored in the union
instance. You can learn more about unions in [the Rust Reference][reference].
-->

La dernière action qui fonctionne uniquement avec `unsafe` est d'accéder aux
champs d'un *union*. Un `union` ressemble à une `struct`, mais un seul champ de
ceux déclarés est utilisé dans une instance précise au même moment. Les unions
sont principalement utilisés pour s'interfacer avec les unions du code C.
L'accès aux champs des unions n'est pas sécurisé car Rust ne peut pas garantir
le type de la donnée qui est actuellement stockée dans l'instance de l'union.
Vous pouvez en apprendre plus sur les unions dans
[the Rust Reference][reference].

<!--
### When to Use Unsafe Code
-->

### Quand utiliser du code non sécurisé

<!--
Using `unsafe` to take one of the five actions (superpowers) just discussed
isn’t wrong or even frowned upon. But it is trickier to get `unsafe` code
correct because the compiler can’t help uphold memory safety. When you have a
reason to use `unsafe` code, you can do so, and having the explicit `unsafe`
annotation makes it easier to track down the source of problems when they occur.
-->

L'utilisation de `unsafe` pour mettre en oeuvre une des cinq actions (ou
super-pouvoirs) que nous venons d'aborder n'est pas une mauvaise chose et ne doit
pas être mal vu. Mais il est plus difficile de sécuriser du code `unsafe` car le
compilateur ne peut pas aider à garantir la sécurité de la mémoire. Lorsque vous
avez une bonne raison d'utiliser du code non sécurisé, vous pouvez le faire, et
vous aurez l'annotation explicite `unsafe` pour faciliter la recherche de la
source des problèmes lorsqu'ils surviennent.

<!--
[dangling-references]:
ch04-02-references-and-borrowing.html#dangling-references
[differences-between-variables-and-constants]:
ch03-01-variables-and-mutability.html#constants
[extensible-concurrency-with-the-sync-and-send-traits]:
ch16-04-extensible-concurrency-sync-and-send.html#extensible-concurrency-with-the-sync-and-send-traits
[the-slice-type]: ch04-03-slices.html#the-slice-type
[reference]: ../reference/items/unions.html
-->

[dangling-references]:
ch04-02-references-and-borrowing.html#les-références-pendouillantes
[differences-between-variables-and-constants]:
ch03-01-variables-and-mutability.html#les-constantes
[extensible-concurrency-with-the-sync-and-send-traits]:
ch16-04-extensible-concurrency-sync-and-send.html
[the-slice-type]: ch04-03-slices.html#le-type-slice
[reference]: https://doc.rust-lang.org/reference/items/unions.html
