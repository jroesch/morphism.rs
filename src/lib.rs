#![crate_name="morphism"]
#![crate_type="lib"]

#![license = "MIT"]
#![doc(html_root_url = "http://www.rust-ci.org/epsilonz/morphism.rs/doc/morphism/")]

//! This crate provides a structure for suspended closure composition.
//! Composition is delayed and executed in a loop when `Morphism` is applied to
//! an argument with `Morphism::run`.

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
    mfns: DList<RingBuf<Box<FnOnce<(*const u8,), *const u8> + 'a>>>,
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
    /// assert_eq!(f.run(42u), 42u);
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
    /// let f: Morphism<uint, Option<String>> = Morphism::new()
    ///     .dom(|x: Option<uint>| x.map(|y| y.to_string()))
    ///     .dom(|x: Option<uint>| x.map(|y| y - 42u))
    ///     .dom(|x: uint| Some(x + 42u + 42u));
    /// assert_eq!(f.run(0u), Some(String::from_str("42")));
    /// ```
    #[inline]
    pub fn dom<A, F:'a>(self, f: F) -> Morphism<'a, A, C>
        where
        F: FnOnce<(A,), B>,
    {
        match self {
            Morphism {
                mut mfns
            } => {
                {
                    // assert!(!mfns.is_empty())
                    let head = mfns.front_mut().unwrap();
                    let g = box move |:ptr: *const u8| { unsafe {
                        transmute::<Box<B>, *const u8>(
                            box f.call_once((
                                *transmute::<*const u8, Box<A>>(ptr)
                            ,))
                        )
                    }};
                    head.push_front(g);
                };
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
    /// let f: Morphism<uint, Option<String>> = Morphism::new()
    ///     .cod(|x: uint| Some(x + 42u + 42u))
    ///     .cod(|x: Option<uint>| x.map(|y| y - 42u))
    ///     .cod(|x: Option<uint>| x.map(|y| y.to_string()));
    /// assert_eq!(f.run(0u), Some(String::from_str("42")));
    /// ```
    #[inline]
    pub fn cod<C, F:'a>(self, f: F) -> Morphism<'a, A, C>
        where
        F: FnOnce<(B,), C>,
    {
        match self {
            Morphism {
                mut mfns
            } => {
                {
                    // assert!(!mfns.is_empty())
                    let tail = mfns.back_mut().unwrap();
                    let g = box move |:ptr: *const u8| { unsafe {
                        transmute::<Box<C>, *const u8>(
                            box f.call_once((
                                *transmute::<*const u8, Box<B>>(ptr)
                            ,))
                        )
                    }};
                    tail.push_back(g);
                };
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
    ///     f = f.cod(|x| x + 42u);
    /// }
    /// // the type changes to Morphism<uint, Option<uint>> so rebind f
    /// let f = f.cod(|x| Some(x));
    ///
    /// let mut g: Morphism<Option<uint>, Option<uint>> = Morphism::new();
    /// for _ in range(0u,  99999u) {
    ///     g = g.cod(|x| x.map(|y| y - 42u));
    /// }
    /// // the type changes to Morphism<Option<uint>, String> so rebind g
    /// let g = g.cod(|x| x.map(|y| y + 1000u).to_string());
    ///
    /// assert_eq!(f.then(g).run(0u), String::from_str("Some(1042)"));
    /// ```
    pub fn then<C>(self, other: Morphism<'a, B, C>) -> Morphism<'a, A, C> {
        match self {
            Morphism {
                mfns: mut lhs,
            } => {
                match other {
                    Morphism {
                        mfns: rhs,
                    } => {
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
    pub fn run(mut self, x: A) -> B { unsafe {
        let mut res = transmute::<Box<A>, *const u8>(box x);
        'morphism: loop {
            match self.mfns.pop_front() {
                None => {
                    break 'morphism;
                },
                Some(mut fns) => {
                    'closure: loop {
                        match fns.pop_front() {
                            None => {
                                break 'closure;
                            },
                            Some(f) => {
                                res = f.call_once((res,));
                            },
                        }
                    }
                },
            }
        };
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
        fm = fm.cod(|:x| x + 42u);
    }
    let mut gm: Morphism<uint, uint> = Morphism::new();
    for _ in range(0u, 100000u) {
        gm = gm.cod(|:x| x - 42u);
    }
    let gm = gm
        .cod(|:x| Some(x))
        .cod(|:x| x.map(|y| y + 42u))
        .cod(|:x| x.map(|y| y.to_string()));
    assert_eq!(fm.then(gm).run(0u), Some(String::from_str("42")));
}
