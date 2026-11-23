//! `curves`: draw random (a, b) over F_p, keep prime-order curves.
use crate::ec::*;
use crate::util::*;
use std::collections::HashSet;

pub fn cmd(args: &[String]) {
    let p = opt_u64(args, "--p", None);
    let count = opt_u64(args, "--count", None);
    let seed = opt_u64(args, "--seed", None);
    assert!(p > 7 && p < (1 << 32) && is_prime(p), "p must be a prime in (7, 2^32)");
    let mut sink = Sink::open(opt(args, "--out"));
    let mut rng = Rng::new(derive(seed, &[p, 0xC0A5]));
    let mut seen_j: HashSet<u64> = HashSet::new();
    let (mut draws, mut kept) = (0u64, 0u64);
    let mut rej = std::collections::BTreeMap::<&str, u64>::new();
    while kept < count {
        draws += 1;
        let (a, b) = (rng.below(p), rng.below(p));
        let c = Curve { p, a, b };
        let mut bump = |k: &'static str| *rej.entry(k).or_insert(0) += 1;
        if !c.disc_nonzero() {
            bump("singular");
            continue;
        }
        if a == 0 {
            bump("j_0");
            continue;
        }
        if b == 0 {
            bump("j_1728");
            continue;
        }
        let n = match order_prime_or_composite(&c, &mut rng) {
            Ok(Order::Prime(n)) => n,
            Ok(Order::Composite) => {
                bump("composite_order");
                continue;
            }
            Err(e) => panic!("{e} at a={a} b={b}"),
        };
        if n == p {
            bump("anomalous_n_eq_p");
            continue;
        }
        if !seen_j.insert(c.j_inv()) {
            bump("duplicate_j");
            continue;
        }
        // verification: n * P = O for several fresh points; G = last point.
        let mut o = 0;
        let mut g = (0, 0);
        for _ in 0..5 {
            g = c.random_point(&mut rng);
            assert!(c.mul(&mut o, n, Some(g)).is_none(), "order check failed a={a} b={b} n={n}");
        }
        kept += 1;
        let t = p as i64 + 1 - n as i64;
        sink.line(&format!(
            "{{\"kind\":\"curve\",\"idx\":{},\"draw\":{},\"p\":{},\"a\":{},\"b\":{},\"n\":{},\"t\":{},\"gx\":{},\"gy\":{},\"j\":{}}}",
            kept - 1, draws, p, a, b, n, t, g.0, g.1, c.j_inv()
        ));
    }
    let rj: Vec<String> = rej.iter().map(|(k, v)| format!("\"{k}\":{v}")).collect();
    sink.line(&format!(
        "{{\"kind\":\"summary\",\"p\":{},\"seed\":{},\"curves\":{},\"draws\":{},\"rejected\":{{{}}}}}",
        p, seed, kept, draws, rj.join(",")
    ));
    sink.flush();
}

pub struct CurveRec {
    pub idx: u64,
    pub c: Curve,
    pub n: u64,
    pub t: i64,
    pub g: (u64, u64),
}
pub fn load(path: &str) -> Vec<CurveRec> {
    read_lines(path, "curve")
        .iter()
        .map(|o| CurveRec {
            idx: num(o, "idx").unwrap() as u64,
            c: Curve { p: num(o, "p").unwrap() as u64, a: num(o, "a").unwrap() as u64, b: num(o, "b").unwrap() as u64 },
            n: num(o, "n").unwrap() as u64,
            t: num(o, "t").unwrap() as i64,
            g: (num(o, "gx").unwrap() as u64, num(o, "gy").unwrap() as u64),
        })
        .collect()
}
