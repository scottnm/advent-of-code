#!/bin/bash
script_root=$(dirname $(realpath "${BASH_SOURCE:-$0}"))

clear
rm "$script_root/day_template.out"
clang "$script_root/day_template.c" "$script_root/../../../helpers/helpers.c" -I "$script_root/../../../helpers/" -o "$script_root/day_template.out" -Wno-format-extra-args
"$script_root/day_template.out" "$script_root/day_template_simple.txt"
