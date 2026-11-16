//! L2's harness (`research/ecc2k130_open7_20261114/L2-refutation-filters/src/main.rs`,
//! sha256 f16bda7a…, and its `analyze.rs`), included verbatim so the Macaulay
//! kernel timed here is byte-for-byte the code whose word ops L2 counted.
//! Only the hook below is new.
#![allow(dead_code, unused_imports, unused_variables, unused_mut, clippy::all)]

include!("l2_main_verbatim.rs");

/// One timed Macaulay call on one of L2's streamed targets.
pub struct LinzTiming {
    pub target: usize,
    pub xr: u32,
    pub sat: bool,
    pub dd: u32,
    pub refuted: bool,
    pub word_ops: u64,
    pub row_xors: u64,
    pub cols: usize,
    pub ns: u128,
}

/// Rebuild L2's `linz` stream (`run_linz`: same field, curve, base, seed and
/// stream law) and time `macaulay` on its first `targets` targets, each degree
/// in `ds` interleaved per target as L2 ran them.  The descent system is built
/// outside the timed region; `macaulay` alone is timed, so the price per word
/// op includes the kernel's row construction, pivot scans and allocation that
/// L2's word-op counter does not count.
pub fn time_linz(n: u32, a: u32, m: u32, d: u32, samples: usize, seed: u64, targets: usize, ds: &[u32]) -> Vec<LinzTiming> {
    let f = Field::new(n);
    let e = Curve { f: &f, a };
    let s = setup(&f, &e);
    let fb = base_points(&e, d, false);
    let (dm, _) = decomposable(&e, &fb, m as usize);
    let ux: Vec<u32> = (0..f.q).filter(|&x| s.gx[x as usize]).collect();
    let mut rng = Rng(seed);
    let stream: Vec<u32> = (0..samples).map(|_| ux[rng.below(ux.len())]).collect();
    let mut out = Vec::new();
    for (t, &xr) in stream.iter().enumerate().take(targets) {
        let (eqs, nv) = descent(&f, xr, m, d);
        let sat = dm[xr as usize];
        for &dd in ds {
            let t0 = std::time::Instant::now();
            let r = macaulay(std::hint::black_box(&eqs), nv, dd);
            let ns = t0.elapsed().as_nanos();
            std::hint::black_box(&r.rank);
            out.push(LinzTiming { target: t, xr, sat, dd, refuted: r.refuted, word_ops: r.word_ops, row_xors: r.row_xors, cols: r.cols, ns });
        }
    }
    out
}
