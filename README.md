# Playground for playing with the Rust language

The project was created to play with features of the Rust language, so the project uses workspaces(packages). The project contains the following workspaces:
- main workspace (`src`) - solving Codewars challanges (Kata's);
- `tmdb_cli` workspace - CLI for working with the [TMDB](https://www.themoviedb.org) service (*mininal functionality*)

## Codewars

All challanges are located inside the Test module.
```bash
& cargo test 
# OR - some tasks have benchmarks
& cargo bench
```

## TMDb CLI

The idea was to try [clap](https://github.com/clap-rs/clap)(`Derive` way) library to implement CLI in Rust with async calls to [TMDB API](https://developers.themoviedb.org/3).

Dependencies:
- [clap](https://github.com/clap-rs/clap) - full featured, fast Command Line Argument Parser for Rust;
- [reqwest](https://github.com/seanmonstar/reqwest) - easy and powerful Rust HTTP Client;
- [tokio](https://github.com/tokio-rs/tokio) - asynchronous runtime for the Rust programming language; 
- [serde](https://github.com/serde-rs/serde) - serialization framework for Rust;

To build package just run the following command:
```bash
& cargo run --package tmdb_cli
```

## 📝 License

Copyright © 2022 [Rodion Belovitskiy](https://github.com/majorkik). <br />
This project is MIT licensed.
