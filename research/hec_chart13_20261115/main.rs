mod config;
#[allow(dead_code)]
mod target {
    include!("../hec_alignment13_20261115/atlas/target.rs");
    include!("../hec_alignment13_20261115/atlas/target_api.rs");
}
#[allow(dead_code)]
mod good {
    include!("good.rs");
    include!("section_api.rs");
    include!("base_control.rs");
}
#[allow(dead_code)]
mod legacy {
    include!("../hec_alignment13_20261115/atlas/target_program.rs");
    include!("legacy.rs");
}
use target::{QClass, QuarticJacobian};
fn proportional(a: [u64; 8], b: [u64; 8]) -> bool {
    (0..8).all(|i| (0..8).all(|j| a[i] * b[j] % 127 == a[j] * b[i] % 127))
}
fn coord(
    j: &QuarticJacobian,
    s: &good::Sections,
    x: QClass,
    shift: QClass,
    overlaps: &mut usize,
) -> Option<([u64; 8], usize)> {
    let x = j.add_raw(&x, &shift);
    let a = target::graph_chart(j, x, false).and_then(|(u, v)| {
        s.coords_chart(u, v, false)
            .map(|c| (c, good::root_degree(u)))
    });
    let b = target::graph_chart(j, x, true).and_then(|(u, v)| {
        s.coords_chart(u, v, true)
            .map(|c| (c, good::root_degree(u)))
    });
    if let (Some((a, d)), Some((b, e))) = (a, b) {
        assert_eq!(d, e);
        assert!(proportional(a, b), "runtime_chart_covariance");
        *overlaps += 1;
    }
    a.or(b)
}
fn transform(m: &[[u64; 8]; 8], x: [u64; 8]) -> [u64; 8] {
    std::array::from_fn(|i| m[i].iter().zip(x).fold(0, |s, (a, b)| (s + a * b) % 127))
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    let f: [u64; 15] = std::fs::read_to_string(&a[1])
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let cfg = config::discover(f);
    config::install(cfg);
    let cfg = config::get();
    let j = QuarticJacobian::with_points(127, f, cfg.points.clone()).unwrap();
    let s = good::Sections::new();
    good::verify_canonical_line();
    let o = cfg.o;
    let mut h = j.zero();
    for p in j.d_inf {
        let d = target::difference(&j, p, o);
        h = j.add_raw(&h, &j.add_raw(&d, &d));
    }
    for p in cfg.canonical {
        h = j.add_raw(&h, &j.neg_raw(&target::difference(&j, p, o)));
    }
    assert_eq!(
        target::scalar(&j, h, 1028893),
        j.zero(),
        "centering_in_odd_subgroup"
    );
    let beta = target::scalar(&j, h, 514447);
    assert_eq!(j.add_raw(&beta, &beta), h);
    let shift = j.neg_raw(&beta);
    let d = target::difference(&j, cfg.points[0], cfg.points[1]);
    let base = j.add_raw(&d, &d);
    assert_ne!(base, j.zero());
    assert_eq!(target::scalar(&j, base, 1028893), j.zero());
    let sa = a[2].parse::<u64>().unwrap();
    let sb = a[3].parse::<u64>().unwrap();
    let map = |p: [u64; 3]| [(p[0] + sa * p[2]) % 127, (p[1] + sb * p[2]) % 127, p[2]];
    let mut reference = legacy::Reference::new(map(cfg.points[0]), map(cfg.points[1]));
    let mut x = j.zero();
    let (mut accepted, mut overlaps) = (0, 0);
    let mut kinds = [0usize; 4];
    let mut samples = vec![];
    for n in 1..=64 {
        x = j.add_raw(&x, &base);
        let old = reference.next();
        let aa = coord(&j, &s, x, shift, &mut overlaps);
        let bb = coord(&j, &s, j.neg_raw(&x), shift, &mut overlaps);
        let (Some((c, d)), Some((nc, e))) = (aa, bb) else {
            println!("NEGATION_EXCLUDE n={n} reason=unsupported_effective_chart");
            continue;
        };
        assert!(proportional(c, nc), "new_anchor_centered_negation");
        accepted += 1;
        kinds[d] += 1;
        kinds[e] += 1;
        let i = c.iter().position(|a| *a != 0).unwrap();
        let mut bad = nc;
        bad[(i + 1) % 8] = (bad[(i + 1) % 8] + 1) % 127;
        assert!(!proportional(c, bad));
        if let Some(old) = old {
            if samples.len() < 16 {
                samples.push((n, c, old));
            }
        }
    }
    assert!(accepted >= 12 && overlaps > 0);
    assert!(kinds[0] > 0 && kinds[1] > 0, "non_split_class_coverage");
    assert_eq!(samples.len(), 16, "comparison_control_cap64");
    let mut rows = vec![];
    for (_, x, y) in &samples[..12] {
        for i in 0..8 {
            for j in i + 1..8 {
                let mut r = vec![0; 64];
                for k in 0..8 {
                    r[8 * j + k] = y[i] * x[k] % 127;
                    r[8 * i + k] = (127 - y[j] * x[k] % 127) % 127;
                }
                rows.push(r);
            }
        }
    }
    let (rank, k) = good::linear_kernel(rows, 64);
    assert_eq!(rank, 63);
    assert_eq!(k.len(), 1);
    let m: [[u64; 8]; 8] = std::array::from_fn(|i| std::array::from_fn(|j| k[0][8 * i + j]));
    let (mrank, _) = good::linear_kernel(m.iter().map(|r| r.to_vec()).collect(), 8);
    assert_eq!(mrank, 8);
    for (n, x, y) in &samples {
        assert!(
            proportional(transform(&m, *x), *y),
            "independent_legacy_coordinate_change"
        );
        println!(
            "BASIS_CONTROL n={n} training={} pass=true",
            samples[..12].iter().any(|a| a.0 == *n)
        );
    }
    let (_, x, y) = samples[12];
    let mut bad = m;
    let i = x.iter().position(|a| *a != 0).unwrap();
    let j = y.iter().position(|a| *a != 0).unwrap();
    bad[(j + 1) % 8][i] = (bad[(j + 1) % 8][i] + 1) % 127;
    assert!(!proportional(transform(&bad, x), y));
    println!("PASS runtime_chart=true archived_candidate_anchor=false negation_pairs={accepted} graph_overlaps={overlaps} graph_root_types={kinds:?} coordinate_rank={rank} matrix_rank={mrank} holdout=4 corrupt_rejected=true field=127 isomorphism_control_only=true timing_claim=false");
}
