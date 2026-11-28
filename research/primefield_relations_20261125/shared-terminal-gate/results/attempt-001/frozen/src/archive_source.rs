//! Exact historical source replay from 90a0314f5be63d5ca56465a884091f4be0f114fd.
//! Reuses original PRNG and source body, with public visibility only on output.
use std::collections::HashSet;
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
    let mut xs=HashSet::with_capacity(3*T);
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
