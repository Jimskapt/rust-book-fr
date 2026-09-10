<!--
## To `panic!` or Not to `panic!`
-->

## Paniquer ou ne pas paniquer, telle est la question

<!--
So how do you decide when you should call `panic!` and when you should return
`Result`? When code panics, there’s no way to recover. You could call `panic!`
for any error situation, whether there’s a possible way to recover or not, but
then you’re making the decision that a situation is unrecoverable on behalf of
the calling code. When you choose to return a `Result` value, you give the
calling code options. The calling code could choose to attempt to recover in a
way that’s appropriate for its situation, or it could decide that an `Err`
value in this case is unrecoverable, so it can call `panic!` and turn your
recoverable error into an unrecoverable one. Therefore, returning `Result` is a
good default choice when you’re defining a function that might fail.
-->

Comment décider si vous devez utiliser `panic!` ou si vous devez retourner un
`Result` ? Quand un code panique, il n'y a pas de moyen de récupérer la
situation. Vous pourriez utiliser `panic!` pour n'importe quelle situation
d'erreur, peu importe s'il est possible de récupérer la situation ou non, mais
vous prenez alors la décision de tout arrêter à la place du code appelant.
Lorsque vous choisissez de retourner une valeur `Result`, vous donnez le choix
au code appelant. Le code appelant peut choisir d'essayer de récupérer l'erreur
de manière appropriée à la situation, ou il peut décider que dans ce cas une
valeur `Err` est irrécupérable, et va donc utiliser `panic!` et transformer
votre erreur récupérable en erreur irrécupérable. Ainsi, retourner `Result` est
un bon choix par défaut lorsque vous définissez une fonction qui peut échouer.

<!--
In situations such as examples, prototype code, and tests, it’s more
appropriate to write code that panics instead of returning a `Result`. Let’s
explore why, then discuss situations in which the compiler can’t tell that
failure is impossible, but you as a human can. The chapter will conclude with
some general guidelines on how to decide whether to panic in library code.
-->

Dans certains cas comme les exemples, les prototypes, et les tests, il est plus
approprié d'écrire du code qui panique plutôt que de retourner un `Result`.
Nous allons voir pourquoi, puis nous verrons des situations dans lesquelles
vous savez en tant qu'humain qu'un code ne peut pas échouer, mais que le
compilateur ne peut pas le déduire par lui-même. Enfin, nous allons conclure le
chapitre par quelques lignes directrices générales pour décider s'il faut
paniquer dans le code d'une bibliothèque.

<!--
### Examples, Prototype Code, and Tests
-->

### Les exemples, les prototypes et les tests

<!--
When you’re writing an example to illustrate some concept, also including robust
error-handling code can make the example less clear. In
examples, it’s understood that a call to a method like `unwrap` that could
panic is meant as a placeholder for the way you’d want your application to
handle errors, which can differ based on what the rest of your code is doing.
-->

Lorsque vous écrivez un exemple pour illustrer un concept, y rajouter un code
de gestion des erreurs très résilient peut nuire à la clarté de l'exemple. Dans
les exemples, il est courant d'utiliser une méthode comme `unwrap` (qui peut
faire un panic) pour remplacer le code de gestion de l'erreur que vous
utiliseriez en temps normal dans votre application, et qui peut changer en
fonction de ce que le reste de votre code va faire.

<!--
Similarly, the `unwrap` and `expect` methods are very handy when prototyping,
before you’re ready to decide how to handle errors. They leave clear markers in
your code for when you’re ready to make your program more robust.
-->

De la même manière, les méthodes `unwrap` et `expect` sont très pratiques pour
coder des prototypes, avant même de décider comment gérer les erreurs. Ce sont
des indicateurs clairs dans votre code pour plus tard quand vous serez prêt à
rendre votre code plus résilient aux échecs.

<!--
If a method call fails in a test, you’d want the whole test to fail, even if
that method isn’t the functionality under test. Because `panic!` is how a test
is marked as a failure, calling `unwrap` or `expect` is exactly what should
happen.
-->

Si l'appel à une méthode échoue dans un test, nous voulons que tout le test
échoue, même si cette méthode n'est pas la fonctionnalité que nous testons.
Puisque c'est `panic!` qui indique qu'un test a échoué, utiliser `unwrap` ou
`expect` est exactement ce qu'il faut faire.

<!--
### Cases in Which You Have More Information Than the Compiler
-->

### Les cas où vous avez plus d'informations que le compilateur

<!--
It would also be appropriate to call `unwrap` when you have some other logic
that ensures the `Result` will have an `Ok` value, but the logic isn’t
something the compiler understands. You’ll still have a `Result` value that you
need to handle: whatever operation you’re calling still has the possibility of
failing in general, even though it’s logically impossible in your particular
situation. If you can ensure by manually inspecting the code that you’ll never
have an `Err` variant, it’s perfectly acceptable to call `unwrap`. Here’s an
example:
-->

Vous pouvez utiliser `unwrap` lorsque vous avez une certaine logique qui
garantit que le `Result` sera toujours une valeur `Ok`, mais que ce n'est pas le
genre de logique que le compilateur arrive à comprendre. Vous aurez quand même
une valeur `Result` à gérer : l'opération que vous utilisez peut échouer de
manière générale, même si dans votre cas c'est logiquement impossible. Si en
inspectant manuellement le code vous vous rendez compte que vous n'aurez jamais
une variante `Err`, vous pouvez tout à fait utiliser `unwrap`. Voici un
exemple :

<!--
```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

<!--
We’re creating an `IpAddr` instance by parsing a hardcoded string. We can see
that `127.0.0.1` is a valid IP address, so it’s acceptable to use `unwrap`
here. However, having a hardcoded, valid string doesn’t change the return type
of the `parse` method: we still get a `Result` value, and the compiler will
still make us handle the `Result` as if the `Err` variant is a possibility
because the compiler isn’t smart enough to see that this string is always a
valid IP address. If the IP address string came from a user rather than being
hardcoded into the program and therefore *did* have a possibility of failure,
we’d definitely want to handle the `Result` in a more robust way instead.
-->

Nous créons une instance de `IpAddr` en interprétant une chaîne de caractères
codée en dur dans le code. Nous savons que `127.0.0.1` est une adresse IP
valide, donc il est acceptable d'utiliser `unwrap` ici. Toutefois, avoir une
chaîne de caractères valide et codée en dur ne change pas le type de retour de
la méthode `parse` : nous obtenons toujours une valeur de type `Result` et le
compilateur va nous demander de gérer le `Result` comme si on pouvait obtenir la
variante `Err`, car le compilateur n'est pas suffisamment intelligent pour
comprendre que cette chaîne de caractères est toujours une adresse IP valide. Si
le texte de l'adresse IP provient de l'utilisateur au lieu d'être codé en dur
dans le programme et donc qu'il y a désormais une possibilité d'erreur, alors
nous devrions vouloir gérer le `Result` d'une manière plus résiliente.

<!--
### Guidelines for Error Handling
-->

### Recommandations pour gérer les erreurs

<!--
It’s advisable to have your code panic when it’s possible that your code
could end up in a bad state. In this context, a *bad state* is when some
assumption, guarantee, contract, or invariant has been broken, such as when
invalid values, contradictory values, or missing values are passed to your
code—plus one or more of the following:
-->

Il est recommandé de faire paniquer votre code dès qu'il risque d'aboutir à un
état invalide. Dans ce contexte, un *état invalide* est lorsqu'un postulat, une
garantie, un contrat ou un invariant a été rompu, comme des valeurs invalides,
contradictoires ou manquantes qui sont fournies à votre code, ainsi qu'un ou
plusieurs des éléments suivants :

<!--
* The bad state is something that is unexpected, as opposed to something that
  will likely happen occasionally, like a user entering data in the wrong
  format.
* Your code after this point needs to rely on not being in this bad state,
  rather than checking for the problem at every step.
* There’s not a good way to encode this information in the types you use. We’ll
  work through an example of what we mean in the [“Encoding States and Behavior
  as Types”][encoding]<!-- ignore -- > section of Chapter 17.
-->

* L'état invalide est quelque chose qui est inattendu, contrairement à quelque
  chose qui devrait arriver occasionnellement, comme par exemple un utilisateur
  qui saisit une donnée dans un mauvais format.
* Après cette instruction, votre code a besoin de ne pas être dans cet état
  invalide, plutôt que d'avoir à vérifier le problème à chaque étape.
* Il n'y a pas de bonne façon d'encoder cette information dans les types que
  vous utilisez. Nous allons pratiquer ceci via un exemple dans [une section du
  chapitre 17][encoding]<!-- ignore -->.

<!--
If someone calls your code and passes in values that don’t make sense, the best
choice might be to call `panic!` and alert the person using your library to the
bug in their code so they can fix it during development. Similarly, `panic!` is
often appropriate if you’re calling external code that is out of your control
and it returns an invalid state that you have no way of fixing.
-->

Si une personne utilise votre bibliothèque et lui fournit des valeurs qui n'ont
pas de sens, la meilleure des choses à faire est d'utiliser `panic!` et
d'avertir cette personne du bogue dans son code afin qu'elle le règle pendant la
phase de développement. De la même manière, `panic!` est parfois approprié si
vous appelez du code externe sur lequel vous n'avez pas la main, et qu'il
retourne un état invalide que vous ne pouvez pas corriger.

<!--
However, when failure is expected, it’s more appropriate to return a `Result`
than to make a `panic!` call. Examples include a parser being given malformed
data or an HTTP request returning a status that indicates you have hit a rate
limit. In these cases, returning a `Result` indicates that failure is an
expected possibility that the calling code must decide how to handle.
-->

Cependant, si l'on s'attend à rencontrer des échecs, il est plus approprié de
retourner un `Result` plutôt que de faire appel à `panic!`. Il peut s'agir par
exemple d'un interpréteur qui reçoit des données erronées, ou une requête HTTP
qui retourne un statut qui indique que vous avez atteint une limite de débit.
Dans ces cas-là, vous devriez indiquer qu'il est possible que cela puisse
échouer en retournant un `Result` afin que le code appelant puisse décider quoi
faire pour gérer le problème.

<!--
When your code performs operations on values, your code should verify the
values are valid first and panic if the values aren’t valid. This is mostly for
safety reasons: attempting to operate on invalid data can expose your code to
vulnerabilities. This is the main reason the standard library will call
`panic!` if you attempt an out-of-bounds memory access: trying to access memory
that doesn’t belong to the current data structure is a common security problem.
Functions often have *contracts*: their behavior is only guaranteed if the
inputs meet particular requirements. Panicking when the contract is violated
makes sense because a contract violation always indicates a caller-side bug and
it’s not a kind of error you want the calling code to have to explicitly
handle. In fact, there’s no reasonable way for calling code to recover; the
calling *programmers* need to fix the code. Contracts for a function,
especially when a violation will cause a panic, should be explained in the API
documentation for the function.
-->

Lorsque votre code effectue des opérations sur des valeurs, votre code devrait
d'abord vérifier que ces valeurs sont valides, et faire un panic si les valeurs
ne sont pas correctes. C'est essentiellement pour des raisons de sécurité :
tenter de travailler avec des données invalides peut exposer votre code à des
vulnérabilités. C'est la principale raison pour laquelle la bibliothèque
standard va appeler `panic!` si vous essayez d'accéder à la mémoire hors
limite : essayer d'accéder à de la mémoire qui n'appartient pas à la structure
de données actuelle est un problème de sécurité fréquent. Les fonctions ont
souvent des *contrats* : leur comportement est garanti uniquement si les données
d'entrée remplissent des conditions particulières. Paniquer lorsque le contrat
est violé est justifié, car une violation de contrat signifie toujours un bogue
du côté de l'appelant, et ce n'est pas le genre d'erreur que vous voulez que le
code appelant gère explicitement. En fait, il n'y a aucun moyen rationnel pour
que le code appelant se corrige : le *développeur* du code appelant doit
corriger le code. Les contrats d'une fonction, en particulier lorsqu'une
violation va causer un panic, doivent être expliqués dans la documentation de
l'API de ladite fonction.

<!--
However, having lots of error checks in all of your functions would be verbose
and annoying. Fortunately, you can use Rust’s type system (and thus the type
checking done by the compiler) to do many of the checks for you. If your
function has a particular type as a parameter, you can proceed with your code’s
logic knowing that the compiler has already ensured you have a valid value. For
example, if you have a type rather than an `Option`, your program expects to
have *something* rather than *nothing*. Your code then doesn’t have to handle
two cases for the `Some` and `None` variants: it will only have one case for
definitely having a value. Code trying to pass nothing to your function won’t
even compile, so your function doesn’t have to check for that case at runtime.
Another example is using an unsigned integer type such as `u32`, which ensures
the parameter is never negative.
-->

Cependant, avoir beaucoup de vérifications d'erreurs dans toutes vos fonctions
serait verbeux et pénible. Heureusement, vous pouvez utiliser le système de
types de Rust (et donc la vérification de type que fait le compilateur) pour
assurer une partie des vérifications à votre place. Si votre fonction a un
paramètre d'un type précis, vous pouvez continuer à écrire votre code en
sachant que le compilateur s'est déjà assuré que vous avez une valeur valide.
Par exemple, si vous obtenez un type de valeur plutôt qu'une `Option`, votre
programme s'attend à obtenir *quelque chose* plutôt que *rien*. Votre code n'a
donc pas à gérer les deux cas de variantes `Some` et `None` : la seule
possibilité est qu'il y a une valeur. Du code qui essaye de ne rien fournir à
votre fonction ne compilera même pas, donc votre fonction n'a pas besoin de
vérifier ce cas-là lors de l'exécution. Un autre exemple est d'utiliser un type
d'entier non signé comme `u32`, qui garantit que le paramètre n'est jamais
strictement négatif.

<!--
### Creating Custom Types for Validation
-->

### Créer des types personnalisés pour la vérification

<!--
Let’s take the idea of using Rust’s type system to ensure we have a valid value
one step further and look at creating a custom type for validation. Recall the
guessing game in Chapter 2 in which our code asked the user to guess a number
between 1 and 100. We never validated that the user’s guess was between those
numbers before checking it against our secret number; we only validated that
the guess was positive. In this case, the consequences were not very dire: our
output of “Too high” or “Too low” would still be correct. But it would be a
useful enhancement to guide the user toward valid guesses and have different
behavior when a user guesses a number that’s out of range versus when a user
types, for example, letters instead.
-->

Allons plus loin dans l'idée d'utiliser le système de types de Rust pour
s'assurer d'avoir une valeur valide en créant un type personnalisé pour la
vérification. Souvenez-vous du jeu du plus ou du moins du chapitre 2 dans lequel
notre code demandait à l'utilisateur de deviner un nombre entre 1 et 100. Nous
n'avons jamais validé que le nombre saisi par l'utilisateur était entre ces
nombres avant de le comparer à notre nombre secret ; nous avons seulement
vérifié que le nombre était positif. Dans ce cas, les conséquences ne sont pas
très graves : notre résultat “C'est plus !” ou “C'est moins !” sera toujours
correct. Mais ce serait une amélioration utile pour aider l'utilisateur à faire
des suppositions valides et pour avoir un comportement différent selon qu'un
utilisateur propose un nombre en dehors des limites ou qu'il saisit, par
exemple, des lettres à la place.

<!--
One way to do this would be to parse the guess as an `i32` instead of only a
`u32` to allow potentially negative numbers, and then add a check for the
number being in range, like so:
-->

Une façon de faire cela serait de stocker le nombre saisi dans un `i32` plutôt
que dans un `u32` afin de permettre d'obtenir potentiellement des nombres
négatifs, et ensuite vérifier que le nombre est dans la plage autorisée, comme
ceci :

<!--
```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

<!--
The `if` expression checks whether our value is out of range, tells the user
about the problem, and calls `continue` to start the next iteration of the loop
and ask for another guess. After the `if` expression, we can proceed with the
comparisons between `guess` and the secret number knowing that `guess` is
between 1 and 100.
-->

L'expression `if` vérifie si la valeur est en dehors des limites et informe
l'utilisateur du problème le cas échéant, puis utilise `continue` pour passer à
la prochaine itération de la boucle et ainsi demander de saisir une nouvelle
supposition. Après l'expression `if`, nous pouvons continuer avec la comparaison
entre `supposition` et le nombre secret tout en sachant que `supposition` est
entre 1 et 100.

<!--
However, this is not an ideal solution: if it was absolutely critical that the
program only operated on values between 1 and 100, and it had many functions
with this requirement, having a check like this in every function would be
tedious (and might impact performance).
-->

Cependant, ce n'est pas une solution idéale : si c'était absolument critique
que le programme ne travaille qu'avec des valeurs entre 1 et 100 et qu'il ait
de nombreuses fonctions qui reposent sur cette condition, cela pourrait être
fastidieux (et cela impacterait potentiellement la performance) de faire une
vérification comme celle-ci dans chacune de ces fonctions.

<!--
Instead, we can make a new type and put the validations in a function to create
an instance of the type rather than repeating the validations everywhere. That
way, it’s safe for functions to use the new type in their signatures and
confidently use the values they receive. Listing 9-13 shows one way to define a
`Guess` type that will only create an instance of `Guess` if the `new` function
receives a value between 1 and 100.
-->

À la place, nous pourrions construire un nouveau type et intégrer les
vérifications dans la fonction de création d'une instance de ce type plutôt que
de répéter partout les vérifications. Il est ainsi plus sûr pour les fonctions
d'utiliser ce nouveau type dans leurs signatures et d'utiliser avec confiance
les valeurs qu'elles reçoivent. L'encart 9-13 montre une façon de définir un
type `Supposition` qui ne créera une instance de `Supposition` que si la
fonction `new` reçoit une valeur entre 1 et 100 :

<!--
<!-- Deliberately not using rustdoc_include here; the `main` function in the
file requires the `rand` crate. We do want to include it for reader
experimentation purposes, but don't want to include it for rustdoc testing
purposes. -- >
-->

<!--
```rust
{{#include ../listings/ch09-error-handling/listing-09-13/src/main.rs:here}}
```
-->

```rust
{{#include ../listings/ch09-error-handling/listing-09-13/src/main.rs:here}}
```

<!--
<span class="caption">Listing 9-13: A `Guess` type that will only continue with
values between 1 and 100</span>
-->

<span class="caption">Encart 9-13 : un type `Supposition` qui ne va continuer
que si la valeur est entre 1 et 100</span>

<!--
First, we define a struct named `Guess` that has a field named `value` that
holds an `i32`. This is where the number will be stored.
-->

D'abord, nous définissons une structure qui s'appelle `Supposition` qui a un
champ `valeur` qui stocke un `i32`. C'est dans ce dernier que le nombre sera
stocké.

<!--
Then we implement an associated function named `new` on `Guess` that creates
instances of `Guess` values. The `new` function is defined to have one
parameter named `value` of type `i32` and to return a `Guess`. The code in the
body of the `new` function tests `value` to make sure it’s between 1 and 100.
If `value` doesn’t pass this test, we make a `panic!` call, which will alert
the programmer who is writing the calling code that they have a bug they need
to fix, because creating a `Guess` with a `value` outside this range would
violate the contract that `Guess::new` is relying on. The conditions in which
`Guess::new` might panic should be discussed in its public-facing API
documentation; we’ll cover documentation conventions indicating the possibility
of a `panic!` in the API documentation that you create in Chapter 14. If
`value` does pass the test, we create a new `Guess` with its `value` field set
to the `value` parameter and return the `Guess`.
-->

Ensuite, nous implémentons une fonction associée `new` sur `Supposition` qui
crée des instances de `Supposition`. La fonction `new` est conçue pour recevoir
un paramètre `valeur` de type `i32` et retourner une `Supposition`. Le code dans
le corps de la fonction `new` teste `valeur` pour s'assurer qu'elle est bien
entre 1 et 100. Si `valeur` échoue à ce test, nous faisons appel à `panic!`, qui
alertera le développeur qui écrit le code appelant qu'il a un bogue qu'il doit
régler, car créer une `Supposition` avec `valeur` en dehors de cette plage va
violer le contrat sur lequel s'appuie `Supposition::new`. Les conditions dans
lesquelles `Supposition::new` va paniquer devraient être expliquées dans la
documentation publique de l'API ; nous verrons les conventions pour indiquer
l'éventualité d'un `panic!` dans la documentation de l'API que vous créerez
au chapitre 14. Si `valeur` passe le test, nous créons une nouvelle
`Supposition` avec son champ `valeur` qui prend la valeur du paramètre `valeur`
et retourne cette `Supposition`.

<!--
Next, we implement a method named `value` that borrows `self`, doesn’t have any
other parameters, and returns an `i32`. This kind of method is sometimes called
a *getter*, because its purpose is to get some data from its fields and return
it. This public method is necessary because the `value` field of the `Guess`
struct is private. It’s important that the `value` field be private so code
using the `Guess` struct is not allowed to set `value` directly: code outside
the module *must* use the `Guess::new` function to create an instance of
`Guess`, thereby ensuring there’s no way for a `Guess` to have a `value` that
hasn’t been checked by the conditions in the `Guess::new` function.
-->

Enfin, nous implémentons une méthode `valeur` qui emprunte `self`, n'a aucun
autre paramètre, et retourne un `i32`. Ce genre de méthode est parfois appelé un
*accesseur*, car son rôle est d'accéder aux données des champs et de les
retourner. Cette méthode publique est nécessaire car le champ `valeur` de la
structure `Supposition` est privé. Il est important que le champ `valeur` soit
privé pour que le code qui utilise la structure `Supposition` ne puisse pas
directement assigner une valeur à `valeur` : le code en dehors du module *doit*
utiliser la fonction `Supposition::new` pour créer une instance de
`Supposition`, ce qui permet d'empêcher la création d'une `Supposition` avec un
champ `valeur` qui n'a pas été vérifié par les conditions dans la fonction
`Supposition:new`.

<!--
A function that has a parameter or returns only numbers between 1 and 100 could
then declare in its signature that it takes or returns a `Guess` rather than an
`i32` and wouldn’t need to do any additional checks in its body.
-->

Une fonction qui prend en paramètre ou qui retourne des nombres uniquement entre
1 et 100 peut ensuite déclarer dans sa signature qu'elle prend en paramètre ou
qu'elle retourne une `Supposition` plutôt qu'un `i32` et n'aura pas besoin de
faire de vérifications supplémentaires dans son corps.

<!--
## Summary
-->

## Résumé

<!--
Rust’s error handling features are designed to help you write more robust code.
The `panic!` macro signals that your program is in a state it can’t handle and
lets you tell the process to stop instead of trying to proceed with invalid or
incorrect values. The `Result` enum uses Rust’s type system to indicate that
operations might fail in a way that your code could recover from. You can use
`Result` to tell code that calls your code that it needs to handle potential
success or failure as well. Using `panic!` and `Result` in the appropriate
situations will make your code more reliable in the face of inevitable problems.
-->

Les fonctionnalités de gestion d'erreurs de Rust sont conçues pour vous aider à
écrire du code plus résilient. La macro `panic!` signale que votre programme
est dans un état qu'il ne peut pas gérer et vous permet de dire au processus de
s'arrêter au lieu d'essayer de continuer avec des valeurs invalides ou
incorrectes. L'énumération `Result` utilise le système de types de Rust pour
signaler que des opérations peuvent échouer de telle façon que votre code puisse
rattraper l'erreur. Vous pouvez utiliser `Result` pour dire au code qui appelle
votre code qu'il a besoin de gérer le résultat et aussi les potentielles
erreurs. Utiliser `panic!` et `Result` de manière appropriée rendra votre code
plus fiable face à des problèmes inévitables.

<!--
Now that you’ve seen useful ways that the standard library uses generics with
the `Option` and `Result` enums, we’ll talk about how generics work and how you
can use them in your code.
-->

Maintenant que vous avez vu la façon dont la bibliothèque standard tire parti de
la généricité avec les énumérations `Option` et `Result`, nous allons voir
comment la généricité fonctionne et comment vous pouvez l'utiliser dans votre code.

<!--
[encoding]: ch17-03-oo-design-patterns.html#encoding-states-and-behavior-as-types
-->

[encoding]: ch17-03-oo-design-patterns.html
