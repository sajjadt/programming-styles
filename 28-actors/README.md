### Actors Style

1. Actors with their own execution threads 
```
cargo run --bin actors [path_to_input_file]
```

2. Lazy rivers style implemented using actors
For this version, actors have mailbox of size 1 and messages are sent synchrounosly. Therefore it runs very slow.   
```
cargo run --bin lazy_rivers [path_to_input_file]
```
