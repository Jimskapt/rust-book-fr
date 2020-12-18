<!--
## Implementing an Object-Oriented Design Pattern
-->

## Implémenter un patron de conception orienté-objet

<!--
The *state pattern* is an object-oriented design pattern. The crux of the
pattern is that a value has some internal state, which is represented by a set
of *state objects*, and the value’s behavior changes based on the internal
state. The state objects share functionality: in Rust, of course, we use
structs and traits rather than objects and inheritance. Each state object is
responsible for its own behavior and for governing when it should change into
another state. The value that holds a state object knows nothing about the
different behavior of the states or when to transition between states.
-->

Le *patron état* est un patron de conception issu de l'orienté objet. Le point
essentiel du modèle est qu'une valeur a un état interne, qui est représenté par
un ensemble *d'états*, et le comportement de la valeur change en fonction de son
état interne. Les objets état partagent des fonctionnalités : en Rust, bien sûr,
nous utilisons des structures et des traits plutôt que des objets et de
l'héritage. Chaque objet état est responsable de son propre comportement et
décide lorsqu'il doit changer pour un autre état. La valeur contenue dans un
objet état ne sait rien sur les différents comportements des états et ne sait
pas quand il va changer d'état.

<!--
Using the state pattern means when the business requirements of the program
change, we won’t need to change the code of the value holding the state or the
code that uses the value. We’ll only need to update the code inside one of the
state objects to change its rules or perhaps add more state objects. Let’s look
at an example of the state design pattern and how to use it in Rust.
-->

L'utilisation du patron état signifie que lorsque les pré-requis du métier du
programme ont changé, nous n'avons pas besoin de changer le code à l'intérieur
de l'objet état ou le code qui utilise l'objet. Nous avons juste besoin de
modifier le code dans un des objets état pour changer son fonctionnement ou pour
ajouter d'autres objets état. Voyons un exemple du patron état et comment
l'utiliser en Rust.

<!--
We’ll implement a blog post workflow in an incremental way. The blog’s final
functionality will look like this:
-->

Nous allons implémenter un processus de publication de billets de blogs de
manière incrémentale. Les fonctionnalités finales du blog seront les suivantes :

<!--
1. A blog post starts as an empty draft.
2. When the draft is done, a review of the post is requested.
3. When the post is approved, it gets published.
4. Only published blog posts return content to print, so unapproved posts can’t
   accidentally be published.
-->

1. Un billet de blog commence par un brouillon vide.
2. Lorsque le brouillon est terminé, une relecture du billet est demandée.
3. Lorsqu'un billet est approuvé, il est publié.
4. Seuls les billets de blog publiés retournent du contenu à afficher, donc les
   billets non approuvés ne peuvent pas être publiés accidentellement.

<!--
Any other changes attempted on a post should have no effect. For example, if we
try to approve a draft blog post before we’ve requested a review, the post
should remain an unpublished draft.
-->

Tous les autres changements effectués sur un billet n'auront pas d'effet. Par
exemple, si nous essayons d'approuver un brouillon de billet de blog avant
d'avoir demandé une relecture, le billet devrait rester à l'état de brouillon
non publié.

<!--
Listing 17-11 shows this workflow in code form: this is an example usage of the
API we’ll implement in a library crate named `blog`. This won’t compile yet
because we haven’t implemented the `blog` crate yet.
-->

L'encart 17-11 présente ce processus de publication sous forme de code : c'est
un exemple d'utilisation de l'API que nous allons implémenter dans une crate de
bibliothèque `blog`. Elle ne va pas encore se compiler car nous n'avons pas
encore implémenté la crate `blog`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-11/src/main.rs:all}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:all}}
```

<!--
<span class="caption">Listing 17-11: Code that demonstrates the desired
behavior we want our `blog` crate to have</span>
-->

<span class="caption">Encart 17-11 : du code qui montre le comportement attendu
de notre crate `blog`</span>

<!--
We want to allow the user to create a new draft blog post with `Post::new`.
Then we want to allow text to be added to the blog post while it’s in the draft
state. If we try to get the post’s content immediately, before approval,
nothing should happen because the post is still a draft. We’ve added
`assert_eq!` in the code for demonstration purposes. An excellent unit test for
this would be to assert that a draft blog post returns an empty string from the
`content` method, but we’re not going to write tests for this example.
-->

Nous voulons permettre à l'utilisateur de créer un nouveau brouillon de billet
de blog avec `Billet::new`. Ensuite nous voulons qu'il puisse ajouter du texte
au billet de blog tant qu'il est à l'état de brouillon. Si nous essayons
d'obtenir immédiatement le contenu du billet, avant qu'il soit relu, rien ne va
se passer car le billet est toujours un brouillon. Nous avons ajouté des
`assert_eq!` dans le code pour les besoins de la démonstration. Un excellent
test unitaire pour cela serait de vérifier qu'un brouillon de billet de blog
retourne bien une chaîne de caractères vide à partir de la méthode `contenu`,
mais nous n'allons pas écrire de tests pour cet exemple.

<!--
Next, we want to enable a request for a review of the post, and we want
`content` to return an empty string while waiting for the review. When the post
receives approval, it should get published, meaning the text of the post will
be returned when `content` is called.
-->

Ensuite, nous voulons permettre de demander une relecture du billet, et nous
souhaitons que `contenu` retourne toujours une chaîne de caractères vide pendant
que nous attendons la relecture. Lorsque la relecture du billet est approuvée,
il doit être publié, ce qui signifie que le texte du billet doit être retourné
lors de l'appel à `contenu`.

<!--
Notice that the only type we’re interacting with from the crate is the `Post`
type. This type will use the state pattern and will hold a value that will be
one of three state objects representing the various states a post can be
in—draft, waiting for review, or published. Changing from one state to another
will be managed internally within the `Post` type. The states change in
response to the methods called by our library’s users on the `Post` instance,
but they don’t have to manage the state changes directly. Also, users can’t
make a mistake with the states, like publishing a post before it’s reviewed.
-->

Remarquez que le seul type avec lequel nous interagissons avec la crate est le
type `Billet`. Ce type va utiliser le patron état et va héberger une valeur qui
sera un des trois objets état représentant les différents états dans lesquels
passent un billet : brouillon, en attente de relecture, ou publié. Le changement
d'un état à un autre sera géré en interne du type `Billet`. Les états vont
changer suite aux méthodes appelées par les utilisateurs de notre bibliothèque
sur l'instance de `Billet`, mais ils n'ont pas à gérer directement les
changements d'état. De plus, les utilisateurs ne peuvent pas faire d'erreur avec
les états, comme par exemple publier un billet avant qu'il soit relu.

<!--
### Defining `Post` and Creating a New Instance in the Draft State
-->

### Définir `Billet` et créer une nouvelle instance à l'état de brouillon

<!--
Let’s get started on the implementation of the library! We know we need a
public `Post` struct that holds some content, so we’ll start with the
definition of the struct and an associated public `new` function to create an
instance of `Post`, as shown in Listing 17-12. We’ll also make a private
`State` trait. Then `Post` will hold a trait object of `Box<dyn State>`
inside an `Option<T>` in a private field named `state`. You’ll see why the
`Option<T>` is necessary in a bit.
-->

Commençons l'implémentation de la bibliothèque ! Nous savons que nous aurons
besoin d'une structure publique `Billet` qui héberge du contenu, donc nous
allons commencer par définir cette structure et une fonction publique `new` qui
lui est associée pour créer une instance de `Billet`, comme dans l'encart 17-12.
Nous allons aussi créer un trait privé `Etat`. Ensuite `Billet` devra avoir un
champ privé `etat` pour y loger une `Option<T>` contenant un objet trait de
`Box<dyn Etat>`. Nous verrons plus tard l'intérêt du `Option<T>`.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-12/src/lib.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-12/src/lib.rs}}
```

<!--
<span class="caption">Listing 17-12: Definition of a `Post` struct and a `new`
function that creates a new `Post` instance, a `State` trait, and a `Draft`
struct</span>
-->

<span class="caption">Encart 17-12 : définition d'une structure `Billet` et
d'une fonction `new` qui crée une nouvelle instance de `Billet`, un trait
`Etat`, et une structure `Brouillon`</span>

<!--
The `State` trait defines the behavior shared by different post states, and the
`Draft`, `PendingReview`, and `Published` states will all implement the `State`
trait. For now, the trait doesn’t have any methods, and we’ll start by defining
just the `Draft` state because that is the state we want a post to start in.
-->

Le trait `Etat` définit le comportement partagé par plusieurs états de billet,
et les états `Brouillon`, `EnRelecture`, et `Publier` vont tous implémenter le
trait `Etat`. Pour l'instant, le trait n'a pas de méthode, et nous allons
commencer par définir uniquement l'état `Brouillon` car c'est l'état dans lequel
nous voulons créer un nouveau billet.

<!--
When we create a new `Post`, we set its `state` field to a `Some` value that
holds a `Box`. This `Box` points to a new instance of the `Draft` struct. This
ensures whenever we create a new instance of `Post`, it will start out as a
draft. Because the `state` field of `Post` is private, there is no way to
create a `Post` in any other state! In the `Post::new` function, we set the
`content` field to a new, empty `String`.
-->

Lorsque nous créons un nouveau `Billet`, nous assignons à son champ `etat` une
valeur `Some` qui contient une `Box`. Cette `Box` pointe sur une nouvelle
instance de la structure `Brouillon`. Cela garantira qu'à chaque fois que nous
créons une nouvelle instance de `Billet`, elle commencera à l'état de brouillon.
Comme le champ `etat` de `Billet` est privé, il n'y a pas d'autre manière de
créer un `Billet` dans un autre état ! Dans la fonction `Billet::new`, nous
assignons une nouvelle `String` vide au champ `contenu`.

<!--
### Storing the Text of the Post Content
-->

### Stocker le texte du contenu du billet

<!--
Listing 17-11 showed that we want to be able to call a method named
`add_text` and pass it a `&str` that is then added to the text content of the
blog post. We implement this as a method rather than exposing the `content`
field as `pub`. This means we can implement a method later that will control
how the `content` field’s data is read. The `add_text` method is pretty
straightforward, so let’s add the implementation in Listing 17-13 to the `impl
Post` block:
-->

L'encart 17-11 a montré que nous souhaitons appeler une méthode `ajouter_texte`
et lui passer un `&str` qui est ensuite ajouté au contenu textuel du billet de
blog. Nous implémentons ceci avec une méthode plutôt que d'exposer publiquement
le champ `contenu` avec `pub`. Cela signifie que nous pouvons implémenter une
méthode plus tard qui va contrôler comment le champ `contenu` sera lu. La
méthode `ajouter_texte` est assez simple, donc ajoutons son implémentation dans
le bloc `Billet` de l'encart 17-13 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-13/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-13/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-13: Implementing the `add_text` method to add
text to a post’s `content`</span>
-->

<span class="caption">Encart 17-13 : implémentation de la méthode
`ajouter_texte` pour ajouter du texte au `contenu` d'un billet</span>

<!--
The `add_text` method takes a mutable reference to `self`, because we’re
changing the `Post` instance that we’re calling `add_text` on. We then call
`push_str` on the `String` in `content` and pass the `text` argument to add to
the saved `content`. This behavior doesn’t depend on the state the post is in,
so it’s not part of the state pattern. The `add_text` method doesn’t interact
with the `state` field at all, but it is part of the behavior we want to
support.
-->

La méthode `ajouter_texte` prend en argument une référence mutable vers `self`,
car nous changeons l'instance `Billet` sur laquelle nous appelons
`ajouter_texte`. Nous faisons ensuite appel à `push_str` sur le `String` dans
`contenu` et nous y envoyons l'argument `texte` pour l'ajouter au `contenu` déjà
stocké. Ce comportement ne dépend pas de l'état dans lequel est le billet, donc
cela ne fait pas partie du patron état. La méthode `ajouter_texte` n'interagit
pas du tout avec le champ `etat`, mais c'est volontaire.

<!--
### Ensuring the Content of a Draft Post Is Empty
-->

### S'assurer que le contenu d'un brouillon est vide

<!--
Even after we’ve called `add_text` and added some content to our post, we still
want the `content` method to return an empty string slice because the post is
still in the draft state, as shown on line 7 of Listing 17-11. For now, let’s
implement the `content` method with the simplest thing that will fulfill this
requirement: always returning an empty string slice. We’ll change this later
once we implement the ability to change a post’s state so it can be published.
So far, posts can only be in the draft state, so the post content should always
be empty. Listing 17-14 shows this placeholder implementation:
-->

Même si nous avons appelé `ajouter_texte` et ajouté du contenu dans notre
billet, nous voulons que la méthode `contenu` retourne toujours une slice de
chaîne de caractères vide car le billet est toujours à l'état de brouillon,
comme le montre la ligne 7 de l'encart 17-11. Implémentons maintenant la méthode
`contenu` de la manière la plus simple qui répond à cette consigne : toujours
retourner un slice de chaîne de caractères vide. Nous la changerons plus tard
lorsque nous implémenterons la capacité de changer l'état d'un billet afin qu'il
puisse être publié. Pour l'instant, les billets ne peuvent qu'être à l'état de
brouillon, donc le contenu du billet devrait toujours être vide. L'encart 17-14
montre l'implémentation de ceci :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-14/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-14/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-14: Adding a placeholder implementation for
the `content` method on `Post` that always returns an empty string slice</span>
-->

<span class="caption">Encart 17-14 : ajout d'une implémentation de la méthode
`contenu` sur `Billet` qui va toujours retourner une slice de chaîne de
caractères vide</span>

<!--
With this added `content` method, everything in Listing 17-11 up to line 7
works as intended.
-->

Avec cette méthode `contenu` ajoutée, tout ce qu'il y a dans l'encart 17-11
fonctionne comme prévu jusqu'à la ligne 7.

<!--
### Requesting a Review of the Post Changes Its State
-->

### Demander une relecture du billet va changer son état

<!--
Next, we need to add functionality to request a review of a post, which should
change its state from `Draft` to `PendingReview`. Listing 17-15 shows this code:
-->

Ensuite, nous avons besoin d'ajouter une fonctionnalité pour demander la
relecture d'un billet, qui devrait changer son état de `Brouillon` à
`EnRelecture`. L'encart 17-15 montre ce code :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-15/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-15/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-15: Implementing `request_review` methods on
`Post` and the `State` trait</span>
-->

<span class="caption">Encart 17-15 : implémentation des méthodes
`demander_relecture` sur `Billet` et le trait `Etat`</span>

<!--
We give `Post` a public method named `request_review` that will take a mutable
reference to `self`. Then we call an internal `request_review` method on the
current state of `Post`, and this second `request_review` method consumes the
current state and returns a new state.
-->

Nous installons la méthode publique `demander_relecture` sur `Billet` qui va
prendre en argument une référence mutable à `self`. Ensuite nous appelons la
méthode interne `demander_relecture` sur l'état interne de `Billet`, et cette
seconde méthode `demander_relecture` consomme l'état en cours et applique un
nouvel état.

<!--
We’ve added the `request_review` method to the `State` trait; all types that
implement the trait will now need to implement the `request_review` method.
Note that rather than having `self`, `&self`, or `&mut self` as the first
parameter of the method, we have `self: Box<Self>`. This syntax means the
method is only valid when called on a `Box` holding the type. This syntax takes
ownership of `Box<Self>`, invalidating the old state so the state value of the
`Post` can transform into a new state.
-->

Nous avons ajouté la méthode `demander_relecture` sur le trait `Etat` ; tous les
types qui implémentent le trait vont maintenant devoir implémenter la méthode
`demander_relecture`. Remarquez qu'au lieu d'avoir `self`, `&self`, ou
`&mut self` en premier paramètre de la méthode, nous avons `self: Box<Self>`.
Cette syntaxe signifie que la méthode est valide uniquement lorsqu'on l'appelle
sur une `Box` qui contient ce type. Cette syntaxe prend possession de
`Box<Self>`, ce qui annule l'ancien état du `Billet` qui peut changer pour un
nouvel état.

<!--
To consume the old state, the `request_review` method needs to take ownership
of the state value. This is where the `Option` in the `state` field of `Post`
comes in: we call the `take` method to take the `Some` value out of the `state`
field and leave a `None` in its place, because Rust doesn’t let us have
unpopulated fields in structs. This lets us move the `state` value out of
`Post` rather than borrowing it. Then we’ll set the post’s `state` value to the
result of this operation.
-->

Pour consommer l'ancien état, la méthode `demander_relecture` a besoin de
prendre possession de la valeur d'état. C'est ce à quoi sert le `Option` dans le
champ `etat` de `Billet` : nous faisons appel à la méthode `take` pour obtenir
la valeur dans le `Some` du champ `etat` et le remplacer par `None`, car Rust ne
nous permet pas d'avoir des champs non renseignés dans des structures. Cela nous
permet d'extraire la valeur de `etat` d'un `Billet`, plutôt que de l'emprunter.
Ensuite, nous allons réaffecter le résultat de cette opération à `etat` du
`Billet` concerné.

<!--
We need to set `state` to `None` temporarily rather than setting it directly
with code like `self.state = self.state.request_review();` to get ownership of
the `state` value. This ensures `Post` can’t use the old `state` value after
we’ve transformed it into a new state.
-->

Nous devons assigner temporairement `None` à `etat` plutôt que de lui donner
directement avec code comme `self.etat = self.etat.demander_relecture();` car
nous voulons prendre possession de la valeur `etat`. Cela garantit que `Billet`
ne peut pas utiliser l'ancienne valeur de `etat` après qu'on ai changé cet état.

<!--
The `request_review` method on `Draft` needs to return a new, boxed instance of
a new `PendingReview` struct, which represents the state when a post is waiting
for a review. The `PendingReview` struct also implements the `request_review`
method but doesn’t do any transformations. Rather, it returns itself, because
when we request a review on a post already in the `PendingReview` state, it
should stay in the `PendingReview` state.
-->

La méthode `demander_relecture` sur `Brouillon` doit retourner une nouvelle
instance d'une structure `EnRelecture` dans une `Box`, qui représente l'état
lorsqu'un billet est en attente de relecture. La structure `EnRelecture`
implémente elle aussi la méthode `demander_relecture` mais ne fait aucune
modification. A la place, il se retourne lui-même, car lorsque nous demandons
une relecture sur un billet déjà à l'état `EnRelecture`, il doit rester à l'état
`EnRelecture`.

<!--
Now we can start seeing the advantages of the state pattern: the
`request_review` method on `Post` is the same no matter its `state` value. Each
state is responsible for its own rules.
-->

Désormais nous commençons à voir les avantages du patron état : la méthode
`demander_relecture` sur `Billet` est la même peu importe la valeur de son
`etat`. Chaque état est maître de son fonctionnement.

<!--
We’ll leave the `content` method on `Post` as is, returning an empty string
slice. We can now have a `Post` in the `PendingReview` state as well as in the
`Draft` state, but we want the same behavior in the `PendingReview` state.
Listing 17-11 now works up to line 10!
-->

Nous allons conserver la méthode `contenu` sur `Billet` comme elle l'est, elle
va continuer à retourner une slice de chaîne de caractères vide. Nous pouvons
maintenant avoir un `Billet` à l'état `Brouillon` ou `EnRelecture`, mais nous
voulons qu'il suive le même comportement lorsqu'il est dans l'état
`EnRelecture`. L'encart 17-11 fonctionne maintenant jusqu'à la ligne 10 !

<!--
### Adding the `approve` Method that Changes the Behavior of `content`
-->

### Ajouter une méthode `approuver` qui change le comportement de `contenu`

<!--
The `approve` method will be similar to the `request_review` method: it will
set `state` to the value that the current state says it should have when that
state is approved, as shown in Listing 17-16:
-->

La méthode `approuver` ressemble à la méthode `demander_relecture` : elle va
changer `etat` pour la valeur que l'état actuel retournera lorsque cet état est
approuvé, comme le montre l'encart 17-16 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-16/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-16/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-16: Implementing the `approve` method on
`Post` and the `State` trait</span>
-->

<span class="caption">Encart 17-16 : implémentation de la méthode `approuver`
sur `Billet` et sur le trait `Etat`</span>

<!--
We add the `approve` method to the `State` trait and add a new struct that
implements `State`, the `Published` state.
-->

Nous avons ajouté la méthode `approuver` au trait `Etat` et ajouté une nouvelle
structure `Publier`, qui implémente `Etat`.

<!--
Similar to `request_review`, if we call the `approve` method on a `Draft`, it
will have no effect because it will return `self`. When we call `approve` on
`PendingReview`, it returns a new, boxed instance of the `Published` struct.
The `Published` struct implements the `State` trait, and for both the
`request_review` method and the `approve` method, it returns itself, because
the post should stay in the `Published` state in those cases.
-->

Comme pour `demander_relecture`, si nous faisons appel à la méthode `approuver`
sur un `Brouillon`, cela n'aura pas d'effet car elle va retourner `self`.
Lorsque nous appellerons `approuver` sur `EnRelecture`, elle va retourner une
nouvelle instance de la structure `Publier` dans une instance de `Box`. La
structure `Publier` implémente le trait `Etat`, et pour chacune des méthodes
`demander_relecture` et `approuver`, elle va retourner elle-même, car le billet
doit rester à l'état `Publier` dans ce cas-là.

<!--
Now we need to update the `content` method on `Post`: if the state is
`Published`, we want to return the value in the post’s `content` field;
otherwise, we want to return an empty string slice, as shown in Listing 17-17:
-->

Nous devons maintenant modifier la méthode `contenu` sur `Billet` : si l'état
est `Publier`, nous voulons retourner la valeur du champ `contenu` du billet ;
sinon nous retournons une slice de chaîne de caractères vide, comme dans
l'encart 17-17 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-17/src/lib.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-17/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-17: Updating the `content` method on `Post` to
delegate to a `content` method on `State`</span>
-->

<span class="caption">Encart 17-17 : correction de la méthode `contenu` de
`Billet` pour déléguer à la méthode `contenu` de `Etat`</span>

<!--
Because the goal is to keep all these rules inside the structs that implement
`State`, we call a `content` method on the value in `state` and pass the post
instance (that is, `self`) as an argument. Then we return the value that is
returned from using the `content` method on the `state` value.
-->

Comme notre but est de conserver toutes ces règles dans les structures qui
implémentent `Etat`, nous appelons une méthode `contenu` sur la valeur de
`etat` et nous lui passons en argument l'instance du billet (avec le `self`).
Nous retournons ensuite la valeur retournée par la méthode `contenu` sur la
valeur de `etat`.

<!-- markdownlint-disable -->
<!--
We call the `as_ref` method on the `Option` because we want a reference to the
value inside the `Option` rather than ownership of the value. Because `state`
is an `Option<Box<dyn State>>`, when we call `as_ref`, an `Option<&Box<dyn State>>` is
returned. If we didn’t call `as_ref`, we would get an error because we can’t
move `state` out of the borrowed `&self` of the function parameter.
-->
<!-- markdownlint-enable -->

Nous faisons appel à la méthode `as_ref` sur `Option` car nous voulons une
référence vers la valeur dans `Option` plutôt que d'en prendre possession. Comme
`etat` est un `Option<Box<dyn Etat>>`, lorsque nous faisons appel à `as_ref`,
une `Option<&Box<dyn Etat>>` est retournée. Si nous n'avions pas fait appel à
`as_ref`, nous aurions obtenu une erreur car nous ne pouvons pas déplacer
`etat` du `&self`, qui est emprunté, qui provient des paramètres de la fonction.

<!--
We then call the `unwrap` method, which we know will never panic, because we
know the methods on `Post` ensure that `state` will always contain a `Some`
value when those methods are done. This is one of the cases we talked about in
the [“Cases In Which You Have More Information Than the
Compiler”][more-info-than-rustc]<!-- ignore -- > section of Chapter 9 when we
know that a `None` value is never possible, even though the compiler isn’t able
to understand that.
-->

Nous faisons ensuite appel à la méthode `unwrap`, mais nous savons qu'elle ne
va jamais paniquer, car nous savons que les méthodes sur `Billet` vont toujours
garantir que `etat` contiendra toujours une valeur `Some` lorsqu'elles seront
utilisées. C'est un des cas dont nous avons parlé dans
[une section][more-info-than-rustc]<!-- ignore --> du chapitre 9 lorsque nous
savions qu'une valeur `None` ne serait jamais possible, même si le compilateur
n'est pas capable de le comprendre.

<!-- markdownlint-disable -->
<!--
At this point, when we call `content` on the `&Box<dyn State>`, deref coercion will
take effect on the `&` and the `Box` so the `content` method will ultimately be
called on the type that implements the `State` trait. That means we need to add
`content` to the `State` trait definition, and that is where we’ll put the
logic for what content to return depending on which state we have, as shown in
Listing 17-18:
-->
<!-- markdownlint-enable -->

A partir de là, lorsque nous faisons appel à `contenu` sur `&Box<dyn Etat>`,
l'extrapolation de déréférencement va s'appliquer sur le `&` et le `Box` pour
que la méthode `contenu` puisse finalement être appelée sur le type qui
implémente le trait `Etat`. Cela signifie que nous devons ajouter `contenu` à la
définition du trait `Etat`, et que c'est ici que nous allons placer la logique
pour le contenu à retourner en fonction de quel état nous avons, comme le montre
l'encart 17-18 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-18/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-18/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-18: Adding the `content` method to the `State`
trait</span>
-->

<span class="caption">Encart 17-18 : ajout de la méthode `contenu` sur le trait
`Etat`</span>

<!--
We add a default implementation for the `content` method that returns an empty
string slice. That means we don’t need to implement `content` on the `Draft`
and `PendingReview` structs. The `Published` struct will override the `content`
method and return the value in `post.content`.
-->

Nous avons ajouté une implémentation par défaut pour la méthode `contenu` qui
retourne une slice de chaîne de caractères vide. Cela nous permet de ne pas
avoir à implémenter `contenu` sur les structures `Brouillon` et `EnRelecture`.
La structure `Publier` va remplacer la méthode `contenu` et retourner la valeur
présente dans `billet.contenu`.

<!--
Note that we need lifetime annotations on this method, as we discussed in
Chapter 10. We’re taking a reference to a `post` as an argument and returning a
reference to part of that `post`, so the lifetime of the returned reference is
related to the lifetime of the `post` argument.
-->

Remarquez aussi que nous devons annoter des durées de vie sur cette méthode,
comme nous l'avons vu au chapitre 10. Nous allons prendre en argument une
référence au `billet` et retourner une référence à une partie de ce `billet`,
donc la durée de vie retournée par la référence est liée à la durée de vie de
l'argument `billet`.

<!--
And we’re done—all of Listing 17-11 now works! We’ve implemented the state
pattern with the rules of the blog post workflow. The logic related to the
rules lives in the state objects rather than being scattered throughout `Post`.
-->

Et nous avons maintenant terminé, tout le code de l'encart 17-11 fonctionne,
désormais ! Nous avons implémenté le patron état avec les règles de notre
processus de publication définies pour notre blog. La logique des règles sont
intégrées dans les objets état plutôt que d'être dispersée un peu partout dans
`Billet`.

<!--
### Trade-offs of the State Pattern
-->

### Les désavantages du patron état

<!--
We’ve shown that Rust is capable of implementing the object-oriented state
pattern to encapsulate the different kinds of behavior a post should have in
each state. The methods on `Post` know nothing about the various behaviors. The
way we organized the code, we have to look in only one place to know the
different ways a published post can behave: the implementation of the `State`
trait on the `Published` struct.
-->

Nous avons démontré que Rust est capable d'implémenter le patron état qui est
orienté objet pour regrouper les différents types de comportement qu'un billet
doit avoir à chaque état. Les méthodes sur `Billet` ne savent rien des
différents comportements. De la manière dont nous avons organisé le code, nous
n'avons qu'à regarder à un seul endroit pour connaître les différents
comportements qu'un billet publié va suivre : l'implémentation du trait `Etat`
sur la structure `Publier`.

<!--
If we were to create an alternative implementation that didn’t use the state
pattern, we might instead use `match` expressions in the methods on `Post` or
even in the `main` code that checks the state of the post and changes behavior
in those places. That would mean we would have to look in several places to
understand all the implications of a post being in the published state! This
would only increase the more states we added: each of those `match` expressions
would need another arm.
-->

Si nous aurions utilité une autre façon d'implémenter ces règles sans utiliser
le patron état, nous aurions dû utiliser des expressions `match` dans les
méthodes de `Billet` ou même dans le code du `main` qui vérifie que l'état du
billet et les comportements associés aux changements d'états. Cela aurait eu
pour conséquence d'avoir à regarder à différents endroits pour comprendre toutes
les conséquences de la publication d'un billet ! Et ce code grossira au fur et
à mesure que nous ajouterons des états : chaque expression `match` devra avoir
des nouvelles branches pour ces nouveaux états.

<!--
With the state pattern, the `Post` methods and the places we use `Post` don’t
need `match` expressions, and to add a new state, we would only need to add a
new struct and implement the trait methods on that one struct.
-->

Avec le patron état, les méthodes de `Billet` et les endroits où nous utilisons
`Billet` n'ont pas besoin d'expressions `match`, et pour ajouter un nouvel état,
nous avons seulement besoin d'ajouter une nouvelle structure et d'implémenter
les méthodes du trait sur cette structure.

<div id="suggestions-implementations">

<!--
The implementation using the state pattern is easy to extend to add more
functionality. To see the simplicity of maintaining code that uses the state
pattern, try a few of these suggestions:
-->

L'implémentation qui utilise le patron état est facile à améliorer pour ajouter
plus de fonctionnalités. Pour découvrir la simplicité de maintenance du code qui
utilise le patron état, essayez d'accomplir certaines de ces suggestions :

<!--
* Add a `reject` method that changes the post’s state from `PendingReview` back
  to `Draft`.
* Require two calls to `approve` before the state can be changed to `Published`.
* Allow users to add text content only when a post is in the `Draft` state.
  Hint: have the state object responsible for what might change about the
  content but not responsible for modifying the `Post`.
-->

* Ajouter une méthode `rejeter` qui fait retourner l'état d'un billet de
  `EnRelecture` en `Brouillon`.
* Attendre deux appels à `approuver` avant que l'état puisse être changé en
  `Publier`.
* Permettre aux utilisateurs pour ajouter du contenu textuel uniquement
  lorsqu'un billet est à l'état `Brouillon`. Astuce : l'objet état est
  responsable de ce qui peut changer dans le contenu mais pas responsable de la
  modification de `Billet`.

</div>

<!--
One downside of the state pattern is that, because the states implement the
transitions between states, some of the states are coupled to each other. If we
add another state between `PendingReview` and `Published`, such as `Scheduled`,
we would have to change the code in `PendingReview` to transition to
`Scheduled` instead. It would be less work if `PendingReview` didn’t need to
change with the addition of a new state, but that would mean switching to
another design pattern.
-->

Un désavantage du patron état est que comme les états implémentent les
transitions en les états, certains des états sont reliés entre eux. Si nous
ajoutons un nouvel état entre `EnRelecture` et `Publier`, comme `Planifier`,
nous devrions alors changer le code dans `EnRelecture` pour qu'il passe ensuite
à l'état `Planifier` à la place de `Publier`. Cela représentera moins de travail
si `EnRelecture` n'a pas besoin de changer lorsqu'on ajoute un nouvel état, mais
cela signifiera alors qu'il faut changer de patron.

<!--
Another downside is that we’ve duplicated some logic. To eliminate some of the
duplication, we might try to make default implementations for the
`request_review` and `approve` methods on the `State` trait that return `self`;
however, this would violate object safety, because the trait doesn’t know what
the concrete `self` will be exactly. We want to be able to use `State` as a
trait object, so we need its methods to be object safe.
-->

Un autre désavantage et que nous avons de la logique en double. Pour éviter ces
doublons, nous devrions essayer de faire en sorte que les méthodes
`demander_relecture` et `approuver` qui retournent `self` deviennent les
implémentations par défaut sur le trait `Etat` ; cependant, cela violera la
sécurité des objets, car le trait ne sait pas ce qu'est exactement `self`. Nous
voulons pouvoir utiliser `Etat` en tant qu'objet trait, donc nous avons besoin
que ses méthodes soient sûres pour les objets.

<!--
Other duplication includes the similar implementations of the `request_review`
and `approve` methods on `Post`. Both methods delegate to the implementation of
the same method on the value in the `state` field of `Option` and set the new
value of the `state` field to the result. If we had a lot of methods on `Post`
that followed this pattern, we might consider defining a macro to eliminate the
repetition (see the [“Macros”][macros]<!-- ignore -- > section in Chapter 19).
-->

Nous avons aussi des doublons dans le code des méthodes `demander_relecture`
et `approuver` sur `Billet`. Ces deux méthodes délèguent leur travail aux mêmes
méthodes respectives sur la valeur du champ `etat` de type `Option` et assignent
la valeur au même champ `etat` à la fin. Si nous avions beaucoup de méthodes sur
`Billet` qui suivent cette logique, nous devrions considérer la création et
l'utilisation d'une macro pour éviter cette répétition (voir la
[section dédiée][macros]<!-- ignore --> dans le chapitre 19).

<!--
By implementing the state pattern exactly as it’s defined for object-oriented
languages, we’re not taking as full advantage of Rust’s strengths as we could.
Let’s look at some changes we can make to the `blog` crate that can make
invalid states and transitions into compile time errors.
-->

En implémentant le patron état exactement comme il l'est défini pour les
langages orientés-objet, nous ne profitons pas pleinement des avantages de
Rust. Voyons voir si nous pouvons faire quelques changements pour que la crate
`blog` puisse lever des erreurs à la compilation lorsqu'elle aura détecté des
états ou des transitions invalides.

<!--
#### Encoding States and Behavior as Types
-->

#### Implémenter les états et les comportements avec des types

<!--
We’ll show you how to rethink the state pattern to get a different set of
trade-offs. Rather than encapsulating the states and transitions completely so
outside code has no knowledge of them, we’ll encode the states into different
types. Consequently, Rust’s type checking system will prevent attempts to use
draft posts where only published posts are allowed by issuing a compiler error.
-->

Nous allons vous montrer comment repenser le patron état pour qu'il offre des
compromis différents. Plutôt que d'intégrer les états et les transitions
complètement de manière à ce que le code externe ne puissent pas les connaître,
nous allons transformer les états dans différents types. En conséquence, le
système de vérification de type de Rust va éviter d'utiliser des brouillons de
billets en provoquant une erreur du compilateur dans les endroits où ils seront
utilisés alors que seuls les billets publiés seront autorisés.

<!--
Let’s consider the first part of `main` in Listing 17-11:
-->

Considérons la première partie du `main` de l'encart 17-11 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-11/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:here}}
```

<!--
We still enable the creation of new posts in the draft state using `Post::new`
and the ability to add text to the post’s content. But instead of having a
`content` method on a draft post that returns an empty string, we’ll make it so
draft posts don’t have the `content` method at all. That way, if we try to get
a draft post’s content, we’ll get a compiler error telling us the method
doesn’t exist. As a result, it will be impossible for us to accidentally
display draft post content in production, because that code won’t even compile.
Listing 17-19 shows the definition of a `Post` struct and a `DraftPost` struct,
as well as methods on each:
-->

Nous pouvons toujours créer de nouveaux billets à l'état de brouillon en
utilisant `Billet::new` et ajouter du texte au contenu du billet. Mais au lieu
d'avoir une méthode `contenu` sur un brouillon de billet qui retourne une chaîne
de caractères vide, nous faisons en sorte que les brouillons de billets n'aient
même pas de méthode `contenu`. Ainsi, si nous essayons de récupérer le contenu
d'un brouillon de billet, nous obtenons une erreur de compilation qui nous
informera que la méthode n'existe pas. Finalement, il nous sera impossible de
publier le contenu d'un brouillon de billet en production, car ce code ne se
compilera même pas. L'encart 17-19 nous propose les définitions d'une structure
`Billet` et d'une structure `BrouillonDeBillet` ainsi que leurs méthodes :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-19/src/lib.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-19/src/lib.rs}}
```

<!--
<span class="caption">Listing 17-19: A `Post` with a `content` method and a
`DraftPost` without a `content` method</span>
-->

<span class="caption">Encart 17-19 : un `Billet` avec une méthode `contenu` et
un `BrouillonDeBillet` sans méthode `contenu`</span>

<!--
Both the `Post` and `DraftPost` structs have a private `content` field that
stores the blog post text. The structs no longer have the `state` field because
we’re moving the encoding of the state to the types of the structs. The `Post`
struct will represent a published post, and it has a `content` method that
returns the `content`.
-->

Les deux structures `Billet` et `BrouillonDeBillet` ont un champ privé `contenu`
qui stocke le texte du billet de blog. Les structures n'ont plus le champ `etat`
car nous avons déplacé la signification de l'état directement dans le nom de ces
types de structures. La structure `Billet` représente un billet publié, et a une
méthode `contenu` qui retourne le `contenu`.

<!--
We still have a `Post::new` function, but instead of returning an instance of
`Post`, it returns an instance of `DraftPost`. Because `content` is private
and there aren’t any functions that return `Post`, it’s not possible to create
an instance of `Post` right now.
-->

Nous avons toujours la fonction `Billet::new`, mais au lieu de retourner une
instance de `Billet`, elle va retourner une instance de `BrouillonDeBillet`.
Comme `contenu` est privé et qu'il n'y a pas de fonction qui retourne `Billet`,
il ne sera pas possible pour le moment de créer une instance de `Billet`.

<!--
The `DraftPost` struct has an `add_text` method, so we can add text to
`content` as before, but note that `DraftPost` does not have a `content` method
defined! So now the program ensures all posts start as draft posts, and draft
posts don’t have their content available for display. Any attempt to get around
these constraints will result in a compiler error.
-->

La structure `BrouillonDeBillet` a une méthode `ajouter_texte`, donc nous
pouvons ajouter du texte à `contenu` comme nous le faisions avant, mais
remarquez toutefois que `BrouillonDeBillet` n'a pas de méthode `contenu` de
défini ! Donc pour l'instant le programme s'assure que tous les billets
démarrent à l'état de brouillon, et que les brouillons ne proposent pas de
contenu à publier. Toute tentative d'outre-passer ces contraintes vont
déclencher des erreurs de compilation.

<!--
#### Implementing Transitions as Transformations into Different Types
-->

#### Implémenter les changements d'état en tant que changement de type

<!--
So how do we get a published post? We want to enforce the rule that a draft
post has to be reviewed and approved before it can be published. A post in the
pending review state should still not display any content. Let’s implement
these constraints by adding another struct, `PendingReviewPost`, defining the
`request_review` method on `DraftPost` to return a `PendingReviewPost`, and
defining an `approve` method on `PendingReviewPost` to return a `Post`, as
shown in Listing 17-20:
-->

Donc, comment publier un billet ? Nous voulons renforcer la règle qui dit qu'un
brouillon de billet doit être relu et approuvé avant de pouvoir être publié. Un
billet à l'état de relecture doit continuer à ne pas montrer son contenu.
Implémentons ces contraintes en introduisant une nouvelle structure,
`BilletEnRelecture`, en définissant la méthode `demander_relecture` sur
`BrouillonDeBillet` pour retourner un `BilletEnRelecture`, et en définissant une
méthode `approuver` sur `BilletEnRelecture` pour qu'elle retourne un `Billet`,
comme le propose l'encart 17-20 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-20/src/lib.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-20/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-20: A `PendingReviewPost` that gets created by
calling `request_review` on `DraftPost` and an `approve` method that turns a
`PendingReviewPost` into a published `Post`</span>
-->

<span class="caption">Encart 17-20 : ajout d'un `BilletEnRelecture` qui est créé
par l'appel à `demander_relecture` sur `BrouillonDeBillet`, ainsi qu'une méthode
`approuver` qui transforme un `BilletEnRelecture` en `Billet` publié</span>

<!--
The `request_review` and `approve` methods take ownership of `self`, thus
consuming the `DraftPost` and `PendingReviewPost` instances and transforming
them into a `PendingReviewPost` and a published `Post`, respectively. This way,
we won’t have any lingering `DraftPost` instances after we’ve called
`request_review` on them, and so forth. The `PendingReviewPost` struct doesn’t
have a `content` method defined on it, so attempting to read its content
results in a compiler error, as with `DraftPost`. Because the only way to get a
published `Post` instance that does have a `content` method defined is to call
the `approve` method on a `PendingReviewPost`, and the only way to get a
`PendingReviewPost` is to call the `request_review` method on a `DraftPost`,
we’ve now encoded the blog post workflow into the type system.
-->

Les méthodes `demander_relecture` et `approuver` prennent possession de `self`,
ce qui consomme les instances de `BrouillonDeBillet` et de `BilletEnRelecture`
pour les transformer respectivement en `BilletEnRelecture` et en `Billet`.
Ainsi, il ne restera plus d'instances de `BrouillonDeBillet` après avoir appelé
`approuver` sur elles, et ainsi de suite. La structure `BilletEnRelecture` n'a
pas de méthode `contenu` qui lui est définie, donc si on essaye de lire son
contenu, on obtient une erreur de compilation, comme avec `BrouillonDeBillet`.
Comme la seule manière d'obtenir une instance de `Billet` qui a une méthode
`contenu` de définie est d'appeler la méthode`approuver` sur un
`BilletEnRelecture`, et que la seule manière d'obtenir un `BilletEnRelecture`
est d'appeler la méthode `demander_relecture` sur un `BrouillonDeBillet`, nous
avons désormais intégré le processus de publication des billets de blog avec le
système de type.

<!--
But we also have to make some small changes to `main`. The `request_review` and
`approve` methods return new instances rather than modifying the struct they’re
called on, so we need to add more `let post =` shadowing assignments to save
the returned instances. We also can’t have the assertions about the draft and
pending review post’s contents be empty strings, nor do we need them: we can’t
compile code that tries to use the content of posts in those states any longer.
The updated code in `main` is shown in Listing 17-21:
-->

Mais nous devons aussi faire quelques petits changements dans le `main`. Les
méthodes `demander_relecture` et `approuver` retournent des nouvelles instances
au lieu de modifier la structure sur laquelle elles ont été appelées, donc nous
devons ajouter des assignations de masquage `let billet =` pour stocker les
nouvelles instances retournées. Nous ne pouvons pas non plus vérifier que le
contenu des brouillons de billets et ceux en cours de relecture soient bien
vides, donc nous n'en avons plus besoin : nous ne pouvons plus compiler du code
qui essaye d'utiliser le contenu d'un billet dans cet état. Le code du `main`
mis à jour est présenté dans l'encart 17-21 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch17-oop/listing-17-21/src/main.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-21/src/main.rs}}
```

<!--
<span class="caption">Listing 17-21: Modifications to `main` to use the new
implementation of the blog post workflow</span>
-->

<span class="caption">Encart 17-21 : modification de `main` pour utiliser la
nouvelle implémentation du processus de publication de billet de blog</span>

<!--
The changes we needed to make to `main` to reassign `post` mean that this
implementation doesn’t quite follow the object-oriented state pattern anymore:
the transformations between the states are no longer encapsulated entirely
within the `Post` implementation. However, our gain is that invalid states are
now impossible because of the type system and the type checking that happens at
compile time! This ensures that certain bugs, such as display of the content of
an unpublished post, will be discovered before they make it to production.
-->

Les modifications dont nous avons besoin de faire au `main` pour réassigner
`billet` impliquent que cette implémentation ne suit plus exactement le patron
état orienté-objet : les changements d'états ne sont plus totalement intégrés
dans l'implémentation de `Billet`. Cependant, nous faisons en sorte que les
états invalides soient impossibles grâce au système de types et que la
vérification de type s'effectue à la compilation ! Cela garantit que certains
bogues, comme l'affichage du contenu d'un billet non publié, sera détecté avant
avant d'arriver en production.

<!--
Try the tasks suggested for additional requirements that we mentioned at the
start of this section on the `blog` crate as it is after Listing 17-20 to see
what you think about the design of this version of the code. Note that some of
the tasks might be completed already in this design.
-->

Essayez d'accomplir [les suggestions proposées pour les conditions
supplémentaires que nous avons listés au début de cette
section](#suggestions-implementations), sur la crate `blog` telle qu'elle est
après l'encart 17-20, pour vous faire une idée sur cette conception du code.
Notez aussi que certaines suggestions sont déjà accomplies implicitement grâce à
cette conception.

<!--
We’ve seen that even though Rust is capable of implementing object-oriented
design patterns, other patterns, such as encoding state into the type system,
are also available in Rust. These patterns have different trade-offs. Although
you might be very familiar with object-oriented patterns, rethinking the
problem to take advantage of Rust’s features can provide benefits, such as
preventing some bugs at compile time. Object-oriented patterns won’t always be
the best solution in Rust due to certain features, like ownership, that
object-oriented languages don’t have.
-->

Nous avons vu que même si Rust est capable d'implémenter des patrons de
conception orientés-objet, les autres patrons, comme intégrer l'état dans le
système de type, est aussi possible en Rust. Ces patrons ont différents
avantages et désavantages. Bien que vous soyez familier avec les patrons
orientés-objet, vous gagnerez à repenser les choses pour tirer avantage des
fonctionnalités de Rust, comme la détection de certains bogues à la compilation.
Les patrons orientés-objet ne sont pas toujours la meilleure solution en Rust à
cause de certaines de ses fonctionnalités, comme la possession, que les langages
orientés-objet n'ont pas.

<!--
## Summary
-->

## Résumé

<!--
No matter whether or not you think Rust is an object-oriented language after
reading this chapter, you now know that you can use trait objects to get some
object-oriented features in Rust. Dynamic dispatch can give your code some
flexibility in exchange for a bit of runtime performance. You can use this
flexibility to implement object-oriented patterns that can help your code’s
maintainability. Rust also has other features, like ownership, that
object-oriented languages don’t have. An object-oriented pattern won’t always
be the best way to take advantage of Rust’s strengths, but is an available
option.
-->

Que vous pensiez ou non que Rust est un langage orienté-objet après avoir lu ce
chapitre, vous savez maintenant que vous pouvez utiliser les objets trait pour
pouvoir obtenir certaines fonctionnalités orienté-objet en Rust. La répartition
dynamique peut offrir de la flexibilité à votre code en échange d'une perte de
performances à l'exécution. Vous pouvez utiliser cette flexibilité pour
implémenter des patrons orientés-objet qui puissent aider la maintenance de
votre code. Rust offre d'autres fonctionnalités, comme la possession, que les
langages orientés-objet n'ont pas. L'utilisation d'un patron orienté-objet n'est
pas toujours la meilleure manière de tirer parti des avantages de Rust, mais
cela reste une option disponible.

<!--
Next, we’ll look at patterns, which are another of Rust’s features that enable
lots of flexibility. We’ve looked at them briefly throughout the book but
haven’t seen their full capability yet. Let’s go!
-->

Dans le chapitre suivant, nous allons étudier les motifs, qui est une autre des
fonctionnalités de Rust qui offre beaucoup de flexibilité. Nous les avons
rencontrés brièvement dans le livre, mais nous n'avons pas encore vu tout leur
potentiel. C'est parti !

<!-- markdownlint-disable -->
<!--
[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler
[macros]: ch19-06-macros.html#macros
-->
<!-- markdownlint-enable -->

[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html
[macros]: ch19-06-macros.html#macros
