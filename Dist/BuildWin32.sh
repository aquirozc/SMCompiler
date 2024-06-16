#!/bin/zsh

cargo build --release --manifest-path=../AnalizadorLex/Cargo.toml --target=x86_64-pc-windows-gnu
cp ../AnalizadorLEX/target/x86_64-pc-windows-gnu/release/analizador_lex.exe .
cargo build --release --manifest-path=../ParserSLR/Cargo.toml --target=x86_64-pc-windows-gnu
cp ../ParserSLR/target/x86_64-pc-windows-gnu/release/parser_slr.exe .

