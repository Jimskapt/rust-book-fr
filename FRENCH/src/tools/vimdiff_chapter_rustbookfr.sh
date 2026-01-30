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

# Editor fallback (au cas où)
EDITOR_CMD="${EDITOR:-vi}"

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

#######################################
# Le gros du boulot:
echo "Lancement de vimdiff..."
vimdiff "$EN_TMP" "$FILENAME"
#######################################

#######################################
# Une fois l'édition terminée, on prépare le commit
echo "Préparation du message de commit: Entrée pour continuer, Ctrl-C pour arrêter..."
read

# Fichiers modifiés (hors supprimés)
FILES=$(git diff --name-only --diff-filter=ACM)

if [ -z "$FILES" ]; then
    echo "Nothing to commit."
    exit 1
fi

# Heuristique simple pour le scope
SCOPE=$(printf "%s\n" "$FILES" \
    | sed 's#.*/##' \
    | sed 's/\..*//' \
    | sort -u \
    | head -n 1)

# Message temporaire
MSG_FILE=$(mktemp /tmp/gitmsg.upstream-sync.XXXXXX)

cat > "$MSG_FILE" <<EOF
upstream-sync: ${SCOPE} - upstream alignment

Source:
- Upstream: rust-lang/book (English)

Changes:
- 

Notes:
- Large diffs are mostly due to upstream rewrites and 80-column wrapping
- English commented sections were selectively updated when meaningfully changed
- Duplicated code blocks (commented out and uncommented), when *strictly*
  redundant, were deleted
EOF

# Édition interactive
"$EDITOR_CMD" "$MSG_FILE"

# Si le message est vide ou inchangé, on annule
if ! grep -q '^upstream-sync:' "$MSG_FILE"; then
    echo "Aborted: commit message missing or invalid."
    rm -f "$MSG_FILE"

    # On renvoie un exit 0 plutôt que 1 pour pouvoir passer au fichier suivant,
    # s'il n'y a pas de commit à faire:
    exit 0
fi

# Pour garder une trace dans le terminal
cat "$MSG_FILE"

# Commit
git commit -aF "$MSG_FILE"

rm -f "$MSG_FILE"

