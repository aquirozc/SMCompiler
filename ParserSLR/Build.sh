!#/bin/zsh

cargo build --release
cp ./target/release/parser_slr ../Dist
