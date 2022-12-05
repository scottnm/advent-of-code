#!/bin/bash
script_root=$(dirname $(realpath "${BASH_SOURCE:-$0}"))

clear
rm "$script_root/day05.out"
clang "$script_root/day05.c" "$script_root/../../../helpers/helpers.c" -I "$script_root/../../../helpers/" -o "$script_root/day05.out" -Wno-format-extra-args -Wall -Werror
"$script_root/day05.out" "$script_root/day05_simple.txt"
