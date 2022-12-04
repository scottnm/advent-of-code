clear
rm day03.out
clang day03.c ../../../helpers/helpers.c -I ../../../helpers/ -o day03.out -Wno-format-extra-args
./day03.out day03_simple.txt
