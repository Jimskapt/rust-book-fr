#!/bin/bash

# -------------------------------------------------------------
# vimdiff_rustbookfr.sh
# À lancer depuis : FRENCH/src/
#
# Compare un fichier du Rust Book anglais avec sa traduction FR.
# Appel :
#   vimdiff_rustbookfr.sh ch06-02-match.md
#
# Comportement :
#   - Récupère la version anglaise dans english-book/main
#   - L'enregistre dans ~/tmp/EN_<file>
#   - Lance vimdiff : EN à gauche / FR à droite
#   - Aucun nettoyage automatique
# -------------------------------------------------------------

set -e

# Vérification de l'argument
if [ -z "$1" ]; then
    echo "Usage: $0 <nom-du-fichier>.md"
    exit 1
fi

FILENAME="$1"

# Vérifier que nous sommes dans FRENCH/src/ du bon dépôt
if ! [ -d "../.." ] || ! [ -d "../../src" ]; then
    echo "Erreur : ce script doit être lancé depuis FRENCH/src/ du dépôt rust-book-fr."
    exit 1
fi

# Vérifier l'existence du fichier FR
if ! [ -f "$FILENAME" ]; then
    echo "Erreur : fichier inexistant dans FRENCH/src/ : $FILENAME"
    exit 1
fi

# Vérifier le remote english-book
if ! git -C ../.. remote | grep -q "^english-book$"; then
    echo "Remote 'english-book' absent, ajout automatique..."
    git -C ../.. remote add english-book https://github.com/rust-lang/book.git
fi

echo "Mise à jour du remote english-book..."
git -C ../.. fetch english-book >/dev/null

ENGLISH_GIT_PATH="src/$FILENAME"
TMPDIR="$HOME/tmp"
EN_TMP="$TMPDIR/EN_$FILENAME"

mkdir -p "$TMPDIR"

echo "Extraction de la version anglaise : english-book/main:$ENGLISH_GIT_PATH"
git -C ../.. show "english-book/main:$ENGLISH_GIT_PATH" > "$EN_TMP"

echo "Lancement de vimdiff..."
vimdiff "$EN_TMP" "$FILENAME"

