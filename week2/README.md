# prg_exercise
Use cargo to run each program. To select the implementaion (i.e., pipeline or cookbook) use --bin flag.

cargo run --bin [imp] [input file] [stop_words_file (was set as an additional constraint only for pipeline version)]


e.g.:

```console
cd week2
cargo run --bin cookbook ../pride-and-prejudice.txt
cargo run --bin pipeline ../pride-and-prejudice.txt ../stop_words.txt
```