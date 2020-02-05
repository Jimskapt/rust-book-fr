<!--
# Object Oriented Programming Features of Rust
-->

# Les fonctionnalités orientées objet de Rust

<!--
Object-oriented programming (OOP) is a way of modeling programs. Objects came
from Simula in the 1960s. Those objects influenced Alan Kay’s programming
architecture in which objects pass messages to each other. He coined the term
*object-oriented programming* in 1967 to describe this architecture. Many
competing definitions describe what OOP is; some definitions would classify
Rust as object oriented, but other definitions would not. In this chapter,
we’ll explore certain characteristics that are commonly considered object
oriented and how those characteristics translate to idiomatic Rust. We’ll then
show you how to implement an object-oriented design pattern in Rust and discuss
the trade-offs of doing so versus implementing a solution using some of Rust’s
strengths instead.
-->

La programmation orientée objet (POO) est une façon de concevoir des programmes.
Les objets sont apparus dans Simula dans les années 1960. Ces objets ont
influencé l'architecture de programmation d'Alan Kay dans laquelle les objets
s'envoient des messages. Il a inventé le terme *programmation orientée objet* en
1967 pour décrire cette architecture. Plusieurs définitions de la POO
s'opposent ; Rust est considéré comme orienté objet selon certaines définitions
mais pas par d'autres. Dans ce chapitre, nous examinerons certaines
caractéristiques généralement considérées comme orientées objet et nous verrons
comment ces caractéristiques se traduisent en code Rust traditionnel. Puis nous
vous montrerons comment implémenter un patron de conception orienté objet en
Rust et nous comparerons les avantages et inconvénients de faire cela plutôt que
d'implémenter une solution qui utilise quelques points forts de Rust.
