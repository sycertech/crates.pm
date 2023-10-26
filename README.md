# search.crates.pm
A crates.io indexer for Meilisearch based on [their demos](https://github.com/meilisearch/demos)

## Usage
### Prerequisites
- Ensure validity of `.envrc`.
- Install [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall#installation)
- Install [`cargo-make`](https://github.com/sagiegurari/cargo-make)

```console
cargo binstall cargo-make
```

### Services
Start Meilisearch with:

```console
./y.sh dev up --wait
```

This will also start an instance of [`dozzle`](https://github.com/amir20/dozzle), which is at http://localhost:9999.
You can also open Meilisearch at http://localhost:7700.

## Initial Run: Loading All Crates
For the first load, we use the crates.io-index.

1. Clone [`rust-lang/crates.io-index`](https://github.com/rust-lang/crates.io-index )

```console
makers clone-index
```

2. Run [`full_init`](./src/bin/full_init.rs)
```console
makers full-init
```
