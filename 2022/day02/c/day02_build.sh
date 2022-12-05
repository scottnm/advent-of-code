#!/bin/bash
script_root=$(dirname $(realpath "${BASH_SOURCE:-$0}"))

clear
rm "$script_root/day02.exe"
clang "$script_root/day02.c" "$script_root/../../../helpers/helpers.c" -I "$script_root/../../../helpers/" -o "$script_root/day02.exe" -Wno-format-extra-args -Wno-deprecated-declarations -O2
"$script_root/day02.exe" "$script_root/day02_simple.txt" 
"$script_root/day02.exe" "$script_root/day02_real.txt"
