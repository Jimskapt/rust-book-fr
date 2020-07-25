<!--
## Working with Environment Variables
-->

## Travailler avec des variables d'environnement

<!--
We’ll improve `minigrep` by adding an extra feature: an option for
case-insensitive searching that the user can turn on via an environment
variable. We could make this feature a command line option and require that
users enter it each time they want it to apply, but instead we’ll use an
environment variable. Doing so allows our users to set the environment variable
once and have all their searches be case insensitive in that terminal session.
-->

Nous allons améliorer `minigrep` en lui ajoutant une fonctionnalité
supplémentaire : une option pour rechercher sans être sensible à la casse que
l'utilisateur pourra activer via une variable d'environnement. Nous pourrions
appliquer cette fonctionnalité avec une option en ligne de commande et demander
à l'utilisateur de la renseigner à chaque fois qu'il veut l'activer, mais à la
place nous allons utiliser une variable d'environnement. Ceci permet à nos
utilisateurs de régler la variable d'environnement une seule fois et d'avoir
leurs recherches insensibles à la casse dans cette session du terminal.

<!--
### Writing a Failing Test for the Case-Insensitive `search` Function
-->

### Ecrire un test qui échoue pour la fonction `rechercher` insensible à la casse

<!--
We want to add a new `search_case_insensitive` function that we’ll call when
the environment variable is on. We’ll continue to follow the TDD process, so
the first step is again to write a failing test. We’ll add a new test for the
new `search_case_insensitive` function and rename our old test from
`one_result` to `case_sensitive` to clarify the differences between the two
tests, as shown in Listing 12-20.
-->

Nous souhaitons ajouter une nouvelle fonction `rechercher_insensible_casse` que
nous allons appeler lorsque la variable d'environnement est active. Nous allons
continuer à suivre le processus de TDD, donc la première étape et d'écrire à
nouveau un test qui échoue. Nous allons ajouter un nouveau test pour la nouvelle
fonction `rechercher_insensible_casse` et renommer notre ancien test
`one_result` en `sensible_casse` pour clarifier les différences entre les deux
tests, comme dans l'encart 12-20.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-20: Adding a new failing test for the
case-insensitive function we’re about to add</span>
-->

<span class="caption">Encart 12-20 : Ajout d'un nouveau test qui échoue pour la
fonction insensible à la casse que nous sommes en train d'ajouter</span>

<!--
Note that we’ve edited the old test’s `contents` too. We’ve added a new line
with the text `"Duct tape."` using a capital D that shouldn’t match the query
`"duct"` when we’re searching in a case-sensitive manner. Changing the old test
in this way helps ensure that we don’t accidentally break the case-sensitive
search functionality that we’ve already implemented. This test should pass now
and should continue to pass as we work on the case-insensitive search.
-->

Remarquez aussi que nous avons aussi modifié le `contenu` des anciens tests.
Nous avons ajouté une nouvelle ligne avec le texte `"Duct tape."` en utilisant
un D majuscule qui ne devrait pas correspondre à la recherche `"duct"` lorsque
nous recherchons de manière à être sensible à la casse. Ce changement de
l'ancien test permet de nous assurer que nous ne casserons pas accidentellement
la fonction de recherche sensible à la casse que nous avons déjà implémenté. Ce
test devrait toujours continuer à réussir au fur et à mesure que nous progressons
sur la recherche insensible à la casse.

<!--
The new test for the case-*insensitive* search uses `"rUsT"` as its query. In
the `search_case_insensitive` function we’re about to add, the query `"rUsT"`
should match the line containing `"Rust:"` with a capital R and match the line
`"Trust me."` even though both have different casing from the query. This is
our failing test, and it will fail to compile because we haven’t yet defined
the `search_case_insensitive` function. Feel free to add a skeleton
implementation that always returns an empty vector, similar to the way we did
for the `search` function in Listing 12-16 to see the test compile and fail.
-->

Le nouveau test pour la recherche insensible à la casse utilise `"rUsT"` comme
recherche. Dans la fonction `rechercher_insensible_casse` que nous sommes en
train d'ajouter, la recherche `"rUsT"` devrait correspondre à la ligne qui
contient `"Rust:"` avec un R majuscule ainsi que la ligne `C'est pas rustique.`
même si ces deux cas ont des casses différentes de la recherche. C'est notre
test qui doit échouer, et il ne devrait pas se compiler car nous n'avons pas
encore défini la fonction `rechercher_insensible_casse`. Ajoutez son
implémentation qui retourne toujours un vecteur vide, de la même manière que
nous l'avions fait pour la fonction `rechercher` dans l'encart 12-16 pour voir
si les tests se compilent et échouent.

<!--
### Implementing the `search_case_insensitive` Function
-->

### Implémenter la fonction `rechercher_insensible_casse`

<!--
The `search_case_insensitive` function, shown in Listing 12-21, will be almost
the same as the `search` function. The only difference is that we’ll lowercase
the `query` and each `line` so whatever the case of the input arguments,
they’ll be the same case when we check whether the line contains the query.
-->

La fonction `rechercher_insensible_casse`, présente dans l'encart 12-21, sera
presque la même que la fonction `rechercher`. La seule différence est que nous
allons transformer en minuscule le contenu de `recherche` et de chaque `ligne`
pour que quel que soit la casse des arguments d'entrée, nous aurons toujours la
même casse lorsque nous vérifierons si la ligne contient la recherche.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-21: Defining the `search_case_insensitive`
function to lowercase the query and the line before comparing them</span>
-->

<span class="caption">Encart 12-21 : Définition de la fonction
`rechercher_insensible_casse` pour obtenir en minuscule la recherche et la
ligne avant de les comparer</span>

<!--
First, we lowercase the `query` string and store it in a shadowed variable with
the same name. Calling `to_lowercase` on the query is necessary so no matter
whether the user’s query is `"rust"`, `"RUST"`, `"Rust"`, or `"rUsT"`, we’ll
treat the query as if it were `"rust"` and be insensitive to the case. While
`to_lowercase` will handle basic Unicode, it won't be 100% accurate. If we were
writing a real application, we'd want to do a bit more work here, but this section
is about environment variables, not Unicode, so we'll leave it at that here.
-->

D'abord, nous obtenons la chaîne de caractères `recherche` en minuscule et nous
l'enregistrons dans une variable masquée avec le même nom. L'appel à
`to_lowercase` sur la recherche est nécessaire afin que quel que soit la
recherche de l'utilisateur, comme `"rust"`, `"RUST"`, `"Rust"`, ou `"rUsT"`,
nous traitons la recherche comme si elle était `"rust"` et par conséquent elle
est insensible à la casse. La méthode `to_lowercase` devrait gérer de l'Unicode
de base, mais ne sera pas fiable à 100%. Si nous avions écris une application
sérieuse, nous aurions dû faire plus de choses à ce propos, toutefois la section
actuelle traite des variables d'environnement et non pas de la gestion de
l'Unicode, donc nous allons conserver ce code simplifié.

<!--
Note that `query` is now a `String` rather than a string slice, because calling
`to_lowercase` creates new data rather than referencing existing data. Say the
query is `"rUsT"`, as an example: that string slice doesn’t contain a lowercase
`u` or `t` for us to use, so we have to allocate a new `String` containing
`"rust"`. When we pass `query` as an argument to the `contains` method now, we
need to add an ampersand because the signature of `contains` is defined to take
a string slice.
-->

Notez que `recherche` est désormais une `String` et non plus une slice de chaîne
de caractères, car l'appel à `to_lowercase` crée des nouvelles données au lieu
de modifier les données déjà existantes. Par exemple, disons que la recherche
est `"rUsT"` : cette slice de chaîne de caractères ne contient pas de `u` ou de
`t` minuscule que nous pourrions utiliser, donc nous devons allouer une nouvelle
`String` qui contient `"rust"`. Maintenant, lorsque nous passons `recherche` en
argument de la méthode `contains`, nous devons rajouter une esperluette car la
signature de `contains` est définie pour prendre une slice de chaîne de
caractères.

<!--
Next, we add a call to `to_lowercase` on each `line` before we check whether it
contains `query` to lowercase all characters. Now that we’ve converted `line`
and `query` to lowercase, we’ll find matches no matter what the case of the
query is.
-->

Ensuite, nous ajoutons un appel à `to_lowercase` sur chaque `ligne` avant de
vérifier si elle contient `recherche` afin d'obtenir tous ses caractères en
minuscule. Maintenant que nous avons `ligne` et `recherche` en minuscule, nous
allons rechercher les correspondances peu importe la casse de la recherche.

<!--
Let’s see if this implementation passes the tests:
-->

Voyons si cette implémentation passe les tests :

<!--
```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

<!--
Great! They passed. Now, let’s call the new `search_case_insensitive` function
from the `run` function. First, we’ll add a configuration option to the
`Config` struct to switch between case-sensitive and case-insensitive search.
Adding this field will cause compiler errors because we aren’t initializing
this field anywhere yet:
-->

Très bien ! Elles ont réussi. Maintenant, utilisons la nouvelle fonction
`rechercher_insensible_casse` dans la fonction `run`. Pour commencer, nous
allons ajouter une option de configuration à la structure `Config` pour changer
entre la recherche sensible et non-sensible à la casse. L'ajout de ce champ va
causer des erreurs de compilation car nous n'avons jamais initialisé ce champ
pour le moment :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:here}}
```

<!--
Note that we added the `case_sensitive` field that holds a Boolean. Next, we
need the `run` function to check the `case_sensitive` field’s value and use
that to decide whether to call the `search` function or the
`search_case_insensitive` function, as shown in Listing 12-22. Note this still
won’t compile yet.
-->

Remarquez que le champ `sensible_casse` que nous avons ajouté est un Booléen.
Ensuite, nous devons faire en sorte que la fonction `run` vérifie la valeur du
champ `sensible_casse` et l'utilise pour décider si elle doit appeler la
fonction `rechercher` ou la fonction `rechercher_insensible_casse`, comme dans
l'encart 12-22. Notez que cela ne se compile pas encore.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:there}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:there}}
```

<!--
<span class="caption">Listing 12-22: Calling either `search` or
`search_case_insensitive` based on the value in `config.case_sensitive`</span>
-->

<span class="caption">Encart 12-22 : Appeler `rechercher` ou
`rechercher_insensible_casse` en fonction de la valeur dans `config.sensible_casse`
</span>

<!--
Finally, we need to check for the environment variable. The functions for
working with environment variables are in the `env` module in the standard
library, so we want to bring that module into scope with a `use std::env;` line
at the top of *src/lib.rs*. Then we’ll use the `var` function from the `env`
module to check for an environment variable named `CASE_INSENSITIVE`, as shown
in Listing 12-23.
-->

Enfin, nous devons vérifier la variable d'environnement. Les fonctions pour
travailler avec les variables d'environnement sont dans le module `env` de la
bibliothèque standard, donc nous allons importer ce module dans la portée avec
une ligne `use std::env;` en haut de *src/lib.rs*. Ensuite, nous allons utiliser
la fonction `var` du module `env` pour vérifier la présence d'une variable
d'environnement `MINIGREP_INSENSIBLE_CASSE`, comme dans l'encart 12-23.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-23: Checking for an environment variable named
`CASE_INSENSITIVE`</span>
-->

<span class="caption">Encart 12-23 : Vérification de la présence de la variable
d'environnement `MINIGREP_INSENSIBLE_CASSE`</span>

<!--
Here, we create a new variable `case_sensitive`. To set its value, we call the
`env::var` function and pass it the name of the `CASE_INSENSITIVE` environment
variable. The `env::var` function returns a `Result` that will be the successful
`Ok` variant that contains the value of the environment variable if the
environment variable is set. It will return the `Err` variant if the
environment variable is not set.
-->

Ici, nous créons une nouvelle variable `sensible_casse`. Pour lui donner une
valeur, nous appelons la fonction `env::var` et nous lui passons le nom de la
variable d'environnement `MINIGREP_INSENSIBLE_CASSE`. La fonction `env::var`
retourne un `Result` qui sera en cas de succès la variante `Ok` qui contiendra
la valeur de la variable d'environnement si cette variable d'environnement est
définie. Elle retournera la variante `Err` si cette variable d'environnement
n'est pas définie.

<!--
We’re using the `is_err` method on the `Result` to check whether it’s an error
and therefore unset, which means it *should* do a case-sensitive search. If the
`CASE_INSENSITIVE` environment variable is set to anything, `is_err` will
return false and the program will perform a case-insensitive search. We don’t
care about the *value* of the environment variable, just whether it’s set or
unset, so we’re checking `is_err` rather than using `unwrap`, `expect`, or any
of the other methods we’ve seen on `Result`.
-->

Nous utilisons la méthode `is_err` sur le `Result` pour vérifier si c'est une
erreur et donc par conséquent non définie, ce qui veut dire que nous *devons*
effectuer une recherche sensible à la casse. Si la variable d'environnement
`MINIGREP_INSENSIBLE_CASSE` a une valeur qui lui a été assignée, `is_err` va
retourner `false` et le programme va procéder à une recherche non sensible à
la casse. Nous ne préoccupons pas de la *valeur* de la variable d'environnement,
mais uniquement de savoir si elle est définie ou non, donc nous utilisons
`is_err` plutôt que `unwrap`, `expect`, ou toute autre méthode que nous avons
vu pour `Result`.

<!--
We pass the value in the `case_sensitive` variable to the `Config` instance so
the `run` function can read that value and decide whether to call `search` or
`search_case_insensitive`, as we implemented in Listing 12-22.
-->

Nous passons la valeur de la variable `sensible_casse` à l'instance de `Config`
afin que la fonction `run` puisse lire cette valeur et décider d'appeler
`rechercher` ou `rechercher_insensible_casse`, comme nous l'avons implémenté
dans l'encart 12-22.

<!--
Let’s give it a try! First, we’ll run our program without the environment
variable set and with the query `to`, which should match any line that contains
the word “to” in all lowercase:
-->

Faisons un essai ! D'abord, nous allons lancer notre programme avec la variable
d'environnement non définie et avec la recherche `to`, qui devrait trouver
toutes les lignes qui contiennent le mot “to” en minuscule :

<!--
```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

<!--
Looks like that still works! Now, let’s run the program with `CASE_INSENSITIVE`
set to `1` but with the same query `to`.
-->

On dirait que cela fonctionne ! Maintenant, lançons le programme avec
`MINIGREP_INSENSIBLE_CASSE` définie à `1` mais avec la même recherche `to`.

<!--
If you're using PowerShell, you will need to set the environment
variable and run the program as separate commands:
-->

Si vous utilisez PowerShell, vous allez avoir besoin d'affecter la variable
d'environnement puis exécuter le programme avec deux commande séparées :

<!--
```console
PS> $Env:CASE_INSENSITIVE=1; cargo run to poem.txt
```
-->

```console
PS> $Env:MINIGREP_INSENSIBLE_CASSE=1; cargo run to poem.txt
```

<!--
This will make `CASE_INSENSITIVE` persist for the remainder of your shell
session. It can be unset with the `Remove-Item` cmdlet:
-->

Cela va faire persister la variable `MINIGREP_INSENSIBLE_CASSE` pour la durée de
votre session de terminal. Elle peut être désaffectée avec la cmdlet
`Remove-Item` :

<!--
```console
PS> Remove-Item Env:CASE_INSENSITIVE
```
-->

```console
PS> Remove-Item Env:MINIGREP_INSENSIBLE_CASSE
```

<!--
We should get lines that contain “to” that might have uppercase letters:
-->

Nous devrions trouver les lignes qui contiennent “to” qui ont des lettres
majuscules :

<!--
<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
CASE_INSENSITIVE=1 cargo run to poem.txt
can't extract because of the environment variable
-- >
-->

<!--
```console
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
-->

```text
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

<!--
Excellent, we also got lines containing “To”! Our `minigrep` program can now do
case-insensitive searching controlled by an environment variable. Now you know
how to manage options set using either command line arguments or environment
variables.
-->

Très bien, nous avons aussi obtenu les lignes qui contiennent “To” ! Notre
programme `minigrep` peut maintenant faire des recherches insensibles à la
casse, pilotées par une variable d'environnement. Vous savez maintenant comment
gérer des options définies soit par des arguments en ligne de commande, soit
par des variables d'environnement.

<!--
Some programs allow arguments *and* environment variables for the same
configuration. In those cases, the programs decide that one or the other takes
precedence. For another exercise on your own, try controlling case
insensitivity through either a command line argument or an environment
variable. Decide whether the command line argument or the environment variable
should take precedence if the program is run with one set to case sensitive and
one set to case insensitive.
-->

Certains programmes permettent d'utiliser les arguments *et* les variables
d'environnement pour un même réglage. Dans ce cas, le programme décide si l'un
ou l'autre a la priorité. Pour vous exercer à nouveau, essayez de piloter la
sensibilité à la casse via un argument de ligne de commande ou une variable
d'environnement. Vous devrez choisir si l'argument de la ligne de commande ou
la variable d'environnement devrait être prioritaire si le programme est exécuté
avec l'option réglée dans un cas pour être sensible à la casse et être
insensible à la casse dans l'autre cas simultanément.

<!--
The `std::env` module contains many more useful features for dealing with
environment variables: check out its documentation to see what is available.
-->

Le module `std::env` contient plein d'autres fonctionnalitées utiles pour
utiliser les variables d'environnement : regardez sa documentation pour voir ce
qu'il est possible de faire.
