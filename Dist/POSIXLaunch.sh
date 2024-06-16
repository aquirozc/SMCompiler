#!/bin/zsh

rm Salida.ALX
./analizador_lex $1
./parser_slr Salida.ALX
