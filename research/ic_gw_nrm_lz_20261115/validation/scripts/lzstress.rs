// Validator stress harness for scalar Lanczos LZ_REUSE (la2::lanczos): CAND without env vs REF la2.rs (counts and
// results), CAND with LZ_REUSE vs without (results identical; lmul/ladd saving = (I-1)(2n-2)-1), oracle la2.rs with
// LZ_REUSE (each reused dot vs the direct dot, stderr ORACLE_LZ). Random sparse matrices, small and large ell.
#![allow(dead_code, unused)]
#[path = "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/val_nl/snap/research/quartic_n4_20261019/common.rs"] mod common;
mod ic { pub type Row = Vec<(u32, u64)>; }
mod ic2 {
use crate::common::*;
#[inline(always)]
pub fn la2(a: u64, b: u64, ell: u64) -> u64 {
    bump_ladd(1);
    let (s, o) = a.overflowing_add(b);
    if o || s >= ell {
        s.wrapping_sub(ell)
    } else {
        s
    }
}
#[inline(always)]
pub fn ls2(a: u64, b: u64, ell: u64) -> u64 {
    bump_ladd(1);
    if a >= b {
        a - b
    } else {
        a + (ell - b)
    }
}
#[inline(always)]
pub fn lm2(a: u64, b: u64, ell: u64) -> u64 {
    bump_lmul(1);
    ((a as u128 * b as u128) % ell as u128) as u64
}
/// mod-ell inversion, charged as 60 lmul (B0 does not charge its Lanczos inversions; noted).
pub fn linv(a: u64, ell: u64) -> u64 {
    bump_lmul(60);
    inv_mod_u64(a, ell)
}
}
#[path = "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/val_nl/snap/research/ic_gw_nrm_lz_20261115/src/la2.rs"] mod la2;
#[path = "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/val_nl/snap/research/ic_gw_bl_reuse_20261115/amendment-1/src/la2.rs"] mod la2ref;
#[path = "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/val_nl/snap/research/ic_gw_nrm_lz_oracle/src/la2.rs"] mod la2orc;
use common::*;
fn main() {
    let trials: u64 = std::env::args().nth(1).map(|s| s.parse().unwrap()).unwrap_or(20000);
    let ells: [u64; 9] = [2, 3, 5, 7, 11, 13, 101, 65537, 4501899029752451];
    let mut g = Rng::new(20261115);
    let (mut ok_ref, mut ok_res, mut ok_cnt, mut bad) = (0u64, 0u64, 0u64, 0u64);
    let (mut n_some, mut n_none, mut n_lz_cheaper) = (0u64, 0u64, 0u64);
    for t in 0..trials {
        let ell = ells[(g.below(ells.len() as u64)) as usize];
        let n = 2 + g.below(40) as usize;
        let nrows = 1 + g.below(2 * n as u64 + 4) as usize;
        let mut rows: Vec<ic::Row> = Vec::new();
        for _ in 0..nrows {
            let w = 1 + g.below(n.min(6) as u64) as usize;
            let mut cols: Vec<u32> = Vec::new();
            while cols.len() < w { let c = g.below(n as u64) as u32; if !cols.contains(&c) { cols.push(c); } }
            cols.sort();
            rows.push(cols.into_iter().map(|c| { let a = match g.below(3) { 0 => 1, 1 => ell - 1, _ => 1 + g.below(ell - 1) }; (c, a % ell) }).filter(|&(_, a)| a != 0).collect());
        }
        let pm1 = g.below(2) == 1;
        let s = g.next();
        std::env::remove_var("LZ_REUSE");
        let c0 = snap(); let r0 = la2::lanczos(&rows, n, ell, &mut Rng(s), pm1); let d0 = snap().since(&c0);
        let c0 = snap(); let rr = la2ref::lanczos(&rows, n, ell, &mut Rng(s), pm1); let dr = snap().since(&c0);
        std::env::set_var("LZ_REUSE", "1");
        let c0 = snap(); let r1 = la2::lanczos(&rows, n, ell, &mut Rng(s), pm1); let d1 = snap().since(&c0);
        let ro = la2orc::lanczos(&rows, n, ell, &mut Rng(s), pm1);
        std::env::remove_var("LZ_REUSE");
        let refeq = r0 == rr && d0.lmul == dr.lmul && d0.ladd == dr.ladd && d0.mul == dr.mul && d0.add == dr.add;
        let reseq = r1 == r0 && ro == r0 && d1.mul == d0.mul && d1.add == d0.add;
        let dl = d0.lmul as i64 - d1.lmul as i64;
        let da = d0.ladd as i64 - d1.ladd as i64;
        let m = 2 * n as i64 - 2;
        let cnt = dl == da && (dl == 0 || ((dl + 1) % m == 0 && (dl + 1) / m >= 0 && (dl + 1) / m <= n as i64 + 6));
        if refeq { ok_ref += 1; } if reseq { ok_res += 1; } if cnt { ok_cnt += 1; }
        if r0.is_some() { n_some += 1; } else { n_none += 1; }
        if dl > 0 { n_lz_cheaper += 1; }
        if !(refeq && reseq && cnt) { bad += 1; if bad <= 10 { println!("BAD trial {} ell {} n {} pm1 {} refeq {} reseq {} dl {} da {}", t, ell, n, pm1, refeq, reseq, dl, da); } }
    }
    println!("STRESS trials={} noenv_equals_ref={} lz_result_equal={} lz_count_formula={} bad={} some={} none={} lz_cheaper={}", trials, ok_ref, ok_res, ok_cnt, bad, n_some, n_none, n_lz_cheaper);
}
