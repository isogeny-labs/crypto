mod config;
mod setup {
    include!("setup.rs");
}
include!("verify_poly.rs");
#[allow(dead_code)]
mod exact {
    include!("verify.rs");
    include!("exact_api.rs");
}
#[allow(dead_code)]
mod target_program {
    include!("runtime_program.rs");
    include!("recover_model.rs");
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
    let model = target_program::Model::new();
    for label in ["D1", "B"] {
        let (e, x) = exact::checked(
            &format!("generated/{label}/target.ms"),
            &format!("solve-{label}.res"),
        );
        let fs = factors(e.clone());
        let mut prod = vec![1];
        for f in &fs {
            prod = mul(&prod, f);
            let d = f.len() - 1;
            assert!(rem(
                &add(&mpow(vec![0, 1], power_u(127, d), f), &[0, 1], true),
                f
            )
            .is_empty());
            if d > 1 {
                assert_eq!(
                    gcd(f.clone(), add(&mpow(vec![0, 1], 127, f), &[0, 1], true)).len(),
                    1
                );
            }
        }
        assert_eq!(prod, e);
        let lambda = input(&format!("generated/{label}-KUMMER.txt"));
        let next = input(&format!("generated/{label}-NEXT-KUMMER.txt"));
        let offset = std::fs::read_to_string(format!("generated/{label}-OFFSET.txt"))
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();
        let mut images = HashSet::new();
        let mut rows = vec![];
        let mut accepted = 0;
        let mut repr = HashSet::new();
        for (i, f) in fs.iter().enumerate() {
            let xx: Vec<_> = x.iter().map(|a| rem(a, f)).collect();
            let Some((u, v)) = ucoeff(&xx, f) else {
                println!("FACTOR_EXCLUDE label={label} factor={i} reason=closed_projection");
                continue;
            };
            let a = ann(u, v);
            let Some((image, positive)) = model.candidate(a.clone(), lambda, next, offset) else {
                println!("FACTOR_EXCLUDE label={label} factor={i} reason=chart_or_orientation");
                continue;
            };
            let mut bad = next;
            bad[0] = (bad[0] + 1) % 127;
            assert!(
                model.candidate(a, lambda, bad, offset).is_none(),
                "corrupt_adjacent_input"
            );
            images.insert(image);
            accepted += 1;
            if repr.insert((u, v, positive)) {
                let mut row = vec![positive as u64, offset];
                row.extend(u);
                row.extend(v);
                rows.push(row.iter().map(u64::to_string).collect::<Vec<_>>().join(" "));
            }
            println!("RECOVERED label={label} factor={i} degree={} u={u:?} v={v:?} positive={positive} offset={offset} image={image:?} original_oracle_used=false",f.len()-1);
        }
        assert!(accepted > 0);
        assert_eq!(images.len(), 1);
        let image = images.into_iter().next().unwrap();
        std::fs::write(
            format!("class-{label}.txt"),
            image.map(|a| a.to_string()).join(" ") + "\n",
        )
        .unwrap();
        std::fs::write(format!("divisors-{label}.txt"), rows.join("\n") + "\n").unwrap();
        println!("CLASS_PASS label={label} factors={} accepted={accepted} distinct_divisors={} unique_images=1 corrupt_adjacency_rejected=true",fs.len(),repr.len());
    }
    println!("PASS both_original_classes_recovered=true known_base=fresh_fiber target_class_or_DLP_supplied=false timing_claim=false");
}
