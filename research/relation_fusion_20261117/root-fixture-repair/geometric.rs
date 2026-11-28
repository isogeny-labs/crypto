use crypto_lib::cryptanalysis::binary_radical_root::FactoredSubspaceOracle;
use crypto_lib::cryptanalysis::semaev_decomp::{Gf2, quartic_in_x3_general};
use rand::{rngs::StdRng, Rng, SeedableRng};
fn field(n: u32) -> Gf2 {
    Gf2::new(&crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible(n).expect("irreducible"))
}
// Lift points only over the existing factor-base span; no new RNG draws or seeds.
// Target selection uses group addition, independently of either decomposition oracle.
fn geometric_controls(span: &[u64], b: u64, gf: &Gf2) -> Vec<([u64;3],u64)> {
    use crypto_lib::cryptanalysis::ic_boundary::ArtinSchreier;
    use crypto_lib::cryptanalysis::koblitz_ic_leads::lambda::add;
    use std::collections::BTreeSet;
    let ash = ArtinSchreier::new(gf);
    let on_curve = |(x,y): (u64,u64)| gf.sqr(y) ^ gf.mul(x,y) == gf.mul(gf.sqr(x),x) ^ gf.sqr(x) ^ b;
    let mut points = vec![];
    for &x in span.iter().filter(|&&x| x != 0) {
        let rhs = x ^ 1 ^ gf.mul(b, gf.sqr(gf.inv(x)));
        if let Some(z) = ash.solve(rhs) {
            let y = gf.mul(x,z);
            assert!(on_curve((x,y)) && on_curve((x,y^x)));
            points.push((x,y)); points.push((x,y^x));
        }
    }
    let mut seen = BTreeSet::new(); let mut out = vec![];
    for i in 0..points.len() {
        for j in i+1..points.len() {
            for k in j+1..points.len() {
                let (p,q,r) = (points[i],points[j],points[k]);
                if p.0==q.0 || p.0==r.0 || q.0==r.0 { continue; }
                let sum = add(gf,1,add(gf,1,Some(p),Some(q)),Some(r));
                if let Some(t) = sum {
                    assert!(on_curve(t));
                    if span.contains(&t.0) || !seen.insert(t.0) { continue; }
                    assert_eq!(crypto_lib::cryptanalysis::semaev_decomp::eval_s4_general(p.0,q.0,r.0,t.0,b,gf),0);
                    out.push(([p.0,q.0,r.0],t.0));
                    if out.len()==12 { return out; }
                }
            }
        }
    }
    panic!("not enough distinct nondegenerate geometric controls: n={} count={}",gf.n,out.len());
}
pub fn run() {
        let mut reports = vec![];
        use crate::cryptanalysis::semaev_decomp::{eval_s4_general, SubspaceOracle};
        for (n, l) in [(13u32, 4u32), (17, 5), (19, 6), (23, 6)] {
            let gf = field(n);
            let mut rng = StdRng::seed_from_u64(900 + n as u64);
            for trial in 0..3 {
                let b = (rng.gen::<u64>() & ((1 << n) - 1)) | 1;
                // A low-order base (the ledger's) and a random subspace.
                let basis: Vec<u64> = if trial == 0 {
                    (0..l).map(|i| 1u64 << i).collect()
                } else {
                    loop {
                        let cand: Vec<u64> =
                            (0..l).map(|_| rng.gen::<u64>() & ((1 << n) - 1)).collect();
                        if FactoredSubspaceOracle::new(&cand, b, &gf).is_some() {
                            break cand;
                        }
                    }
                };
                let base = SubspaceOracle::new(&basis, b, &gf);
                let fac = FactoredSubspaceOracle::new(&basis, b, &gf).unwrap();
                let (mut yes, mut no) = (0, 0);
                for i in 0..200 {
                    // Plant targets as x of an S₄ root over the base, half the time.
                    let xr = if i % 2 == 0 {
                        let pick = |rng: &mut StdRng| base.span[rng.gen_range(0..base.span.len())];
                        let (x1, x2, x3) = (pick(&mut rng), pick(&mut rng), pick(&mut rng));
                        let q = quartic_in_x3_general(x1, x2, x3, b, &gf);
                        let ev = |t: u64| {
                            (0..q.c.len())
                                .rev()
                                .fold(0u64, |v, j| gf.mul(v, t) ^ q.c[j])
                        };
                        (1..1u64 << n.min(14)).find(|&t| ev(t) == 0).unwrap_or(1)
                    } else {
                        rng.gen::<u64>() & ((1 << n) - 1)
                    };
                    let (want, _) = base.decompose(xr, &gf);
                    let (got, pairs) = fac.decompose(xr, &gf);
                    assert_eq!(
                        got.is_some(),
                        want.is_some(),
                        "n={n} l={l} b={b:#x} xr={xr:#x}"
                    );
                    if let Some([a, c, d]) = got {
                        assert!(fac.contains(a) && fac.contains(c) && fac.contains(d));
                        assert_eq!(eval_s4_general(a, c, d, xr, b, &gf), 0);
                        yes += 1;
                    } else {
                        let k = base.span.len() as u64;
                        assert_eq!(pairs, k * (k + 1) / 2);
                        no += 1;
                    }
                }
                let legacy_yes = yes;
                let legacy_no = no;
                let controls = geometric_controls(&base.span, b, &gf);
                for (xs, xr) in &controls {
                    assert_eq!(eval_s4_general(xs[0], xs[1], xs[2], *xr, b, &gf), 0);
                    let (want, _) = base.decompose(*xr, &gf);
                    let (got, _) = fac.decompose(*xr, &gf);
                    assert!(want.is_some(), "geometric control missing in original oracle");
                    let [a, c, d] = got.expect("geometric control missing in factored oracle");
                    assert!(fac.contains(a) && fac.contains(c) && fac.contains(d));
                    assert_eq!(eval_s4_general(a, c, d, *xr, b, &gf), 0);
                    yes += 1;
                }
                assert!(yes > 10 && no > 10, "n={n}: yes={yes} no={no}");
                reports.push(serde_json::json!({"n":n,"l":l,"trial":trial,"b":b,"basis":basis,"legacy_cases":200,"legacy_yes":legacy_yes,"legacy_no":legacy_no,"geometric_controls":controls,"combined_yes":yes,"combined_no":no}));
            }
        }
        println!("{}", serde_json::json!({"schema":"root-geometric-positive-controls/v1","trials":reports,"legacy_cases":2400,"geometric_positive_controls":144,"original_seeds_and_assertions_preserved":true,"counts_are_not_yield_estimates":true,"performance_comparison":false}));
    }
