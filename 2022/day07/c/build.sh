#!/bin/bash
script_root=$(dirname $(realpath "${BASH_SOURCE:-$0}"))

clear
rm "$script_root/day07.out"
clang \
    -I "$script_root/../../../helpers/" \
    -Wno-format-extra-args -Wall -Werror \
    "$script_root/day07.c" "$script_root/../../../helpers/helpers.c" \
    -o "$script_root/day07.out"

CYAN=$'\033[1;36m'
DARKGRAY=$'\033[1;30m'
NO_COLOUR=$'\033[0m'
echo "${CYAN}Testing simple input...${NO_COLOUR}"
"$script_root/day07.out" "$script_root/day07_simple.txt"
echo "${DARKGRAY}...done${NO_COLOUR}"

echo ""
if [ "$1" == "real" ]
then
    echo "${CYAN}Testing REAL input...${NO_COLOUR}"
    "$script_root/day07.out" "$script_root/day07_real.txt"
    echo "${DARKGRAY}...done${NO_COLOUR}"
fi
