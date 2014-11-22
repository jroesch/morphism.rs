#![crate_name="morphism"]
#![crate_type="lib"]

#![license = "MIT"]
#![doc(html_root_url = "http://www.rust-ci.org/epsilonz/morphism.rs/doc/morphism/")]

//! This crate provides a monoid for suspended closure composition. Composition
//! is delayed and executed in a loop when `Morphism` is applied to an argument
//! with `Morphism::run`.

#![feature(unboxed_closures)]

use std::collections::ring_buf::{
    RingBuf,
};

use std::mem::{
    transmute,
};

/// A suspended chain of functions.
pub struct Morphism<'a, A, B> {
    fns: RingBuf<Box<FnOnce<(*const u8,), *const u8> + 'a>>,
}

impl<'a, A:'a> Morphism<'a, A, A> {
    /// Creates the identity function chain.
    ///
    /// # Example
    ///
    /// ```rust
    /// use morphism::Morphism;
    ///
    /// let f: Morphism<uint, uint> = Morphism::new();
    /// assert_eq!(f.run(42u), 42u);
    /// ```
    pub fn new() -> Morphism<'a, A, A> {
        Morphism {
            fns: RingBuf::new(),
        }
    }
}

impl<'a, A: 'a, B: 'a> Morphism<'a, A, B> {
    /// Push a new function onto the end of the chain. This corresponds to
    /// function composition.
    ///
    /// # Example
    ///
    /// ```rust
    /// use morphism::Morphism;
    ///
    /// let f: Morphism<uint, uint> = Morphism::new();
    /// let f = f
    ///     .push(|x| x * 42u)
    ///     .push(|x| x - 42u);
    /// assert_eq!(f.run(42u), 1722u);
    /// ```
    pub fn push<C, F:'a>(self, f: F) -> Morphism<'a, A, C>
        where
        F: FnOnce<(B,), C>,
    {
        match self {
            Morphism {
                mut fns
            } => {
                let g = box move |:ptr: *const u8| { unsafe {
                    transmute::<Box<C>, *const u8>(
                        box f.call_once((
                            *transmute::<*const u8, Box<B>>(ptr)
                        ,))
                    )
                }};
                fns.push_back(g);
                Morphism {
                    fns: fns,
                }
            },
        }
    }

    /// Compose with another `Morphism`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use morphism::Morphism;
    ///
    /// let mut f: Morphism<uint, uint> = Morphism::new();
    /// for _ in range(0u, 100000u) {
    ///     f = f.push(|x| x + 42u);
    /// }
    /// // the type changes to Morphism<uint, Option<uint>> so rebind f
    /// let f = f.push(|x| Some(x));
    ///
    /// let mut g: Morphism<Option<uint>, Option<uint>> = Morphism::new();
    /// for _ in range(0u,  99999u) {
    ///     g = g.push(|x| x.map(|y| y - 42u));
    /// }
    /// // the type changes to Morphism<Option<uint>, String> so rebind g
    /// let g = g.push(|x| x.map(|y| y + 1000u).to_string());
    ///
    /// assert_eq!(f.then(g).run(0u), String::from_str("Some(1042)"));
    /// ```
    pub fn then<C>(self, other: Morphism<'a, B, C>) -> Morphism<'a, A, C> {
        match self {
            Morphism {
                fns: mut lhs,
            } => {
                match other {
                    Morphism {
                        fns: mut rhs,
                    } => {
                        Morphism {
                            fns: {
                                if lhs.len() > rhs.len() {
                                    loop { match rhs.pop_front() {
                                        None => break,
                                        Some(f) => lhs.push_back(f),
                                    }};
                                    lhs
                                } else {
                                    loop { match lhs.pop_back() {
                                        None => break,
                                        Some(f) => rhs.push_front(f),
                                    }};
                                    rhs
                                }
                            }
                        }
                    },
                }
            },
        }
    }

    /// Given an argument, run the chain of functions in a loop and return the
    /// final result.
    pub fn run(mut self, x: A) -> B { unsafe {
        let mut res = transmute::<Box<A>, *const u8>(box x);
        loop { match self.fns.pop_front() {
            None => {
                break;
            },
            Some(f) => {
                res = f.call_once((res,));
            },
        }}
        *transmute::<*const u8, Box<B>>(res)
    }}
}

// FIXME: we can't implement this at the moment; see #18835
// impl<'a, A, B> FnOnce<(A,), B> for Morphism<'a, A, B> {
//     extern "rust-call" fn call_once(self, x:A) -> B {
//         unimplemented!()
//     }
// }

#[test]
fn test() {
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
    let hm = fm.then(gm);
    let res = hm.run(0u);
    assert_eq!(res, Some(String::from_str("42")));
}
