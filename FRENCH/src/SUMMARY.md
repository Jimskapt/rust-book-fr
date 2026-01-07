# Le langage de programmation Rust

[Le langage de programmation Rust](title-page.md)
[Avant-propos](foreword.md)
[Introduction](ch00-00-introduction.md)
[Traduction des termes](translation-terms.md)

## Prise en main

- [Prise en main](ch01-00-getting-started.md)
    - [Installation](ch01-01-installation.md)
    - [Hello, world!](ch01-02-hello-world.md)
    - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [Programmer le jeu du plus ou du moins](ch02-00-guessing-game-tutorial.md)

- [Les concepts courants de programmation](ch03-00-common-programming-concepts.md)
    - [Les variables et la mutabilité](ch03-01-variables-and-mutability.md)
    - [Les types de données](ch03-02-data-types.md)
    - [Les fonctions](ch03-03-how-functions-work.md)
    - [Les commentaires](ch03-04-comments.md)
    - [Les structures de contrôle](ch03-05-control-flow.md)

- [Comprendre la possession](ch04-00-understanding-ownership.md)
    - [Qu'est-ce que la possession ?](ch04-01-what-is-ownership.md)
    - [Les références et l'emprunt](ch04-02-references-and-borrowing.md)
    - [Le type slice](ch04-03-slices.md)

- [Utiliser les structures pour structurer des données apparentées](ch05-00-structs.md)
    - [Définir et instancier des structures](ch05-01-defining-structs.md)
    - [Un exemple de programme qui utilise des structures](ch05-02-example-structs.md)
    - [La syntaxe des méthodes](ch05-03-method-syntax.md)

- [Les énumérations et le filtrage par motif](ch06-00-enums.md)

    - [Définir une énumération](ch06-01-defining-an-enum.md)
    - [La structure de contrôle `match`](ch06-02-match.md)
    - [Une structure de contrôle concise : `if let`](ch06-03-if-let.md)

## Connaissances de base sur Rust

- [Gérer des projets grandissants avec les paquets, crates et modules](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [Les paquets et les crates](ch07-01-packages-and-crates.md)
    - [Définir des modules pour gérer la portée et la visibilité](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [Désigner un élément dans l'arborescence de modules](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [Importer des chemins dans la portée via le mot-clé `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [Séparer les modules dans différents fichiers](ch07-05-separating-modules-into-different-files.md)

- [Les collections standard](ch08-00-common-collections.md)
    - [Stocker des listes de valeurs avec des vecteurs](ch08-01-vectors.md)
    - [Stocker du texte encodé en UTF-8 avec les Strings](ch08-02-strings.md)
    - [Stocker des clés associées à des valeurs dans des tables de hachage](ch08-03-hash-maps.md)

- [La gestion des erreurs](ch09-00-error-handling.md)
    - [Les erreurs irrécupérables avec `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
    - [Des erreurs récupérables avec `Result`](ch09-02-recoverable-errors-with-result.md)
    - [Paniquer ou ne pas paniquer, telle est la question](ch09-03-to-panic-or-not-to-panic.md)

- [Les types génériques, les traits et les durées de vie](ch10-00-generics.md)
    - [Les types de données génériques](ch10-01-syntax.md)
    - [Définir des comportements partagés avec les traits](ch10-02-traits.md)
    - [La conformité des références avec les durées de vies](ch10-03-lifetime-syntax.md)

- [Écrire des tests automatisés](ch11-00-testing.md)
    - [Comment écrire des tests](ch11-01-writing-tests.md)
    - [Gérer l'exécution des tests](ch11-02-running-tests.md)
    - [L'organisation des tests](ch11-03-test-organization.md)

- [Un projet d'entrée/sortie : construire un programme en ligne de commande](ch12-00-an-io-project.md)
    - [Récupérer les arguments de la ligne de commande](ch12-01-accepting-command-line-arguments.md)
    - [Lire un fichier](ch12-02-reading-a-file.md)
    - [Remanier le code pour améliorer sa modularité et la gestion des erreurs](ch12-03-improving-error-handling-and-modularity.md)
    - [Développer les fonctionnalités de la bibliothèque avec le TDD](ch12-04-testing-the-librarys-functionality.md)
    - [Travailler avec des variables d'environnement](ch12-05-working-with-environment-variables.md)
    - [Écrire les messages d'erreur sur la sortie d'erreurs standard au lieu de la sortie normale](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Repenser les choses en Rust

- [Les fonctionnalités des langages fonctionnels : les itérateurs et les fermetures](ch13-00-functional-features.md)
    - [Les fermetures : fonctions anonymes qui peuvent utiliser leur environnement](ch13-01-closures.md)
    - [Traiter une série d'éléments avec un itérateur](ch13-02-iterators.md)
    - [Amélioration de notre projet d'entrée/sortie](ch13-03-improving-our-io-project.md)
    - [Comparaison des performances : les boucles et les itérateurs](ch13-04-performance.md)

- [En savoir plus sur cargo et crates.io](ch14-00-more-about-cargo.md)
    - [Personnaliser les compilations avec les profils de publication](ch14-01-release-profiles.md)
    - [Publier une crate sur crates.io](ch14-02-publishing-to-crates-io.md)
    - [Les espaces de travail de cargo](ch14-03-cargo-workspaces.md)
    - [Installer des binaires à partir de crates.io avec `cargo install`](ch14-04-installing-binaries.md)
    - [Etendre les fonctionnalités de cargo avec des commandes personnalisées](ch14-05-extending-cargo.md)

- [Les pointeurs intelligents](ch15-00-smart-pointers.md)
    - [Utiliser `Box<T>` pour pointer sur des données présentes sur le tas](ch15-01-box.md)
    - [Considérer les pointeurs intelligents comme des références grâce au trait `Deref`](ch15-02-deref.md)
    - [Exécuter du code au nettoyage avec le trait `Drop`](ch15-03-drop.md)
    - [`Rc<T>`, le pointeur intelligent qui compte les références](ch15-04-rc.md)
    - [`RefCell<T>` et le motif de mutabilité interne](ch15-05-interior-mutability.md)
    - [Les boucles de références qui peuvent provoquer des fuites de mémoire](ch15-06-reference-cycles.md)

- [La concurrence sans craintes](ch16-00-concurrency.md)
    - [Utiliser les tâches pour exécuter simultanément du code](ch16-01-threads.md)
    - [Utiliser l'envoi de messages pour transférer des données entre les tâches](ch16-02-message-passing.md)
    - [Le partage d'état en concurrence](ch16-03-shared-state.md)
    - [Etendre la concurrence avec les traits `Sync` et `Send`](ch16-04-extensible-concurrency-sync-and-send.md)

- [Les fonctionnalités orientées objet de Rust](ch17-00-oop.md)
    - [Les caractéristiques des langages orientés objet](ch17-01-what-is-oo.md)
    - [Utiliser les objets traits qui permettent des valeurs de types différents](ch17-02-trait-objects.md)
    - [Implémenter un patron de conception orienté-objet](ch17-03-oo-design-patterns.md)

## Sujets avancés

- [Les motifs et le filtrage par motif](ch18-00-patterns.md)
    - [Tous les endroits où les motifs peuvent être utilisés](ch18-01-all-the-places-for-patterns.md)
    - [La réfutabilité : lorsqu'un motif peut échouer à correspondre](ch18-02-refutability.md)
    - [La syntaxe des motifs](ch18-03-pattern-syntax.md)

- [Les fonctionnalités avancées](ch19-00-advanced-features.md)
    - [Le Rust non sécurisé (`unsafe`)](ch19-01-unsafe-rust.md)
    - [Les traits avancés](ch19-03-advanced-traits.md)
    - [Les types avancés](ch19-04-advanced-types.md)
    - [Les fonctions et fermetures avancées](ch19-05-advanced-functions-and-closures.md)
    - [Les macros](ch19-06-macros.md)

- [Projet final : construire un serveur web multitâches](ch20-00-final-project-a-web-server.md)
    - [Développer un serveur web monotâche](ch20-01-single-threaded.md)
    - [Transformer notre serveur monotâche en serveur multitâches](ch20-02-multithreaded.md)
    - [Arrêt propre et nettoyage](ch20-03-graceful-shutdown-and-cleanup.md)

- [Annexes](appendix-00.md)
    - [A - les mots-clés](appendix-01-keywords.md)
