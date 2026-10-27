// Exact digit-model cross-checks (PROTOCOL.md section 4.4, claim S6). Tiny fixtures and pure integer arithmetic only.
use g6_large_m::arms::{self, Arm};
use g6_large_m::census;
use g6_large_m::curve::{Curve, Pt};
use g6_large_m::digits;
use g6_large_m::field::Fq;
use g6_large_m::rng::Rng;
use std::collections::HashMap;

/// A point of large order and its Frobenius chain of length l.
fn chain_points(c: &Curve, ne: u32, l: usize, seed: u64) -> Vec<Pt> {
    let f = c.f;
    let mut rng = Rng::new(seed);
    for _ in 0..1000 {
        let x = 2 + rng.below(f.q as u64 - 2) as u32;
        if let Some(y) = c.lift_x(x) {
            let p = Pt::Aff(x, y);
            if digits::point_order(c, p, ne) as f64 > digits::norm_bound(l) {
                let mut v = vec![p];
                for i in 1..l {
                    v.push(c.frob_pt(v[i - 1]));
                }
                return v;
            }
        }
    }
    panic!("no point of large order");
}

fn chain_slots(c: &Curve, pts: &[Pt]) -> Vec<Vec<Pt>> {
    pts.iter().map(|&p| vec![c.t2(), p, c.neg(p)]).collect()
}

#[test]
fn digit_model_equals_group_law_sums_n13() {
    // the model counts digit strings in Z[tau]; the group law counts sums with the T legs included
    for &(a, l) in &[(0u32, 7usize), (1, 7), (0, 6), (1, 6), (0, 5)] {
        let f = Fq::new(13);
        let c = Curve::koblitz(&f, a);
        let ne = c.count();
        let pts = chain_points(&c, ne, l, 5 + a as u64);
        let slots = chain_slots(&c, &pts);
        let mut sums = vec![Pt::Inf];
        for sl in &slots {
            let mut nxt = vec![];
            for &s in &sums {
                for &q in sl {
                    nxt.push(c.add(s, q));
                }
            }
            sums = nxt;
        }
        let mut cnt: HashMap<usize, u64> = HashMap::new();
        for s in &sums {
            *cnt.entry(c.key(*s)).or_insert(0) += 1;
        }
        let s2: f64 = cnt.values().map(|&m| (m * m) as f64).sum();
        let (dist, ms2) = digits::digit_counts(l, a);
        assert_eq!(cnt.len() as u64, dist, "distinct a={a} l={l}");
        assert_eq!(s2, ms2, "sum of squares a={a} l={l}");
    }
    // the reviewer's anchors (a = 0): 543 of 2187 at L = 7, 257 of 729 at L = 6
    assert_eq!(digits::digit_counts(7, 0).0, 543);
    assert_eq!(digits::digit_counts(6, 0).0, 257);
}

#[test]
fn census_of_a_chain_equals_the_prediction_exactly() {
    for &a in &[0u32, 1] {
        let f = Fq::new(13);
        let c = Curve::koblitz(&f, a);
        let ne = c.count();
        let pts = chain_points(&c, ne, 6, 11);
        let mut slots = chain_slots(&c, &pts);
        slots.push(vec![c.t2()]); // size-1 slots are the T leg
        slots.push(vec![c.t2()]);
        let pred = digits::predict_slots(&c, &slots, ne).expect("chain detected");
        let mut h = Vec::new();
        let cen = census::histogram_into(&c, &slots, &mut h);
        let st = census::stats(&h, ne, cen.t);
        assert!((st.p - pred.p).abs() < 1e-15, "p {} vs {}", st.p, pred.p);
        assert!(
            (st.disp - pred.disp).abs() < 1e-9 * pred.disp,
            "disp {} vs {}",
            st.disp,
            pred.disp
        );
        assert!(st.conserved);
    }
}

#[test]
fn detect_chain_rejects_non_chains() {
    let f = Fq::new(13);
    let c = Curve::koblitz(&f, 0);
    let ne = c.count();
    let pts = chain_points(&c, ne, 5, 3);
    let mut slots = chain_slots(&c, &pts);
    assert!(digits::detect_chain(&c, &slots).is_some());
    slots.swap(1, 2); // consecutive Frobenius images broken
    assert!(digits::detect_chain(&c, &slots).is_none());
    let mut s2 = chain_slots(&c, &pts);
    s2[0].push(c.neg(pts[1]));
    assert!(digits::detect_chain(&c, &s2).is_none()); // a slot of another size
}

#[test]
fn real_ni_chain_instance_at_n19_matches_the_prediction() {
    // the actual NI builder at (n, m) = (19, 12): class (0,1) draws are chains of 7 size-3 slots (PROTOCOL 4.4)
    let f = Fq::new(19);
    let c = Curve::koblitz(&f, 0);
    let ne = c.count();
    let mut checked = 0;
    for seed in 0..200u64 {
        let slots = arms::build_normal(&f, &c, Arm::NI, 12, seed);
        let Some(pred) = digits::predict_slots(&c, &slots, ne) else {
            continue;
        };
        assert_eq!(pred.l, 7);
        assert_eq!(pred.distinct, 543);
        let mut h = Vec::new();
        let cen = census::histogram_into(&c, &slots, &mut h);
        let st = census::stats(&h, ne, cen.t);
        assert!((st.p - pred.p).abs() < 1e-15 && (st.disp - pred.disp).abs() < 1e-9 * pred.disp);
        checked += 1;
        if checked >= 3 {
            break;
        }
    }
    assert!(checked >= 1, "no class (0,1) chain draw in 200 seeds");
}

#[test]
fn predicted_ni_vs_generic_ratios_at_the_class_01_cells() {
    // the numbers quoted in PROTOCOL.md 4.4 (UP is ~Poisson-like: eff ~ 1 - mu/2, disp ~ 1)
    for &(n, m, l, ne_log2) in &[
        (23u32, 12usize, 11usize, 23.0f64),
        (23, 14, 9, 23.0),
        (19, 12, 7, 19.0),
    ] {
        let ne = (2f64).powf(ne_log2) as u32; // E ~ 2^n, enough for the printed orders of magnitude
        let pr = digits::predict(l, 0, ne);
        let eff = pr.p / (1.0 - (-pr.mu).exp());
        println!(
            "n={n} m={m} L={l}: tuples={} distinct={} distinct/tuples={:.4} size-biased={:.2} mu={:.4} eff_NI={:.4} (UP ~ {:.4}) disp_NI={:.2}",
            pr.tuples, pr.distinct, pr.distinct as f64 / pr.tuples, pr.size_biased, pr.mu, eff, 1.0 - pr.mu / 2.0, pr.disp
        );
        assert!(pr.distinct as f64 / pr.tuples < 0.3 && pr.size_biased > 5.0);
    }
    assert_eq!(digits::digit_counts(11, 0).0, 9711);
    assert_eq!(digits::digit_counts(9, 0).0, 2335);
}

#[test]
fn norm_bound_and_order_condition_at_the_cell_sizes() {
    // the model is exact when ord(P) exceeds the bound; at L = 11 the bound is ~4.6e4, the orders are ~E/4 >= 2^17 at n >= 19
    assert!(digits::norm_bound(11) < 5.0e4 && digits::norm_bound(17) < 1.0e7);
}
