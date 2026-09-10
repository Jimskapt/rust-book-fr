<!--
## Storing Keys with Associated Values in Hash Maps
-->

## Stocker des clés associées à des valeurs dans des tables de hachage

<!--
The last of our common collections is the *hash map*. The type `HashMap<K, V>`
stores a mapping of keys of type `K` to values of type `V` using a
*hashing function*, which determines how it places these keys and values into
memory. Many programming languages support this kind of data structure, but
they often use a different name, such as hash, map, object, hash table,
dictionary, or associative array, just to name a few.
-->

La dernière des collections les plus courantes est la *table de hachage (hash
map)*. Le type `HashMap<K, V>` stocke une association de clés de type `K` à des
valeurs de type `V` en utilisant une *fonction de hachage*, qui détermine
comment elle va ranger ces clés et valeurs dans la mémoire. De nombreux
langages de programmation prennent en charge ce genre de structure de données,
mais ils ont souvent un nom différent, tel que hash, map, objet, table
d'association, dictionnaire ou tableau associatif, pour n'en nommer que
quelques-uns.

<!--
Hash maps are useful when you want to look up data not by using an index, as
you can with vectors, but by using a key that can be of any type. For example,
in a game, you could keep track of each team’s score in a hash map in which
each key is a team’s name and the values are each team’s score. Given a team
name, you can retrieve its score.
-->

Les tables de hachage sont utiles lorsque vous voulez rechercher des données non
pas en utilisant des indices, comme vous pouvez le faire avec les vecteurs, mais
en utilisant une clé qui peut être de n'importe quel type. Par exemple, dans un
jeu, vous pouvez consigner le score de chaque équipe dans une table de hachage
dans laquelle chaque clé est le nom d'une équipe et la valeur est le score de
l'équipe. Si vous avez le nom d'une équipe, vous pouvez récupérer son score.

<!--
We’ll go over the basic API of hash maps in this section, but many more goodies
are hiding in the functions defined on `HashMap<K, V>` by the standard library.
As always, check the standard library documentation for more information.
-->

Nous allons passer en revue l'API de base des tables de hachage dans cette
section, mais bien d'autres fonctionnalités se cachent dans les fonctions
définies sur `HashMap<K, V>` par la bibliothèque standard. Comme d'habitude,
consultez la documentation de la bibliothèque standard pour plus d'informations.

<!--
### Creating a New Hash Map
-->

### Créer une nouvelle table de hachage

<!--
One way to create an empty hash map is using `new` and adding elements with
`insert`. In Listing 8-20, we’re keeping track of the scores of two teams whose
names are *Blue* and *Yellow*. The Blue team starts with 10 points, and the
Yellow team starts with 50.
-->

Une façon de créer une table de hachage vide est d'utiliser `new` et d'ajouter
des éléments avec `insert`. Dans l'encart 8-20, nous consignons les scores de
deux équipes qui s'appellent *Bleu* et *Jaune*. L'équipe Bleu commence avec 10
points, et l'équipe Jaune commence avec 50.

<!--
```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-20: Creating a new hash map and inserting some
keys and values</span>
-->

<span class="caption">Encart 8-20 : création d'une nouvelle table de hachage et
insertion de quelques clés et valeurs</span>

<!--
Note that we need to first `use` the `HashMap` from the collections portion of
the standard library. Of our three common collections, this one is the least
often used, so it’s not included in the features brought into scope
automatically in the prelude. Hash maps also have less support from the
standard library; there’s no built-in macro to construct them, for example.
-->

Notez que nous devons d'abord importer `HashMap` via `use` depuis la partie des
collections de la bibliothèque standard. De nos trois collections courantes,
cette dernière est la moins utilisée, donc elle n'est pas présente dans les
fonctionnalités importées automatiquement dans la portée par l'étape
préliminaire. Les tables de hachage sont aussi moins gérées par la bibliothèque
standard ; il n'y a pas de macro intégrée pour les construire, par exemple.

<!--
Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `String` and values of type `i32`. Like vectors, hash maps are
homogeneous: all of the keys must have the same type, and all of the values
must have the same type.
-->

Exactement comme les vecteurs, les tables de hachage stockent leurs données sur
le tas. Cette `HashMap` a des clés de type `String` et des valeurs de type
`i32`. Et comme les vecteurs, les tables de hachage sont homogènes : toutes les
clés doivent être du même type, et toutes les valeurs doivent aussi être du
même type.

<!--
Another way of constructing a hash map is by using iterators and the `collect`
method on a vector of tuples, where each tuple consists of a key and its value.
We’ll be going into more detail about iterators and their associated methods in
the [”Processing a Series of Items with Iterators” section of Chapter
13][iterators]<!-- ignore -- >. The `collect` method gathers data into a number
of collection types, including `HashMap`. For example, if we had the team names
and initial scores in two separate vectors, we could use the `zip` method to
create an iterator of tuples where “Blue” is paired with 10, and so forth. Then
we could use the `collect` method to turn that iterator of tuples into a hash
map, as shown in Listing 8-21.
-->

Une autre façon de construire une table de hachage est d'utiliser les itérateurs
et la méthode `collect` sur un vecteur de tuples, où chaque tuple représente une
clé et sa valeur. Nous aborderons en détail les itérateurs et leurs méthodes
associées dans [une section du chapitre 13][iterators]<!-- ignore -->. La
méthode `collect` regroupe les données dans quelques types de collections, dont
`HashMap`. Par exemple, si nous avions les noms des équipes et les scores
initiaux dans deux vecteurs séparés, nous pourrions utiliser la méthode `zip`
pour créer un itérateur de tuples où “Bleu” est associé à 10, et ainsi de suite.
Ensuite, nous pourrions utiliser la méthode `collect` pour transformer cet
itérateur de tuples en table de hachage, comme dans l'encart 8-21.

<!--
```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-21: Creating a hash map from a list of teams
and a list of scores</span>
-->

<span class="caption">Encart 8-21 : création d'une table de hachage à partir
d'une liste d'équipes et d'une liste de scores</span>

<!--
The type annotation `HashMap<_, _>` is needed here because it’s possible to
`collect` into many different data structures and Rust doesn’t know which you
want unless you specify. For the parameters for the key and value types,
however, we use underscores, and Rust can infer the types that the hash map
contains based on the types of the data in the vectors. In Listing 8-21, the
key type will be `String` and the value type will be `i32`, just as in Listing
8-20.
-->

L'annotation de type `HashMap<_, _>` est nécessaire ici car `collect` peut
générer plusieurs types de structures de données et Rust ne sait pas laquelle
vous souhaitez si vous ne le précisez pas. Mais pour les paramètres qui
correspondent aux types de clé et de valeur, nous utilisons des tirets bas, et
Rust peut déduire les types que la table de hachage contient en fonction des
types des données présentes dans les vecteurs. Dans l'encart 8-21, le type des
clés sera `String` et le type des valeurs sera `i32`, comme dans l'encart 8-20.

<!--
### Hash Maps and Ownership
-->

### Les tables de hachage et la possession

<!--
For types that implement the `Copy` trait, like `i32`, the values are copied
into the hash map. For owned values like `String`, the values will be moved and
the hash map will be the owner of those values, as demonstrated in Listing 8-22.
-->

Pour les types qui implémentent le trait `Copy`, comme `i32`, les valeurs sont
copiées dans la table de hachage. Pour les valeurs qui sont possédées comme
`String`, les valeurs seront déplacées et la table de hachage sera la
propriétaire de ces valeurs, comme démontré dans l'encart 8-22.

<!--
```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-22: Showing that keys and values are owned by
the hash map once they’re inserted</span>
-->

<span class="caption">Encart 8-22 : démonstration que les clés et les valeurs
sont possédées par la table de hachage une fois qu'elles sont insérées</span>

<!--
We aren’t able to use the variables `field_name` and `field_value` after
they’ve been moved into the hash map with the call to `insert`.
-->

Nous ne pouvons plus utiliser les variables `nom_champ` et `valeur_champ` après
qu'elles ont été déplacées dans la table de hachage lors de l'appel à `insert`.

<!--
If we insert references to values into the hash map, the values won’t be moved
into the hash map. The values that the references point to must be valid for at
least as long as the hash map is valid. We’ll talk more about these issues in
the [“Validating References with
Lifetimes”][validating-references-with-lifetimes]<!-- ignore -- > section in
Chapter 10.
-->

Si nous insérons dans la table de hachage des références vers des valeurs, ces
valeurs ne seront pas déplacées dans la table de hachage. Les valeurs vers
lesquelles les références pointent doivent rester en vigueur au moins aussi
longtemps que la table de hachage. Nous verrons ces problématiques dans [une
section du chapitre 10][validating-references-with-lifetimes]<!-- ignore -->.

<!--
### Accessing Values in a Hash Map
-->

### Accéder aux valeurs dans une table de hachage

<!--
We can get a value out of the hash map by providing its key to the `get`
method, as shown in Listing 8-23.
-->

Nous pouvons obtenir une valeur d'une table de hachage en passant sa clé à la
méthode `get`, comme dans l'encart 8-23.

<!--
```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-23: Accessing the score for the Blue team
stored in the hash map</span>
-->

<span class="caption">Encart 8-23 : récupération du score de l'équipe `Bleu`,
stocké dans la table de hachage</span>

<!--
Here, `score` will have the value that’s associated with the Blue team, and the
result will be `Some(&10)`. The result is wrapped in `Some` because `get`
returns an `Option<&V>`; if there’s no value for that key in the hash map,
`get` will return `None`. The program will need to handle the `Option` in one
of the ways that we covered in Chapter 6.
-->

Dans notre cas, `score` aura la valeur qui est associée à l'équipe `Bleu`, et le
résultat sera `Some(&10)`. Le résultat est encapsulé dans un `Some` car `get`
retourne une `Option<&V>` : s'il n'y a pas de valeur pour cette clé dans la
table de hachage, `get` va retourner `None`. Le programme doit gérer cette
`Option` d'une des manières dont nous avons parlé au chapitre 6.

<!--
We can iterate over each key/value pair in a hash map in a similar manner as we
do with vectors, using a `for` loop:
-->

Nous pouvons itérer sur chaque paire de clé/valeur dans une table de hachage de
la même manière que nous le faisons avec les vecteurs, en utilisant une boucle
`for` :

<!--
```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

<!--
This code will print each pair in an arbitrary order:
-->

Ce code va afficher chaque paire dans un ordre arbitraire :

<!--
```text
Yellow: 50
Blue: 10
```
-->

```text
Jaune : 50
Bleu : 10
```

<!--
### Updating a Hash Map
-->

### Modifier une table de hachage

<!--
Although the number of key and value pairs is growable, each key can only have
one value associated with it at a time. When you want to change the data in a
hash map, you have to decide how to handle the case when a key already has a
value assigned. You could replace the old value with the new value, completely
disregarding the old value. You could keep the old value and ignore the new
value, only adding the new value if the key *doesn’t* already have a value. Or
you could combine the old value and the new value. Let’s look at how to do each
of these!
-->

Bien que le nombre de paires de clé-valeur puisse augmenter, chaque clé ne peut
être associée qu'à une seule valeur à la fois. Lorsque vous souhaitez modifier
les données d'une table de hachage, vous devez choisir comment gérer le cas où
une clé a déjà une valeur qui lui est associée. Vous pouvez remplacer l'ancienne
valeur avec la nouvelle valeur, en ignorant complètement l'ancienne valeur. Vous
pouvez garder l'ancienne valeur et ignorer la nouvelle valeur, en insérant la
nouvelle valeur uniquement si la clé *n'a pas* déjà une valeur. Ou vous pouvez
fusionner l'ancienne valeur et la nouvelle. Découvrons dès maintenant comment
faire chacune de ces actions !

<!--
#### Overwriting a Value
-->

#### Réécrire une valeur

<!--
If we insert a key and a value into a hash map and then insert that same key
with a different value, the value associated with that key will be replaced.
Even though the code in Listing 8-24 calls `insert` twice, the hash map will
only contain one key/value pair because we’re inserting the value for the Blue
team’s key both times.
-->

Si nous ajoutons une clé et une valeur dans une table de hachage et que nous
ajoutons à nouveau la même clé avec une valeur différente, la valeur associée
à cette clé sera remplacée. Même si le code dans l'encart 8-24 appelle deux
fois `insert`, la table de hachage contiendra une seule paire de clé/valeur car
nous ajoutons la valeur pour l'équipe `Bleu` à deux reprises.

<!--
```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-24: Replacing a value stored with a particular
key</span>
-->

<span class="caption">Encart 8-24 : remplacement d'une valeur stockée sous une
clé spécifique</span>

<!--
This code will print `{"Blue": 25}`. The original value of `10` has been
overwritten.
-->

Ce code va afficher `{"Bleu": 25}`. La valeur initiale `10` a été remplacée.

<!--
#### Only Inserting a Value If the Key Has No Value
-->

#### Ajouter une valeur seulement si la clé n'a pas déjà de valeur

<!--
It’s common to check whether a particular key has a value and, if it doesn’t,
insert a value for it. Hash maps have a special API for this called `entry`
that takes the key you want to check as a parameter. The return value of the
`entry` method is an enum called `Entry` that represents a value that might or
might not exist. Let’s say we want to check whether the key for the Yellow team
has a value associated with it. If it doesn’t, we want to insert the value 50,
and the same for the Blue team. Using the `entry` API, the code looks like
Listing 8-25.
-->

Il est courant de vérifier si une clé spécifique a déjà une valeur, et si ce
n'est pas le cas, de lui associer une valeur. Les tables de hachage ont une API
spécifique pour ce cas-là qui s'appelle `entry` et qui prend en paramètre la
clé que vous voulez vérifier. La valeur de retour de la méthode `entry` est une
énumération qui s'appelle `Entry` qui représente une valeur qui existe ou non.
Imaginons que nous souhaitons vérifier si la clé pour l'équipe `Jaune` a une
valeur qui lui est associée. Si ce n'est pas le cas, nous voulons lui associer
la valeur 50, et faire de même pour l'équipe `Bleu`. En utilisant l'API `entry`,
ce code va ressembler à l'encart 8-25.

<!--
```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-25: Using the `entry` method to only insert if
the key does not already have a value</span>
-->

<span class="caption">Encart 8-25 : utilisation de la méthode `entry` pour
ajouter la clé uniquement si elle n'a pas déjà de valeur associée</span>

<!--
The `or_insert` method on `Entry` is defined to return a mutable reference to
the value for the corresponding `Entry` key if that key exists, and if not,
inserts the parameter as the new value for this key and returns a mutable
reference to the new value. This technique is much cleaner than writing the
logic ourselves and, in addition, plays more nicely with the borrow checker.
-->

La méthode `or_insert` sur `Entry` est conçue pour retourner une référence
mutable vers la valeur correspondant à la clé du `Entry` si cette clé existe,
et sinon, d'ajouter son paramètre comme nouvelle valeur pour cette clé et
retourner une référence mutable vers la nouvelle valeur. Cette technique est
plus propre que d'écrire la logique nous-mêmes et, de plus, elle fonctionne
mieux avec le vérificateur d'emprunt.

<!--
Running the code in Listing 8-25 will print `{"Yellow": 50, "Blue": 10}`. The
first call to `entry` will insert the key for the Yellow team with the value
50 because the Yellow team doesn’t have a value already. The second call to
`entry` will not change the hash map because the Blue team already has the
value 10.
-->

L'exécution du code de l'encart 8-25 va afficher `{"Jaune": 50, "Bleu": 10}`.
Le premier appel à `entry` va ajouter la clé pour l'équipe `Jaune` avec la
valeur `50` car l'équipe `Jaune` n'a pas encore de valeur. Le second appel à
`entry` ne va pas changer la table de hachage car l'équipe `Bleu` a déjà la
valeur `10`.

<!--
#### Updating a Value Based on the Old Value
-->

#### Modifier une valeur en fonction de l'ancienne valeur

<!--
Another common use case for hash maps is to look up a key’s value and then
update it based on the old value. For instance, Listing 8-26 shows code that
counts how many times each word appears in some text. We use a hash map with
the words as keys and increment the value to keep track of how many times we’ve
seen that word. If it’s the first time we’ve seen a word, we’ll first insert
the value 0.
-->

Une autre utilisation courante avec les tables de hachage est de regarder la
valeur d'une clé et ensuite la modifier en fonction de l'ancienne valeur. Par
exemple, l'encart 8-26 contient du code qui compte combien de fois chaque mot
apparaît dans du texte. Nous utilisons une table de hachage avec les mots comme
clés et nous incrémentons la valeur pour compter combien de fois nous avons vu
ce mot. Si c'est la première fois que nous voyons un mot, nous allons d'abord
insérer la valeur `0`.

<!--
```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-26/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-26/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-26: Counting occurrences of words using a hash
map that stores words and counts</span>
-->

<span class="caption">Encart 8-26 : comptage des occurrences des mots en
utilisant une table de hachage qui stocke les mots et leur quantité</span>

<!--
This code will print `{"world": 2, "hello": 1, "wonderful": 1}`. The
`split_whitespace` method iterates over sub-slices, separated by whitespace, of
the value in `text`. The `or_insert` method returns a mutable reference (`&mut
V`) to the value for the specified key. Here we store that mutable reference in
the `count` variable, so in order to assign to that value, we must first
dereference `count` using the asterisk (`*`). The mutable reference goes out of
scope at the end of the `for` loop, so all of these changes are safe and
allowed by the borrowing rules.
-->

Ce code va afficher `{"monde": 2, "bonjour": 1, "magnifique": 1, "le": 1}`. La
méthode `split_whitespace` va itérer sur les sous-slices, séparées par des
espaces vides, sur la valeur dans `texte`. La méthode `or_insert` retourne une
référence mutable (`&mut V`) vers la valeur de la clé spécifiée. Nous stockons
ici la référence mutable dans la variable `compteur`, donc pour affecter une
valeur, nous devons d'abord déréférencer `compteur` en utilisant l'astérisque
(`*`). La référence mutable sort de la portée à la fin de la boucle `for`, donc
tous ces changements sont sûrs et autorisés par les règles d'emprunt.

<!--
### Hashing Functions
-->

### Fonctions de hachage

<!--
By default, `HashMap` uses a hashing function called *SipHash* that can provide
resistance to Denial of Service (DoS) attacks involving hash
tables[^siphash]<!-- ignore -- >. This is not the fastest hashing algorithm
available, but the trade-off for better security that comes with the drop in
performance is worth it. If you profile your code and find that the default
hash function is too slow for your purposes, you can switch to another function
by specifying a different hasher. A *hasher* is a type that implements the
`BuildHasher` trait. We’ll talk about traits and how to implement them in
Chapter 10. You don’t necessarily have to implement your own hasher from
scratch; [crates.io](https://crates.io/)<!-- ignore -- > has libraries shared by
other Rust users that provide hashers implementing many common hashing
algorithms.
-->

Par défaut, `HashMap` utilise une fonction de hachage nommée SipHash qui résiste
aux attaques par déni de service (DoS) envers les tables de hachage[^siphash].
Ce n'est pas l'algorithme de hachage le plus rapide qui existe, mais le
compromis entre une meilleure sécurité et la baisse de performances en vaut la
peine. Si vous analysez la performance de votre code et que vous vous rendez
compte que la fonction de hachage par défaut est trop lente pour vos besoins,
vous pouvez la remplacer par une autre fonction en spécifiant un hacheur
différent. Un *hacheur* est un type qui implémente le trait `BuildHasher`. Nous
verrons les traits et comment les implémenter au chapitre 10. Vous n'avez pas
forcément besoin d'implémenter votre propre hacheur à partir de zéro ;
[crates.io](https://crates.io/) héberge des bibliothèques partagées par d'autres
utilisateurs de Rust qui fournissent de nombreux algorithmes de hachage
répandus.

<!--
[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)
-->

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

<!--
## Summary
-->

## Résumé

<!--
Vectors, strings, and hash maps will provide a large amount of functionality
necessary in programs when you need to store, access, and modify data. Here are
some exercises you should now be equipped to solve:
-->

Les vecteurs, Strings, et tables de hachage vont vous apporter de nombreuses
fonctionnalités nécessaires à vos programmes lorsque vous aurez besoin de
stocker, accéder, et modifier des données. Voici quelques exercices que vous
devriez maintenant être en mesure de résoudre :

<!--
* Given a list of integers, use a vector and return the median (when sorted,
  the value in the middle position) and mode (the value that occurs most often;
  a hash map will be helpful here) of the list.
* Convert strings to pig latin. The first consonant of each word is moved to
  the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
  that start with a vowel have “hay” added to the end instead (“apple” becomes
  “apple-hay”). Keep in mind the details about UTF-8 encoding!
* Using a hash map and vectors, create a text interface to allow a user to add
  employee names to a department in a company. For example, “Add Sally to
  Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
  people in a department or all people in the company by department, sorted
  alphabetically.
-->

* À partir d'une liste d'entiers, utiliser un vecteur et retourner la médiane
  (la valeur au milieu lorsque la liste est triée) et le mode (la valeur qui
  apparaît le plus souvent ; une table de hachage sera utile dans ce cas) de la
  liste.
* Convertir des chaînes de caractères dans une variante du louchébem.
  La consonne initiale de chaque mot est remplacée par la lettre `l` et est
  rétablie à la fin du mot suivie du suffixe argotique “em” ; ainsi, “bonjour”
  devient “*l*onjour*bem*”. Si le mot commence par une voyelle, ajouter un `l`
  au début du mot et ajouter à la fin le suffixe “muche”. Et gardez en tête les
  détails à propos de l'encodage UTF-8 !
* En utilisant une table de hachage et des vecteurs, créez une interface
  textuelle pour permettre à un utilisateur d'ajouter des noms d'employés dans
  un département d'une entreprise. Par exemple, “Ajouter Sally au bureau
  d'études” ou “Ajouter Amir au service commercial”. Ensuite, donnez la
  possibilité à l'utilisateur de récupérer une liste de toutes les personnes
  dans un département, ou tout le monde dans l'entreprise trié par département,
  et classés dans l'ordre alphabétique dans tous les cas.

<!--
The standard library API documentation describes methods that vectors, strings,
and hash maps have that will be helpful for these exercises!
-->

La documentation de l'API de la bibliothèque standard décrit les méthodes qu'ont
les vecteurs, chaînes de caractères et tables de hachage, ce qui vous sera bien
utile pour mener à bien ces exercices !

<!--
We’re getting into more complex programs in which operations can fail, so, it’s
a perfect time to discuss error handling. We’ll do that next!
-->

Nous nous lançons dans des programmes de plus en plus complexes dans lesquels
les opérations peuvent échouer, c'est donc le moment idéal pour voir comment
bien gérer les erreurs. C'est ce que nous allons faire au prochain chapitre !

<!--
[iterators]: ch13-02-iterators.html
[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
-->

[iterators]: ch13-02-iterators.html
[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html
