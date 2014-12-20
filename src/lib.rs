#![crate_name="morphism"]
#![crate_type="lib"]

#![doc(html_root_url = "http://www.rust-ci.org/epsilonz/morphism.rs/doc/morphism/")]

//! This crate provides a structure for suspended closure composition.
//! Composition is delayed and executed in a loop when a `Morphism` is
//! applied to an argument.

#![feature(unboxed_closures)]

use std::collections::dlist::{
    DList,
};
use std::collections::ring_buf::{
    RingBuf,
};
use std::mem::{
    transmute,
};

/// A suspended chain of closures.
pub struct Morphism<'a, A, B> {
    mfns: DList<RingBuf<Box<Fn<(*const u8,), *const u8> + 'a>>>,
}

impl<'a, A:'a> Morphism<'a, A, A> {
    /// Create the identity chain.
    ///
    /// # Example
    ///
    /// ```rust
    /// use morphism::Morphism;
    ///
    /// let f: Morphism<uint, uint> = Morphism::new();
    /// assert_eq!(f(42u), 42u);
    /// ```
    #[inline]
    pub fn new() -> Morphism<'a, A, A> {
        Morphism {
            mfns: {
                let mut mfns = DList::new();
                mfns.push_back(RingBuf::new());
                mfns
            },
        }
    }
}

impl<'a, B: 'a, C: 'a> Morphism<'a, B, C> {
    /// Attach a closure to the front of the closure chain. This corresponds to
    /// closure composition at the domain (pre-composition).
    ///
    /// # Example
    ///
    /// ```rust
    /// use morphism::Morphism;
    ///
    /// let f: Morphism<Option<String>, Option<String>> = Morphism::new();
    /// let f = f // becomes Morphism<uint, Option<String>>
    ///     .head(|x: Option<uint>| x.map(|y| y.to_string()))
    ///     .head(|x: Option<uint>| x.map(|y| y - 42u))
    ///     .head(|x: uint| Some(x + 42u + 42u));
    /// assert_eq!(f(0u), Some(String::from_str("42")));
    /// ```
    #[inline]
    pub fn head<A, F:'a>(self, f: F) -> Morphism<'a, A, C>
        where
        F: Fn<(A,), B>,
    {
        match self {
            Morphism {
                mut mfns
            }
            =>
            {
                // assert!(!mfns.is_empty())
                { // borrow mfns
                    let head = mfns.front_mut().unwrap();
                    let g = box move |&:ptr: *const u8| { unsafe {
                        transmute::<Box<B>, *const u8>(
                            box f.call((
                                *transmute::<*const u8, Box<A>>(ptr)
                            ,))
                        )
                    }};
                    head.push_front(g);
                }; // forget mfns
                Morphism {
                    mfns: mfns,
                }
            },
        }
    }
}

impl<'a, A: 'a, B: 'a> Morphism<'a, A, B> {
    /// Attach a closure to the back of the closure chain. This corresponds to
    /// closure composition at the codomain (post-composition).
    ///
    /// # Example
    ///
    /// ```rust
    /// use morphism::Morphism;
    ///
    /// let f: Morphism<uint, uint> = Morphism::new();
    /// let f = f // becomes Morphism<uint, Option<String>>
    ///     .tail(|x| Some(x + 42u + 42u))
    ///     .tail(|x| x.map(|y| y - 42u))
    ///     .tail(|x| x.map(|y| y.to_string()));
    /// assert_eq!(f(0u), Some(String::from_str("42")));
    /// ```
    #[inline]
    pub fn tail<C, F:'a>(self, f: F) -> Morphism<'a, A, C>
        where
        F: Fn<(B,), C>,
    {
        match self {
            Morphism {
                mut mfns
            }
            =>
            {
                // assert!(!mfns.is_empty())
                { // borrow mfns
                    let tail = mfns.back_mut().unwrap();
                    let g = box move |&:ptr: *const u8| { unsafe {
                        transmute::<Box<C>, *const u8>(
                            box f.call((
                                *transmute::<*const u8, Box<B>>(ptr)
                            ,))
                        )
                    }};
                    tail.push_back(g);
                }; // forget mfns
                Morphism {
                    mfns: mfns,
                }
            },
        }
    }

    /// Compose one `Morphism` with another.
    ///
    /// # Example
    ///
    /// ```rust
    /// use morphism::Morphism;
    ///
    /// let mut f: Morphism<uint, uint> = Morphism::new();
    /// for _ in range(0u, 100000u) {
    ///     f = f.tail(|x| x + 42u);
    /// }
    /// // the type changes to Morphism<uint, Option<uint>> so rebind f
    /// let f = f.tail(|x| Some(x));
    ///
    /// let mut g: Morphism<Option<uint>, Option<uint>> = Morphism::new();
    /// for _ in range(0u,  99999u) {
    ///     g = g.tail(|x| x.map(|y| y - 42u));
    /// }
    /// // the type changes to Morphism<Option<uint>, String> so rebind g
    /// let g = g.tail(|x| x.map(|y| y + 1000u).to_string());
    ///
    /// assert_eq!(f.then(g)(0u), String::from_str("Some(1042)"));
    /// ```
    #[inline]
    pub fn then<C>(self, other: Morphism<'a, B, C>) -> Morphism<'a, A, C> {
        match self {
            Morphism {
                mfns: mut lhs,
            }
            =>
            {
                match other {
                    Morphism {
                        mfns: rhs,
                    }
                    =>
                    {
                        Morphism {
                            mfns: {
                                lhs.append(rhs);
                                lhs
                            },
                        }
                    },
                }
            },
        }
    }

    /// Given an argument, run the chain of closures in a loop and return the
    /// final result.
    #[inline]
    fn run(&self, x: A) -> B { unsafe {
        let mut res = transmute::<Box<A>, *const u8>(box x);
        for fns in self.mfns.iter() {
            for f in fns.iter() {
                res = f.call((res,));
            }
        }
        *transmute::<*const u8, Box<B>>(res)
    }}
}

// NOTE: we can't implement this for FnOnce; see #18835
impl<'a, A:'a, B:'a> Fn<(A,), B> for Morphism<'a, A, B> {
    extern "rust-call" fn call(&self, (x,): (A,)) -> B {
        self.run(x)
    }
}

#[test]
fn readme() {
    let mut f: Morphism<uint, uint> = Morphism::new();
    for _ in range(0u, 100000u) {
        f = f.tail(|x| x + 42u);
    }

    let mut g: Morphism<Option<uint>, Option<uint>> = Morphism::new();
    for _ in range(0u,  99999u) {
        g = g.tail(|x| x.map(|y| y - 42u));
    }

    // type becomes Morphism<uint, (Option<uint>, bool, String)> so rebind g
    let g = g
        .tail(|x| (x.map(|y| y + 1000u), String::from_str("welp")))
        .tail(|(l, r)| (l.map(|y| y + 42u), r))
        .tail(|(l, r)| (l, l.is_some(), r))
        .head(|x| Some(x));

    let h = f.then(g);

    assert_eq!(h(0u), (Some(1084), true, String::from_str("welp")));
    assert_eq!(h(1000u), (Some(2084), true, String::from_str("welp")));
}
