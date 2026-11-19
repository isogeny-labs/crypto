//! Prime31 same-runner signed baby-step giant-step CONTROL (generic).
//! Reuses native pinned Prime31 Arithmetic; not an algebraic/RR9 extractor.
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Instant;

const N: u64 = 2_147_446_831;
const M: u64 = 32_768;
const STEP: u64 = 2*M+1;
const G: Point = Some((625_037_715, 2_100_627_251));
const SEED: u64 = 0x7738_4234_4238_3231;
const TARGET_XOR: u64 = 0x5441_5247_4554_5338;

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
fn nonzero_scalar(state: &mut u64) -> u64 {
    let threshold = N.wrapping_neg() % N;
    loop {
        let x = splitmix64(state);
        if x >= threshold {
            let k = x % N;
            if k != 0 { return k; }
        }
    }
}
fn secret_for_target(t: usize) -> u64 {
    let mut st = SEED ^ TARGET_XOR
        ^ (t as u64).wrapping_mul(0xa076_1d64_78bd_642f)
        ^ 0x7365_6372_6574_0001;
    nonzero_scalar(&mut st)
}

struct SharedSignedBsgs {
    // baby_x -> (baby_j, canonical observed y). x alone identifies +/- baby.
    baby: HashMap<u64, (u64, u64)>,
    neg_stride: Point,
}
impl SharedSignedBsgs {
    fn new(ar: &mut Arithmetic) -> Self {
        let mut baby = HashMap::with_capacity((M+1) as usize);
        let mut point = None;
        for j in 1..=M {
            point = ar.add(PRIME31,point,G);
            let (x,y) = point.expect("baby point cannot be infinity");
            assert!(baby.insert(x,(j,y)).is_none(), "unexpected signed baby collision");
        }
        assert_eq!(baby.len(),M as usize);
        let neg_stride = ar.neg(PRIME31,ar.scalar(PRIME31,STEP,G));
        Self { baby,neg_stride }
    }
    fn solve(&self, ar: &mut Arithmetic, target: Point) -> (u64,u64) {
        // For 0 <= d < N, i=floor((d+M)/(2M+1)) and r=d-i*STEP
        // satisfy |r| <= M. Thus 0..=((N-1+M)/STEP) covers every log.
        let max_giant = (N - 1 + M)/STEP;
        let mut giant = target;
        for i in 0..=max_giant {
            if giant.is_none() {
                return ((i*STEP)%N, i+1);
            }
            let (x,y) = giant.expect("nonidentity");
            if let Some(&(j,baby_y)) = self.baby.get(&x) {
                let sign: i8 = if y == baby_y {
                    1
                } else if y == (PRIME31.p-baby_y)%PRIME31.p {
                    -1
                } else { panic!("x match without +/- baby y on original curve") };
                let base = (i*STEP)%N;
                let d = if sign==1 { (base+j)%N } else { (base+N-j)%N };
                // Strong candidate soundness gate, charged via original EC arithmetic.
                if ar.scalar(PRIME31,d,G)==target { return (d,i+1); }
                panic!("false signed BSGS candidate");
            }
            if i < max_giant {
                giant=ar.add(PRIME31,giant,self.neg_stride);
            }
        }
        panic!("full-range signed BSGS coverage failed");
    }
}
fn study(target_count:usize)->Value {
    assert!((1..=1024).contains(&target_count));
    assert_eq!(PRIME31.n,N);
    let start=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).expect("original cofactor-one curve");
    let bsgs=SharedSignedBsgs::new(&mut ar);
    let setup_ns=start.elapsed().as_nanos();
    let setup_adds=ar.counts.group_additions;
    let run_start=Instant::now();
    let mut solutions=Vec::with_capacity(target_count);
    let mut probes=0u64;
    for t in 0..target_count {
        // Fixture-only secret; solver receives ONLY public q.
        let secret=secret_for_target(t);
        let q=ar.scalar(PRIME31,secret,G);
        let (d,search_probes)=bsgs.solve(&mut ar,q);
        assert_eq!(d,secret,"fixture-only scalar audit");
        assert_eq!(ar.scalar(PRIME31,d,G),q,"independent solution replay");
        probes+=search_probes;
        solutions.push(json!({"target_index":t,"public_q":q,"log":d,
            "test_secret":secret,"giant_probes":search_probes}));
    }
    let recovery_ns=run_start.elapsed().as_nanos();
    json!({
        "schema":"prime31.shared-signed-bsgs.v1",
        "date":"2026-11-19",
        "status":"producer-side [pcc:unverified]",
        "generic_algorithm":true,
        "genuine_nongeneric_rr9_extractors":0,
        "curve":{"p":PRIME31.p,"a":PRIME31.a,"b":PRIME31.b,"n":N,
            "g":G,"cofactor":1},
        "target_count":target_count,
        "verified_target_count":solutions.len(),
        "table_entries":bsgs.baby.len(),
        "m":M,"step":STEP,
        "last_giant_index":(N-1+M)/STEP,
        "total_giant_probes":probes,
        "times_ns":{"setup":setup_ns.to_string(),"recovery_and_verification":recovery_ns.to_string()},
        "counts":{"setup_ec_additions":setup_adds,
            "online_ec_additions":ar.counts.group_additions-setup_adds,
            "total_ec_additions":ar.counts.group_additions,
            "total_fp_multiplies":ar.counts.fp_multiplications,
            "total_fp_inversions":ar.counts.fp_inversions},
        "recovered_targets":solutions
    })
}
fn main() {
    println!("{}",serde_json::to_string_pretty(&study(1024)).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_signed_block_coverage_for_all_scalar_boundaries() {
        let max=(N-1+M)/STEP;
        assert_eq!(max,32_767);
        for d in [0,1,M,M+1,STEP-1,STEP,STEP+1,N-1-M,N-2,N-1] {
            let i=(d+M)/STEP;
            let err=(d as i64)-(i*STEP) as i64;
            assert!(i<=max && err.unsigned_abs()<=M);
        }
    }
    #[test]
    fn original_prime31_full_coverage_on_public_32() {
        let v=study(32);
        assert_eq!(v["verified_target_count"],32);
        assert_eq!(v["table_entries"],M);
        assert_eq!(v["generic_algorithm"],true);
        assert_eq!(v["genuine_nongeneric_rr9_extractors"],0);
    }
}