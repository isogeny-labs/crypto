//! A checkpointed F4 run that is stopped and resumed is the uninterrupted run:
//! same basis, same steps, matrix sizes and counted field operations.
//!
//! One test in its own binary: `field_ops` is a process-wide counter, so any
//! concurrent F4 run would add to it.

use crypto_lib::cryptanalysis::f4_fp::{
    checkpoint_elapsed, f4, normalise, F4Options, F4Report, Ordering, Poly,
};
use std::time::Duration;

fn lcg(s: &mut u64) -> u64 {
    *s = s
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *s >> 33
}

/// `n` random polynomials of degree 2 in `n` variables over `F_p`.
fn random_quadrics(n: usize, p: u64, seed: u64) -> Vec<Poly> {
    let mut s = seed;
    let mut monos: Vec<Vec<u32>> = vec![vec![0; n]];
    for i in 0..n {
        let mut e = vec![0; n];
        e[i] = 1;
        monos.push(e);
        for j in i..n {
            let mut e = vec![0; n];
            e[i] += 1;
            e[j] += 1;
            monos.push(e);
        }
    }
    (0..n)
        .map(|_| {
            let t: Vec<(Vec<u32>, u64)> =
                monos.iter().map(|m| (m.clone(), lcg(&mut s) % p)).collect();
            normalise(&t, p, Ordering::Grevlex)
        })
        .collect()
}

fn same(a: &F4Report, b: &F4Report, what: &str) {
    assert_eq!(a.basis, b.basis, "{what}: basis");
    assert_eq!(a.inconsistent, b.inconsistent, "{what}: inconsistent");
    assert_eq!(a.degree_reached, b.degree_reached, "{what}: degree_reached");
    assert_eq!(a.solving_degree, b.solving_degree, "{what}: solving_degree");
    assert_eq!(
        a.solving_degree_max, b.solving_degree_max,
        "{what}: solving_degree_max"
    );
    assert_eq!(
        a.max_cols_to_solution, b.max_cols_to_solution,
        "{what}: max_cols_to_solution"
    );
    assert_eq!(
        a.steps_to_solution, b.steps_to_solution,
        "{what}: steps_to_solution"
    );
    assert_eq!(a.steps, b.steps, "{what}: steps");
    assert_eq!(a.max_rows, b.max_rows, "{what}: max_rows");
    assert_eq!(a.max_cols, b.max_cols, "{what}: max_cols");
    assert_eq!(a.field_ops, b.field_ops, "{what}: field_ops");
    assert_eq!(a.blocked_steps, b.blocked_steps, "{what}: blocked_steps");
    assert_eq!(
        a.pairs_above_bound, b.pairs_above_bound,
        "{what}: pairs_above_bound"
    );
    assert_eq!(a.timed_out, b.timed_out, "{what}: timed_out");
}

#[test]
fn a_resumed_run_is_the_uninterrupted_run() {
    let dir = std::env::temp_dir().join(format!("f4-ckpt-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let hour = Duration::from_secs(3600);
    let mut interrupted = 0;
    for &(n, p, seed, bound) in &[
        (4usize, 101u64, 7u64, 24u32),
        (5, 101, 11, 24),
        (5, 32003, 5, 24),
        (6, 101, 3, 24),
        (6, 786_433, 9, 24),
    ] {
        let sys = random_quadrics(n, p, seed);
        let plain = f4(&sys, n, p, &F4Options::new(Ordering::Grevlex, bound));
        assert!(plain.steps >= 3, "n={n} p={p}: only {} steps", plain.steps);
        // Stop at several steps, including a second stop after a resume.
        for stops in [vec![1usize], vec![2], vec![plain.steps - 1], vec![1, 2, 3]] {
            let path = dir.join(format!("c-{n}-{p}-{seed}-{}.ckpt", stops.len()));
            let _ = std::fs::remove_file(&path);
            let _ = std::fs::remove_file(path.with_extension("ckpt.prev"));
            for &k in &stops {
                let mut o =
                    F4Options::new(Ordering::Grevlex, bound).checkpointing(path.clone(), hour);
                o.checkpoint_stop_after = Some(k);
                let r = f4(&sys, n, p, &o);
                assert!(r.timed_out, "n={n} p={p} stop {k}: should have stopped");
                assert!(checkpoint_elapsed(&path).is_some(), "no valid checkpoint");
                interrupted += 1;
            }
            let o = F4Options::new(Ordering::Grevlex, bound).checkpointing(path.clone(), hour);
            let resumed = f4(&sys, n, p, &o);
            same(
                &plain,
                &resumed,
                &format!("n={n} p={p} seed={seed} stops={stops:?}"),
            );
        }
    }
    assert!(interrupted >= 15);

    // In-step snapshots (the wide arithmetic, p > 2^16): stop inside a step's dense
    // row reduction, at column boundaries, and resume from the snapshot.
    let mut in_step = 0;
    for &(n, p, seed) in &[
        (4usize, 786_433u64, 21u64),
        (5, 786_433, 4),
        (6, 786_433, 9),
    ] {
        let sys = random_quadrics(n, p, seed);
        let plain = f4(&sys, n, p, &F4Options::new(Ordering::Grevlex, 24));
        for &(stop_after, stop_in_rref) in &[
            (None, 1usize),
            (None, 7),
            (None, 40),
            (Some(2usize), 3),
            (Some(3), 25),
        ] {
            let path = dir.join(format!(
                "r-{n}-{seed}-{}-{stop_in_rref}.ckpt",
                stop_after.unwrap_or(0)
            ));
            for ext in ["", ".prev", ".rref"] {
                let _ = std::fs::remove_file(format!("{}{ext}", path.display()));
            }
            let mut stops: Vec<(Option<usize>, Option<usize>)> = Vec::new();
            if let Some(k) = stop_after {
                stops.push((Some(k), None));
            }
            stops.push((None, Some(stop_in_rref)));
            let mut stopped_in_step = false;
            for (after, in_rref) in stops {
                let mut o = F4Options::new(Ordering::Grevlex, 24)
                    .checkpointing(path.clone(), hour)
                    .checkpointing_in_step(Duration::ZERO);
                o.checkpoint_stop_after = after;
                o.checkpoint_stop_in_rref = in_rref;
                let r = f4(&sys, n, p, &o);
                if in_rref.is_some() && r.timed_out {
                    stopped_in_step = true;
                    assert!(
                        std::path::Path::new(&format!("{}.rref", path.display())).exists(),
                        "no in-step snapshot after stopping in the reduction"
                    );
                }
            }
            if !stopped_in_step {
                continue; // the run finished before that many snapshots
            }
            let o = F4Options::new(Ordering::Grevlex, 24)
                .checkpointing(path.clone(), hour)
                .checkpointing_in_step(hour);
            let resumed = f4(&sys, n, p, &o);
            same(
                &plain,
                &resumed,
                &format!(
                    "in-step: n={n} p={p} seed={seed} stop_after={stop_after:?} snapshots={stop_in_rref}"
                ),
            );
            in_step += 1;
        }
    }
    assert!(in_step >= 6, "only {in_step} runs stopped inside a step");

    // A checkpoint of another system is refused and the run starts afresh.
    let a = random_quadrics(4, 101, 7);
    let b = random_quadrics(4, 101, 8);
    let path = dir.join("other.ckpt");
    let mut o = F4Options::new(Ordering::Grevlex, 24).checkpointing(path.clone(), hour);
    o.checkpoint_stop_after = Some(1);
    assert!(f4(&a, 4, 101, &o).timed_out);
    let plain_b = f4(&b, 4, 101, &F4Options::new(Ordering::Grevlex, 24));
    let on_b = f4(
        &b,
        4,
        101,
        &F4Options::new(Ordering::Grevlex, 24).checkpointing(path.clone(), hour),
    );
    same(&plain_b, &on_b, "checkpoint of another system");

    // A corrupt newest file falls back to the previous one.
    let path = dir.join("corrupt.ckpt");
    let mut o = F4Options::new(Ordering::Grevlex, 24).checkpointing(path.clone(), hour);
    o.checkpoint_stop_after = Some(1);
    assert!(f4(&a, 4, 101, &o).timed_out);
    o.checkpoint_stop_after = Some(2);
    assert!(f4(&a, 4, 101, &o).timed_out);
    let mut bytes = std::fs::read(&path).unwrap();
    let mid = bytes.len() / 2;
    bytes[mid] ^= 0xff;
    std::fs::write(&path, bytes).unwrap();
    let plain_a = f4(&a, 4, 101, &F4Options::new(Ordering::Grevlex, 24));
    let fell_back = f4(
        &a,
        4,
        101,
        &F4Options::new(Ordering::Grevlex, 24).checkpointing(path.clone(), hour),
    );
    same(&plain_a, &fell_back, "corrupt newest checkpoint");

    // Without `checkpointing` nothing is written.
    let path = dir.join("none.ckpt");
    let _ = f4(&a, 4, 101, &F4Options::new(Ordering::Grevlex, 24));
    assert!(!path.exists());
    std::fs::remove_dir_all(&dir).ok();
}
