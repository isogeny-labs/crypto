use hec_native_ic::{
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    poly::Poly,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};
fn n(s: &str) -> u64 {
    s.parse().unwrap()
}
fn poly(s: &str) -> Poly {
    let a = s.split("c: [").nth(1).unwrap().split(']').next().unwrap();
    let v: Vec<u64> = a.split(',').map(|x| n(x.trim())).collect();
    let len: usize = s
        .split("n: ")
        .nth(1)
        .unwrap()
        .split(' ')
        .next()
        .unwrap()
        .trim_end_matches('}')
        .parse()
        .unwrap();
    assert!(len <= v.len());
    assert!(v[len..].iter().all(|&x| x == 0));
    Poly::from_coeffs(&v[..len])
}
fn curve(p: u64, k: u64) -> Curve {
    Curve::new(
        Fp::new(p),
        Poly::from_coeffs(&[0, 2, 0, k % 4, 0, (k / 4) % 4, 0, k / 16, 0, 1]),
    )
}
fn main() {
    let root = "research/hec_residual_span_20261116/order4-strong";
    let read = |s: &str| -> Vec<Vec<String>> {
        fs::read_to_string(format!("{root}/{s}.tsv"))
            .unwrap()
            .lines()
            .skip(1)
            .map(|l| l.split('\t').map(str::to_string).collect())
            .collect()
    };
    let orbits = read("ORBITS");
    let mut total = 0;
    let mut models = 0;
    println!("p\tmodel\tbase_points\torbit_rows\tsource_projection\taction\torbits");
    for v in read("SOURCES") {
        let p = n(&v[0]);
        let k = n(&v[1]);
        let order = n(&v[2]);
        let ell = n(&v[3]);
        let h = n(&v[4]);
        let i = n(&v[5]);
        let lambda = n(&v[6]);
        let c = curve(p, k);
        assert_eq!(order, ell * h);
        assert_eq!(c.fp.mul(i, i), p - 1);
        let ef = Fp::new(ell);
        assert_eq!(ef.mul(lambda, lambda), ell - 1);
        let u = poly(v[7].split("u: Poly").nth(1).unwrap());
        let vv = poly(v[7].split("v: Poly").nth(1).unwrap());
        let source = Div { u, v: vv };
        assert!(c.is_valid(&source));
        assert!(!c.is_zero(&source));
        assert!(c.is_zero(&c.mul(&source, ell)));
        let fb = factor_base(&c);
        let projected: BTreeMap<_, _> = fb
            .iter()
            .map(|&(x, y)| (x, c.mul(&c.place(x, y), h)))
            .collect();
        let expected = fb
            .iter()
            .map(|&(x, y)| c.mul(&c.place(x, y), h))
            .find(|d| !c.is_zero(d))
            .unwrap();
        assert_eq!(source, expected);
        for &(x, y) in &fb {
            let raw = c.place(x, y);
            assert!(c.is_zero(&c.mul(&raw, order)));
            assert!(c.is_zero(&c.mul(&projected[&x], ell)));
            let image = c.mul(&c.place(c.fp.neg(x), c.fp.mul(i, y)), h);
            assert_eq!(image, c.mul(&projected[&x], lambda));
            if y == 0 {
                assert!(c.is_zero(&projected[&x]))
            }
        }
        let mut seen = BTreeSet::new();
        let mut reps = BTreeSet::new();
        for row in &orbits {
            if n(&row[0]) != p || n(&row[1]) != k {
                continue;
            }
            let x = n(&row[2]);
            let y = n(&row[3]);
            assert!(fb.contains(&(x, y)));
            assert!(seen.insert(x));
            let w = n(&row[5]);
            if row[4] == "none" {
                assert_eq!(w, 0);
                assert!(c.is_zero(&projected[&x]));
                assert_eq!(y, 0)
            } else {
                let rx = n(&row[4]);
                reps.insert(rx);
                assert!(w < ell);
                assert_eq!(projected[&x], c.mul(&projected[&rx], w));
                assert!(x == rx || x == c.fp.neg(rx));
            }
            total += 1;
        }
        assert_eq!(seen.len(), fb.len());
        let control = read("CONTROL")
            .into_iter()
            .find(|r| n(&r[0]) == p && n(&r[1]) == k)
            .unwrap();
        assert_eq!(n(&control[14]), fb.len() as u64);
        assert_eq!(n(&control[15]), reps.len() as u64);
        models += 1;
        println!("{p}\t{k}\t{}\t{}\ttrue\ttrue\ttrue", fb.len(), seen.len());
    }
    assert_eq!(models, 3);
    assert_eq!(total, 211);
    assert_eq!(total, orbits.len());
    eprintln!("models={models} source_order_and_projection=3 orbit_rows={total} all_catalogue_actions=true shared_native_group_arithmetic=true");
}
