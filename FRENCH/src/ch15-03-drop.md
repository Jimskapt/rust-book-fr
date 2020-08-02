<!--
## Running Code on Cleanup with the `Drop` Trait
-->

## Exécuter du code au nettoyage avec le trait `Drop`

<!--
The second trait important to the smart pointer pattern is `Drop`, which lets
you customize what happens when a value is about to go out of scope. You can
provide an implementation for the `Drop` trait on any type, and the code you
specify can be used to release resources like files or network connections.
We’re introducing `Drop` in the context of smart pointers because the
functionality of the `Drop` trait is almost always used when implementing a
smart pointer. For example, when a `Box<T>` is dropped it will deallocate the space
on the heap that the box points to.
-->

Le second trait important pour les pointeurs intelligents est `Drop`, qui vous
permet de personnaliser ce qui se passe lorsqu'une valeur est en train de sortir
d'une portée. Vous pouvez fournir une implémentation du trait `Drop` sur
n'importe quel type, et le code que vous renseignez peut être utilisé pour
libérer des ressources comme des fichiers ou des connections réseau. Nous
présentons `Drop` dans le contexte des pointeurs intelligents car la
fonctionnalité du trait `Drop` est quasiment systématiquement utilisé
lorsque nous implémentons un pointeur intelligent. Par exemple, lorsqu'une
`Box<T>` est libérée, elle va désallouer l'espace occupé sur le tas sur lequel
la boite pointe.

<!--
In some languages, the programmer must call code to free memory or resources
every time they finish using an instance of a smart pointer. If they forget,
the system might become overloaded and crash. In Rust, you can specify that a
particular bit of code be run whenever a value goes out of scope, and the
compiler will insert this code automatically. As a result, you don’t need to be
careful about placing cleanup code everywhere in a program that an instance of
a particular type is finished with—you still won’t leak resources!
-->

Dans certains langages, le développeur doit appeler du code pour libérer la
mémoire ou des ressources à chaque fois qu'il finit d'utiliser une instance ou
un pointeur intelligent. S'il oublie de le faire, le système peut surcharger et
planter. Avec Rust, vous pouvez renseigner du code qui sera exécuté à chaque
fois qu'une valeur sort de la portée, et le compilateur va insérer
automatiquement ce code. Au final, vous n'avez pas besoin de concentrer votre
attention à placer du code de nettoyage à chaque fois qu'une instance d'un type
particulier n'est plus utilisée — vous ne risquez pas d'avoir des fuites de
ressources !

<!--
Specify the code to run when a value goes out of scope by implementing the
`Drop` trait. The `Drop` trait requires you to implement one method named
`drop` that takes a mutable reference to `self`. To see when Rust calls `drop`,
let’s implement `drop` with `println!` statements for now.
-->

Vous renseignez le code à exécuter lorsqu'une valeur sort de la portée en
implémentant le trait `Drop`. Le trait `Drop` nécessite que vous implémentiez
une méthode `drop` qui prend en paramètre une référence mutable à `self`. Pour
visualiser lorsque Rust appelle `drop`, implémentons `drop` avec une instruction
`println!` à l'intérieur, pour le moment.

<!--
Listing 15-14 shows a `CustomSmartPointer` struct whose only custom
functionality is that it will print `Dropping CustomSmartPointer!` when the
instance goes out of scope. This example demonstrates when Rust runs the `drop`
function.
-->

L'encart 15-14 montre une structure `PointeurPerso` dont la seule fonctionnalité
personnalisée est qu'elle va écrire `Nettoyage d'un PointeurPerso !` lorsque
l'instance sort de la portée. Cet exemple montre lorsque Rust exécute la
fonction `drop`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-14/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

<!--
<span class="caption">Listing 15-14: A `CustomSmartPointer` struct that
implements the `Drop` trait where we would put our cleanup code</span>
-->

<span class="caption">Encart 15-14 : Une structure `PointeurPerso` qui
implémente le trait `Drop` dans lequel nous plaçons notre code de nettoyage
</span>

<!--
The `Drop` trait is included in the prelude, so we don’t need to bring it into
scope. We implement the `Drop` trait on `CustomSmartPointer` and provide an
implementation for the `drop` method that calls `println!`. The body of the
`drop` function is where you would place any logic that you wanted to run when
an instance of your type goes out of scope. We’re printing some text here to
demonstrate when Rust will call `drop`.
-->

Le trait `Drop` est importé dans l'étape préliminaire, donc nous n'avons pas
besoin de l'importer dans la portée. Nous implémentons le trait `Drop` sur
`PointeurPerso` et nous fournissons une implémentation de la méthode `drop` qui
appelle `println!`. Le corps de la fonction `drop` est l'endroit où vous placez
la logique que vous souhaitez exécuter lorsqu'une instance du type concerné sort
de la portée. Ici nous affichons un petit texte pour visionner lorsque Rust
appelle `drop`.

<!--
In `main`, we create two instances of `CustomSmartPointer` and then print
`CustomSmartPointers created`. At the end of `main`, our instances of
`CustomSmartPointer` will go out of scope, and Rust will call the code we put
in the `drop` method, printing our final message. Note that we didn’t need to
call the `drop` method explicitly.
-->

Dans le `main`, nous créons deux instances de `PointeurPerso` et ensuite on
affiche `PointeurPerso créés`. A la fin du `main`, nos instances de
`PointeurPerso` vont sortir de la portée, et Rust va appeler le code que nous
avons placé explicitement dans a méthode `drop`.

<!--
When we run this program, we’ll see the following output:
-->

Lorsque nous exécutons ce programme, nous devrions voir la sortie suivante :

<!--
```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```
-->

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

<!--
Rust automatically called `drop` for us when our instances went out of scope,
calling the code we specified. Variables are dropped in the reverse order of
their creation, so `d` was dropped before `c`. This example gives you a visual
guide to how the `drop` method works; usually you would specify the cleanup
code that your type needs to run rather than a print message.
-->

Rust a appelé automatiquement `drop` pour nous lorsque nos instances sont
sorties de la portée, et appelé le code que nous avons renseigné. Les variables
sont libérées dans l'ordre inverse à leur création, donc `d` a été libéré avant
`c`. Cet exemple vous fournit une illustration de comment la méthode `drop`
fonctionne ; normalement vous devriez renseigner le code de nettoyage que votre
type a besoin d'exécuter plutôt que d'afficher un message.

<!--
### Dropping a Value Early with `std::mem::drop`
-->

### Libérer prématurément une valeur avec `std::mem::drop`

<!--
Unfortunately, it’s not straightforward to disable the automatic `drop`
functionality. Disabling `drop` isn’t usually necessary; the whole point of the
`Drop` trait is that it’s taken care of automatically. Occasionally, however,
you might want to clean up a value early. One example is when using smart
pointers that manage locks: you might want to force the `drop` method that
releases the lock so that other code in the same scope can acquire the lock.
Rust doesn’t let you call the `Drop` trait’s `drop` method manually; instead
you have to call the `std::mem::drop` function provided by the standard library
if you want to force a value to be dropped before the end of its scope.
-->

Malheureusement, il n'est pas simple de désactiver la fonctionnalité automatique
`drop`. La désactivation de `drop` n'est généralement pas nécessaire ; tout
l'intérêt du trait `Drop` est qu'il est pris en charge automatiquement.
Occasionnellement, cependant, vous pourriez avoir besoin de nettoyer
prématurément une valeur. Un exemple est lorsque vous utilisez des pointeurs
intelligents qui gèrent un système de verrouillage : vous pourriez vouloir
forcer la méthode `drop` qui libère le verrou afin qu'un autre code dans la même
portée puisse prendre ce verrou. Rust ne vous autorise pas à appeler
manuellement la méthode `drop` du trait `Drop` ; à la place vous devez appeler
la fonction `std::mem::drop`, fournie par la bibliothèque standard, si vous
souhaitez forcer une valeur à être libérée avant la fin de sa portée.

<!--
If we try to call the `Drop` trait’s `drop` method manually by modifying the
`main` function from Listing 15-14, as shown in Listing 15-15, we’ll get a
compiler error:
-->

Si nous essayons d'appeler manuellement la méthode `drop` du trait `Drop` en
modifiant la fonction `main` de l'encart 15-14, comme dans l'encart 15-15, nous
aurons une erreur de compilation :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-15: Attempting to call the `drop` method from
the `Drop` trait manually to clean up early</span>
-->

<span class="caption">Encart 15-15 : tentative d'appel manuel à la méthode
`drop` du trait `Drop` afin de nettoyer prématurément</span>

<!--
When we try to compile this code, we’ll get this error:
-->

Lorsque nous essayons de compiler ce code, nous obtenons l'erreur suivante :

<!--
```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```
-->

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

<!--
This error message states that we’re not allowed to explicitly call `drop`. The
error message uses the term *destructor*, which is the general programming term
for a function that cleans up an instance. A *destructor* is analogous to a
*constructor*, which creates an instance. The `drop` function in Rust is one
particular destructor.
-->

Ce message d'erreur signifie que nous ne sommes pas autorisés à appeler
explicitement `drop`. Le message d'erreur utilise le terme de *destructeur*
(`destructor`) qui est un terme général de programmation qui désigne une
fonction qui nettoie une instance. Un *destructeur* est analogue à un
*constructeur*, qui construit une instance. La fonction `drop` en Rust est un
destructeur particulier.

<!--
Rust doesn’t let us call `drop` explicitly because Rust would still
automatically call `drop` on the value at the end of `main`. This would be a
*double free* error because Rust would be trying to clean up the same value
twice.
-->

Rust ne nous laisse pas appeler explicitement `drop` car Rust appellera toujours
automatiquement `drop` sur la valeur à la fin du `main`. Cela serait une erreur
de *double libération* car Rust va essayer de nettoyer la même valeur deux fois.

<!--
We can’t disable the automatic insertion of `drop` when a value goes out of
scope, and we can’t call the `drop` method explicitly. So, if we need to force
a value to be cleaned up early, we can use the `std::mem::drop` function.
-->

Nous ne pouvons pas désactiver l'ajout automatique de `drop` lorsqu'une valeur
sort de la portée, et nous ne pouvons pas désactiver explicitement la méthode
`drop`. Donc, si nous avons besoin de forcer une valeur à être nettoyée
prématurément, nous pouvons utiliser la fonction `std::mem::drop`.

<!--
The `std::mem::drop` function is different from the `drop` method in the `Drop`
trait. We call it by passing the value we want to force to be dropped early as
an argument. The function is in the prelude, so we can modify `main` in Listing
15-15 to call the `drop` function, as shown in Listing 15-16:
-->

La fonction `std::mem::drop` est différente de la méthode `drop` du trait
`Drop`. Nous pouvons l'appeler en lui passant en argument la valeur que nous
souhaitons libérer prématurément. La fonction est présente dans l'étape
préliminaire, donc nous pouvons modifier `main` de l'encart 15-15 pour appeler
la fonction `drop`, comme dans l'encart 15-16 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-16: Calling `std::mem::drop` to explicitly
drop a value before it goes out of scope</span>
-->

<span class="caption">Encart 15-16 : appel à `std::mem::drop` pour libérer
explicitement une valeur avant qu'elle sorte de la portée</span>

<!--
Running this code will print the following:
-->

L'exécution de code va afficher ceci :

<!--
```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```
-->

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

<!--
The text ```Dropping CustomSmartPointer with data `some data`!``` is printed
between the `CustomSmartPointer created.` and `CustomSmartPointer dropped
before the end of main.` text, showing that the `drop` method code is called to
drop `c` at that point.
-->

Le texte ```Nettoyage d'un CustomSmartPointer avec la donnée `des trucs` !```
est affiché entre `PointeurPerso créé` et
`PointeurPerso libéré avant la fin du main`, ce qui démontre que la méthode
`drop` a été appelée pour libérer `c` à cet endroit.

<!--
You can use code specified in a `Drop` trait implementation in many ways to
make cleanup convenient and safe: for instance, you could use it to create your
own memory allocator! With the `Drop` trait and Rust’s ownership system, you
don’t have to remember to clean up because Rust does it automatically.
-->

Vous pouvez utiliser le code renseigné dans une implémentation du trait `Drop`
de plusieurs manières afin de rendre le nettoyage pratique et sûr : par exemple,
vous pouvez l'utiliser pour créer votre propre alloueur de mémoire ! Grâce au
trait `Drop` et le système de possession de Rust, vous n'avez pas à vous
souvenir de nettoyer car Rust le fait automatiquement.

<!--
You also don’t have to worry about problems resulting from accidentally
cleaning up values still in use: the ownership system that makes sure
references are always valid also ensures that `drop` gets called only once when
the value is no longer being used.
-->

Vous n'avez pas non plus à vous soucier des problèmes résultants du nettoyage
accidentel de valeurs toujours utilisées : le système de possession assurant que
les références sont toujours en vigueur assure également que `drop` n'est appelé
qu'une seule fois lorsque la valeur n'est plus utilisée.

<!--
Now that we’ve examined `Box<T>` and some of the characteristics of smart
pointers, let’s look at a few other smart pointers defined in the standard
library.
-->

Maintenant que nous avons examiné `Box<T>` et certaines des caractéristiques des
pointeurs intelligents, découvrons d'autres pointeurs intelligents définis dans
la bibliothèque standard.
