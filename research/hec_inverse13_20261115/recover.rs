include!("verify_poly.rs");
#[allow(dead_code)]
mod exact {
    include!("verify.rs");
    include!("exact_api.rs");
}
#[allow(dead_code)]
mod target_program {
    include!("../hec_alignment13_20261115/atlas/target_program.rs");
    include!("recover_api.rs");
}
use std::{collections::HashSet, fs};
fn rng(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
fn edf(f: U, d: usize, s: &mut u64, out: &mut Vec<U>) {
    if f.len() - 1 == d {
        out.push(f);
        return;
    }
    for _ in 0..256 {
        let a: U = (0..f.len() - 1).map(|_| rng(s) % 127).collect();
        let h = add(&mpow(a, power_u(127, d) / 2, &f), &[1], true);
        let g = gcd(f.clone(), h);
        if g.len() > 1 && g.len() < f.len() {
            let (q, r) = div(&f, &g);
            assert!(r.is_empty());
            edf(g, d, s, out);
            edf(q, d, s, out);
            return;
        }
    }
    panic!("factor split cap")
}
fn power_u(a: u64, d: usize) -> u64 {
    (0..d).fold(1, |v, _| v * a)
}
fn factors(e: U) -> Vec<U> {
    let mut f = e;
    let mut xp = vec![0, 1];
    let mut out = Vec::new();
    let mut seed = 20261115;
    for d in 1..=3 {
        xp = mpow(xp, 127, &f);
        let g = gcd(f.clone(), add(&xp, &[0, 1], true));
        if g.len() > 1 {
            edf(g.clone(), d, &mut seed, &mut out);
            let (q, r) = div(&f, &g);
            assert!(r.is_empty());
            f = q;
            if f.len() == 1 {
                break;
            }
            xp = rem(&xp, &f);
        }
    }
    assert_eq!(f, vec![1], "unexpected factor degree");
    out.sort();
    out
}
fn ucoeff(x: &[U], m: &[u64]) -> Option<([u64; 4], [u64; 3])> {
    let xs = [x[1].clone(), x[3].clone(), x[5].clone()];
    let ys = [x[2].clone(), x[4].clone(), x[6].clone()];
    let mut u = vec![vec![1]];
    for a in &xs {
        let mut z = vec![vec![]; u.len() + 1];
        for i in 0..u.len() {
            z[i] = add(&z[i], &mm(&u[i], a, m), true);
            z[i + 1] = add(&z[i + 1], &u[i], false);
        }
        u = z;
    }
    let mut v = vec![vec![]; 3];
    for i in 0..3 {
        let mut top = vec![vec![1]];
        let mut den = vec![1];
        for j in 0..3 {
            if i == j {
                continue;
            }
            let diff = add(&xs[i], &xs[j], true);
            if diff.is_empty() {
                return None;
            }
            den = mm(&den, &diff, m);
            let mut z = vec![vec![]; top.len() + 1];
            for k in 0..top.len() {
                z[k] = add(&z[k], &mm(&top[k], &xs[j], m), true);
                z[k + 1] = add(&z[k + 1], &top[k], false);
            }
            top = z;
        }
        let c = mm(&ys[i], &inverse(&den, m), m);
        for k in 0..3 {
            v[k] = add(&v[k], &mm(&top[k], &c, m), false);
        }
    }
    if u.iter().chain(v.iter()).any(|a| a.len() > 1) {
        return None;
    }
    Some((
        std::array::from_fn(|i| u[i].first().copied().unwrap_or(0)),
        std::array::from_fn(|i| v[i].first().copied().unwrap_or(0)),
    ))
}
fn ann(u: [u64; 4], v: [u64; 3]) -> Vec<Vec<u64>> {
    let ms = target_program::Model::monomials();
    let mut a = vec![vec![0; 10]; 3];
    for (k, (x, y, _)) in ms.iter().enumerate() {
        let p = mm(
            &mpow(vec![0, 1], *x as u64, &u),
            &mpow(v.to_vec(), *y as u64, &u),
            &u,
        );
        for i in 0..3 {
            a[i][k] = p.get(i).copied().unwrap_or(0);
        }
    }
    a
}
fn input(p: &str) -> [u64; 8] {
    fs::read_to_string(p)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
fn main() {
    let (e, x) = exact::checked("generated/target.ms", "run-target.res");
    let fs = factors(e.clone());
    let mut prod = vec![1];
    for f in &fs {
        prod = mul(&prod, f);
        let d = f.len() - 1;
        assert_eq!(
            rem(
                &add(&mpow(vec![0, 1], power_u(127, d), f), &[0, 1], true),
                f
            ),
            vec![]
        );
        if d > 1 {
            assert_eq!(
                gcd(f.clone(), add(&mpow(vec![0, 1], 127, f), &[0, 1], true)).len(),
                1
            );
        }
    }
    assert_eq!(prod, e);
    println!(
        "FACTORS degrees={:?} exact_product=true",
        fs.iter().map(|f| f.len() - 1).collect::<Vec<_>>()
    );
    let model = target_program::Model::new();
    let lambda = input("PUBLIC-KUMMER.txt");
    let next = input("PUBLIC-NEXT-KUMMER.txt");
    let mut images = HashSet::new();
    let mut accepted = 0;
    for (i, f) in fs.iter().enumerate() {
        let xx: Vec<_> = x.iter().map(|a| rem(a, f)).collect();
        let Some((u, v)) = ucoeff(&xx, f) else {
            println!("FACTOR_REJECT {i} unsupported_projection_or_descent");
            continue;
        };
        println!(
            "CLOSED_DIVISOR factor={i} field_degree={} u={u:?} v={v:?}",
            f.len() - 1
        );
        let rows = ann(u, v);
        let Some(image) = model.candidate(rows.clone(), lambda, next, true) else {
            println!("FACTOR_REJECT {i} target_chart_or_orientation");
            continue;
        };
        let mut bad = next;
        bad[0] = (bad[0] + 1) % 127;
        assert!(
            model.candidate(rows, lambda, bad, false).is_none(),
            "corrupt adjacent input accepted"
        );
        images.insert(image);
        accepted += 1;
    }
    assert!(accepted > 0);
    assert_eq!(images.len(), 1);
    println!("RECOVERY_PASS algebraic_factors={} accepted={accepted} unique_B_images=1 adjacent_sign_verified=true corruption_rejected=true no_group_enumeration=true timing_claim=false",fs.len());
}
