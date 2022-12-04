clear
rm day_template.out
clang day_template.c ../../../helpers/helpers.c -I ../../../helpers/ -o day_template.out -Wno-format-extra-args
./day_template.out day_template_simple.txt
