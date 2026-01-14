<!--
## Recoverable Errors with `Result`
-->

## Des erreurs récupérables avec `Result`

<!--
Most errors aren’t serious enough to require the program to stop entirely.
Sometimes, when a function fails, it’s for a reason that you can easily
interpret and respond to. For example, if you try to open a file and that
operation fails because the file doesn’t exist, you might want to create the
file instead of terminating the process.
-->

La plupart des erreurs ne sont pas assez graves au point d'arrêter complètement
le programme. Parfois, lorsqu'une fonction échoue, c'est pour une raison que
vous pouvez facilement comprendre et pour laquelle vous pouvez agir en
conséquence. Par exemple, si vous essayez d'ouvrir un fichier et que l'opération
échoue parce que le fichier n'existe pas, vous pourriez vouloir créer le fichier
plutôt que d'arrêter le processus.

<!--
Recall from [“Handling Potential Failure with the `Result`
Type”][handle_failure]<!-- ignore -- > in Chapter 2 that the `Result` enum is
defined as having two variants, `Ok` and `Err`, as follows:
-->

Souvenez-vous de la section [“Gérer les erreurs potentielles avec le type
`Result`”][handle_failure]<!-- ignore --> du chapitre 2 que l'énumération
`Result` possède deux variantes, `Ok` et `Err`, comme ci-dessous :

<!--
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
-->

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<!--
The `T` and `E` are generic type parameters: we’ll discuss generics in more
detail in Chapter 10. What you need to know right now is that `T` represents
the type of the value that will be returned in a success case within the `Ok`
variant, and `E` represents the type of the error that will be returned in a
failure case within the `Err` variant. Because `Result` has these generic type
parameters, we can use the `Result` type and the functions defined on it in
many different situations where the successful value and error value we want to
return may differ.
-->

Le `T` et le `E` sont des paramètres de type génériques : nous parlerons plus en
détail de la généricité au chapitre 10. Tout ce que vous avez besoin de savoir
pour le moment, c'est que `T` représente le type de valeur imbriquée dans la
variante `Ok` qui sera retournée dans le cas d'un succès, et `E` représente le
type d'erreur imbriquée dans la variante `Err` qui sera retournée dans le cas
d'un échec. Comme `Result` a ces paramètres de type génériques, nous pouvons
utiliser le type `Result` et les fonctions associées dans différentes
situations où la valeur de succès et la valeur d'erreur peuvent varier.

<!--
Let’s call a function that returns a `Result` value because the function could
fail. In Listing 9-3 we try to open a file.
-->

Utilisons une fonction qui retourne une valeur de type `Result` car la fonction
peut échouer. Dans l'encart 9-3, nous essayons d'ouvrir un fichier :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch09-error-handling/listing-09-03/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<!--
<span class="caption">Listing 9-3: Opening a file</span>
-->

<span class="caption">Encart 9-3 : ouverture d'un fichier</span>

<!--
How do we know `File::open` returns a `Result`? We could look at the [standard
library API documentation](../std/index.html)<!-- ignore -- >, or we could ask
the compiler! If we give `f` a type annotation that we know is *not* the return
type of the function and then try to compile the code, the compiler will tell
us that the types don’t match. The error message will then tell us what the
type of `f` *is*. Let’s try it! We know that the return type of `File::open`
isn’t of type `u32`, so let’s change the `let f` statement to this:
-->

Comment savons-nous que `File::open` retourne un `Result` ? Nous pouvons
consulter la [documentation de l'API de la bibliothèque
standard](https://doc.rust-lang.org/std/index.html)<!-- ignore -->, ou nous
pouvons demander au compilateur ! Si nous appliquons à `f` une annotation de
type dont nous savons qu'elle n'est *pas* le type de retour de la fonction et
que nous essayons ensuite de compiler le code, le compilateur va nous dire que
les types ne correspondent pas. Le message d'erreur va ensuite nous dire *quel
est le type* de `f`. Essayons cela ! Nous savons que le type de retour de
`File::open` n'est pas `u32`, alors essayons de changer l'instruction `let f`
par ceci :

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch09-error-handling/no-listing-02-ask-compiler-for-type/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/src/main.rs:here}}
```

<!--
Attempting to compile now gives us the following output:
-->

Tenter de compiler ce code nous donne maintenant le résultat suivant :

<!--
```console
{{#include ../listings-sources/ch09-error-handling/no-listing-02-ask-compiler-for-type/output.txt}}
```
-->

```console
{{#include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/output.txt}}
```

<!--
This tells us the return type of the `File::open` function is a `Result<T, E>`.
The generic parameter `T` has been filled in here with the type of the success
value, `std::fs::File`, which is a file handle. The type of `E` used in the
error value is `std::io::Error`.
-->

Cela nous dit que le type de retour de la fonction `File::open` est de la forme
`Result<T, E>`. Le paramètre générique `T` a été remplacé dans ce cas par le
type en cas de succès, `std::fs::File`, qui permet d'interagir avec le fichier.
Le `E` utilisé pour la valeur d'erreur est du type `std::io::Error`.

<!--
This return type means the call to `File::open` might succeed and return a file
handle that we can read from or write to. The function call also might fail:
for example, the file might not exist, or we might not have permission to
access the file. The `File::open` function needs to have a way to tell us
whether it succeeded or failed and at the same time give us either the file
handle or error information. This information is exactly what the `Result` enum
conveys.
-->

Ce type de retour veut dire que l'appel à `File::open` peut réussir et nous
retourner un manipulateur de fichier qui peut nous permettre de le lire ou d'y
écrire. L'utilisation de cette fonction peut aussi échouer : par exemple, si le
fichier n'existe pas, ou si nous n'avons pas le droit d'accéder au fichier. La
fonction `File::open` doit avoir un moyen de nous dire si son utilisation a
réussi ou échoué et en même temps nous fournir soit le manipulateur de fichier,
soit des informations sur l'erreur. C'est exactement ces informations que
l'énumération `Result` se charge de nous transmettre.

<!--
In the case where `File::open` succeeds, the value in the variable `f` will be
an instance of `Ok` that contains a file handle. In the case where it fails,
the value in `f` will be an instance of `Err` that contains more information
about the kind of error that happened.
-->

Dans le cas où `File::open` réussit, la valeur que nous obtiendrons dans la
variable `f` sera une instance de `Ok` qui contiendra un manipulateur de
fichier. Dans le cas où cela échoue, la valeur dans `f` sera une instance de
`Err` qui contiendra plus d'information sur le type d'erreur qui a eu lieu.

<!--
We need to add to the code in Listing 9-3 to take different actions depending
on the value `File::open` returns. Listing 9-4 shows one way to handle the
`Result` using a basic tool, the `match` expression that we discussed in
Chapter 6.
-->

Nous avons besoin d'ajouter différentes actions dans le code de l'encart 9-3 en
fonction de la valeur que `File::open` retourne. L'encart 9-4 montre une façon
de gérer le `Result` en utilisant un outil basique, l'expression `match` que
nous avons vue au chapitre 6.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,should_panic
{{#rustdoc_include ../listings-sources/ch09-error-handling/listing-09-04/src/main.rs}}
```
-->

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<!--
<span class="caption">Listing 9-4: Using a `match` expression to handle the
`Result` variants that might be returned</span>
-->

<span class="caption">Encart 9-4 : utilisation de l'expression `match` pour
gérer les variantes de `Result` qui peuvent être retournées</span>

<!--
Note that, like the `Option` enum, the `Result` enum and its variants have been
brought into scope by the prelude, so we don’t need to specify `Result::`
before the `Ok` and `Err` variants in the `match` arms.
-->

Remarquez que, tout comme l'énumération `Option`, l'énumération `Result` et ses
variantes ont été importées par l'étape préliminaire, donc vous n'avez pas
besoin de préciser `Result::` devant les variantes `Ok` et `Err` dans les
branches du `match`.

<!--
When the result is `Ok`, this code will return the inner `file` value out of
the `Ok` variant, and we then assign that file handle value to the variable
`f`. After the `match`, we can use the file handle for reading or writing.
-->

Lorsque le résultat est `Ok`, ce code va retourner la valeur `fichier` contenue
dans la variante `Ok`, et nous assignons ensuite cette valeur à la variable
`f`. Après le `match`, nous pourrons ensuite utiliser le manipulateur de
fichier pour lire ou écrire.

<!--
The other arm of the `match` handles the case where we get an `Err` value from
`File::open`. In this example, we’ve chosen to call the `panic!` macro. If
there’s no file named *hello.txt* in our current directory and we run this
code, we’ll see the following output from the `panic!` macro:
-->

L'autre branche du bloc `match` gère le cas où nous obtenons un `Err` à l'appel
de `File::open`. Dans cet exemple, nous avons choisi de faire appel à la macro
`panic!`. S'il n'y a pas de fichier qui s'appelle *hello.txt* dans notre
répertoire actuel et que nous exécutons ce code, nous allons voir la sortie
suivante suite à l'appel de la macro `panic!` :

<!--
```console
{{#include ../listings-sources/ch09-error-handling/listing-09-04/output.txt}}
```
-->

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

<!--
As usual, this output tells us exactly what has gone wrong.
-->

Comme d'habitude, cette sortie nous explique avec précision ce qui s'est mal
passé.

<!--
### Matching on Different Errors
-->

### Gérer les différentes erreurs

<!--
The code in Listing 9-4 will `panic!` no matter why `File::open` failed.
However, we want to take different actions for different failure reasons: if
`File::open` failed because the file doesn’t exist, we want to create the file
and return the handle to the new file. If `File::open` failed for any other
reason—for example, because we didn’t have permission to open the file—we still
want the code to `panic!` in the same way as it did in Listing 9-4. For this we
add an inner `match` expression, shown in Listing 9-5.
-->

Le code dans l'encart 9-4 va faire un `panic!` peu importe la raison de l'échec
de `File::open`. Cependant, nous voulons réagir différemment en fonction de
différents cas d'erreurs : si `File::open` a échoué parce que le
fichier n'existe pas, nous voulons créer le fichier et retourner le manipulateur
de fichier pour ce nouveau fichier. Si `File::open` échoue pour toute autre
raison, par exemple si nous n'avons pas l'autorisation d'ouvrir le fichier,
nous voulons quand même que le code lance un `panic!` de la même manière qu'il
l'a fait dans l'encart 9-4. C'est pourquoi nous avons ajouté dans l'encart 9-5
une expression `match` imbriquée :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -- >
-->

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch09-error-handling/listing-09-05/src/main.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<!--
<span class="caption">Listing 9-5: Handling different kinds of errors in
different ways</span>
-->

<span class="caption">Encart 9-5 : gestion des différents cas d'erreurs avec des
actions différentes</span>

<!--
The type of the value that `File::open` returns inside the `Err` variant is
`io::Error`, which is a struct provided by the standard library. This struct
has a method `kind` that we can call to get an `io::ErrorKind` value. The enum
`io::ErrorKind` is provided by the standard library and has variants
representing the different kinds of errors that might result from an `io`
operation. The variant we want to use is `ErrorKind::NotFound`, which indicates
the file we’re trying to open doesn’t exist yet. So we match on `f`, but we
also have an inner match on `error.kind()`.
-->

La valeur de retour de `File::open` logée dans la variante `Err` est de type
`io::Error`, qui est une structure fournie par la bibliothèque standard. Cette
structure a une méthode `kind` que nous pouvons appeler pour obtenir une valeur
de type `io::ErrorKind`. L'énumération `io::ErrorKind` est fournie elle aussi
par la bibliothèque standard et a des variantes qui représentent les différents
types d'erreurs qui pourraient résulter d'une opération provenant du module
`io`. La variante que nous voulons utiliser est `ErrorKind::NotFound`, qui
indique que le fichier que nous essayons d'ouvrir n'existe pas encore. Donc nous
utilisons `match` sur `f`, mais nous avons dans celle-ci un autre `match` sur
`erreur.kind()`.

<!--
The condition we want to check in the inner match is whether the value returned
by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is,
we try to create the file with `File::create`. However, because `File::create`
could also fail, we need a second arm in the inner `match` expression. When the
file can’t be created, a different error message is printed. The second arm of
the outer `match` stays the same, so the program panics on any error besides
the missing file error.
-->

Nous souhaitons vérifier dans le `match` interne si la valeur de retour de
`error.kind()` est la variante `NotFound` de l'énumération `ErrorKind`. Si c'est
le cas, nous essayons de créer le fichier avec `File::create`. Cependant, comme
`File::create` peut aussi échouer, nous avons besoin d'une seconde branche dans
le `match` interne. Lorsque le fichier ne peut pas être créé, un message
d'erreur différent est affiché. La seconde branche du `match` principal reste
inchangée, donc le programme panique lorsqu'on rencontre une autre erreur que
l'absence de fichier.

<!--
> ### Alternatives to Using `match` with `Result<T, E>`
>
> That’s a lot of `match`! The `match` expression is very useful but also very
> much a primitive. In Chapter 13, you’ll learn about closures, which are used
> with many of the methods defined on `Result<T, E>`. These methods can be more
> concise than using `match` when handling `Result<T, E>` values in your code.
>
> For example, here’s another way to write the same logic as shown in Listing
> 9-5 but using closures and the `unwrap_or_else` method:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -- >
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let f = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {:?}", error);
>             })
>         } else {
>             panic!("Problem opening the file: {:?}", error);
>         }
>     });
> }
> ```
>
> Although this code has the same behavior as Listing 9-5, it doesn’t contain
> any `match` expressions and is cleaner to read. Come back to this example
> after you’ve read Chapter 13, and look up the `unwrap_or_else` method in the
> standard library documentation. Many more of these methods can clean up huge
> nested `match` expressions when you’re dealing with errors.
-->

> ### D'autres solutions pour utiliser `match` avec `Result<T, E>`
>
> Cela commence à faire beaucoup de `match` ! L'expression `match` est très
> utile mais elle est aussi assez rudimentaire. Dans le chapitre 13, vous en
> apprendrez plus sur les fermetures, qui sont utilisées avec de nombreuses
> méthodes définies sur `Result<T, E>`. Ces méthodes peuvent s'avérer être plus
> concises que l'utilisation de `match` lorsque vous travaillez avec des
> valeurs `Result<T, E>` dans votre code.
>
> Par exemple, voici une autre manière d'écrire la même logique que celle dans
> l'encart 9-5 mais en utilisant les fermetures et la méthode
> `unwrap_or_else` :
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let f = File::open("hello.txt").unwrap_or_else(|erreur| {
>         if erreur.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|erreur| {
>                 panic!("Erreur de création du fichier : {:?}", erreur);
>             })
>         } else {
>             panic!("Erreur d'ouverture du fichier : {:?}", erreur);
>         }
>     });
> }
> ```
>
> Bien que ce code ait le même comportement que celui de l'encart 9-5, il ne
> contient aucune expression `match` et est plus facile à lire. Revenez sur cet
> exemple après avoir lu le chapitre 13, et renseignez-vous sur la méthode
> `unwrap_or_else` dans la documentation de la bibliothèque standard. De
> nombreuses méthodes de ce type peuvent clarifier de grosses expressions
> `match` imbriquées lorsque vous traitez les erreurs.

<!--
### Shortcuts for Panic on Error: `unwrap` and `expect`
-->

### Raccourcis pour faire un panic lors d'une erreur : `unwrap` et `expect`

<!--
Using `match` works well enough, but it can be a bit verbose and doesn’t always
communicate intent well. The `Result<T, E>` type has many helper methods
defined on it to do various, more specific tasks. The `unwrap` method is a
shortcut method implemented just like the `match` expression we wrote in
Listing 9-4. If the `Result` value is the `Ok` variant, `unwrap` will return
the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will
call the `panic!` macro for us. Here is an example of `unwrap` in action:
-->

L'utilisation de `match` fonctionne assez bien, mais elle peut être un peu
verbeuse et ne communique pas forcément bien son intention. Le type
`Result<T, E>` a de nombreuses méthodes qui lui ont été définies pour
différents cas. La méthode `unwrap` est une méthode de raccourci implémentée
comme l'expression `match` que nous avons écrite dans l'encart 9-4. Si la
valeur de `Result` est la variante `Ok`, `unwrap` va retourner la valeur
contenue dans le `Ok`. Si le `Result` est la variante `Err`, `unwrap` va
appeler la macro `panic!` pour nous. Voici un exemple de `unwrap` en action :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,should_panic
{{#rustdoc_include ../listings-sources/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```
-->

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

<!--
If we run this code without a *hello.txt* file, we’ll see an error message from
the `panic!` call that the `unwrap` method makes:
-->

Si nous exécutons ce code alors qu'il n'y a pas de fichier *hello.txt*, nous
allons voir un message d'erreur suite à l'appel à `panic!` que la méthode
`unwrap` a fait :

<!--
```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```
-->

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

<!--
Similarly, the `expect` method lets us also choose the `panic!` error message.
Using `expect` instead of `unwrap` and providing good error messages can convey
your intent and make tracking down the source of a panic easier. The syntax of
`expect` looks like this:
-->

De la même manière, la méthode `expect` nous donne la possibilité de définir le
message d'erreur du `panic!`. Utiliser `expect` plutôt que `unwrap` et lui
fournir un bon message d'erreur permet de mieux exprimer le problème et
faciliter la recherche de la source d'un panic. La syntaxe de `expect` est la
suivante :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,should_panic
{{#rustdoc_include ../listings-sources/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```
-->

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

<!--
We use `expect` in the same way as `unwrap`: to return the file handle or call
the `panic!` macro. The error message used by `expect` in its call to `panic!`
will be the parameter that we pass to `expect`, rather than the default
`panic!` message that `unwrap` uses. Here’s what it looks like:
-->

Nous utilisons `expect` de la même manière que `unwrap` : pour retourner le
manipulateur de fichier ou appeler la macro `panic!`. Le message d'erreur
utilisé par `expect` lors de son appel à `panic!` sera le paramètre que nous
avons passé à `expect`, plutôt que le message par défaut de `panic!` qu'utilise
`unwrap`. Voici ce que cela donne :

<!--
```text
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```
-->

```text
thread 'main' panicked at 'Échec à l'ouverture de hello.txt: Error { repr: Os {
code: 2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

<!--
Because this error message starts with the text we specified, `Failed to open
hello.txt`, it will be easier to find where in the code this error message is
coming from. If we use `unwrap` in multiple places, it can take more time to
figure out exactly which `unwrap` is causing the panic because all `unwrap`
calls that panic print the same message.
-->

Comme ce message d'erreur commence par le texte que nous avons précisé, `Échec à
l'ouverture de hello.txt`, ce sera plus facile de trouver là d'où provient ce
message d'erreur dans le code. Si nous utilisons `unwrap` à plusieurs endroits,
cela peut prendre plus de temps de comprendre exactement quel `unwrap` a causé
le panic, car tous les appels à `unwrap` vont afficher le même message.

<!--
### Propagating Errors
-->

### Propager les erreurs

<!--
When a function’s implementation calls something that might fail, instead of
handling the error within the function itself, you can return the error to the
calling code so that it can decide what to do. This is known as *propagating*
the error and gives more control to the calling code, where there might be more
information or logic that dictates how the error should be handled than what
you have available in the context of your code.
-->

Lorsqu'une fonction dont l'implémentation utilise quelque chose qui peut
échouer, au lieu de gérer l'erreur directement dans cette fonction, vous pouvez
retourner cette erreur au code qui l'appelle pour qu'il décide quoi faire.
C'est ce que l'on appelle *propager* l'erreur et donne ainsi plus de contrôle
au code qui appelle la fonction, dans lequel il peut y avoir plus
d'informations ou d'instructions pour traiter l'erreur que dans le contexte de
votre code.

<!--
For example, Listing 9-6 shows a function that reads a username from a file. If
the file doesn’t exist or can’t be read, this function will return those errors
to the code that called the function.
-->

Par exemple, l'encart 9-6 montre une fonction qui lit un pseudo à partir d'un
fichier. Si ce fichier n'existe pas ou ne peut pas être lu, cette fonction va
retourner ces erreurs au code qui a appelé la fonction.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -- >
-->

<!--
```rust
{{#include ../listings-sources/ch09-error-handling/listing-09-06/src/main.rs:here}}
```
-->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 9-6: A function that returns errors to the
calling code using `match`</span>
-->

<span class="caption">Encart 9-6 : une fonction qui retourne les erreurs au code
qui l'appelle en utilisant `match`</span>

<!--
This function can be written in a much shorter way, but we’re going to start by
doing a lot of it manually in order to explore error handling; at the end,
we’ll show the shorter way. Let’s look at the return type of the function
first: `Result<String, io::Error>`. This means the function is returning a
value of the type `Result<T, E>` where the generic parameter `T` has been
filled in with the concrete type `String`, and the generic type `E` has been
filled in with the concrete type `io::Error`. If this function succeeds without
any problems, the code that calls this function will receive an `Ok` value that
holds a `String`—the username that this function read from the file. If this
function encounters any problems, the calling code will receive an `Err` value
that holds an instance of `io::Error` that contains more information about what
the problems were. We chose `io::Error` as the return type of this function
because that happens to be the type of the error value returned from both of
the operations we’re calling in this function’s body that might fail: the
`File::open` function and the `read_to_string` method.
-->

Cette fonction peut être écrite de façon plus concise, mais nous avons décidé de
commencer par faire un maximum de choses manuellement pour découvrir la gestion
d'erreurs ; mais à la fin, nous verrons comment raccourcir le code. Commençons
par regarder le type de retour de la fonction : `Result<String, io::Error>`.
Cela signifie que la fonction retourne une valeur de type `Result<T, E>` où le
paramètre générique `T` a été remplacé par le type `String` et le paramètre
générique `E` a été remplacé par le type `io::Error`. Si cette fonction réussit
sans problème, le code appellant va obtenir une valeur `Ok` qui contient
une `String`, le pseudo que cette fonction lit dans le fichier. Si cette
fonction rencontre un problème, le code qui appelle cette fonction va obtenir
une valeur `Err` qui contient une instance de `io::Error` qui donne plus
d'informations sur la raison du problème. Nous avons choisi `io::Error` comme
type de retour de cette fonction parce qu'il se trouve que c'est le type
d'erreur de retour pour les deux opérations qui peuvent échouer que l'on utilise
dans le corps de cette fonction : la fonction `File::open` et la méthode
`read_to_string`.

<!--
The body of the function starts by calling the `File::open` function. Then we
handle the `Result` value with a `match` similar to the `match` in Listing 9-4.
If `File::open` succeeds, the file handle in the pattern variable `file`
becomes the value in the mutable variable `f` and the function continues. In
the `Err` case, instead of calling `panic!`, we use the `return` keyword to
return early out of the function entirely and pass the error value from
`File::open`, now in the pattern variable `e`, back to the calling code as this
function’s error value.
-->

Le corps de la fonction commence par appeler la fonction `File::open`. Ensuite,
nous gérons la valeur du `Result` avec un `match` similaire au `match` de
l'encart 9-4. Si le `File::open` est un succès, le manipulateur de fichier dans
la variable `fichier` du motif devient la valeur dans la variable mutable `f`
et la fonction continue son déroulement. Dans le cas d'un `Err`, au lieu
d'appeler `panic!`, nous utilisons `return` pour sortir prématurément de toute
la fonction et en passant la valeur du `File::open`, désormais dans la variable
`e`, au code appelant comme valeur de retour de cette fonction.

<!--
So if we have a file handle in `f`, the function then creates a new `String` in
variable `s` and calls the `read_to_string` method on the file handle in `f` to
read the contents of the file into `s`. The `read_to_string` method also
returns a `Result` because it might fail, even though `File::open` succeeded.
So we need another `match` to handle that `Result`: if `read_to_string`
succeeds, then our function has succeeded, and we return the username from the
file that’s now in `s` wrapped in an `Ok`. If `read_to_string` fails, we return
the error value in the same way that we returned the error value in the `match`
that handled the return value of `File::open`. However, we don’t need to
explicitly say `return`, because this is the last expression in the function.
-->

Donc si nous avons un manipulateur de fichier dans `f`, la fonction crée
ensuite une nouvelle `String` dans la variable `s` et nous appelons la méthode
`read_to_string` sur le manipulateur de fichier `f` pour extraire le contenu du
fichier dans `s`. La méthode `read_to_string` retourne aussi un `Result` car
elle peut échouer, même si `File::open` a réussi. Nous avons donc besoin d'un
nouveau `match` pour gérer ce `Result` : si `read_to_string` réussit, alors
notre fonction a réussi, et nous retournons le pseudo que nous avons extrait du
fichier qui est maintenant intégré dans un `Ok`, lui-même stocké dans `s`. Si
`read_to_string` échoue, nous retournons la valeur d'erreur de la même façon
que nous avons retourné la valeur d'erreur dans le `match` qui gérait la valeur
de retour de `File::open`. Cependant, nous n'avons pas besoin d'écrire
explicitement `return`, car c'est la dernière expression de la fonction.

<!--
The code that calls this code will then handle getting either an `Ok` value
that contains a username or an `Err` value that contains an `io::Error`. It’s
up to the calling code to decide what to do with those values. If the calling
code gets an `Err` value, it could call `panic!` and crash the program, use a
default username, or look up the username from somewhere other than a file, for
example. We don’t have enough information on what the calling code is actually
trying to do, so we propagate all the success or error information upward for
it to handle appropriately.
-->

Le code qui appelle ce code va devoir ensuite gérer les cas où il récupère une
valeur `Ok` qui contient un pseudo, ou une valeur `Err` qui contient une
`io::Error`. Il revient au code appelant de décider quoi faire avec ces
valeurs. Si le code appelant obtient une valeur `Err`, il peut appeler `panic!`
et faire planter le programme, utiliser un pseudo par défaut, ou chercher le
pseudo autre part que dans ce fichier, par exemple. Nous n'avons pas assez
d'informations sur ce que le code appelant a l'intention de faire, donc nous
remontons toutes les informations de succès ou d'erreur pour qu'elles soient
gérées correctement.

<!--
This pattern of propagating errors is so common in Rust that Rust provides the
question mark operator `?` to make this easier.
-->

Cette façon de propager les erreurs est si courante en Rust que Rust fournit
l'opérateur point d'interrogation `?` pour faciliter ceci.

<!--
#### A Shortcut for Propagating Errors: the `?` Operator
-->

#### Un raccourci pour propager les erreurs : l'opérateur `?`

<!--
Listing 9-7 shows an implementation of `read_username_from_file` that has the
same functionality as in Listing 9-6, but this implementation uses the
`?` operator.
-->

L'encart 9-7 montre une implémentation de `lire_pseudo_depuis_fichier` qui a
les mêmes fonctionnalités que dans l'encart 9-6, mais cette implémentation
utilise l'opérateur point d'interrogation `?` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -- >
-->

<!--
```rust
{{#include ../listings-sources/ch09-error-handling/listing-09-07/src/main.rs:here}}
```
-->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<!--
<span class="caption">Listing 9-7: A function that returns errors to the
calling code using the `?` operator</span>
-->

<span class="caption">Encart 9-7 : une fonction qui retourne les erreurs au code
appelant en utilisant l'opérateur `?`</span>

<!--
The `?` placed after a `Result` value is defined to work in almost the same way
as the `match` expressions we defined to handle the `Result` values in Listing
9-6. If the value of the `Result` is an `Ok`, the value inside the `Ok` will
get returned from this expression, and the program will continue. If the value
is an `Err`, the `Err` will be returned from the whole function as if we had
used the `return` keyword so the error value gets propagated to the calling
code.
-->

Le `?` placé après une valeur `Result` est conçu pour fonctionner presque de la
même manière que les expressions `match` que nous avons définies pour gérer les
valeurs `Result` dans l'encart 9-6. Si la valeur du `Result` est un `Ok`, la
valeur dans le `Ok` sera retournée par cette expression et le programme
continuera. Si la valeur est un `Err`, le `Err` sera retourné par la fonction
comme si nous avions utilisé le mot-clé `return` afin que la valeur d'erreur
soit propagée au code appelant.

<!--
There is a difference between what the `match` expression from Listing 9-6 does
and what the `?` operator does: error values that have the `?` operator called
on them go through the `from` function, defined in the `From` trait in the
standard library, which is used to convert errors from one type into another.
When the `?` operator calls the `from` function, the error type received is
converted into the error type defined in the return type of the current
function. This is useful when a function returns one error type to represent
all the ways a function might fail, even if parts might fail for many different
reasons. As long as there’s an `impl From<OtherError> for ReturnedError` to
define the conversion in the trait’s `from` function, the `?` operator takes
care of calling the `from` function automatically.
-->

Il y a une différence entre ce que fait l'expression `match` de l'encart 9-6 et
ce que fait l'opérateur `?` : les valeurs d'erreurs sur lesquelles est utilisé
l'opérateur `?` passent par la fonction `from`, définie dans le trait `From` de
la bibliothèque standard, qui est utilisée pour convertir les erreurs d'un type
à un autre. Lorsque l'opérateur `?` appelle la fonction `from`, le type d'erreur
reçu est converti dans le type d'erreur déclaré dans le type de retour de la
fonction concernée. C'est utile lorsqu'une fonction retourne un type d'erreur
qui peut couvrir tous les cas d'échec de la fonction, même si certaines de ses
parties peuvent échouer pour différentes raisons. À partir du moment qu'il y a
un `impl From<AutreErreur>` sur `ErreurRetournee` pour expliquer la conversion
dans la fonction `from` du trait, l'opérateur `?` se charge d'appeler la
fonction `from` automatiquement.

<!--
In the context of Listing 9-7, the `?` at the end of the `File::open` call will
return the value inside an `Ok` to the variable `f`. If an error occurs, the
`?` operator will return early out of the whole function and give any `Err`
value to the calling code. The same thing applies to the `?` at the end of the
`read_to_string` call.
-->

Dans le cas de l'encart 9-7, le `?` à la fin de l'appel à `File::open` va
retourner la valeur à l'intérieur d'un `Ok` à la variable `f`. Si une erreur se
produit, l'opérateur `?` va quitter prématurément la fonction et retourner une
valeur `Err` au code appelant. La même chose se produira au `?` à la fin de
l'appel à `read_to_string`.

<!--
The `?` operator eliminates a lot of boilerplate and makes this function’s
implementation simpler. We could even shorten this code further by chaining
method calls immediately after the `?`, as shown in Listing 9-8.
-->

L'opérateur `?` allège l'écriture de code et facilite l'implémentation de la
fonction. Nous pouvons même encore plus réduire ce code en enchaînant
immédiatement les appels aux méthodes après le `?` comme dans l'encart 9-8 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -- >
-->

<!--
```rust
{{#include ../listings-sources/ch09-error-handling/listing-09-08/src/main.rs:here}}
```
-->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<!--
<span class="caption">Listing 9-8: Chaining method calls after the `?`
operator</span>
-->

<span class="caption">Encart 9-8 : enchaînement des appels aux méthodes après
l'opérateur `?`</span>

<!--
We’ve moved the creation of the new `String` in `s` to the beginning of the
function; that part hasn’t changed. Instead of creating a variable `f`, we’ve
chained the call to `read_to_string` directly onto the result of
`File::open("hello.txt")?`. We still have a `?` at the end of the
`read_to_string` call, and we still return an `Ok` value containing the
username in `s` when both `File::open` and `read_to_string` succeed rather than
returning errors. The functionality is again the same as in Listing 9-6 and
Listing 9-7; this is just a different, more ergonomic way to write it.
-->

Nous avons déplacé la création de la nouvelle `String` dans `s` au début de la
fonction ; cette partie n'a pas changé. Au lieu de créer la variable `f`, nous
enchaînons directement l'appel à `read_to_string` sur le résultat de
`File::open("hello.txt")?`. Nous avons toujours le `?` à la fin de l'appel à
`read_to_string`, et nous retournons toujours une valeur `Ok` contenant le
pseudo dans `s` lorsque `File::open` et `read_to_string` réussissent toutes les
deux plutôt que de retourner des erreurs. Cette fonctionnalité est toujours la
même que dans l'encart 9-6 et l'encart 9-7 ; c'est juste une façon différente et
plus ergonomique de l'écrire.

<!--
Listing 9-9 shows a way to make this even shorter using `fs::read_to_string`.
-->

L'encart 9-9 nous montre comment encore plus raccourcir tout ceci en utilisant
`fs::read_to_string`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -- >
-->

<!--
```rust
{{#include ../listings-sources/ch09-error-handling/listing-09-09/src/main.rs:here}}
```
-->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 9-9: Using `fs::read_to_string` instead of
opening and then reading the file</span>
-->

<span class="caption">Encart 9-9 : utilisation de `fs::read_to_string` plutôt
que d'ouvrir puis lire le fichier</span>

<!--
Reading a file into a string is a fairly common operation, so the standard
library provides the convenient `fs::read_to_string` function that opens the
file, creates a new `String`, reads the contents of the file, puts the contents
into that `String`, and returns it. Of course, using `fs::read_to_string`
doesn’t give us the opportunity to explain all the error handling, so we did it
the longer way first.
-->

Récupérer le contenu d'un fichier dans une `String` est une opération assez
courante, donc la bibliothèque standard fournit la fonction assez pratique
`fs::read_to_string`, qui ouvre le fichier, crée une nouvelle `String`, lit le
contenu du fichier, insère ce contenu dans cette `String`, et la retourne.
Évidemment, l'utilisation de `fs:read_to_string` ne nous offre pas l'occasion
d'expliquer toute la gestion des erreurs, donc nous avons d'abord utilisé la
manière la plus longue.

<!--
#### Where The `?` Operator Can Be Used
-->

#### Où l'opérateur `?` peut être utilisé

<!--
The `?` operator can only be used in functions whose return type is compatible
with the value the `?` is used on. This is because the `?` operator is defined
to perform an early return of a value out of the function, in the same manner
as the `match` expression we defined in Listing 9-6. In Listing 9-6, the
`match` was using a `Result` value, and the early return arm returned an
`Err(e)` value. The return type of the function has to be a `Result` so that
it’s compatible with this `return`.
-->

L'opérateur `?` ne peut être utilisé que dans des fonctions dont le type de
retour est compatible avec ce sur quoi le `?` est utilisé. C'est parce que
l'opérateur `?` est conçu pour retourner prématurémment une valeur de la
fonction, de la même manière que le faisait l'expression `match` que nous avons
définie dans l'encart 9-6. Dans l'encart 9-6, le `match` utilisait une valeur
de type `Result`, et la branche de retour prématuré retournait une valeur de
type `Err(e)`. Le type de retour de cette fonction doit être un `Result` afin
d'être compatible avec ce `return`.

<!--
In Listing 9-10, let’s look at the error we’ll get if we use the `?` operator
in a `main` function with a return type incompatible with the type of the value
we use `?` on:
-->

Dans l'encart 9-10, découvrons l'erreur que nous allons obtenir si nous
utilisons l'opérateur `?` dans une fonction `main` qui a un type de retour
incompatible avec le type de valeur sur laquelle nous utilisons `?` :

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch09-error-handling/listing-09-10/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

<!--
<span class="caption">Listing 9-10: Attempting to use the `?` in the `main`
function that returns `()` won’t compile</span>
-->

<span class="caption">Encart 9-10 : tentative d'utilisation du `?` dans la
fonction `main` qui retourne un `()`, qui ne devrait pas pouvoir se
compiler</span>

<!--
This code opens a file, which might fail. The `?` operator follows the `Result`
value returned by `File::open`, but this `main` function has the return type of
`()`, not `Result`. When we compile this code, we get the following error
message:
-->

Ce code ouvre un fichier, ce qui devrait échouer. L'opérateur `?` est placée
derrière la valeur de type `Result` retournée par `File::open`, mais cette
fonction `main` a un type de retour `()` et non pas `Result`. Lorsque nous
compilons ce code, nous obtenons le message d'erreur suivant :

<!--
```console
{{#include ../listings-sources/ch09-error-handling/listing-09-10/output.txt}}
```
-->

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

<!--
This error points out that we’re only allowed to use the `?` operator in a
function that returns `Result`, `Option`, or another type that implements
`FromResidual`. To fix the error, you have two choices. One choice is to change
the return type of your function to be compatible with the value you’re using
the `?` operator on as long as you have no restrictions preventing that. The
other technique is to use a `match` or one of the `Result<T, E>` methods to
handle the `Result<T, E>` in whatever way is appropriate.
-->

Cette erreur explique que nous sommes autorisés à utiliser l'opérateur `?`
uniquement dans une fonction qui retourne `Result`, `Option`, ou un autre type
qui implémente `FromResidual`. Pour corriger l'erreur, vous avez deux choix. Le
premier est de changer le type de retour de votre fonction pour être compatible
avec la valeur avec lequel vous utilisez l'opérateur `?`, si vous pouvez le
faire. L'autre solution est d'utiliser un `match` ou une des méthodes de
`Result<T, E>` pour gérer le `Result<T, E>` de la manière la plus appropriée.

<!--
The error message also mentioned that `?` can be used with `Option<T>` values
as well. As with using `?` on `Result`, you can only use `?` on `Option` in a
function that returns an `Option`. The behavior of the `?` operator when called
on an `Option<T>` is similar to its behavior when called on a `Result<T, E>`:
if the value is `None`, the `None` will be returned early from the function at
that point. If the value is `Some`, the value inside the `Some` is the
resulting value of the expression and the function continues. Listing 9-11 has
an example of a function that finds the last character of the first line in the
given text:
-->

Le message d'erreur indique également que `?` peut aussi être utilisé avec des
valeurs de type `Option<T>`. Comme pour pouvoir utiliser `?` sur un `Result`,
vous devez utiliser `?` sur `Option` uniquement dans une fonction qui retourne
une `Option`. Le comportement de l'opérateur `?` sur une `Option<T>` est
identique au comportement sur un `Result<T, E>` : si la valeur est `None`, le
`None` sera retourné prématurémment à la fonction dans laquelle il est utilisé.
Si la valeur est `Some`, la valeur dans le `Some` sera la valeur résultante de
l'expression et la fonction continuera son déroulement. L'encart 9-11 est un
exemple de fonction qui trouve le dernier caractère de la première ligne dans
le texte qu'on lui fournit :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch09-error-handling/listing-09-11/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 9-11: Using the `?` operator on an `Option<T>`
value</span>
-->

<span class="caption">Encart 9-11 : utilisation de l'opérateur `?` sur une
valeur du type `Option<T>`</span>

<!--
This function returns `Option<char>` because it’s possible that there is a
character there, but it’s also possible that there isn’t. This code takes the
`text` string slice argument and calls the `lines` method on it, which returns
an iterator over the lines in the string. Because this function wants to
examine the first line, it calls `next` on the iterator to get the first value
from the iterator. If `text` is the empty string, this call to `next` will
return `None`, in which case we use `?` to stop and return `None` from
`last_char_of_first_line`. If `text` is not the empty string, `next` will
return a `Some` value containing a string slice of the first line in `text`.
-->

Cette fonction retourne un type `Option<char>` car il est possible qu'il y ait
un caractère à cet endroit, mais il est aussi possible qu'il n'y soit pas. Ce
code prends l'argument `texte` slice de chaîne de caractère et appelle sur elle
la méthode `lines`, qui retourne un itérateur des lignes dans la chaîne. Comme
cette fonction veut traiter la première ligne, elle appelle `next` sur
l'itérateur afin d'obtenir la première valeur de cet itérateur. Si `texte` est
une chaîne vide, cet appel à `next` va retourner `None`, et dans ce cas nous
utilisons `?` pour arrêter le déroulement de la fonction et retourner `None`.
Si `texte` n'est pas une chaîne vide, `next` va retourner une valeur de type
`Some` contenant une slice de chaîne de caractères de la première ligne de
`texte`.

<!--
The `?` extracts the string slice, and we can call `chars` on that string slice
to get an iterator of its characters. We’re interested in the last character in
this first line, so we call `last` to return the last item in the iterator.
This is an `Option` because it’s possible that the first line is the empty
string, for example if `text` starts with a blank line but has characters on
other lines, as in `"\nhi"`. However, if there is a last character on the first
line, it will be returned in the `Some` variant. The `?` operator in the middle
gives us a concise way to express this logic, allowing us to implement the
function in one line. If we couldn’t use the `?` operator on `Option`, we’d
have to implement this logic using more method calls or a `match` expression.
-->

Le `?` extrait la slice de la chaîne de caractères, et nous pouvons ainsi
appeller `chars` sur cette slice de chaîne de caractères afin d'obtenir un
itérateur de ses caractères. Nous nous intéressons au dernier caractère de
cette première ligne, donc nous appelons `last` pour retourner le dernier
élément dans l'itérateur. C'est une `Option` car il est possible que la
première ligne soit une chaîne de caractères vide, par exemple si `texte`
commence par une ligne vide mais a des caractères sur les autres lignes, comme
par exemple `"\nhi"`. Cependant, si il y a un caractère à la fin de la première
ligne, il sera retourné dans la variante `Some`. L'opérateur `?` au millieu
nous donne un moyen concret d'exprimer cette logique, nous permettant
d'implémenter la fonction en une ligne. Si nous n'avions pas pu utiliser
l'opérateur `?` sur `Option`, nous aurions dû implémenter cette logique en
utilisant plus d'appels à des méthodes ou des expressions `match`.

<!--
Note that you can use the `?` operator on a `Result` in a function that returns
`Result`, and you can use the `?` operator on an `Option` in a function that
returns `Option`, but you can’t mix and match. The `?` operator won’t
automatically convert a `Result` to an `Option` or vice versa; in those cases,
you can use methods like the `ok` method on `Result` or the `ok_or` method on
`Option` to do the conversion explicitly.
-->

Notez bien que vous pouvez utiliser l'opérateur `?` sur un `Result` dans une
fonction qui retourne `Result`, et vous pouvez utiliser l'opérateur `?` sur une
`Option` dans une fonction qui retourne une `Option`, mais vous ne pouvez pas
mélanger les deux. L'opérateur `?` ne va pas convertir un `Result` en `Option`
et vice-versa ; dans ce cas, vous pouvez utiliser des méthodes comme la méthode
`ok` sur `Result` ou la méthode `ok_or` sur `Option` pour faire explicitement
la conversion.

<!--
So far, all the `main` functions we’ve used return `()`. The `main` function is
special because it’s the entry and exit point of executable programs, and there
are restrictions on what its return type can be for the programs to behave as
expected.
-->

Jusqu'ici, toutes les fonctions `main` que nous avons utilisé retournent `()`.
La fonction `main` est spéciale car c'est le point d'entrée et de sortie des
programmes exécutables, et il y a quelques limitations sur ce que peut être
le type de retour pour que les programmes se comportent correctement.

<!--
Luckily, `main` can also return a `Result<(), E>`. Listing 9-12 has the
code from Listing 9-10 but we’ve changed the return type of `main` to be
`Result<(), Box<dyn Error>>` and added a return value `Ok(())` to the end. This
code will now compile:
-->

Heureusement, `main` peut aussi retourner un `Result<(), E>`. L'encart 9-12
reprend le code de l'encart 9-10 mais nous avons changé le type de retour du
`main` pour être `Result<(), Box<dyn Error>>` et nous avons ajouté la valeur de
retour `Ok(())` à la fin. Ce code devrait maintenant pouvoir se compiler :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch09-error-handling/listing-09-12/src/main.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

<!--
<span class="caption">Listing 9-12: Changing `main` to return `Result<(), E>`
allows the use of the `?` operator on `Result` values</span>
-->

<span class="caption">Encart 9-12 : changement du `main` pour qu'elle retourne
un `Result<(), E>` permettant d'utiliser l'opérateur `?` sur des valeurs de type
`Result`</span>

<!--
The `Box<dyn Error>` type is a *trait object*, which we’ll talk about in the
[“Using Trait Objects that Allow for Values of Different
Types”][trait-objects]<!-- ignore -- > section in Chapter 17. For now, you can
read `Box<dyn Error>` to mean “any kind of error.” Using `?` on a `Result`
value in a `main` function with the error type `Box<dyn Error>` is allowed,
because it allows any `Err` value to be returned early.
-->

Le type `Box<dyn Error>` est un *objet trait*, que nous verrons dans une
section du [chapitre 17][trait-objects]<!-- ignore -->. Pour l'instant, vous
pouvez interpréter `Box<dyn Error>` en “tout type d'erreur”. L'utilisation de
`?` sur une valeur type `Result` dans la fonction `main` avec le type
`Box<dyn Error>` est donc permise, car cela permet à n'importe quelle une
valeur de type `Err` d'être retournée prématurément.

<!--
When a `main` function returns a `Result<(), E>`, the executable will
exit with a value of `0` if `main` returns `Ok(())` and will exit with a
nonzero value if `main` returns an `Err` value. Executables written in C return
integers when they exit: programs that exit successfully return the integer
`0`, and programs that error return some integer other than `0`. Rust also
returns integers from executables to be compatible with this convention.
-->

Lorsqu'une fonction `main` retourne un `Result<(), E>`, l'exécutable va
terminer son exécution avec une valeur de `0` si le `main` retourne `Ok(())` et
va se terminer avec une valeur différente de zéro si `main` retourne une valeur
`Err`. Les exécutables écrits en C retournent des entiers lorsqu'ils se
terminent : les programmes qui se terminent avec succès retournent l'entier
`0`, et les programmes qui sont en erreur retournent un entier autre que `0`.
Rust retourne également des entiers avec des exécutables pour être compatible
avec cette convention.

<!--
The `main` function may return any types that implement [the
`std::process::Termination` trait][termination]<!-- ignore -- >. As of this
writing, the `Termination` trait is an unstable feature only available in
Nightly Rust, so you can’t yet implement it for your own types in Stable Rust,
but you might be able to someday!
-->

La fonction `main` peut retourner n'importe quel type qui implémente [le trait
`std::process::Termination`][termination]<!-- ignore -->. Au moment de
l'écriture de ces mots, le trait `Termination` est une fonctionnalité instable
seulement disponible avec la version expérimentale de Rust, donc vous ne pouvez
pas l'implémenter sur vos propres types avec la version stable de Rust, mais
vous pourrez peut-être le faire un jour !

<!--
Now that we’ve discussed the details of calling `panic!` or returning `Result`,
let’s return to the topic of how to decide which is appropriate to use in which
cases.
-->

Maintenant que nous avons vu les détails pour utiliser `panic!` ou retourner
`Result`, voyons maintenant comment choisir ce qu'il faut faire en fonction des
cas.

<!--
[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type
[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[termination]: ../std/process/trait.Termination.html
-->

[handle_failure]: ch02-00-guessing-game-tutorial.html#gérer-les-erreurs-potentielles-avec-le-type-result
[trait-objects]: ch17-02-trait-objects.html
[termination]: https://doc.rust-lang.org/std/process/trait.Termination.html
