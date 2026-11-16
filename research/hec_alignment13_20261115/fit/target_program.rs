#[allow(dead_code)]
mod good {
    include!("good.rs");
    include!("section_api.rs");
}
#[allow(dead_code)]
mod target {
    include!("target.rs");
    include!("target_api.rs");
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
) -> Option<([u64; 8], usize)> {
    let x = j.add_raw(&x, &shift);
    let (u, v) = target::graph(j, x)?;
    let c = s.coords(u, v)?;
    Some((c, good::root_degree(u)))
}
fn main() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("CAP aggregate600seconds");
        std::process::exit(124)
    });
    let f = [1, 58, 103, 47, 66, 88, 51, 80, 84, 123, 34, 83, 109, 1, 111];
    let pts = vec![
        [18, 52, 1],
        [28, 86, 1],
        [43, 125, 1],
        [101, 43, 1],
        [34, 113, 1],
    ];
    let j = QuarticJacobian::with_points(127, f, pts).unwrap();
    let s = good::Sections::new();
    good::verify_canonical_line();
    let o = [18, 52, 1];
    let ell = 1028893;
    let mut h = j.zero();
    for p in j.d_inf {
        let d = target::difference(&j, p, o);
        h = j.add_raw(&h, &j.add_raw(&d, &d));
    }
    for p in [[101, 43, 1], [34, 113, 1]] {
        h = j.add_raw(&h, &j.neg_raw(&target::difference(&j, p, o)));
    }
    assert_eq!(
        target::scalar(&j, h, ell),
        j.zero(),
        "H odd-subgroup premise"
    );
    let beta = target::scalar(&j, h, (ell + 1) / 2);
    assert_eq!(j.add_raw(&beta, &beta), h);
    let shift = j.neg_raw(&beta);
    println!(
        "CENTER h={:?} beta={:?} twice_beta_verified=true",
        h.0, beta.0
    );
    let a = target::scalar(&j, target::difference(&j, [28, 86, 1], o), 2);
    assert_ne!(a, j.zero());
    assert_eq!(target::scalar(&j, a, ell), j.zero());
    let (mut accepted, mut irreducible, mut nonsplit, mut wrongcenter) = (0, 0, 0, 0);
    let mut x = j.zero();
    for n in 1..=32 {
        x = j.add_raw(&x, &a);
        let nx = j.neg_raw(&x);
        let (Some((c, d)), Some((nc, nd))) = (coord(&j, &s, x, shift), coord(&j, &s, nx, shift))
        else {
            println!("EXCLUDE n={n} affine_squarefree_chart");
            continue;
        };
        assert!(proportional(c, nc), "negation centering n={n}");
        accepted += 1;
        irreducible += usize::from(d == 0) + usize::from(nd == 0);
        nonsplit += usize::from(d < 3) + usize::from(nd < 3);
        if let (Some((b, _)), Some((nb, _))) =
            (coord(&j, &s, x, j.zero()), coord(&j, &s, nx, j.zero()))
        {
            wrongcenter += usize::from(!proportional(b, nb));
        }
        let mut broken = c;
        let pivot = c.iter().position(|z| *z != 0).unwrap();
        let k = (pivot + 1) % 8;
        broken[k] = (broken[k] + 1) % 127;
        assert!(!proportional(c, broken));
        println!("CENTERED n={n} roots={d}/{nd} positive={c:?} negative={nc:?}");
    }
    assert!(accepted >= 16, "insufficient registered chart coverage");
    assert!(irreducible > 0);
    assert!(nonsplit > 0);
    assert!(
        wrongcenter > 0,
        "centering negative control did not detect error"
    );
    println!("PASS centered_pairs={accepted} irreducible_curve_divisors={irreducible} nonsplit_curve_divisors={nonsplit} wrong_center_detected={wrongcenter} source_alignment=false timing_claim=false");
}
