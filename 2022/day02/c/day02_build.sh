#!/bin/bash
script_root=$(dirname $(realpath "${BASH_SOURCE:-$0}"))

clear
rm "$script_root/day02.out"
clang "$script_root/day02.c" "$script_root/../../../helpers/helpers.c" -I "$script_root/../../../helpers/" -o "$script_root/day02.out" -Wno-format-extra-args
"$script_root/day02.out" "$script_root/day02_simple.txt" pt1
"$script_root/day02.out" "$script_root/day02_simple.txt" pt2
