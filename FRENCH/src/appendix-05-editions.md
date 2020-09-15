<!--
## Appendix E - Editions
-->

## Annexe E - Les éditions

<!--
In Chapter 1, you saw that `cargo new` adds a bit of metadata to your
*Cargo.toml* file about an edition. This appendix talks about what that means!
-->

Au chapitre 1, vous avez constaté que `cargo new` ajoutait une petite métadonnée
à propos d'une édition dans votre fichier *Cargo.toml*. Cette annexe vous
explique ce que cela signifie !

<!--
The Rust language and compiler have a six-week release cycle, meaning users get
a constant stream of new features. Other programming languages release larger
changes less often; Rust releases smaller updates more frequently. After a
while, all of these tiny changes add up. But from release to release, it can be
difficult to look back and say, “Wow, between Rust 1.10 and Rust 1.31, Rust has
changed a lot!”
-->

Le langage Rust et son compilateur suivent un cycle de publication de six
semaines, ce qui signifie que leurs utilisateurs suivent un flux constant de
nouvelles fonctionnalités. Les autres langages de programmation publient moins
souvent des changements mais qui sont plus gros ; Rust a fait le choix de
publier des petits changements plus fréquemment. Au bout d'un certain moment,
tous ces petits changements s'accumulent. Mais de mise à jour en mise à jour, il
devient difficile de regarder en arrière et de dire : “Ouah, Rust a beaucoup
changé entre Rust 1.10 et Rust 1.31”.

<!--
Every two or three years, the Rust team produces a new Rust *edition*. Each
edition brings together the features that have landed into a clear package with
fully updated documentation and tooling. New editions ship as part of the usual
six-week release process.
-->

Tous les deux ou trois ans, l'équipe Rust produit une nouvelle *édition* de
Rust. Chaque édition rassemble des fonctionnalités qui ont convergé en un
ensemble clair, avec une documentation et des outils complètement à jour. Les
nouvelles éditions sont livrées comme faisant partie du cycle habituel de
publication toutes les six semaines.

<!--
Editions serve different purposes for different people:
-->

Les éditions apportent différentes choses pour différentes personnes :

<!--
* For active Rust users, a new edition brings together incremental changes into
  an easy-to-understand package.
* For non-users, a new edition signals that some major advancements have
  landed, which might make Rust worth another look.
* For those developing Rust, a new edition provides a rallying point for the
  project as a whole.
-->

* Pour les utilisateurs actifs de Rust, une nouvelle édition regroupe les
  différents changements progressifs dans un ensemble clair.
* Pour ceux qui n'utilisent pas Rust, une nouvelle édition signale la livraison
  d'avancées majeures, qui pourrait être le signal que Rust mériterait un
  nouveau coup d'œil.
* Pour ceux qui développent Rust, une nouvelle édition est un point de
  ralliement pour l'ensemble du projet.

<!--
At the time of this writing, two Rust editions are available: Rust 2015 and
Rust 2018. This book is written using Rust 2018 edition idioms.
-->

Au moment de cette écriture, deux éditions de Rust sont disponibles : Rust 2015
et Rust 2018. Ce livre est écrit selon les termes de l'édition Rust 2018.

<!--
The `edition` key in *Cargo.toml* indicates which edition the compiler should
use for your code. If the key doesn’t exist, Rust uses `2015` as the edition
value for backward compatibility reasons.
-->

La clé `edition` dans *Cargo.toml* indique quelle édition le compilateur doit
utiliser dans votre code. Si la clé n'existe pas, Rust utilise `2015` comme
valeur de l'édition, pour des raisons de rétro-compatibilité.

<!--
Each project can opt in to an edition other than the default 2015 edition.
Editions can contain incompatible changes, such as including a new keyword that
conflicts with identifiers in code. However, unless you opt in to those
changes, your code will continue to compile even as you upgrade the Rust
compiler version you use.
-->

Chaque projet peut opter pour une autre édition que l'édition 2015 par défaut.
Les éditions peuvent impliquer des changements incompatibles, comme
l'introduction d'un nouveau mot-clé qui rentre en conflit avec des
identificateurs (noms de variables, de fonctions, ...) utilisés dans le code.
Cependant, à moins que vous ne décidiez d'opter pour ces changements, votre code
va continuer à se compiler même si vous augmentez la version du compilateur Rust
que vous utilisez.

<!--
All Rust compiler versions support any edition that existed prior to that
compiler’s release, and they can link crates of any supported editions
together. Edition changes only affect the way the compiler initially parses
code. Therefore, if you’re using Rust 2015 and one of your dependencies uses
Rust 2018, your project will compile and be able to use that dependency. The
opposite situation, where your project uses Rust 2018 and a dependency uses
Rust 2015, works as well.
-->

Toutes les versions du compilateur Rust supporte toutes les éditions qui ont
existé avant la publication courante du compilateur, et ils peuvent lier
ensemble les crates de n'importe quelle édition supportée. Les changements de
chaque édition changent uniquement la façon dont le compilateur interprète
initialement le code. Par conséquent, si vous utilisez Rust 2015 et qu'une de
vos dépendances utilise Rust 2018, votre programme va se compiler et être
capable d'utiliser cette dépendance. La situation inverse, dans laquelle votre
projet utilise Rust 2018 et qu'une dépendance utilise Rust 2015, va aussi
fonctionner.

<!--
To be clear: most features will be available on all editions. Developers using
any Rust edition will continue to see improvements as new stable releases are
made. However, in some cases, mainly when new keywords are added, some new
features might only be available in later editions. You will need to switch
editions if you want to take advantage of such features.
-->

En clair : la plupart des fonctionnalités seront disponibles sur toutes les
versions. Les développeurs qui utilisent n'importe quelle édition de Rust vont
continuer à constater des améliorations au fur et à mesure que des nouvelles
éditions stables sont publiées. Cependant, dans certains cas, principalement
lorsque des nouveaux mot-clés serons rajoutés, certaines nouvelles
fonctionnalités ne seront disponibles que dans les nouvelles éditions. Vous
aurez alors besoin de changer d'édition si vous souhaitez profiter des avantages
de ces fonctionnalités.

<!--
For more details, the [*Edition
Guide*](https://doc.rust-lang.org/stable/edition-guide/) is a complete book
about editions that enumerates the differences between editions and explains
how to automatically upgrade your code to a new edition via `cargo fix`.
-->

Pour en savoir plus, le
[*Edition Guide*](https://doc.rust-lang.org/stable/edition-guide/) est un livre
complet sur les éditions, qui énumère les différences entre les éditions et qui
explique comment mettre à jour automatiquement votre code vers une nouvelle
édition via `cargo fix`.
