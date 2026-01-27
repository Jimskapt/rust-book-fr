#!/bin/bash

# Pour mettre à jour la traduction du Livre Rust en français,
# en comparant avec la version anglaise la plus à jour, et en
# utilisant le merveilleux vimdiff.

set -e

# Le chemin suivant est à adapter, au besoin:
cd ~/dev/rust-book-fr/FRENCH/src || exit 1

while IFS= read -r file; do
  # ignorer lignes vides
  [ -z "$file" ] && continue

  # ignorer commentaires
  case "$file" in
    \#*) continue ;;
  esac

  echo "=== $file ==="
  vimdiff_chapter_rustbookfr.sh "$file"

  echo
  echo "$file traité."
  echo "Entrée pour continuer (Ctrl+C pour arrêter)…"
  read
done < book_order.txt

