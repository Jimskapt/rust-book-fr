<!--
## Appendix B: Operators and Symbols
-->

## Annexe B : les opérateurs et les symboles

<!--
This appendix contains a glossary of Rust’s syntax, including operators and
other symbols that appear by themselves or in the context of paths, generics,
trait bounds, macros, attributes, comments, tuples, and brackets.
-->

Cette annexe contient un glossaire de syntaxes Rust, comprenant les opérateurs
et les autres symboles qui s'utilisent tout seuls ou alors dans le cadre de
chemins, de génériques, de traits liés, de macros, d'attributs, de commentaires,
de tuples, de crochets ou d'accolades.

<!--
### Operators
-->

### Opérateurs

<!--
Table B-1 contains the operators in Rust, an example of how the operator would
appear in context, a short explanation, and whether that operator is
overloadable. If an operator is overloadable, the relevant trait to use to
overload that operator is listed.
-->

Le tableau B-1 contient une liste d'opérateurs en Rust, un exemple de comment
l'opérateur devrait être utilisé dans ce contexte, une petite explication, et si
cet opérateur est surchargeable. Si un opérateur est surchargeable, le trait
concerné à utiliser pour la surcharge est indiqué.

<!--
<span class="caption">Table B-1: Operators</span>
-->

<span class="caption">Tableau B-1 : les opérateurs</span>

<!--
| Operator | Example | Explanation | Overloadable? |
|----------|---------|-------------|---------------|
| `!` | `ident!(...)`, `ident!{...}`, `ident![...]` | Macro expansion | |
| `!` | `!expr` | Bitwise or logical complement | `Not` |
| `!=` | `var != expr` | Nonequality comparison | `PartialEq` |
| `%` | `expr % expr` | Arithmetic remainder | `Rem` |
| `%=` | `var %= expr` | Arithmetic remainder and assignment | `RemAssign` |
| `&` | `&expr`, `&mut expr` | Borrow | |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | Borrowed pointer type | |
| `&` | `expr & expr` | Bitwise AND | `BitAnd` |
| `&=` | `var &= expr` | Bitwise AND and assignment | `BitAndAssign` |
| `&&` | `expr && expr` | Short-circuiting logical AND | |
| `*` | `expr * expr` | Arithmetic multiplication | `Mul` |
| `*=` | `var *= expr` | Arithmetic multiplication and assignment | `MulAssign` |
| `*` | `*expr` | Dereference | |
| `*` | `*const type`, `*mut type` | Raw pointer | |
| `+` | `trait + trait`, `'a + trait` | Compound type constraint | |
| `+` | `expr + expr` | Arithmetic addition | `Add` |
| `+=` | `var += expr` | Arithmetic addition and assignment | `AddAssign` |
| `,` | `expr, expr` | Argument and element separator | |
| `-` | `- expr` | Arithmetic negation | `Neg` |
| `-` | `expr - expr` | Arithmetic subtraction | `Sub` |
| `-=` | `var -= expr` | Arithmetic subtraction and assignment | `SubAssign` |
| `->` | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | Function and closure return type | |
| `.` | `expr.ident` | Member access | |
| `..` | `..`, `expr..`, `..expr`, `expr..expr` | Right-exclusive range literal | |
| `..=` | `..=expr`, `expr..=expr` | Right-inclusive range literal | |
| `..` | `..expr` | Struct literal update syntax | |
| `..` | `variant(x, ..)`, `struct_type { x, .. }` | “And the rest” pattern binding | |
| `...` | `expr...expr` | In a pattern: inclusive range pattern | |
| `/` | `expr / expr` | Arithmetic division | `Div` |
| `/=` | `var /= expr` | Arithmetic division and assignment | `DivAssign` |
| `:` | `pat: type`, `ident: type` | Constraints | |
| `:` | `ident: expr` | Struct field initializer | |
| `:` | `'a: loop {...}` | Loop label | |
| `;` | `expr;` | Statement and item terminator | |
| `;` | `[...; len]` | Part of fixed-size array syntax | |
| `<<` | `expr << expr` | Left-shift | `Shl` |
| `<<=` | `var <<= expr` | Left-shift and assignment | `ShlAssign` |
| `<` | `expr < expr` | Less than comparison | `PartialOrd` |
| `<=` | `expr <= expr` | Less than or equal to comparison | `PartialOrd` |
| `=` | `var = expr`, `ident = type` | Assignment/equivalence | |
| `==` | `expr == expr` | Equality comparison | `PartialEq` |
| `=>` | `pat => expr` | Part of match arm syntax | |
| `>` | `expr > expr` | Greater than comparison | `PartialOrd` |
| `>=` | `expr >= expr` | Greater than or equal to comparison | `PartialOrd` |
| `>>` | `expr >> expr` | Right-shift | `Shr` |
| `>>=` | `var >>= expr` | Right-shift and assignment | `ShrAssign` |
| `@` | `ident @ pat` | Pattern binding | |
| `^` | `expr ^ expr` | Bitwise exclusive OR | `BitXor` |
| `^=` | `var ^= expr` | Bitwise exclusive OR and assignment | `BitXorAssign` |
| <code>&vert;</code> | <code>pat &vert; pat</code> | Pattern alternatives | |
| <code>&vert;</code> | <code>expr &vert; expr</code> | Bitwise OR | `BitOr` |
| <code>&vert;=</code> | <code>var &vert;= expr</code> | Bitwise OR and assignment | `BitOrAssign` |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code> | Short-circuiting logical OR | |
| `?` | `expr?` | Error propagation | |
-->

| Opérateur | Exemple | Explication | Surchargeable ? |
|-----------|---------|-------------|-----------------|
| `!` | `ident!(...)`, `ident!{...}`, `ident![...]` | Identificateur de macro | |
| `!` | `!expr` | Négation binaire ou logique | `Not` |
| `!=` | `var != expr` | Comparaison de non-égalité | `PartialEq` |
| `%` | `expr % expr` | Reste arithmétique | `Rem` |
| `%=` | `var %= expr` | Reste arithmétique et assignation | `RemAssign` |
| `&` | `&expr`, `&mut expr` | Emprunt | |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | Type de pointeur emprunté | |
| `&` | `expr & expr` | ET binaire | `BitAnd` |
| `&=` | `var &= expr` | ET binaire et assignation | `BitAndAssign` |
| `&&` | `expr && expr` | ET logique | |
| `*` | `expr * expr` | Multiplication arithmétique | `Mul` |
| `*=` | `var *= expr` | Multiplication arithmétique et assignation | `MulAssign` |
| `*` | `*expr` | Déréférencement | |
| `*` | `*const type`, `*mut type` | Pointeur brut | |
| `+` | `trait + trait`, `'a + trait` | Contrainte de type composé | |
| `+` | `expr + expr` | Addition arithmétique | `Add` |
| `+=` | `var += expr` | Addition arithmétique et assignation | `AddAssign` |
| `,` | `expr, expr` | Séparateur d'arguments et d'éléments | |
| `-` | `- expr` | Négation arithmétique | `Neg` |
| `-` | `expr - expr` | Soustraction arithmétique | `Sub` |
| `-=` | `var -= expr` | Soustraction arithmétique et assignation | `SubAssign` |
| `->` | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | Type de retour de fonction et de fermeture | |
| `.` | `expr.ident` | Accès à un membre | |
| `..` | `..`, `expr..`, `..expr`, `expr..expr` | Littéral d'intervalle d'exclusion | |
| `..=` | `..=expr`, `expr..=expr` | Littéral d'intervalle d'inclusion | |
| `..` | `..expr` | Syntaxe de mise à jour de litéraux de structure | |
| `..` | `variant(x, ..)`, `struct_type { x, .. }` | Motif “ainsi que la suite” | |
| `...` | `expr...expr` | Dans un motif : motif d'intervalle inclusif | |
| `/` | `expr / expr` | Division arithmétique | `Div` |
| `/=` | `var /= expr` | Division arithmétique et assignation | `DivAssign` |
| `:` | `pat: type`, `ident: type` | Contrainte | |
| `:` | `ident: expr` | Initialisateur de champ de structure | |
| `:` | `'a: loop {...}` | Une identification de boucle | |
| `;` | `expr;` | Fin d'élément et d'instruction | |
| `;` | `[...; len]` | Syntaxe désignant une partie d'un tableau à taille finie | |
| `<<` | `expr << expr` | Décalage à gauche | `Shl` |
| `<<=` | `var <<= expr` | Décalage à gauche et assignation | `ShlAssign` |
| `<` | `expr < expr` | Comparaison "inférieur à" | `PartialOrd` |
| `<=` | `expr <= expr` | Comparaison "inférieur ou égal à" | `PartialOrd` |
| `=` | `var = expr`, `ident = type` | Assignation ou équivalence | |
| `==` | `expr == expr` | Comparaison d'égalité | `PartialEq` |
| `=>` | `pat => expr` | Syntaxe d'une partie d'une branche correspondante | |
| `>` | `expr > expr` | Comparaison "supérieur à" | `PartialOrd` |
| `>=` | `expr >= expr` | Comparaison "supérieur ou égal à" | `PartialOrd` |
| `>>` | `expr >> expr` | Décalage à droite | `Shr` |
| `>>=` | `var >>= expr` | Décalage à droite et assignation | `ShrAssign` |
| `@` | `ident @ pat` | Création d'un identificateur à partir du motif | |
| `^` | `expr ^ expr` | OU exclusif binaire | `BitXor` |
| `^=` | `var ^= expr` | OU exclusif binaire et assignation | `BitXorAssign` |
| <code>&vert;</code> | <code>pat &vert; pat</code> | Alternatives à un motif | |
| <code>&vert;</code> | <code>expr &vert; expr</code> | OU binaire | `BitOr` |
| <code>&vert;=</code> | <code>var &vert;= expr</code> | OU binaire et assignation | `BitOrAssign` |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code> | OU logique | |
| `?` | `expr?` | Propagation d'erreur | |

<!--
### Non-operator Symbols
-->

### Les symboles non-opérateurs

<!--
The following list contains all non-letters that don’t function as operators;
that is, they don’t behave like a function or method call.
-->

La liste suivante contient tout ce qui n'est pas une lettre et qui ne fonctionne
pas comme un opérateur ; autrement dit tout ce qui ne se comporte pas comme un
appel de fonction ou de méthode.

<!--
Table B-2 shows symbols that appear on their own and are valid in a variety of
locations.
-->

Le tableau B-2 montre des symboles qui s'utilisent tout seuls et qui sont
valables dans plusieurs situations.

<!--
<span class="caption">Table B-2: Stand-Alone Syntax</span>
-->

<span class="caption">Tableau B-2 : syntaxes autonomes</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `'ident` | Named lifetime or loop label |
| `...u8`, `...i32`, `...f64`, `...usize`, etc. | Numeric literal of specific type |
| `"..."` | String literal |
| `r"..."`, `r#"..."#`, `r##"..."##`, etc. | Raw string literal, escape characters not processed |
| `b"..."` | Byte string literal; constructs a `[u8]` instead of a string |
| `br"..."`, `br#"..."#`, `br##"..."##`, etc. | Raw byte string literal, combination of raw and byte string literal |
| `'...'` | Character literal |
| `b'...'` | ASCII byte literal |
| <code>&vert;...&vert; expr</code> | Closure |
| `!` | Always empty bottom type for diverging functions |
| `_` | “Ignored” pattern binding; also used to make integer literals readable |
-->

| Symbole | Explication |
|---------|-------------|
| `'ident` | Nom d'une durée de vie ou nom boucle |
| `...u8`, `...i32`, `...f64`, `...usize`, etc. | Nombre littéral d'un type spécifique |
| `"..."` | Chaîne de caractère littérale |
| `r"..."`, `r#"..."#`, `r##"..."##`, etc. | Chaîne de caractères brute littérale, les caractères d'échappement ne sont pas traités |
| `b"..."` | Chaîne d'octet littéral ; construit un `[u8]` au lieu d'une chaîne de caractères |
| `br"..."`, `br#"..."#`, `br##"..."##`, etc. | Chaîne d'octets brute littérale, combinaison de la chaîne d'octets brute et de la chaîne d'octets littérale |
| `'...'` | Caractère littéral |
| `b'...'` | Octet ASCII littéral |
| <code>&vert;...&vert; expr</code> | Une fermeture |
| `!` | Le type “jamais", toujours vide pour les fonctions divergentes |
| `_` | Le motif “ignoré" ; aussi utilisé pour rendre lisibles les nombres entiers littéraux |

<!--
Table B-3 shows symbols that appear in the context of a path through the module
hierarchy to an item.
-->

Le tableau B-3 montre des symboles qui s'utilisent dans le contexte d'un chemin
dans une structure de modules pour obtenir un élément.

<!--
<span class="caption">Table B-3: Path-Related Syntax</span>
-->

<span class="caption">Tableau B-3 : syntaxes utilisés pour les chemins</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `ident::ident` | Namespace path |
| `::path` | Path relative to the crate root (i.e., an explicitly absolute path) |
| `self::path` | Path relative to the current module (i.e., an explicitly relative path).
| `super::path` | Path relative to the parent of the current module |
| `type::ident`, `<type as trait>::ident` | Associated constants, functions, and types |
| `<type>::...` | Associated item for a type that cannot be directly named (e.g., `<&T>::...`, `<[T]>::...`, etc.) |
| `trait::method(...)` | Disambiguating a method call by naming the trait that defines it |
| `type::method(...)` | Disambiguating a method call by naming the type for which it’s defined |
| `<type as trait>::method(...)` | Disambiguating a method call by naming the trait and type |
-->

| Symbole | Explication |
|---------|-------------|
| `ident::ident` | Chemin d'un espace de nom |
| `::path` | Chemin relatif à la crate racine (c'est à dire un chemin explicitement absolu) |
| `self::path` | Chemin relatif au module courant (c'est à dire un chemin explicitement relatif) |
| `super::path` | Chemin relatif au parent du module courant |
| `type::ident`, `<type as trait>::ident` | Des constantes, fonctions et types associées |
| `<type>::...` | Un élément associé pour un type qui ne peut pas être directement nommé (par exemple, `<&T>::...`, `<[T]>::...`, etc) |
| `trait::method(...)` | Clarifier l'appel d'une méthode en nommant le trait qui le définit |
| `type::method(...)` | Clarifier l'appel d'une fonction en nommant le type pour laquelle elle est définie |
| `<type as trait>::method(...)` | Clarifier l'appel d'une méthode en nommant le trait et le type |

<!--
Table B-4 shows symbols that appear in the context of using generic type
parameters.
-->

Le tableau B-4 montre des symboles qui apparaissent dans le contexte
d'utilisation de paramètres de type génériques.

<!--
<span class="caption">Table B-4: Generics</span>
-->

<span class="caption">Tableau B-4 : génériques</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `path<...>` | Specifies parameters to generic type in a type (e.g., `Vec<u8>`) |
| `path::<...>`, `method::<...>` | Specifies parameters to generic type, function, or method in an expression; often referred to as turbofish (e.g., `"42".parse::<i32>()`) |
| `fn ident<...> ...` | Define generic function |
| `struct ident<...> ...` | Define generic structure |
| `enum ident<...> ...` | Define generic enumeration |
| `impl<...> ...` | Define generic implementation |
| `for<...> type` | Higher-ranked lifetime bounds |
| `type<ident=type>` | A generic type where one or more associated types have specific assignments (e.g., `Iterator<Item=T>`) |
-->

| Symbole | Explication |
|---------|-------------|
| `path<...>` | Précise des paramètres sur un type générique utilisé dans un type (par exemple, `Vec<u8>`) |
| `path::<...>`, `method::<...>` | Précise des paramètres sur un type générique, une fonction, ou une méthode dans une expression ; parfois appelé turbofish (par exemple, `"42".parse::<i32>()`) |
| `fn ident<...> ...` | Définit une fonction générique |
| `struct ident<...> ...` | Définit une structure générique |
| `enum ident<...> ...` | Définit une énumération générique |
| `impl<...> ...` | Définit une implémentation générique |
| `for<...> type` | Augmente la durée de vie |
| `type<ident=type>` | Un type générique sur lequel un ou plusieurs types associés ont des affectations spécifiques (par exemple, `Iterator<Item=T>`) |

<!--
Table B-5 shows symbols that appear in the context of constraining generic type
parameters with trait bounds.
-->

Le tableau B-5 montre des symboles qui s'utilisent pour contraindre des
paramètres de type génériques avec des traits liés.

<!--
<span class="caption">Table B-5: Trait Bound Constraints</span>
-->

<span class="caption">Tableau B-5 : contraintes de trait lié</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `T: U` | Generic parameter `T` constrained to types that implement `U` |
| `T: 'a` | Generic type `T` must outlive lifetime `'a` (meaning the type cannot transitively contain any references with lifetimes shorter than `'a`) |
| `T : 'static` | Generic type `T` contains no borrowed references other than `'static` ones |
| `'b: 'a` | Generic lifetime `'b` must outlive lifetime `'a` |
| `T: ?Sized` | Allow generic type parameter to be a dynamically sized type |
| `'a + trait`, `trait + trait` | Compound type constraint |
-->

| Symbole | Explication |
|---------|-------------|
| `T: U` | Paramètre générique `T` contraint aux types qui implémentent `U` |
| `T: 'a` | Type générique `T` doit vivre aussi longtemps que la durée de vie `'a` (ce qui signifie que le type ne peut pas contenir temporairement de références avec une durée de vie plus petite que `'a`) |
| `T : 'static` | Type générique `T` qui ne contient pas d'autres références empruntées autres que des `'static` |
| `'b: 'a` | La durée de vie générique `'b` doit vivre aussi longtemps que `'a` |
| `T: ?Sized` | Permet aux paramètres de type génériques d'être de type à taille dynamique |
| `'a + trait`, `trait + trait` | Contrainte de type composé |

<!--
Table B-6 shows symbols that appear in the context of calling or defining
macros and specifying attributes on an item.
-->

Le tableau B-6 montre des symboles qui s'utilisent lors de l'appel ou de la
définition de macros et pour spécifier des attributs sur un élément.

<!--
<span class="caption">Table B-6: Macros and Attributes</span>
-->

<span class="caption">Tableau B-6 : macros et attributs</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `#[meta]` | Outer attribute |
| `#![meta]` | Inner attribute |
| `$ident` | Macro substitution |
| `$ident:kind` | Macro capture |
| `$(…)…` | Macro repetition |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Macro invocation |
-->

| Symbole | Explication |
|---------|-------------|
| `#[meta]` | Attribut externe |
| `#![meta]` | Attribut interne |
| `$ident` | Substitution de macro |
| `$ident:kind` | Capture de macro |
| `$(…)…` | Répétition de macro |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Appel d'une macro |

<!--
Table B-7 shows symbols that create comments.
-->

Le tableau B-7 montre des symboles pour créer des commentaires.

<!--
<span class="caption">Table B-7: Comments</span>
-->

<span class="caption">Tableau B-7 : commentaires</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `//` | Line comment |
| `//!` | Inner line doc comment |
| `///` | Outer line doc comment |
| `/*...*/` | Block comment |
| `/*!...*/` | Inner block doc comment |
| `/**...*/` | Outer block doc comment |
-->

| Symbole | Explication |
|---------|-------------|
| `//` | Ligne commentée |
| `//!` | Commentaire de documentation sur l'élément contenant actuel |
| `///` | Commentaire de documentation sur l'élément suivant ce commentaire |
| `/*...*/` | Bloc de commentaire |
| `/*!...*/` | Bloc de commentaire de documentation sur l'élément contenant actuel |
| `/**...*/` | Bloc de commentaire de documentation sur l'élément suivant ce commentaire |

<!--
Table B-8 shows symbols that appear in the context of using tuples.
-->

Le tableau B-8 montre des symboles utilisés avec les tuples.

<!--
<span class="caption">Table B-8: Tuples</span>
-->

<span class="caption">Tableau B-8 : les tuples</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `()` | Empty tuple (aka unit), both literal and type |
| `(expr)` | Parenthesized expression |
| `(expr,)` | Single-element tuple expression |
| `(type,)` | Single-element tuple type |
| `(expr, ...)` | Tuple expression |
| `(type, ...)` | Tuple type |
| `expr(expr, ...)` | Function call expression; also used to initialize tuple `struct`s and tuple `enum` variants |
| `expr.0`, `expr.1`, etc. | Tuple indexing |
-->

| Symbole | Explication |
|---------|-------------|
| `()` | Un tuple vide (aussi appelé unitaire), à la fois un type et un litéral |
| `(expr)` | Une expression entre parenthèses |
| `(expr,)` | Un tuple d'un seul élement qui est une expression |
| `(type,)` | Un tuple d'un seul élement qui est un type |
| `(expr, ...)` | Une expression dans un tuple |
| `(type, ...)` | Un type dans un tuple |
| `expr(expr, ...)` | Une expression d'appel à une fonction ; aussi utilisé pour initialiser une structure tuple ou une variante d'énumération tuple |
| `expr.0`, `expr.1`, etc. | Utilisation d'indices sur un tuple |

<!--
Table B-9 shows the contexts in which curly braces are used.
-->

Le tableau B-9 montre les contextes d'utilisation des accolades.

<!--
<span class="caption">Table B-9: Curly Brackets</span>
-->

<span class="caption">Tableau B-9 : accolades</span>

<!--
| Context | Explanation |
|---------|-------------|
| `{...}` | Block expression |
| `Type {...}` | `struct` literal |
-->

| Symbole | Explication |
|---------|-------------|
| `{...}` | Bloc d'expression |
| `Type {...}` | Un littéral de `struct` |

<!--
Table B-10 shows the contexts in which square brackets are used.
-->

Le tableau B-10 montre les contextes d'utilisation des crochets.

<!--
<span class="caption">Table B-10: Square Brackets</span>
-->

<span class="caption">Tableau B-10 : crochets</span>

<!--
| Context | Explanation |
|---------|-------------|
| `[...]` | Array literal |
| `[expr; len]` | Array literal containing `len` copies of `expr` |
| `[type; len]` | Array type containing `len` instances of `type` |
| `expr[expr]` | Collection indexing. Overloadable (`Index`, `IndexMut`) |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Collection indexing pretending to be collection slicing, using `Range`, `RangeFrom`, `RangeTo`, or `RangeFull` as the “index” |
-->

| Symbole | Explication |
|---------|-------------|
| `[...]` | Un littéral de tableau |
| `[expr; len]` | Un littéral de tableau qui contient `len` copies de `expr` |
| `[type; len]` | Un type de tableau qui contient `len` instances de `type` |
| `expr[expr]` | Une collection indexée. C'est surchargeable (via `Index` et `IndexMut`) |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Une collection indexée qui se comporte comme une slice de collection, grâce à l'utilisation de `Range`, `RangeFrom`, `RangeTo`, ou de `RangeFull` comme “indice” |
