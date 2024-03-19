# Simple text analyzer for large files

## How to build

`cargo build --release`


## How to load data

`cargo run -- --file combined.txt --load`


## To generate map of words/frequency

`cargo run -- --file combined.txt frequency-map`


## To search for a word in file

`cargo run -- --file combined.txt search License`


## To run tests

`cargo test`