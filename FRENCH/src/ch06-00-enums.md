<!--
# Enums and Pattern Matching
-->

# Les énumérations et le filtrage par motif

<!--
In this chapter we’ll look at *enumerations*, also referred to as *enums*.
Enums allow you to define a type by enumerating its possible *variants*. First,
we’ll define and use an enum to show how an enum can encode meaning along with
data. Next, we’ll explore a particularly useful enum, called `Option`, which
expresses that a value can be either something or nothing. Then we’ll look at
how pattern matching in the `match` expression makes it easy to run different
code for different values of an enum. Finally, we’ll cover how the `if let`
construct is another convenient and concise idiom available to you to handle
enums in your code.
-->

Dans ce chapitre, nous allons aborder les *énumérations*, aussi appelées
*enums*. Les énumérations vous permettent de définir un type en énumérant ses
*variantes* possibles. Pour commencer, nous allons définir et utiliser une
énumération pour voir comment une énumération peut donner du sens aux données.
Ensuite, nous examinerons une énumération particulièrement utile qui s'appelle
`Option` et qui permet de décrire des situations où la valeur peut être soit
quelque chose, soit rien. Ensuite, nous regarderons comment le filtrage par
motif avec l'expression `match` peut faciliter l'exécution de codes différents
pour chaque valeur d'une énumération. Enfin, nous analyserons pourquoi la
construction `if let` est un autre outil commode et concis à votre disposition
pour traiter les énumérations dans votre code.

<!--
Enums are a feature in many languages, but their capabilities differ in each
language. Rust’s enums are most similar to *algebraic data types* in functional
languages, such as F#, OCaml, and Haskell.
-->

Les énumérations sont des fonctionnalités présentes dans de nombreux langages,
mais leurs aptitudes varient d'un langage à l'autre. Les énumérations de Rust
sont plus proches des *types de données algébriques* des langages fonctionnels,
comme F#, OCaml et Haskell.
