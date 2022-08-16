# collatz-conjecture-check

collatz-conjecture-check is a simple CLI tool that will output the path a given value would take to `1` following the [Collatz conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture). 

## Usage
```
USAGE:
    collatz-conjecture-check.exe --n <VALUE_YOU_WANT_TO_EXPLORE>

OPTIONS:
    -n, --n                                Starting value        
    -h, --help                             Print help information      
    -V, --version                          Print version information
```
To run with `cargo`, simply call `cargo run -- n <test value>`. Example:
```
cargo run -- -n 14

14, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1,
```

## Building for Release
```
cargo build --release
```

## Dependencies
* [clap](https://crates.io/crates/clap)