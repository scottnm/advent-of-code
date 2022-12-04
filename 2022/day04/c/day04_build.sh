clear
rm day04.out
clang day04.c ../../../helpers/helpers.c -I ../../../helpers/ -o day04.out -Wno-format-extra-args
./day04.out day04_simple.txt
