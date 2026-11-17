//! Where a lane-batched signed-Frobenius rho step spends its time: the
//! batched addition, the canonicalisation (basis change, least rotation,
//! the Frobenius power of x and y), and the whole walk, on one curve.
//!
//! `cargo run --release --example rho_step_profile -- <a> <n> [reps]`
use crypto_lib::cryptanalysis::ic_boundary::{koblitz_instance, ParallelRho};
use crypto_lib::cryptanalysis::koblitz_fast::{
    BatchScratch, FastPoint, FrobeniusCanon, FrobeniusPowers,
};
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let a: u8 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    let n: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(53);
    let reps: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(5);
    let inst = koblitz_instance(a, n).expect("koblitz instance");
    let fc = &inst.fast;
    let canon = FrobeniusCanon::new(&fc.field, n).unwrap();
    assert!(
        canon.frobenius_reps_available(),
        "no batched kernel on this CPU"
    );
    let powers = FrobeniusPowers::new(&fc.field, n);

    // 1024 points: multiples of G by a Weyl sequence of scalars.
    let mut pts: Vec<FastPoint> = Vec::new();
    let mut k = 0x9e37_79b9_7f4au64 % inst.r;
    for _ in 0..1024 {
        pts.push(fc.mul_u64(inst.generator, k.max(2)));
        k = (k
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407))
            % inst.r;
    }
    let lanes = ParallelRho::default_lanes(&inst).max(32);
    let mut scratch = BatchScratch::default();
    let mut out = Vec::new();
    let iters = 20_000usize;

    let mut add_ns = Vec::new();
    let mut canon_ns = Vec::new();
    let mut batch_ns = Vec::new();
    let mut walk_ns = Vec::new();
    let mut walk_scalar_ns = Vec::new();
    let mut walk_steps = 0u64;
    for _ in 0..reps {
        // batched pairwise addition of `lanes` pairs
        let t = Instant::now();
        let mut sink = 0u64;
        for i in 0..iters {
            let o = (i * 7) & 511;
            out.clear();
            fc.add_pairwise(
                &pts[o..o + lanes],
                &pts[512 + (o & 255)..512 + (o & 255) + lanes],
                &mut out,
                &mut scratch,
            );
            sink ^= out[0].x;
        }
        black_box(sink);
        add_ns.push(t.elapsed().as_nanos() as f64 / (iters * lanes) as f64);

        // scalar canon_parts equivalent
        let t = Instant::now();
        let mut sink = 0u64;
        for i in 0..iters * lanes {
            let p = pts[i & 1023];
            let (_, tt) = canon.canon_with_shift(p.x);
            let (x, y) = if tt == 0 {
                (p.x, p.y)
            } else {
                (powers.apply(tt, p.x), powers.apply(tt, p.y))
            };
            let neg = x ^ y;
            sink ^= if neg < y { neg } else { y } ^ x;
        }
        black_box(sink);
        canon_ns.push(t.elapsed().as_nanos() as f64 / (iters * lanes) as f64);

        // batched representatives, `lanes` at a time
        let xs: Vec<u64> = pts.iter().map(|p| p.x).collect();
        let ys: Vec<u64> = pts.iter().map(|p| p.y).collect();
        let mut reps = Vec::new();
        let t = Instant::now();
        let mut sink = 0u64;
        for i in 0..iters {
            let o = (i * 7) & 511;
            canon.frobenius_reps(&xs[o..o + lanes], &ys[o..o + lanes], &mut reps);
            sink ^= reps[0].0;
        }
        black_box(sink);
        batch_ns.push(t.elapsed().as_nanos() as f64 / (iters * lanes) as f64);

        // whole walk on a target, per-point tables then batched, same walk
        let target = fc.mul_u64(inst.generator, 0x1234_5678_9abc % inst.r);
        let mut shapes = [(0.0f64, 0u64); 2];
        for (k, on) in [false, true].into_iter().enumerate() {
            let rho = ParallelRho::new(&inst, 1, ParallelRho::default_lanes(&inst), None)
                .unwrap()
                .with_batched_canon(on);
            let t = Instant::now();
            let res = rho.solve(target, 1 << 40);
            let el = t.elapsed().as_nanos() as f64;
            assert!(res.verified, "walk did not verify");
            shapes[k] = (el / res.steps as f64, res.steps);
        }
        assert_eq!(shapes[0].1, shapes[1].1, "the two walks differ");
        walk_steps = shapes[0].1;
        walk_scalar_ns.push(shapes[0].0);
        walk_ns.push(shapes[1].0);
    }
    let med = |v: &mut Vec<f64>| {
        v.sort_by(|a, b| a.total_cmp(b));
        v[v.len() / 2]
    };
    println!(
        "n={n} a={a} r={} lanes={} walk_steps={} add_ns/pt={:.2} scalar_canon_ns/pt={:.2} batch_canon_ns/pt={:.2} walk_scalar_ns/step={:.2} walk_batched_ns/step={:.2}",
        inst.r,
        ParallelRho::default_lanes(&inst),
        walk_steps,
        med(&mut add_ns),
        med(&mut canon_ns),
        med(&mut batch_ns),
        med(&mut walk_scalar_ns),
        med(&mut walk_ns)
    );
}
