# morphism.rs

A structure for suspended closure composition in Rust

[![build status](https://api.travis-ci.org/epsilonz/morphism.rs.svg?branch=master)](https://travis-ci.org/epsilonz/morphism.rs)

## Example

```rust
let mut fm: Morphism<uint, uint> = Morphism::new();
for _ in range(0u, 100000u) {
    fm = fm.push(|:x| x + 42u);
}
let mut gm: Morphism<uint, uint> = Morphism::new();
for _ in range(0u, 100000u) {
    gm = gm.push(|:x| x - 42u);
}
let gm = gm
    .push(|:x| Some(x))
    .push(|:x| x.map(|y| y + 42u))
    .push(|:x| x.map(|y| y.to_string()));
assert_eq!(fm.then(gm).run(0u), Some(String::from_str("42")));
```

## Documentation

See the API documentation [here](http://www.rust-ci.org/epsilonz/morphism.rs/doc/morphism/).

## Requirements

1.   [Rust](http://www.rust-lang.org/)
2.   [Cargo](http://crates.io/)

You can install both with the following:

```
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

See [Installing Rust](http://doc.rust-lang.org/guide.html#installing-rust) for further details.

## Usage

```
$ cargo build       ## build library and binary
$ cargo test        ## run tests in ./tests
$ cargo bench       ## run benchmarks in ./benches
```

## Discussion

There is an IRC channel on [freenode](https://freenode.net) (chat.freenode.net) at [#epsilonz](http://webchat.freenode.net/?channels=%23epsilonz).
