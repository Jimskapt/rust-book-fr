#!/bin/bash

# -------------------------------------------------------------
# Various utilities, for debugging, logging, common code, etc.
# -------------------------------------------------------------



# A utility function: {{{
confirm() {
  # Confirmations function for user interaction
  # Arguments: $1: 0 or 1        (optional)  => default answer
  #            rest of arguments (optional)  => question displayed
  #
  # If a first numeric argument is given, 0 or 1, it is the default answer;
  # it follows standard shell convention: 0 ok, 1 ko.
  # The first argument different from 0 and 1 is the question displayed to the
  # user; if there is no argument, a default question is printed.
  # The function displays a prompt and then waits for Y N or Enter keypress;
  # defaults to No (N or Enter keypress) and returns 0 (yes) or 1 (no).

  # EXAMPLES:
  #  ask a question, expect a Yes default answer:
  #    confirm 0 "c'est sûr?" ; echo "Retour: $?"
  #  ask a question, expect a No default answer:
  #     confirm 1 "ah?"        ; echo "Retour: $?"

  # Typical usage inside a bash script:
  #  if confirm 0 "Run deployment of configuration to all hosts (y/n, default No)? "; then
  #    install_config_on_all_hosts
  #  fi



  # By default, Yes:
  local default=0
  local msg_yesno="(Y/n)"
  if [[ $# -ge 1 ]]; then
    case $1 in
      ("0") default=0; msg_yesno="(Y/n)"; shift ;;
      ("1") default=1; msg_yesno="(y/N)"; shift ;;
    esac
  fi
    local key
    while true; do
      read -rn 1 -e -p "${*:-Continue?} ${msg_yesno} " key </dev/tty
      case ${key} in
        ([yY]) echo; return 0          ;;
        ([nN]) echo; return 1          ;;
        ("")   echo; return ${default} ;;
        (*)    printf " \033[31m %s \033[0m\n" "Invalid key" ;;
      esac
    done
}
# }}}

