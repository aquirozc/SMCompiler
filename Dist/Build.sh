#!/bin/zsh

cargo build --release --manifest-path=../AnalizadorLex/Cargo.toml
cp ../AnalizadorLEX/target/release/analizador_lex .
cargo build --release --manifest-path=../ParserSLR/Cargo.toml
cp ../ParserSLR/target/release/parser_slr .

