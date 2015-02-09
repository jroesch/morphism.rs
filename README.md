# morphism.rs

A structure for suspended closure composition in Rust

[![build status](https://api.travis-ci.org/epsilonz/morphism.rs.svg?branch=master)](https://travis-ci.org/epsilonz/morphism.rs)

## Synopsis

The motivation for `Morphism` is to provide a means of composing and evaluating an unbounded (within heap constraints) number of closures without blowing the stack. In other words, `Morphism` is one way to work around the lack of tail-call optimization in Rust.

Because `Morphism` implements the `Fn` trait, it is callable like a normal closure and can even be passed around as if it were a closure. See the second example below for an instance of using `Morphism` with iterators.

## Examples

* Compose `Morphism` with closures or another `Morphism`:



```rust
let mut f = Morphism::new::<u64>();
for _ in range(0u64, 100000u64) {
    f = f.tail(|x| x + 42u64);
}

let mut g = Morphism::new::<Option<u64>>();
for _ in range(0u64,  99999u64) {
    g = g.tail(|x| x.map(|y| y - 42u64));
}

// type becomes Morphism<u64, (Option<u64>, bool, String)> so rebind g
let g = g
    .tail(|x| (x.map(|y| y + 1000u64), String::from_str("welp")))
    .tail(|(l, r)| (l.map(|y| y + 42u64), r))
    .tail(|(l, r)| (l, l.is_some(), r))
    .head(|x| Some(x));

let h = f.then(g);

assert_eq!(h(0u64), (Some(1084), true, String::from_str("welp")));
assert_eq!(h(1000u64), (Some(2084), true, String::from_str("welp")));
```

* Use `Morphism` in place of a closure when a `Fn`-like is expected:

```rust
use std::iter::AdditiveIterator;

let mut f = Morphism::new::<u64>();
for _ in range(0u64, 10000) {
    f = f.tail(|x| x + 42);
}

// ::map treats f like any other Fn
let res = range(0u64, 100).map(f).sum();

assert_eq!(res, 42004950);
```

## Documentation

See the API documentation [here](http://epsilonz.github.io/morphism.rs/doc/morphism/).

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
