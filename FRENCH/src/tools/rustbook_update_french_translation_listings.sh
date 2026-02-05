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
commit_msg="$TMP_BASE/COMMIT_MESSAGE.txt"

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

create_commit_msg() { # Fonction intentionnellement non indentée
if [[ ! -f "$commit_msg" ]]; then
cat >"$commit_msg" <<EOF
chore(rust-book-fr): update listings

- File: $rel
- English source: 2025 edition
- Notes:
  -
EOF
else
echo "$rel" >> "$commit_msg"
fi
}


# --- itération sur les fichiers français ---
find "$DIR_LISTINGS_FR" -type f -print0 | sort -z | while read -r -d '' fr_file; do
  echo -e "\n### Traitement du fichier $fr_file ###"
  confirm 0 "Traiter ce fichier?" || continue
  rel="${fr_file#$DIR_LISTINGS_FR/}" # Ce nom de variable est bien peu explicite => nom RELatif
  [[ -n "$ONLY_CHAPITRES" && "$rel" != "$ONLY_CHAPITRES-"* ]] && continue
  en_new="$DIR_LISTINGS_EN_NEW/$rel"

  [[ -f "$en_new" ]] || echo "Attention, $en_new est introuvable"; continue

  en_old="$(extract_old "listings/$rel")"

  echo
  echo "###========================== $rel ==========================###"

  if [[ -n "$en_old" ]]; then
    # Il y a un fichier ancien, on compare donc 3 versions du fichier:
    #  - ENglish old: en_old  => la version anglaise qui a servi de base à la traduction en français
    #  - English new: en_new  => la version anglaise dans sa dernière mouture
    #  - FRançais:    fr_file => la version traduite en français, à mettre à jour si besoin

    # Première version, avec diff3, assez peu éloquente, commentée:
    # diff3 "$en_new" "$en_old" "$fr_file"

    # Seconde version, on affiche les différences 2 par 2:
    echo "############################################"
    echo "## Changements dans la version anglaise:  ##"
    if diff -u --color=always "$en_old" "$en_new"; then
      echo "Pas de différences entre les version anglaises, donc rien à mettre à jour du côté traduction française"
    else
      echo "           => ** il y a des différences en amont ! **"
      # echo "DEBUG: en_old=$en_old ; fr_file=$fr_file"
      echo "############################################"
      echo "## Changements dans la version française: ##"
      if diff -u --color=always "$en_old" "$fr_file"; then
        echo "Pas de différences entre la version anglaise ancienne et la traduction française: il faut mettre à jour la traduction française"
      else
        echo "############################################"
        echo "           => il y a des différences de traduction"
      fi
      echo "############################################"
      if confirm 1 "Voir les diffs ci-haut: édition avec vimdiff"; then
        create_commit_msg
        # vimdiff </dev/tty "$en_new" "$en_old" "$fr_file"
        vim </dev/tty -d "$en_new" "$en_old" "$fr_file" \
          -c "botright split $commit_msg" \
          -c 'resize 8' \
          -c 'setlocal nodiff buftype= bufhidden=hide noswapfile' \
          -c 'autocmd VimEnter * wincmd k | wincmd l | wincmd l'
      else
        echo "Passage au fichier suivant"
      fi
    fi
  else
    echo "⚠️  Pas de version dans $DIR_LISTINGS_EN_OLD_BRANCH → diff à 2"
    echo "## Changements dans la version française: ##"
    diff --side-by-side --color=always "$en_new" "$fr_file"

    if confirm 1 "Voir les diffs ci-haut: édition avec vimdiff"; then
      create_commit_msg
      # echo $LINENO; confirm ### DEBUG
      # vim     </dev/tty -d "$en_new"           "$fr_file" "$commit_msg" -c 'wincmd J' -c 'resize 8' -c 'setlocal nodiff' -c 'setlocal buftype=' -c 'setlocal bufhidden=hide'
      vim </dev/tty -d "$en_new"           "$fr_file" \
        -c "botright split $commit_msg" \
        -c 'resize 8' \
        -c 'setlocal nodiff buftype= bufhidden=hide noswapfile' \
        -c "buf $fr_file"
    fi
    if ! confirm 0 "Continuer"; then
      echo "On arrête."
      exit 0
    fi
  fi
  echo -e "\nFichier suivant...\n\n\n"
done

# Finaloumen, faire un git commit, avec le message préparé durant la boucle:
if confirm 0 "Faire un git commit avec le message: "\n$(cat $commit_msg)\n ?"; then
  git commit -a -F "$commit_msg"
  if confirm 0 "Supprimer le fichier de brouillon de message de commit?"; then
    rm "$commit_msg"
  fi
fi

