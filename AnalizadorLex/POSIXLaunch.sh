!#/bin/zsh

cargo build --release
time ./target/release/analizador_lex Sample.txt
