# Outils pour la mise à jour de la traduction française du Livre de Rust

Ce répertoire contient des scripts utilitaires destinés à faciliter
la mise à jour et la relecture de la traduction française du *Rust Book*,
en comparaison avec la version anglaise officielle la plus récente.

Ces outils sont principalement pensés pour les traducteurs et relecteurs
souhaitant synchroniser le texte français avec les évolutions du texte
source anglais.

---

## Prérequis

- `git`
- `vim` avec `vimdiff`
- Accès réseau pour récupérer la version anglaise du livre
- Le dépôt `rust-book-fr` correctement cloné

Les scripts supposent l’arborescence suivante :

```
rust-book-fr/
├── src/              # version anglaise
└── FRENCH/
    └── src/          # version française
```

---

## Scripts disponibles

### `vimdiff_chapter_rustbookfr.sh`

Compare un chapitre du livre en anglais (version `english-book/main`)
avec sa traduction française.

#### Usage

À lancer depuis `FRENCH/src/` :

```sh
vimdiff_chapter_rustbookfr.sh ch06-02-match.md
```

#### Comportement

- récupère la version anglaise la plus récente du chapitre
- l’enregistre dans `~/tmp/EN_<nom_du_fichier>`
- lance `vimdiff` :
  - anglais à gauche
  - français à droite
- aucun nettoyage automatique des fichiers temporaires

Ce script est conçu pour être utilisé de manière interactive lors de la
mise à jour de la traduction.

---

### `rustbook_update_french_translation.sh`

Lance un processus interactif de relecture et de mise à jour de la
traduction française, chapitre par chapitre, dans l’ordre du livre.

#### Fonctionnement

- lit la liste des chapitres depuis `book_order.txt`
- ignore :
  - les lignes vides
  - les lignes commentées avec `#`
- pour chaque chapitre :
  - lance `vimdiff_chapter_rustbookfr.sh` avec le fichier en argument
  - attend une confirmation utilisateur avant de passer au suivant

Cela permet :
- de reprendre le travail après interruption
- de marquer les chapitres déjà traités en les commentant dans
  `book_order.txt`

#### Usage

```sh
./rustbook_update_french_translation.sh
```

---

## Fichier `book_order.txt`

Ce fichier liste les chapitres du Livre de Rust dans leur ordre logique.

- une ligne = un fichier Markdown
- une ligne commentée (`#`) = chapitre déjà traité ou volontairement ignoré

Il sert à la fois :
- de checklist
- de journal de progression
- de point de reprise en cas d’interruption

---

## Philosophie

Ces scripts privilégient :
- la simplicité
- la lisibilité
- l’interactivité humaine

Ils n’ont pas vocation à automatiser la traduction, mais à fournir un
cadre de travail confortable et reproductible pour une relecture
attentive et qualitative.

---

## Licence

Ces scripts suivent la licence du dépôt `rust-book-fr`.

