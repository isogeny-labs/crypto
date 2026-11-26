//! Exact historical source replay from 90a0314f5be63d5ca56465a884091f4be0f114fd.
//! Original source SHA256 3ed2eb9cf632caf21195c194a781f0563573216f75316ec9facfe7b12c9d1fd4.
//! Measurement-only change: deterministic HashSet hasher; PRNG, ordering,
//! coefficients, duplicate predicate, and arithmetic calls remain unchanged.
//! DefaultHasher is pinned by the measurement's Rust 1.100.0 toolchain.
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::BuildHasherDefault;
use crate::reused_native::{Arithmetic, PRIME31};
const N: u64 = 2_147_446_831;
const T: usize = 1_536;
#[derive(Clone, Copy, Debug)]
pub struct Labelled {
    pub pt: (u64, u64),
    pub a: u64,
    pub b: u64,
}
fn prng(state:&mut u64)->u64 {
    *state=state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z=*state;
    z=(z^(z>>30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z=(z^(z>>27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z^(z>>31)
}
pub fn source(q:(u64,u64),seed:u64,ar:&mut Arithmetic)
    ->Result<([Vec<Labelled>;3],u64,u64),String> {
    let mut rng=seed;
    let mut out:[Vec<Labelled>;3]=std::array::from_fn(|_|Vec::with_capacity(T));
    let mut xs=HashSet::with_capacity_and_hasher(3*T,BuildHasherDefault::<DefaultHasher>::default());
    let mut identity=0_u64;
    let mut duplicated=0_u64;
    for part in &mut out {
        while part.len()<T {
            let a=prng(&mut rng)%N;
            let b=prng(&mut rng)%N;
            if a==0&&b==0 {continue;}
            let ag=ar.scalar(PRIME31,a,Some(PRIME31.g));
            let bq=ar.scalar(PRIME31,b,Some(q));
            let Some(pt)=ar.add(PRIME31,ag,bq) else {
                identity+=1;continue;
            };
            if !xs.insert(pt.0) {duplicated+=1;continue;}
            part.push(Labelled{pt,a,b});
        }
    }
    Ok((out,identity,duplicated))
}
