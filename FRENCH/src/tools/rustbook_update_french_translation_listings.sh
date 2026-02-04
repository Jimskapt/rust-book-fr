#!/bin/bash

# Pour mettre à jour la traduction des codes du Livre Rust en français,
# en comparant avec la version anglaise la plus à jour, et avec la 
# version anglaise qui a été archivée au moment de la première traduction
# en français, et en utilisant le merveilleux vimdiff.

# rustbook_update_listings.sh [OPTIONS]
# 
# Options:
#   --auto-cargo-toml     Applique automatiquement les updates Cargo.toml
#   --only-chXX           Ne traite que chXX-* (ex: --only-ch17)
#   --stat-only           Affiche uniquement des stats de diff, sans interaction


set -euo pipefail


# --- CONFIG ---
DEBUG=0
AUTO_CARGO=0
ONLY_CHAPITRES=""
STAT_ONLY=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --auto-cargo-toml)
      AUTO_CARGO=1
      ;;
    --only-ch*)
      ONLY_CHAPITRES="${1#--only-}"
      ;;
    --stat-only)
      STAT_ONLY=1
      ;;
    --debug)
      DEBUG=1
      ;;
    *)
      echo "Option inconnue: $1" >&2
      exit 1
      ;;
  esac
  shift
done

DIR_RACINE_FR="$HOME/dev/rust-book-fr/FRENCH"
DIR_LISTINGS_FR="$DIR_RACINE_FR/listings"
DIR_LISTINGS_EN_NEW="$DIR_RACINE_FR/listings-sources"

DIR_LISTINGS_EN_OLD_BRANCH="update-from-english-2025"

TMP_BASE="$HOME/tmp/rustbook_en_old"
mkdir -p "$TMP_BASE"

cd "$DIR_RACINE_FR"

if [[ $DEBUG -eq 1 ]]; then
  # Source some utility functions:
  source $(dirname $0)/common.sh
else
  # Empty function instead of confirm:
  confirm() {
    echo
  }
fi

echo $LINENO; confirm ### DEBUG


################################################################################
echo "Configuration:" # DEBUG
echo "AUTO_CARGO=$AUTO_CARGO"
echo "ONLY_CHAPITRES=$ONLY_CHAPITRES"
echo "STAT_ONLY=$STAT_ONLY"
echo "DIR_RACINE_FR=$DIR_RACINE_FR"
echo "DIR_LISTINGS_FR=$DIR_LISTINGS_FR"
echo "DIR_LISTINGS_EN_NEW=$DIR_LISTINGS_EN_NEW"
echo "DIR_LISTINGS_EN_OLD_BRANCH=$DIR_LISTINGS_EN_OLD_BRANCH"
echo "TMP_BASE=$TMP_BASE"
echo "Répertoire courant: $(pwd)"
echo $LINENO; confirm ### DEBUG
################################################################################


# --- extrait un fichier depuis la branche anglaise de référence ---
extract_old() {
  local relpath="$1"
  local out="$TMP_BASE/$relpath"

  mkdir -p "$(dirname "$out")"

  if git cat-file -e "$DIR_LISTINGS_EN_OLD_BRANCH:$relpath" 2>/dev/null; then
    git show "$DIR_LISTINGS_EN_OLD_BRANCH:$relpath" > "$out"
    echo "$out"
  else
    echo ""
  fi
}

# --- itération sur les fichiers français ---
find "$DIR_LISTINGS_FR" -type f -print0 | sort -z | while read -r -d '' fr_file; do
  echo "### Traitement du fichier $fr_file ###"
  echo $LINENO; confirm ### DEBUG
  [[ -n "$ONLY_CHAPITRES" && "$rel" != "$ONLY_CHAPITRES-"* ]] && continue
  rel="${fr_file#$DIR_LISTINGS_FR/}" # Ce nom de variable est bien peu explicite
  en_new="$DIR_LISTINGS_EN_NEW/$rel" # Ce nom de variable est bien peu explicite
  echo "rel=$rel" # DEBUG
  echo "en_new=$en_new" # DEBUG
  echo $LINENO; confirm ### DEBUG

  [[ -f "$en_new" ]] || continue # TODO Ce serait pas mal de mettre un message d'alerte ici

  en_old="$(extract_old "listings/$rel")"  # Ce nom de variable est bien peu explicite
  echo "en_old=$en_old" # DEBUG
  echo $LINENO; confirm ### DEBUG

  echo
  echo "=== $rel ==="

  if [[ -n "$en_old" ]]; then
    echo $LINENO; confirm ### DEBUG
    diff3 "$en_new" "$en_old" "$fr_file"
    echo $LINENO; confirm ### DEBUG
    if confirm 1 "Voir les diffs ci-haut: édition avec vimdiff"; then
      vimdiff </dev/tty "$en_new" "$en_old" "$fr_file"
    fi
  else
    echo "⚠️  Pas de version dans $DIR_LISTINGS_EN_OLD_BRANCH → diff à 2"
    echo $LINENO; confirm ### DEBUG
    diff "$en_new" "$fr_file"
    echo $LINENO; confirm ### DEBUG
    if confirm 1 "Voir les diffs ci-haut: édition avec vimdiff"; then
      commit_msg="$TMP_BASE/COMMIT_MESSAGE.txt"
      if [[ ! -f "$commit_msg" ]]; then
        cat >"$commit_msg" <<EOF
chore(rust-book-fr): update listings

- File: $rel
- English source: 2025 edition
- Notes:
  -
EOF
      fi
      vimdiff </dev/tty "$en_new" "$fr_file"
      vim </dev/tty -d "$en_new" "$en_old" "$fr_file" "$commit_msg" \
        -c 'wincmd J' \
        -c 'resize 8' \
        -c 'setlocal nodiff' \
        -c 'setlocal buftype=' \
        -c 'setlocal bufhidden=hide'
    fi
    if ! confirm 0 "Continuer"; then
      echo "On arrête."
    fi
  fi
done

