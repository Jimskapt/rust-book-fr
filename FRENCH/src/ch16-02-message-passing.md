<!--
## Using Message Passing to Transfer Data Between Threads
-->

## Utiliser l'envoi de messages pour transférer des données entre les tâches

<!--
One increasingly popular approach to ensuring safe concurrency is *message
passing*, where threads or actors communicate by sending each other messages
containing data. Here’s the idea in a slogan from [the Go language
documentation](https://golang.org/doc/effective_go.html#concurrency): 
“Do not communicate by sharing memory; instead, share memory by communicating.”
-->

Une approche de plus en plus populaire pour garantir la sécurité de la
concurrence est l'*envoi de message*, avec lequel les tâches ou les acteurs
communiquent en envoyant aux autres des messages contenant des données. Voici
l'idée résumée, tirée d'un slogan provenant de [la documentation du langage
Go](https://golang.org/doc/effective_go.html#concurrency) : “Ne communiquez pas
en partageant la mémoire ; partagez plutôt la mémoire en communiquant”.

<!--
One major tool Rust has for accomplishing message-sending concurrency is the
*channel*, a programming concept that Rust’s standard library provides an
implementation of. You can imagine a channel in programming as being like a
channel of water, such as a stream or a river. If you put something like a
rubber duck or boat into a stream, it will travel downstream to the end of the
waterway.
-->

Un des outils majeurs que Rust a pour accomplir l'envoi de messages pour la
concurrence est le *canal*, un concept de programmation dont la bibliothèque
standard de Rust fournit une implémentation. Vous pouvez imaginer un canal de
programmation comme étant un canal d'eau, comme un ruisseau ou une rivière. Si
vous posez quelque chose comme un canard en plastique ou un bateau sur un
ruisseau, il se déplacera en descendant le long la voie d'eau.

<!--
A channel in programming has two halves: a transmitter and a receiver. The
transmitter half is the upstream location where you put rubber ducks into the
river, and the receiver half is where the rubber duck ends up downstream. One
part of your code calls methods on the transmitter with the data you want to
send, and another part checks the receiving end for arriving messages. A
channel is said to be *closed* if either the transmitter or receiver half is
dropped.
-->

Un canal de programmation est divisé en deux parties : un transmetteur et un
receveur. La partie du transmetteur est le lieu en amont où vous déposez les
canards en plastique sur la rivière et la partie du receveur est où les
canards en plastique finissent leur voyage. Une partie de votre code fait appel
à des méthodes sur le transmetteur avec les données que vous souhaitez envoyer,
et une autre partie attends les messages à l'arrivée. Un canal est dit *fermé*
lorsque la partie correspondante au transmetteur ou la partie du récepteur est
libérée.

<!--
Here, we’ll work up to a program that has one thread to generate values and
send them down a channel, and another thread that will receive the values and
print them out. We’ll be sending simple values between threads using a channel
to illustrate the feature. Once you’re familiar with the technique, you could
use channels to implement a chat system or a system where many threads perform
parts of a calculation and send the parts to one thread that aggregates the
results.
-->

Ici, nous allons concevoir un programme qui a une tâche pour générer des
valeurs et les envoyer dans un canal, et une autre tâche qui va recevoir les
valeurs et les afficher. Nous allons envoyer de simples valeurs entre les
tâches en utilisant un canal pour illustrer cette fonctionnalité. Une fois que
vous serez familier avec cette technique, vous pourrez utiliser les canaux
pour créer un système de dialogue en ligne ou un système où de nombreuses
tâches font chacune une partie d'un gros calcul et envoient leur résultat à une
tâche chargée d'agréger ces résultats.

<!--
First, in Listing 16-6, we’ll create a channel but not do anything with it.
Note that this won’t compile yet because Rust can’t tell what type of values we
want to send over the channel.
-->

Pour commencer, dans l'encart 16-6, nous allons créer un canal mais nous
n'allons rien faire avec. Remarquez qu'il ne se compilera pas encore car Rust
ne peut pas savoir le type de valeurs que nous souhaitons envoyer dans le
canal.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

<!--
<span class="caption">Listing 16-6: Creating a channel and assigning the two
halves to `tx` and `rx`</span>
-->

<span class="caption">Encart 16-6 : création d'un canal et assignation de ses
deux parties à `tx` et `rx`</span>

<!--
We create a new channel using the `mpsc::channel` function; `mpsc` stands for
*multiple producer, single consumer*. In short, the way Rust’s standard library
implements channels means a channel can have multiple *sending* ends that
produce values but only one *receiving* end that consumes those values. Imagine
multiple streams flowing together into one big river: everything sent down any
of the streams will end up in one river at the end. We’ll start with a single
producer for now, but we’ll add multiple producers when we get this example
working.
-->

Nous créons un nouveau canal en utilisant la fonction `mpsc::channel` ; `mpsc`
signifie *multiple producer, single consumer*, c'est-à-dire
*plusieurs producteurs, un seul consommateur*. En bref, la façon dont la
bibliothèque standard de Rust a implémenté ces canaux permet d'avoir plusieurs
extrémités *émettrices* qui produisent des valeurs, mais seulement une seule
extrémité *réceptrice* qui consomme ces valeurs. Immaginez plusieurs ruisseaux
qui se rejoignent en une seule grosse rivière : tout ce qui est déposé sur les
ruisseaux va finir dans une seule rivière à la fin. Nous allons commencer avec
un seul producteur pour le moment, mais nous allons ajouter d'autres
producteurs lorsque notre exemple fonctionnera.

<!--
The `mpsc::channel` function returns a tuple, the first element of which is the
sending end and the second element is the receiving end. The abbreviations `tx`
and `rx` are traditionally used in many fields for *transmitter* and *receiver*
respectively, so we name our variables as such to indicate each end. We’re
using a `let` statement with a pattern that destructures the tuples; we’ll
discuss the use of patterns in `let` statements and destructuring in Chapter
18. Using a `let` statement this way is a convenient approach to extract the
pieces of the tuple returned by `mpsc::channel`.
-->

La fonction `mpsc::channel` retourne un tuple, le premier élément est celui qui
permet d'envoyer et le second est celui qui reçoit. Les abbréviations `tx` et
`rx` sont utilisés traditionnellement dans de nombreux domaines pour signifier
respectivement *transmetteur* et *récepteur*, nous avons donc nommé nos
variables comme ceci pour marquer chaque élément.  Nous utilisons une
instruction `let` avec un motif qui déstructure les tuples ; nous allons voir
l'utilisation des motifs dans les instructions `let` et la déstructuration au
chapitre 18. L'utilisation d'une instruction `let` de cette manière est une
approche facile pour extraire les éléments du tuple retourné par
`mpsc::channel`.

<!--
Let’s move the transmitting end into a spawned thread and have it send one
string so the spawned thread is communicating with the main thread, as shown in
Listing 16-7. This is like putting a rubber duck in the river upstream or
sending a chat message from one thread to another.
-->

Déplaçons maintenant l'élément de transmission dans une nouvelle tâche et
faisons-lui envoyer une chaîne de caractères afin que la nouvelle tâche
communique avec la tâche principale, comme dans l'encart 16-7. C'est comme
poser un canard en plastique sur l'amont de la rivière ou envoyer un message
instantané d'une tâche à une autre.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

<!--
<span class="caption">Listing 16-7: Moving `tx` to a spawned thread and sending
“hi”</span>
-->

<span class="caption">Encart 16-7 : déplacement de `tx` dans la nouvelle tâche
et envoi de “salut”</span>

<!--
Again, we’re using `thread::spawn` to create a new thread and then using `move`
to move `tx` into the closure so the spawned thread owns `tx`. The spawned
thread needs to own the transmitting end of the channel to be able to send
messages through the channel.
-->

Nous utilisons à nouveau `thread::spawn` pour créer une nouvelle tâche et
ensuite utiliser `move` pour déplacer `tx` dans la fermeture afin que la
nouvelle tâche possède désormais ``tx`. La nouvelle tâche a besoin de posséder
la partie émettrice du canal pour être en capacité d'envoyer des messages
dans ce canal.

<!--
The transmitting end has a `send` method that takes the value we want to send.
The `send` method returns a `Result<T, E>` type, so if the receiving end has
already been dropped and there’s nowhere to send a value, the send operation
will return an error. In this example, we’re calling `unwrap` to panic in case
of an error. But in a real application, we would handle it properly: return to
Chapter 9 to review strategies for proper error handling.
-->

La partie émettrice a une méthode `send` qui prend en argument la valeur que
nous souhaitons envoyer. La méthode `send` retourne un type `Result<T, E>`,
donc si la partie réceptrice a déjà été libérée et qu'il n'y a nulle part où
envoyer la valeur, l'opération d'envoi va retourner une erreur. Dans cet
exemple, nous faisons appel à `unwrap` pour paniquer en cas d'erreur. Mais dans
un vrai programme, nous devrions gérer ce cas correctement : retournez au
chapitre 9 pour revoir les stratégies pour gérer correctement les erreurs.

<!--
In Listing 16-8, we’ll get the value from the receiving end of the channel in
the main thread. This is like retrieving the rubber duck from the water at the
end of the river or like getting a chat message.
-->

Dans l'encart 16-8, nous allons obtenir la valeur de l'extrémité réceptrice du
canal dans la tâche principale. C'est comme récupérer le canard en plastique
dans l'eau à la fin de la rivière, ou récupérer un message instantané.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

<!--
<span class="caption">Listing 16-8: Receiving the value “hi” in the main thread
and printing it</span>
-->

<span class="caption">Encart 16-8 : réception de la valeur “salut” dans la
tâche principale pour l'afficher</span>

<!--
The receiving end of a channel has two useful methods: `recv` and `try_recv`.
We’re using `recv`, short for *receive*, which will block the main thread’s
execution and wait until a value is sent down the channel. Once a value is
sent, `recv` will return it in a `Result<T, E>`. When the sending end of the
channel closes, `recv` will return an error to signal that no more values will
be coming.
-->

La partie réception d'un canal a deux modes intéressants : `recv` et
`try_recv`. Nous avons utilisé `recv`, un raccourci pour *recevoir*, qui va
bloquer l'exécution de la tâche principale et attendre jusqu'à ce qu'une valeur
soit envoyée dans le canal. Une fois qu'une valeur est envoyée, `recv` va
la retourner dans un `Result<T, E>`. Lorsque la partie transmission du canal se
ferme, `recv` va retourner une erreur pour signaler qu'il n'y aura plus de
valeurs qui arriveront.

<!--
The `try_recv` method doesn’t block, but will instead return a `Result<T, E>`
immediately: an `Ok` value holding a message if one is available and an `Err`
value if there aren’t any messages this time. Using `try_recv` is useful if
this thread has other work to do while waiting for messages: we could write a
loop that calls `try_recv` every so often, handles a message if one is
available, and otherwise does other work for a little while until checking
again.
-->

La méthode `try_recv` ne bloque pas, mais va plutôt retourner immédiatement un
`Result<T, E>` : une valeur `Ok` qui contiendra un message s'il y en a un de
disponible, et une valeur `Err` s'il n'y a pas de message cette fois-ci.
L'utilisation de `try_recv` est pratique si cette tâche à d'autres choses à
faire pendant qu'elle attend les messages : nous pouvons ainsi écrire une
boucle qui appelle régulièrement `try_recv`, gère le message s'il y en a un, et
sinon fait d'autres choses jusqu'à ce qu'elle vérifiera à nouveau.

<!--
We’ve used `recv` in this example for simplicity; we don’t have any other work
for the main thread to do other than wait for messages, so blocking the main
thread is appropriate.
-->

Nous avons utilisé `recv` dans cet exemple pour des raisons de simplicité ;
nous n'avons rien d'autres à faire dans la tâche principale que d'attendre les
messages, donc bloquer la tâche principale est acceptable.

<!--
When we run the code in Listing 16-8, we’ll see the value printed from the main
thread:
-->

Lorsque nous exécutons le code de l'encart 16-8, nous allons voir la valeur
s'afficher grâce à la tâche principale :

<!--
<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -- >
-->

<!--
```text
Got: hi
```
-->

```text
On a reçu : salut
```

<!--
Perfect!
-->

C'est parfait ainsi !

<!--
### Channels and Ownership Transference
-->

### Les canaux et le transfert de possession

<!--
The ownership rules play a vital role in message sending because they help you
write safe, concurrent code. Preventing errors in concurrent programming is the
advantage of thinking about ownership throughout your Rust programs. Let’s do
an experiment to show how channels and ownership work together to prevent
problems: we’ll try to use a `val` value in the spawned thread *after* we’ve
sent it down the channel. Try compiling the code in Listing 16-9 to see why
this code isn’t allowed:
-->

Les règles de possession jouent un rôle vital dans l'envoi de messages car
elles vous aident à écrire du code sûr et concurrent. Réfléchir à la possession
avec vos programmes Rust vous offre l'avantage d'éviter des erreurs de
développement avec la concurrence. Faisons une expérience pour montrer comment
la possession et les canaux fonctionnent ensemble pour éviter les problèmes :
nous allons essayer d'utiliser la `valeur` dans la nouvelle tâche *après* que
nous l'ayons envoyé dans le canal. Essayez de compiler le code de l'encart 16-9
pour découvrir pourquoi ce code n'est pas autorisé :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

<!--
<span class="caption">Listing 16-9: Attempting to use `val` after we’ve sent it
down the channel</span>
-->

<span class="caption">Encart 16-9 : tentative d'utiliser `valeur` après que
nous l'ayons envoyé dans le canal</span>

<!--
Here, we try to print `val` after we’ve sent it down the channel via `tx.send`.
Allowing this would be a bad idea: once the value has been sent to another
thread, that thread could modify or drop it before we try to use the value
again. Potentially, the other thread’s modifications could cause errors or
unexpected results due to inconsistent or nonexistent data. However, Rust gives
us an error if we try to compile the code in Listing 16-9:
-->

Ici, nous essayons d'afficher `valeur` après que nous l'ayons envoyé dans le
canal avec `tx.send`. Ce serait une mauvaise idée de permettre cela : une fois
que la valeur a été envoyée à une autre tâche, cette tâche peut la modifier ou
la libérer avant que nous essayons de l'utiliser à nouveau. Il est possible que
des modifications de l'autre tâche puissent causer des erreurs ou des résultats
inattendus à cause de données incohérentes ou manquantes. Toutefois, Rust nous
affiche une erreur si nous essayons de compiler le code de l'encart 16-9 :

<!--
```console
{{#include ../listings-sources/ch16-fearless-concurrency/listing-16-09/output.txt}}
```
-->

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

<!--
Our concurrency mistake has caused a compile time error. The `send` function
takes ownership of its parameter, and when the value is moved, the receiver
takes ownership of it. This stops us from accidentally using the value again
after sending it; the ownership system checks that everything is okay.
-->

Notre erreur de concurrence a provoqué une erreur à la compilation. La fonction
`send` prend possession de ses paramètres, et lorsque la valeur est déplacée,
le récepteur en prend possession. Cela nous évite d'utiliser à nouveau
accidentellement la valeur après l'avoir envoyée ; le système de possession
vérifie que tout est en ordre.

<!--
### Sending Multiple Values and Seeing the Receiver Waiting
-->

### Envoyer plusieurs valeurs et voir le récepteur les attendre

<!--
The code in Listing 16-8 compiled and ran, but it didn’t clearly show us that
two separate threads were talking to each other over the channel. In Listing
16-10 we’ve made some modifications that will prove the code in Listing 16-8 is
running concurrently: the spawned thread will now send multiple messages and
pause for a second between each message.
-->

Le code de l'encart 16-8 s'est compilé et exécuté, mais il ne nous a pas
clairement indiqué que deux tâches séparées communiquaient entre elles via le
canal. Dans l'encart 16-10 nous avons fait quelques modifications qui prouvent
que le code de l'encart 16-8 est exécuté avec de la concurrence : la nouvelle
tâche va maintenant envoyer plusieurs messages et faire une pause d'une seconde
entre chaque message.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

<!--
<span class="caption">Listing 16-10: Sending multiple messages and pausing
between each</span>
-->

<span class="caption">Encart 16-10 : envoi de plusieurs messages en faisant une
pause entre chacun</span>

<!--
This time, the spawned thread has a vector of strings that we want to send to
the main thread. We iterate over them, sending each individually, and pause
between each by calling the `thread::sleep` function with a `Duration` value of
1 second.
-->

Cette fois-ci, la nouvelle tâche a un vecteur de chaînes de caractères que nous
souhaitons envoyer à la tâche principale. Nous itérons sur celui-ci, on les
envoie individuellement, et on fait une pause entre chaque envoi en appelant la
fonction `thread::sleep` avec une valeur `Duration` de 1 seconde.

<!--
In the main thread, we’re not calling the `recv` function explicitly anymore:
instead, we’re treating `rx` as an iterator. For each value received, we’re
printing it. When the channel is closed, iteration will end.
-->

Dans la tâche principale, nous n'appelons plus explicitement la fonction
`recv` : à la place, nous utilisons `rx` comme un itérateur. Pour chaque valeur
reçue, nous l'affichons. Lorsque le canal se fermera, l'itération se terminera.

<!--
When running the code in Listing 16-10, you should see the following output
with a 1-second pause in between each line:
-->

Lorsque nous exécutons le code de l'encart 16-10, nous devrions voir la sortie
suivante, avec une pause de 1 seconde entre chaque ligne :

<!--
<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -- >
-->

<!--
```text
Got: hi
Got: from
Got: the
Got: thread
```
-->

```text
On a reçu : salutations
On a reçu : à partir
On a reçu : de la
On a reçu : nouvelle tâche
```

<!--
Because we don’t have any code that pauses or delays in the `for` loop in the
main thread, we can tell that the main thread is waiting to receive values from
the spawned thread.
-->

Comme nous n'avons pas de code qui met en pause ou retarde la boucle `for` de
la tâche principale, nous pouvons dire que la tâche principale est en attente
de réception des valeurs de la part de la nouvelle tâche.

<!--
### Creating Multiple Producers by Cloning the Transmitter
-->

### Créer plusieurs producteurs en clonant le transmetteur

<!--
Earlier we mentioned that `mpsc` was an acronym for *multiple producer,
single consumer*. Let’s put `mpsc` to use and expand the code in Listing 16-10
to create multiple threads that all send values to the same receiver. We can do
so by cloning the transmitting half of the channel, as shown in Listing 16-11:
-->

Précédemment, nous avions évoqué que `mpsc` était un acronyme pour
*multiple producer, single consumer*. Mettons `mpsc` en œuvre en élargissant le
code de l'encart 16-10 pour créer plusieurs tâches qui vont toutes envoyer des
valeurs au même récepteur. Nous pouvons faire ceci en clonant la partie
émettrice du canal, comme dans l'encart 16-11 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 16-11: Sending multiple messages from multiple
producers</span>
-->

<span class="caption">Encart 16-11 : envoi de plusieurs messages à partir de
plusieurs producteurs</span>

<!--
This time, before we create the first spawned thread, we call `clone` on the
sending end of the channel. This will give us a new sending handle we can pass
to the first spawned thread. We pass the original sending end of the channel to
a second spawned thread. This gives us two threads, each sending different
messages to the receiving end of the channel.
-->

Cette fois-ci, avant de créer la première nouvelle tâche, nous appelons `clone`
sur la partie émettrice du canal. Cela va nous donner un nouveau transmetteur
que nous pourrons passer à la seconde nouvelle tâche. Cela va nous donner deux
tâches, chacune envoyant des messages différents à la partie réceptrice du
canal.

<!--
When you run the code, your output should look something like this:
-->

Lorsque vous exécuterez ce code, votre sortie devrait ressembler à ceci :

<!--
<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -- >
-->

<!--
```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```
-->

```text
On a reçu : salutations
On a reçu : encore plus
On a reçu : de messages
On a reçu : pour
On a reçu : à partir
On a reçu : de la
On a reçu : nouvelle tâche
On a reçu : pour vous
```

<!--
You might see the values in another order; it depends on your system. This is
what makes concurrency interesting as well as difficult. If you experiment with
`thread::sleep`, giving it various values in the different threads, each run
will be more nondeterministic and create different output each time.
-->

Vous pourrez peut-être constater que les valeurs sont dans un autre ordre chez
vous ; cela dépend de votre système. C'est ce qui rend la concurrence aussi
intéressante que difficile. Si vous jouez avec la valeur de `thread::sleep` en
lui donnant différentes valeurs dans différentes tâches, chaque exécution sera
encore moins déterminée et créera une sortie différente à chaque fois.

<!--
Now that we’ve looked at how channels work, let’s look at a different method of
concurrency.
-->

Maintenant que nous avons découvert le fonctionnement des canaux, examinons un
autre genre de concurrence.
