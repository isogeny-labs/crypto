//! Dependency-free bookkeeping test of the real ecbench model source.
//! This shim does NOT establish that the root crate builds or replays.
mod cryptanalysis {
    pub mod ic_boundary {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct GroupOps {
            pub adds: u64,
            pub doubles: u64,
            pub scalar_mults: u64,
        }
        pub trait CountedGroup {
            type Elt: Copy + PartialEq + std::fmt::Debug;
            fn identity(&self) -> Self::Elt;
            fn is_identity(&self, p: &Self::Elt) -> bool;
            fn add(&self, o: &mut GroupOps, p: Self::Elt, q: Self::Elt) -> Self::Elt;
            fn double(&self, o: &mut GroupOps, p: Self::Elt) -> Self::Elt;
            fn neg(&self, p: Self::Elt) -> Self::Elt;
            fn key(&self, p: &Self::Elt) -> u64;
            fn mul(&self, o: &mut GroupOps, p: Self::Elt, k: u64) -> Self::Elt {
                o.scalar_mults += 1;
                let mut q = self.identity();
                let mut p = p;
                let mut k = k;
                while k > 0 {
                    if k & 1 == 1 {
                        q = self.add(o, q, p);
                    }
                    k >>= 1;
                    if k > 0 {
                        p = self.double(o, p);
                    }
                }
                q
            }
        }
        pub fn addmod(a: u64, b: u64, m: u64) -> u64 {
            ((a as u128 + b as u128) % m as u128) as u64
        }
        pub fn mulmod(a: u64, b: u64, m: u64) -> u64 {
            ((a as u128 * b as u128) % m as u128) as u64
        }
        pub fn invmod(a: u64, m: u64) -> Option<u64> {
            if a == 0 {
                return None;
            }
            let (mut x, mut y, mut s, mut t) = (m as i128, a as i128, 0i128, 1i128);
            while y != 0 {
                let q = x / y;
                (x, y) = (y, x - q * y);
                (s, t) = (t, s - q * t);
            }
            if x == 1 {
                Some(s.rem_euclid(m as i128) as u64)
            } else {
                None
            }
        }
    }
}
mod generic {
    use crate::cryptanalysis::ic_boundary::GroupOps;
    #[derive(Default)]
    pub struct GenericOutcome {
        pub recovered: Option<u64>,
        pub setup: GroupOps,
        pub search: GroupOps,
        pub counters: std::collections::BTreeMap<String, u64>,
        pub exhausted: bool,
    }
}
#[path = "../../src/cryptanalysis/ecbench/best2_model.rs"]
mod best2_model;
