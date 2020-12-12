<!--
# Advanced Features
-->

# Les fonctionnalités avancées

<!--
By now, you’ve learned the most commonly used parts of the Rust programming
language. Before we do one more project in Chapter 20, we’ll look at a few
aspects of the language you might run into every once in a while. You can use
this chapter as a reference for when you encounter any unknowns when using
Rust. The features you’ll learn to use in this chapter are useful in very
specific situations. Although you might not reach for them often, we want to
make sure you have a grasp of all the features Rust has to offer.
-->

Jusqu'ici, vous avez appris les fonctionnalités les plus utilisées du langage
de programmation Rust. Avant de commencer le nouveau projet du chapitre 20, nous
allons regarder quelques aspects du langage que vous pourriez rencontrer de
temps à autre. Vous pouvez utiliser ce chapitre comme étant une référence
lorsque vous rencontrerez des éléments inconnus lorsque vous utiliserez Rust.
Les fonctionnalités que vous allez découvrir dans ce chapitre sont utiles dans
de nombreuses situations spécifiques. Même si vous n'allez pas les rencontrer
très souvent, nous voulons nous assurer que vous comprenez bien toutes les
fonctionnalités qu'offrent Rust.

<!--
In this chapter, we’ll cover:
-->

Dans ce chapitre, nous allons voir :

<!--
* Unsafe Rust: how to opt out of some of Rust’s guarantees and take
  responsibility for manually upholding those guarantees
* Advanced traits: associated types, default type parameters, fully qualified
  syntax, supertraits, and the newtype pattern in relation to traits
* Advanced types: more about the newtype pattern, type aliases, the never type,
  and dynamically sized types
* Advanced functions and closures: function pointers and returning closures
* Macros: ways to define code that defines more code at compile time
-->

* Le *unsafe* de Rust : comment désactiver certaines garanties de Rust et
  prendre la responsabilité de veiller manuellement à ces garanties
* Les traits avancés : les types associés, les types de paramètres par défaut,
  la syntaxe entièrement détaillée, les supertraits, et le motif newtype en lien
  avec les traits
* Les types avancés : en savoir plus sur le motif newtype, les alias de type, le
  type never, et les types à taille dynamique
* Les fonctions et fermetures avancées : les pointeurs de fonctions et retourner
  des fermetures
* Les macros : une manière de définir du code qui écrit encore plus de code au
  moment de la compilation

<!--
It’s a panoply of Rust features with something for everyone! Let’s dive in!
-->

Voilà une pléthore de fonctionnalités de Rust dans lesquelles chacun y trouvera
son compte ! Commençons tout de suite !
