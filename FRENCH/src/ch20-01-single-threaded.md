<!--
## Building a Single-Threaded Web Server
-->

## Développer un serveur web monotâche

<!--
We’ll start by getting a single-threaded web server working. Before we begin,
let’s look at a quick overview of the protocols involved in building web
servers. The details of these protocols are beyond the scope of this book, but
a brief overview will give you the information you need.
-->

Nous allons commencer par faire fonctionner un serveur web avec une seule tâche.
Avant de commencer, faisons un survol rapide des protocoles utilisés dans les
serveurs web. Les détails de ces protocoles ne sont pas le sujet de ce livre,
mais un rapide aperçu vous donnera les informations dont vous avez besoin.

<!--
The two main protocols involved in web servers are the *Hypertext Transfer
Protocol* *(HTTP)* and the *Transmission Control Protocol* *(TCP)*. Both
protocols are *request-response* protocols, meaning a *client* initiates
requests and a *server* listens to the requests and provides a response to the
client. The contents of those requests and responses are defined by the
protocols.
-->

Les deux principaux protocoles utilisés dans les serveurs web sont le
*Hypertext Transfer Protocol* *(HTTP)* et le *Transmission Control Protocol*
*(TCP)*. Ces deux protocoles sont des protocoles de type *demande-réponse*, ce
qui signifie qu'un *client* démarre les requêtes, et le *serveur* écoute les
requêtes et fournit une réponse au client. Le contenu de ces requêtes et
réponses est défini par les protocoles.

<!--
TCP is the lower-level protocol that describes the details of how information
gets from one server to another but doesn’t specify what that information is.
HTTP builds on top of TCP by defining the contents of the requests and
responses. It’s technically possible to use HTTP with other protocols, but in
the vast majority of cases, HTTP sends its data over TCP. We’ll work with the
raw bytes of TCP and HTTP requests and responses.
-->

TCP est le protocole le plus bas-niveau qui décrit les détails de comment une
information passe d'un serveur à un autre mais ne précise pas ce qu'est cette
information. HTTP est construit sur TCP en définissant le contenu des requêtes et
des réponses. Il est techniquement possible d'utiliser HTTP avec d'autres
protocoles, mais dans la grande majorité des cas, HTTP envoie ses données via
TCP. Nous allons travailler avec les octets bruts des requêtes et des réponses
de TCP et HTTP.

<!--
### Listening to the TCP Connection
-->

### Ecouter les connexions TCP

<!--
Our web server needs to listen to a TCP connection, so that’s the first part
we’ll work on. The standard library offers a `std::net` module that lets us do
this. Let’s make a new project in the usual fashion:
-->

Notre serveur web a besoin d'écouter les connexions TCP, donc cela sera la
première partie sur laquelle nous travaillerons. La bibliothèque standard offre
un module `std::net` qui nous permet de faire ceci. Créons un nouveau projet de
manière habituelle :

<!--
```console
$ cargo new hello
     Created binary (application) `hello` project
$ cd hello
```
-->

```console
$ cargo new salutations
     Created binary (application) `salutations` project
$ cd salutations
```

<!--
Now enter the code in Listing 20-1 in *src/main.rs* to start. This code will
listen at the address `127.0.0.1:7878` for incoming TCP streams. When it gets
an incoming stream, it will print `Connection established!`.
-->

Maintenant, saisissez le code de l'encart 20-1 dans *src/main.rs* pour
commencer. Ce code va écouter les flux TCP entrants à l'adresse
`127.0.0.1:7878`. Lorsqu'il obtiendra un flux entrant, il va afficher
`Connexion établie !`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,no_run
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-01/src/main.rs}}
```
-->

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-01/src/main.rs}}
```

<!--
<span class="caption">Listing 20-1: Listening for incoming streams and printing
a message when we receive a stream</span>
-->

<span class="caption">Encart 20-1 : écoute des flux entrants et affichage d'un
message lorsque nous recevons un flux</span>

<!--
Using `TcpListener`, we can listen for TCP connections at the address
`127.0.0.1:7878`. In the address, the section before the colon is an IP address
representing your computer (this is the same on every computer and doesn’t
represent the authors’ computer specifically), and `7878` is the port. We’ve
chosen this port for two reasons: HTTP is normally accepted on this port, and
7878 is *rust* typed on a telephone.
-->

En utilisant `TcpListener`, nous pouvons écouter les connexions TCP à l'adresse
`127.0.0.1:7878`. Dans cette adresse, la partie avant les double-points est une
adresse IP qui représente votre ordinateur (c'est la même sur chaque ordinateur
et ne représente pas précisément l'ordinateur de l'auteur), et `7878` est le
port. Nous avons choisi ce port pour deux raisons : HTTP est habituellement
accepté sur ce port, et 7878 correspond aux touches utilisées sur un clavier de
téléphone pour écrire *Rust*.

<!--
The `bind` function in this scenario works like the `new` function in that it
will return a new `TcpListener` instance. The reason the function is called
`bind` is that in networking, connecting to a port to listen to is known as
“binding to a port.”
-->

La fonction `bind` dans ce scénario fonctionne comme la fonction `new` dans le
sens où elle retourne une nouvelle instance de `TcpListener`. La raison pour
laquelle cette fonction s'appelle `bind` *(NdT : signifie "lier")* est que dans
les réseaux, connecter un port à écouter se dit aussi “lier à un port”.

<!--
The `bind` function returns a `Result<T, E>`, which indicates that binding
might fail. For example, connecting to port 80 requires administrator
privileges (nonadministrators can listen only on ports higher than 1024), so if
we tried to connect to port 80 without being an administrator, binding wouldn’t
work. As another example, binding wouldn’t work if we ran two instances of our
program and so had two programs listening to the same port. Because we’re
writing a basic server just for learning purposes, we won’t worry about
handling these kinds of errors; instead, we use `unwrap` to stop the program if
errors happen.
-->

La fonction `bind` retourne un `Result<T, E>`, ce qui signifie que la création
de lien peut échouer. Par exemple, la connexion au port 80 nécessite d'être
administrateur (les utilisateurs non-administrateur ne peuvent écouter que sur
les ports supérieurs à 1024), donc si nous essayons de connecter un port 80
sans être administrateur, le lien ne va pas fonctionner. Un autre exemple, le
lien ne va pas fonctionner si nous exécutons deux instances de notre programme
et que nous avons deux programmes qui écoutent sur le même port. Comme nous
écrivons un serveur basique uniquement à but pédagogique, nous n'avons pas à
nous soucier de la gestion de ce genre d'erreur ; c'est pourquoi nous utilisons
`unwrap` pour arrêter l'exécution du programme si des erreurs arrivent.

<!--
The `incoming` method on `TcpListener` returns an iterator that gives us a
sequence of streams (more specifically, streams of type `TcpStream`). A single
*stream* represents an open connection between the client and the server. A
*connection* is the name for the full request and response process in which a
client connects to the server, the server generates a response, and the server
closes the connection. As such, `TcpStream` will read from itself to see what
the client sent and then allow us to write our response to the stream. Overall,
this `for` loop will process each connection in turn and produce a series of
streams for us to handle.
-->

La méthode `incoming` d'un `TcpListener` retourne l'itérateur qui nous donne une
séquence de flux (plus précisément, des flux de type `TcpStream`). Un seul
*flux* représente une connexion entre le client et le serveur. Une *connexion*
est le nom qui désigne tout le processus désignant la requête ainsi que la
réponse, durant lequel le client se connecte au serveur, le serveur génère une
réponse, et le serveur ferme la connexion. Ainsi, `TcpStream` va se lire
lui-même pour voir ce que le client a envoyé et nous permettre ensuite d'écrire
notre réponse dans le flux. De manière générale, cette boucle `for` traitera
chaque connexion dans l'ordre et produire nous une série de flux pour que nous
puissions les gérer.

<!--
For now, our handling of the stream consists of calling `unwrap` to terminate
our program if the stream has any errors; if there aren’t any errors, the
program prints a message. We’ll add more functionality for the success case in
the next listing. The reason we might receive errors from the `incoming` method
when a client connects to the server is that we’re not actually iterating over
connections. Instead, we’re iterating over *connection attempts*. The
connection might not be successful for a number of reasons, many of them
operating system specific. For example, many operating systems have a limit to
the number of simultaneous open connections they can support; new connection
attempts beyond that number will produce an error until some of the open
connections are closed.
-->

Pour l'instant, notre gestion des flux consiste à appeler `unwrap` pour arrêter
notre programme si le flux rencontre une erreur ; s'il n'y a pas d'erreurs, le
programme affiche un message. Nous allons ajouter plus de fonctionnalités dans
le cas de succès dans le prochain encart. La raison pour laquelle nous pourrions
recevoir des erreurs de la méthode `incoming` lorsqu'un client se connecte au
serveur est qu'en réalité nous n'itérons pas sur les connexions. En effet, nous
itérons sur des *tentatives de connexion*. La connexion peut échouer pour de
nombreuses raisons, beaucoup d'entre elles sont spécifiques au système
d'exploitation. Par exemple, de nombreux systèmes d'exploitation ont une limite
sur le nombre de connexions ouvertes simultanément qu'ils peuvent supporter ;
les tentatives de nouvelles connexions une fois ce nombre dépassé produiront une
erreur jusqu'à ce que certaines connexions soient fermées.

<!--
Let’s try running this code! Invoke `cargo run` in the terminal and then load
*127.0.0.1:7878* in a web browser. The browser should show an error message
like “Connection reset,” because the server isn’t currently sending back any
data. But when you look at your terminal, you should see several messages that
were printed when the browser connected to the server!
-->

Essayons d'exécuter ce code ! Saisissez `cargo run` dans le terminal et ensuite
ouvrez *127.0.0.1:7878* dans un navigateur web. Le navigateur devrait afficher
un message d'erreur comme “La connexion a été réinitialisée”, car le serveur ne
renvois pas de données pour le moment. Mais si vous regardez le terminal, vous
devriez voir quelques messages qui se sont affichés lorsque le navigateur s'est
connecté au serveur !

<!--
```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```
-->

```text
     Running `target/debug/salutations`
Connexion établie !
Connexion établie !
Connexion établie !
```

<!--
Sometimes, you’ll see multiple messages printed for one browser request; the
reason might be that the browser is making a request for the page as well as a
request for other resources, like the *favicon.ico* icon that appears in the
browser tab.
-->

Des fois, vous pourriez voir plusieurs messages s'afficher pour une seule
requête de navigateur ; la raison à cela est peut-être que le navigateur fait
une requête pour la page ainsi que des requêtes pour d'autres ressources, comme
l'icone *favicon.ico* qui s'affiche dans l'onglet du navigateur.

<!--
It could also be that the browser is trying to connect to the server multiple
times because the server isn’t responding with any data. When `stream` goes out
of scope and is dropped at the end of the loop, the connection is closed as
part of the `drop` implementation. Browsers sometimes deal with closed
connections by retrying, because the problem might be temporary. The important
factor is that we’ve successfully gotten a handle to a TCP connection!
-->

Peut-être que le navigateur essaye aussi de se connecter plusieurs fois au
serveur car le serveur ne répond aucune donnée. Lorsque `flux` sort de la portée
et est nettoyé à la fin de la boucle, la connexion est fermée car cela est
implémenté dans le `drop`. Les navigateurs réagissent à ces connexions fermées
en ré-essayant, car le problème peut être temporaire. La partie importante est
que nous avons obtenu avec succès un manipulateur de connexion TCP !

<!--
Remember to stop the program by pressing <span class="keystroke">ctrl-c</span>
when you’re done running a particular version of the code. Then restart `cargo
run` after you’ve made each set of code changes to make sure you’re running the
newest code.
-->

Souvenez-vous que vous pouvez arrêter le programme en appuyant sur
<span class="keystroke">ctrl-c</span> lorsque vous avez fini d'exécuter une
version du code. Relancez ensuite `cargo run` après avoir appliqué un jeu de
modifications pour vous assurer d'exécuter le nouveau code.

<!--
### Reading the Request
-->

### Lire la requête

<!--
Let’s implement the functionality to read the request from the browser! To
separate the concerns of first getting a connection and then taking some action
with the connection, we’ll start a new function for processing connections. In
this new `handle_connection` function, we’ll read data from the TCP stream and
print it so we can see the data being sent from the browser. Change the code to
look like Listing 20-2.
-->

Commençons à implémenter la fonctionnalité pour lire la requête du navigateur !
Pour séparer les parties où nous obtenons une connexion et celle où nous
agissons avec la connexion, nous allons créer une nouvelle fonction pour traiter
les connexions. Dans cette nouvelle fonction `gestion_connexion`, nous allons
lire des données provenant du flux TCP et les afficher afin que nous puissions
voir les données envoyées par le navigateur. Changez le code pour qu'il
ressemble à l'encart 20-2.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,no_run
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-02/src/main.rs}}
```
-->

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-02/src/main.rs}}
```

<!--
<span class="caption">Listing 20-2: Reading from the `TcpStream` and printing
the data</span>
-->

<span class="caption">Encart 20-2 : lecture du `TcpStream` et affichage des
données</span>

<!--
We bring `std::io::prelude` into scope to get access to certain traits that let
us read from and write to the stream. In the `for` loop in the `main` function,
instead of printing a message that says we made a connection, we now call the
new `handle_connection` function and pass the `stream` to it.
-->

Nous avons importé `std::io::prelude` dans la portée pour accéder à certains
traits qui nous permettent de lire et d'écrire dans le flux. Dans la boucle
`for` de la fonction `main`, au lieu d'afficher un message qui dit que nous
avons établi une connexion, nous faisons maintenant appel à `gestion_connexion`
et nous lui passons le `flux`.

<!--
In the `handle_connection` function, we’ve made the `stream` parameter mutable.
The reason is that the `TcpStream` instance keeps track of what data it returns
to us internally. It might read more data than we asked for and save that data
for the next time we ask for data. It therefore needs to be `mut` because its
internal state might change; usually, we think of “reading” as not needing
mutation, but in this case we need the `mut` keyword.
-->

Dans la fonction `gestion_connexion`, nous avons fait en sorte que le paramètre
`flux` soit mutable. La raison à cela est que l'instance de `TcpStream` garde en
mémoire interne quelles données il nous a retourné. Il peut avoir plus de
données que celles que nous avons demandé, et il peut alors conserver ces
données jusqu'à la prochaine fois où nous demanderons des données. Il doit donc
être `mut` car son état interne doit pouvoir changer ; d'habitude, nous n'avons
pas besoin que la “lecture” nécessite d'être mutable, mais dans ce cas nous
avons besoin du mot-clé `mut`.

<!--
Next, we need to actually read from the stream. We do this in two steps:
first, we declare a `buffer` on the stack to hold the data that is read in.
We’ve made the buffer 1024 bytes in size, which is big enough to hold the
data of a basic request and sufficient for our purposes in this chapter. If
we wanted to handle requests of an arbitrary size, buffer management would
need to be more complicated; we’ll keep it simple for now. We pass the buffer
to `stream.read`, which will read bytes from the `TcpStream` and put them in
the buffer.
-->

Ensuite, nous devons lire les données du flux. Nous faisons cela en deux
temps : d'abord, nous déclarons un `tampon` sur la pile pour y stocker les
données qui seront lues. Nous avons fait en sorte que le tampon fasse 1024
octets, ce qui est suffisamment grand pour stocker les données d'un requête
basique, ce qui est suffisant pour nos besoins dans ce chapitre. Si nous
aurions voulu gérer des requêtes de tailles quelconques, la gestion du tampon
aurait été plus complexe ; nous allons la garder simplifiée pour l'instant.
Nous envoyons le tampon dans `flux.read`, qui va lire les octets provenant du
`TcpStream` et les ajouter dans le tampon.

<!--
Second, we convert the bytes in the buffer to a string and print that string.
The `String::from_utf8_lossy` function takes a `&[u8]` and produces a `String`
from it. The “lossy” part of the name indicates the behavior of this function
when it sees an invalid UTF-8 sequence: it will replace the invalid sequence
with `�`, the `U+FFFD REPLACEMENT CHARACTER`. You might see replacement
characters for characters in the buffer that aren’t filled by request data.
-->

Ensuite, nous convertissons les octets présents dans le tampon en chaînes de
caractères et nous affichons cette chaîne de caractères. La fonction
`String::from_utf8_lossy` prends en paramètres un `&[u8]` le transforme en une
`String`. La partie “lossy” du nom indique le comportement de cette fonction
lorsqu'elle rencontre une séquence UTF-8 invalide : elle va remplacer la
séquence invalide par `�`, le caractère `U+FFFD REPLACEMENT CHARACTER`. Vous
devriez voir ces caractères de remplacement pour les caractères dans le
tampon qui ne correspondent pas aux données de la demande.

<!--
Let’s try this code! Start the program and make a request in a web browser
again. Note that we’ll still get an error page in the browser, but our
program’s output in the terminal will now look similar to this:
-->

Essayons ce code ! Démarrez le programme et faites à nouveau une requête dans
un navigateur web. Notez que nous obtenons toujours une page d'erreur dans le
navigateur web, mais que la sortie de notre programme dans le terminal devrait
ressembler à ceci :

<!--
```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101
Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
������������������������������������
```
-->

```console
$ cargo run
   Compiling salutations v0.1.0 (file:///projects/salutations)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/salutations`
Requête : GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101
Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
������������������������������������
```

<!--
Depending on your browser, you might get slightly different output. Now that
we’re printing the request data, we can see why we get multiple connections
from one browser request by looking at the path after `Request: GET`. If the
repeated connections are all requesting */*, we know the browser is trying to
fetch */* repeatedly because it’s not getting a response from our program.
-->

En fonction de votre navigateur, vous pourriez voir une sortie légèrement
différente. Maintenant que nous affichons les données des requêtes, nous
pouvons constater pourquoi nous obtenons pourquoi nous obtenons plusieurs
connexions pour un seul chargement de page dans le navigateur web en analysant
le chemin après le `Requête : GET`. Si les connexions répétées sont toutes vers
*/*, nous pouvons constater que le navigateur essaye d'obtenir */* à répétition
car il n'obtient pas de réponse de la part de notre programme.

<!--
Let’s break down this request data to understand what the browser is asking of
our program.
-->

Décomposons les données de cette requête pour comprendre ce que le navigateur
demande à notre programme.

<!--
### A Closer Look at an HTTP Request
-->

### Une analyse plus poussée d'une requête HTTP

<!--
HTTP is a text-based protocol, and a request takes this format:
-->

HTTP est un protocole basé sur du texte, et une requête doit suivre cette
forme :

<!--
```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```
-->

```text
Méthode URI-Demandée Version-HTTP CRLF
entêtes CRLF
corps-du-message
```

<!--
The first line is the *request line* that holds information about what the
client is requesting. The first part of the request line indicates the *method*
being used, such as `GET` or `POST`, which describes how the client is making
this request. Our client used a `GET` request.
-->

La première ligne est la *ligne de requête* qui contient les informations sur
ce que demande le client. La première partie de la ligne de requête indique la
*méthode* utilisée, comme `GET` ou `POST`, qui décrit comment le client fait sa
requête. Notre client a utilisé une requête `GET`.

<!--
The next part of the request line is */*, which indicates the *Uniform Resource
Identifier* *(URI)* the client is requesting: a URI is almost, but not quite,
the same as a *Uniform Resource Locator* *(URL)*. The difference between URIs
and URLs isn’t important for our purposes in this chapter, but the HTTP spec
uses the term URI, so we can just mentally substitute URL for URI here.
-->

La prochaine partie de la ligne de requête est */*, qui indique *l'URI*
*(Uniform Resource Identifier)* que demande le client : une URI est presque,
mais pas complètement, la même chose qu'une *URL* *(Uniform Resource Locator)*.
La différence entre les URI et les URL n'est pas très importante pour nos
besoins dans ce chapitre, mais la spécification de HTTP utilise le terme URI,
donc nous pouvons simplement remplacer URL par URI dans ce cas-ci.

<!--
The last part is the HTTP version the client uses, and then the request line
ends in a *CRLF sequence*. (CRLF stands for *carriage return* and *line feed*,
which are terms from the typewriter days!) The CRLF sequence can also be
written as `\r\n`, where `\r` is a carriage return and `\n` is a line feed. The
CRLF sequence separates the request line from the rest of the request data.
Note that when the CRLF is printed, we see a new line start rather than `\r\n`.
-->

La dernière partie est la version HTTP que le client utilise, et ensuite la
ligne de requête termine avec une *séquence CRLF* (CRLF signifie
*Carriage Return, retour chariot*, et *Line Feed, saut de ligne* qui sont des
termes qui remontent à l'époque des machines à écrire !). La séquence CRLF peut
aussi être écrite `\r\n`, dans laquelle `\r` est un retour chariot, et `\n` est
un saut de ligne. La séquence CRLF sépare la ligne de requête du reste des
données de la requête. Notez toutefois que lorsqu'un CRLF est affiché, nous
voyons une nouvelle ligne plutôt qu'un `\r\n`.

<!--
Looking at the request line data we received from running our program so far,
we see that `GET` is the method, */* is the request URI, and `HTTP/1.1` is the
version.
-->

D'après la ligne de requête que nous avons reçu après avoir exécuté notre
programme précédemment, nous constatons que la méthode est `GET`, */* est l'URI
demandée, et `HTTP/1.1` est la version.

<!--
After the request line, the remaining lines starting from `Host:` onward are
headers. `GET` requests have no body.
-->

Après la ligne de requête, les lignes après celle où nous avons `Host:` sont
des entêtes. Les requêtes `GET` n'ont pas de corps.

<!--
Try making a request from a different browser or asking for a different
address, such as *127.0.0.1:7878/test*, to see how the request data changes.
-->

Essayez de faire une requête dans un navigateur différent ou de demander une
adresse différente, comme *127.0.0.1:7878/test*, pour observer comment les
données de requête changent.

<!--
Now that we know what the browser is asking for, let’s send back some data!
-->

Maintenant que nous savons ce que demande le navigateur, envoyons-lui quelques
données !

<!--
### Writing a Response
-->

### Ecrire une réponse

<!--
Now we’ll implement sending data in response to a client request. Responses
have the following format:
-->

Maintenant, nous allons implémenter l'envoi d'une réponse à requête client. Les
réponses suivent le format suivant :

<!--
```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```
-->

```text
Version-HTTP Code-Statut Phrase-De-Raison CRLF
entêtes CRLF
corps-message
```

<!--
The first line is a *status line* that contains the HTTP version used in the
response, a numeric status code that summarizes the result of the request, and
a reason phrase that provides a text description of the status code. After the
CRLF sequence are any headers, another CRLF sequence, and the body of the
response.
-->

La première ligne est une *ligne de statut* qui contient la version HTTP
utilisée dans la réponse, un code numérique de statut qui résume le résultat
de la requête, et une phrase de raison qui fournit une description textuelle du
code de statut. Après la séquence CRLF viennent tous les entêtes, une autre
séquence CRLF, et enfin le corps de la réponse.

<!--
Here is an example response that uses HTTP version 1.1, has a status code of
200, an OK reason phrase, no headers, and no body:
-->

Voici un exemple de réponse qui utilise HTTP version 1.1, qui a un code de
statut de 200, une phrase de raison à OK, pas d'entêtes, et pas de corps :

<!--
```text
HTTP/1.1 200 OK\r\n\r\n
```
-->

```text
HTTP/1.1 200 OK\r\n\r\n
```

<!--
The status code 200 is the standard success response. The text is a tiny
successful HTTP response. Let’s write this to the stream as our response to a
successful request! From the `handle_connection` function, remove the
`println!` that was printing the request data and replace it with the code in
Listing 20-3.
-->

Le code de statut 200 est la réponse standard de succès. Le texte est une toute
petite réponse HTTP de succès. Ecrivons ceci dans le flux de notre réponse à
une requête avec succès ! Dans la fonction `gestion_connexion`, enlevez le
`println!` qui affiche les données de requête et remplacez-la par le code de
l'encart 20-3.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,no_run
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-03/src/main.rs:here}}
```
-->

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-3: Writing a tiny successful HTTP response to
the stream</span>
-->

<span class="caption">Encart 20-3 : écriture d'une toute petite réponse HTTP de
réussite dans le flux</span>

<!--
The first new line defines the `response` variable that holds the success
message’s data. Then we call `as_bytes` on our `response` to convert the string
data to bytes. The `write` method on `stream` takes a `&[u8]` and sends those
bytes directly down the connection.
-->

La première ligne définit la variable `reponse` qui contient les données du
message de réussite. Ensuite, nous faisons appel à `as_bytes` sur notre
`reponse` pour convertir la chaîne de caractères en octets. La méthode `write`
sur le `flux` prend en argument un `&[u8]` et envoie ces octets directement
dans la connexion.

<!--
Because the `write` operation could fail, we use `unwrap` on any error result
as before. Again, in a real application you would add error handling here.
Finally, `flush` will wait and prevent the program from continuing until all
the bytes are written to the connection; `TcpStream` contains an internal
buffer to minimize calls to the underlying operating system.
-->

Comme l'opération `write` peut échouer, nous utilisons `unwrap` sur toutes les
erreurs, comme précédemment. Encore une fois ; dans un véritable application,
vous devriez gérer les cas d'erreur ici. Enfin, `flush` va attendre et empêcher
le programme de continuer à s'exécuter jusqu'à ce que tous les octets soient
écrits dans la connexion ; `TcpStream` contient un tampon interne pour réduire
les appels au système d'exploitation concerné.

<!--
With these changes, let’s run our code and make a request. We’re no longer
printing any data to the terminal, so we won’t see any output other than the
output from Cargo. When you load *127.0.0.1:7878* in a web browser, you should
get a blank page instead of an error. You’ve just hand-coded an HTTP request
and response!
-->

Avec ces modifications, exécutons à nouveau notre code et lançons une requête
dans le navigateur. Nous n'affichons plus les données dans le terminal, donc
nous ne voyons plus aucune sortie autre que celle de Cargo. Lorsque vous
chargez *127.0.0.1:7878* dans un navigateur web, vous devriez obtenir une page
blanche plutôt qu'une erreur. Vous venez de coder en dur une réponse à une
requête HTTP !

<!--
### Returning Real HTML
-->

### Retourner du vrai HTML

<!--
Let’s implement the functionality for returning more than a blank page. Create
a new file, *hello.html*, in the root of your project directory, not in the
*src* directory. You can input any HTML you want; Listing 20-4 shows one
possibility.
-->

Implémentons la fonctionnalité pour retourner plus qu'une page blanche. Créez
un nouveau fichier, *hello.html*, à la racine de votre dossier de projet, et
non pas dans le dossier *src*. Vous pouvez ajouter le HTML que vous souhaitez ;
l'encart 20-4 vous montre une possibilité.

<!--
<span class="filename">Filename: hello.html</span>
-->

<span class="filename">Fichier : salutation.html</span>

<!--
```html
{{#include ../listings-sources/ch20-web-server/listing-20-04/hello.html}}
```
-->

```html
{{#include ../listings/ch20-web-server/listing-20-04/hello.html}}
```

<!--
<span class="caption">Listing 20-4: A sample HTML file to return in a
response</span>
-->

<span class="caption">Encart 20-4 : un exemple de fichier HTML à retourner dans
une réponse</span>

<!--
This is a minimal HTML5 document with a heading and some text. To return this
from the server when a request is received, we’ll modify `handle_connection` as
shown in Listing 20-5 to read the HTML file, add it to the response as a body,
and send it.
-->

Ceci est un document HTML5 minimal avec des entêtes et un peu de texte. Pour
retourner ceci à partir d'un serveur lorsqu'une requête est reçue, nous allons
modifier `gestion_connexion` comme proposé dans l'encart 20-5 pour lire le
fichier HTML, l'ajouter dans la réponse comme faisant partie de son corps, et
l'envoyer.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,no_run
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-05/src/main.rs:here}}
```
-->

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-5: Sending the contents of *hello.html* as the
body of the response</span>
-->

<span class="caption">Encart 20-5 : envoi du contenu de *salutation.html* dans
le corps de la réponse</span>

<!--
We’ve added a line at the top to bring the standard library’s filesystem module
into scope. The code for reading the contents of a file to a string should look
familiar; we used it in Chapter 12 when we read the contents of a file for our
I/O project in Listing 12-4.
-->

Nous avons ajouté une ligne en haut pour importer le module de système de
fichiers de la bibliothèque standard. Le code pour lire le contenu d'un fichier
dans une `String` devrait vous être familier ; nous n'avons utilisé dans le
chapitre 12 lorsque nous lisions le contenu d'un fichier pour notre projet
d'entrée/sortie, dans l'encart 12-4.

<!--
Next, we use `format!` to add the file’s contents as the body of the success
response. To ensure a valid HTTP response, we add the `Content-Length` header
which is set to the size of our response body, in this case the size of `hello.html`.
-->

Ensuite, nous avons utilisé `format!` pour ajouter le contenu du fichier comme
étant le corps de la réponse avec succès. Pour garantir que ce soit une réponse
HTTP valide, nous avons ajouté l'entête `Content-Length` qui définit la taille
du corps de notre réponse, qui dans ce cas est la taille de `hello.html`.

<!--
Run this code with `cargo run` and load *127.0.0.1:7878* in your browser; you
should see your HTML rendered!
-->

Exécutez ce code avec `cargo run` et ouvrez *127.0.0.1:7878* dans votre
navigateur web ; vous devriez voir le résultat de votre HTML !

<!--
Currently, we’re ignoring the request data in `buffer` and just sending back
the contents of the HTML file unconditionally. That means if you try requesting
*127.0.0.1:7878/something-else* in your browser, you’ll still get back this
same HTML response. Our server is very limited and is not what most web servers
do. We want to customize our responses depending on the request and only send
back the HTML file for a well-formed request to */*.
-->

Pour le moment, nous ignorons les données de la requête présentes dans
`tampon` et nous renvoyons sans conditions le contenu du fichier HTML. Cela
signifie que si vous essayez de demander *127.0.0.1:7878/autre-chose* dans
votre navigateur web, vous obtiendrez la même réponse HTML. Notre serveur est
très limité, et ne correspond pas à ce que font la plupart des serveurs web.
Nous souhaitons désormais personnaliser nos réponses en fonction de la requête
et renvoyer seulement le fichier HTML pour la bonne requête faite à */*.

<!--
### Validating the Request and Selectively Responding
-->

### Valider la requête et répondre de manière sélective

<!--
Right now, our web server will return the HTML in the file no matter what the
client requested. Let’s add functionality to check that the browser is
requesting */* before returning the HTML file and return an error if the
browser requests anything else. For this we need to modify `handle_connection`,
as shown in Listing 20-6. This new code checks the content of the request
received against what we know a request for */* looks like and adds `if` and
`else` blocks to treat requests differently.
-->

Jusqu'à présent, notre serveur web devrait retourner le HTML du fichier peu
importe ce que demande le client. Ajoutons une fonctionnalité pour vérifier que
le navigateur demande bien */* avant de retourner le fichier HTML et retournons
une erreur si le navigateur demande autre chose. Pour cela, nous devons
modifier `gestion_connexion` comme l'encart 20-6. Ce nouveau code compare le
contenu de la requête que nous recevons à une requête que nous voudrions pour
*/* en ajoutant des blocs `if` et `else` pour traiter différemment les requêtes.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,no_run
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-06/src/main.rs:here}}
```
-->

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-6: Matching the request and handling requests
to */* differently from other requests</span>
-->

<span class="caption">Encart 20-6 : détection et gestion des requêtes vers */*,
qui est différente en fonction des autres requêtes</span>

<!--
First, we hardcode the data corresponding to the */* request into the `get`
variable. Because we’re reading raw bytes into the buffer, we transform `get`
into a byte string by adding the `b""` byte string syntax at the start of the
content data. Then we check whether `buffer` starts with the bytes in `get`. If
it does, it means we’ve received a well-formed request to */*, which is the
success case we’ll handle in the `if` block that returns the contents of our
HTML file.
-->

D'abord, nous codons en dur les données correspondant à la requête */* dans la
variable `get`. Comme nous lisons des octets bruts provenant du tampon, nous
transformons `get` en une chaîne d'octets en ajoutant la syntaxe de chaîne
d'octets `b""` au début du contenu des données. Ensuite, nous attendons le
moment où le `tampon` commence par les mêmes octets que ceux dans `get`.
Lorsque c'est le cas, cela signifie que nous avons reçu une requête
correctement bien formulée vers */*, qui est le cas de réussite que nous allons
gérer dans le bloc `if` qui retourne le contenu de notre fichier HTML.

<!--
If `buffer` does *not* start with the bytes in `get`, it means we’ve received
some other request. We’ll add code to the `else` block in a moment to respond
to all other requests.
-->

Si `tampon` ne *commence pas* avec les octets présents dans `get`, cela
signifie que nous avons reçu une autre requête. Nous allons bientôt ajouter du
code au bloc `else` pour répondre à toutes les autres requêtes.

<!--
Run this code now and request *127.0.0.1:7878*; you should get the HTML in
*hello.html*. If you make any other request, such as
*127.0.0.1:7878/something-else*, you’ll get a connection error like those you
saw when running the code in Listing 20-1 and Listing 20-2.
-->

Exécutez ce code maintenant et demandez *127.0.0.1:7878* ; vous devriez obtenir
le HTML de *salutation.html*. Si vous faites n'importe quelle autre requête,
comme *127.0.0.1:7878/autre-chose*, vous allez obtenir une erreur de connexion
comme celle que vous avez vu lorsque vous exécutiez le code l'encart 20-1 et de
l'encart 20-2.

<!--
Now let’s add the code in Listing 20-7 to the `else` block to return a response
with the status code 404, which signals that the content for the request was
not found. We’ll also return some HTML for a page to render in the browser
indicating the response to the end user.
-->

Maintenant ajoutons le code de l'encart 20-7 au bloc `else` pour retourner une
réponse avec le code de statut 404, qui signale que le contenu demandé par
cette requête n'est pas trouvé. Nous allons aussi retourner du HTML pour qu'une
page s'affiche dans le navigateur, indiquant la réponse à l'utilisateur final.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,no_run
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-07/src/main.rs:here}}
```
-->

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-07/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-7: Responding with status code 404 and an
error page if anything other than */* was requested</span>
-->

<span class="caption">Encart 20-7 : répondre un code de statut 404 et une page
d'erreur s'il y autre chose que */* qui est demandé</span>

<!--
Here, our response has a status line with status code 404 and the reason
phrase `NOT FOUND`. We’re still not returning headers, and the body of the
response will be the HTML in the file *404.html*. You’ll need to create a
*404.html* file next to *hello.html* for the error page; again feel free to use
any HTML you want or use the example HTML in Listing 20-8.
-->

Ici, notre réponse a une ligne de statut avec le code de statut 404 et la
phrase de raison `NOT FOUND`. Nous ne retournons toujours pas d'entêtes, et le
corps de la réponse sera le HTML présent dans le fichier *404.html*. Nous aurons
besoin de créer un fichier `404.html` à côté de *salutation.html* pour la page
d'erreur ; n'hésitez pas à nouveau à utiliser le HTML que vous souhaitez ou à
défaut utiliser le HTML d'exemple présent dans l'encart 20-8.

<!--
<span class="filename">Filename: 404.html</span>
-->

<span class="filename">Fichier : 404.html</span>

<!--
```html
{{#include ../listings-sources/ch20-web-server/listing-20-08/404.html}}
```
-->

```html
{{#include ../listings/ch20-web-server/listing-20-08/404.html}}
```

<!--
<span class="caption">Listing 20-8: Sample content for the page to send back
with any 404 response</span>
-->

<span class="caption">Encart 20-8 : contenu d'exemple pour la page à renvoyer
avec les réponses 404</span>

<!--
With these changes, run your server again. Requesting *127.0.0.1:7878*
should return the contents of *hello.html*, and any other request, like
*127.0.0.1:7878/foo*, should return the error HTML from *404.html*.
-->

Une fois ces modifications appliquées, exécutez à nouveau votre serveur. Les
requêtes vers *127.0.0.1:7878* devraient retourner le contenu de
*salutation.html*, et toutes les autres requêtes, comme
*127.0.0.1:7878/autre-chose*, devraient retourner le HTML d'erreur présent dans
*404.html*.

<!--
### A Touch of Refactoring
-->

### Un peu de remaniement

<!--
At the moment the `if` and `else` blocks have a lot of repetition: they’re both
reading files and writing the contents of the files to the stream. The only
differences are the status line and the filename. Let’s make the code more
concise by pulling out those differences into separate `if` and `else` lines
that will assign the values of the status line and the filename to variables;
we can then use those variables unconditionally in the code to read the file
and write the response. Listing 20-9 shows the resulting code after replacing
the large `if` and `else` blocks.
-->

Pour l'instant, les blocs `if` et `else` contiennent beaucoup de code répété :
ils lisent tous les deux des fichiers et écrivent le contenu de ces fichiers
dans le flux. La seule différence entre eux sont la ligne de statut et le nom
du fichier. Rendons le code plus concis en isolant ces différences dans des
lignes `if` et `else` qui vont assigner les valeurs de la ligne de statut et du
nom de fichier à des variables ; nous pourrons ensuite utiliser ces variables
sans avoir à se préoccuper du contexte dans du code qui va lire le fichier et
écrire la réponse. L'encart 20-9 montre le code qui découle du remplacement des
gros blocs `if` et `else`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,no_run
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-09/src/main.rs:here}}
```
-->

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-9: Refactoring the `if` and `else` blocks to
contain only the code that differs between the two cases</span>
-->

<span class="caption">Encart 20-9 : remaniement des blocs `if` et `else` pour
qu'ils contiennent uniquement le code qui différencie les deux cas</span>

<!--
Now the `if` and `else` blocks only return the appropriate values for the
status line and filename in a tuple; we then use destructuring to assign these
two values to `status_line` and `filename` using a pattern in the `let`
statement, as discussed in Chapter 18.
-->

Maintenant que les blocs `if` et `else` retournent uniquement les valeurs
correctes pour la ligne de statut et le nom du fichier dans un tuple ; nous
pouvons utiliser la déstructuration pour assigner ces deux valeurs à
`ligne_statut` et `nom_fichier` en utilisant un motif dans l'instruction `let`,
comme nous l'avons vu dans le chapitre 18.

<!--
The previously duplicated code is now outside the `if` and `else` blocks and
uses the `status_line` and `filename` variables. This makes it easier to see
the difference between the two cases, and it means we have only one place to
update the code if we want to change how the file reading and response writing
work. The behavior of the code in Listing 20-9 will be the same as that in
Listing 20-8.
-->

Le code précédent qui était en double se trouve maintenant à l'extérieur des
blocs `if` et `else` et utilise les variables `ligne_statut` et `nom_fichier`.
Cela permet de mettre en évidence plus facilement les différences entre les
deux cas, et cela signifie que nous n'avons qu'un seul endroit du code à
modifier si nous souhaitons changer le fonctionnement de lecture du fichier et
d'écriture de la réponse. Le comportement du code de l'encart 20-9 devrait être
identique à celui de l'encart 20-8.

<!--
Awesome! We now have a simple web server in approximately 40 lines of Rust code
that responds to one request with a page of content and responds to all other
requests with a 404 response.
-->

Super ! Nous avons maintenant un serveur web simple qui tient dans environ 40
lignes de code, qui répond à une requête précise par sa page de contenu et
répond à toutes les autres avec une réponse 404.

<!--
Currently, our server runs in a single thread, meaning it can only serve one
request at a time. Let’s examine how that can be a problem by simulating some
slow requests. Then we’ll fix it so our server can handle multiple requests at
once.
-->

Actuellement, notre serveur fonctionne dans une seule tâche, ce qui signifie
qu'il ne peut répondre qu'à une seule requête à la fois. Examinons maintenant
à quel point cela peut être un problème en simulant des réponses lentes à des
requêtes. Ensuite, nous corrigerons notre serveur pour qu'il puisse gérer
plusieurs requêtes à la fois.
