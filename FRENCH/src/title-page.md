<!--
# The Rust Programming Language
-->

# Le langage de programmation Rust

<!--
*by Steve Klabnik and Carol Nichols, with contributions from the Rust Community*
-->

*par Steve Klabnik et Carol Nichols, avec la participation de la Communauté
Rust*

<!--
This version of the text assumes you’re using Rust 1.37.0 or later with
`edition="2018"` in *Cargo.toml* of all projects to use Rust 2018 Edition
idioms. See the [“Installation” section of Chapter 1][install]<!-- ignore -- >
to install or update Rust, and see the new [Appendix E][editions]<!-- ignore
-- > for information on editions.
-->

Cette version du document suppose que vous utilisez Rust 1.37.0 ou ultérieur
avec `edition="2018"` dans *Cargo.toml* de tous les projets afin d'utiliser les
expressions idiomatiques de l'édition 2018 de Rust.
Voir la [section “Installation” du chapitre 1][install]<!-- ignore -->
pour installer ou mettre à jour Rust, et
voir la nouvelle [annexe E][editions]<!-- ignore --> pour plus d'informations
sur les éditions.

<!--
The 2018 Edition of the Rust language includes a number of improvements that
make Rust more ergonomic and easier to learn. This iteration of the book
contains a number of changes to reflect those improvements:
-->

L'édition 2018 du langage Rust apporte quelques améliorations qui rendent Rust
plus ergonomique et plus facile à apprendre. Cette version du livre comprend un
certain nombre de changements pour refléter ces améliorations :

<!--
- Chapter 7, “Managing Growing Projects with Packages, Crates, and Modules,”
  has been mostly rewritten. The module system and the way paths work in the
  2018 Edition were made more consistent.
- Chapter 10 has new sections titled “Traits as Parameters” and “Returning
  Types that Implement Traits” that explain the new `impl Trait` syntax.
- Chapter 11 has a new section titled “Using `Result<T, E>` in Tests” that
  shows how to write tests that use the `?` operator.
- The “Advanced Lifetimes” section in Chapter 19 was removed because compiler
  improvements have made the constructs in that section even rarer.
- The previous Appendix D, “Macros,” has been expanded to include procedural
  macros and was moved to the “Macros” section in Chapter 19.
- Appendix A, “Keywords,” also explains the new raw identifiers feature that
  enables code written in the 2015 Edition and the 2018 Edition to interoperate.
- Appendix D is now titled “Useful Development Tools” and covers recently
  released tools that help you write Rust code.
- We fixed a number of small errors and imprecise wording throughout the book.
  Thank you to the readers who reported them!
-->

- Le chapitre 7 a été réécrit en grande partie. Le système de modules et le
  fonctionnement des chemins dans l'édition 2018 ont été rendus plus cohérents.
- Le chapitre 10 a deux nouvelles parties qui expliquent la nouvelle syntaxe
  `impl Trait`.
- Le chapitre 11 a une nouvelle partie qui montre comment écrire des tests qui
  utilisent l'opérateur `?`.
- La partie "Durée de vie avancée" du chapitre 19 a été retirée, car les
  améliorations du compilateur ont rendu encore plus rare les concepts de cette
  section.
- L'ancienne Annexe D a été complétée pour couvrir les macros procédurales et a
  été déplacée dans la section "Macros" du chapitre 19.
- L'annexe A explique aussi la nouvelle fonctionnalité d'identificateurs bruts
  qui rend du code écrit avec l'édition 2015 interopérable avec l'édition 2018.
- L'annexe D a été renommée et couvre les nouveaux outils ajoutés récemment
  pour vous aider à écrire du code Rust.
- Nous avons corrigé quelques petites erreurs et certaines formulations floues
  dans tout le livre. Merci aux lecteurs qui nous les ont signalées !

<!--
Note that any code in earlier iterations of *The Rust Programming Language*
that compiled will continue to compile without `edition="2018"` in the
project’s *Cargo.toml*, even as you update the Rust compiler version you’re
using. That’s Rust’s backward compatibility guarantees at work!
-->

Veuillez noter que tout le code dans les versions précédentes du livre
*Le langage de programmation Rust* qui se compilait va continuer à compiler sans
`edition="2018"` dans le *Cargo.toml* du projet, même si vous mettez à jour le
compilateur Rust que vous utilisez. C'est la garantie de la rétrocompatibilité
de Rust qui est à l'œuvre ici !

<!--
The HTML format is available online at
[https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)
and offline with installations of Rust made with `rustup`; run `rustup docs
--book` to open.
-->

Le format HTML de la version anglaise est disponible en ligne à l'adresse
[https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)
et en hors-ligne avec l'installation de Rust qui a été effectuée avec `rustup` ;
vous pouvez lancer `rustup docs --book` pour l'ouvrir.

<!--
This text is available in [paperback and ebook format from No Starch
Press][nsprust].
-->

La version anglaise de ce livre est disponible
[au format papier et e-book chez No Starch Press][nsprust].

[install]: ch01-01-installation.html
[editions]: appendix-05-editions.html
[nsprust]: https://nostarch.com/rust
