//! Counted multi-target baby-step giant-step on a prime-order curve group (reference for `ic_multi_target`).
//! Baby steps: x-coordinate of j*G for j = 1..M in a hash table (negation map: x-only key, y recorded).
//! Giant steps per target: Q - i*S*G with S = 2M + 1; a hit x(Q - iSG) = x(+-jG) gives x_t = i*S +- j.
//! Exact group additions are counted here (a scalar multiplication is priced at ceil(1.5 bits(n))); every
//! recovered log is verified by one priced scalar multiplication.  Hash operations are reported separately
//! (baby inserts, giant lookups) so the 0 / 0.1 / 1 GAE band can be applied to both arms symmetrically.
//!
//! usage: ic_multi_bsgs <instance.json> <targets> <seed> <mfactor>   (M = mfactor * sqrt(n * T / 4))
use crypto_lib::ecc::{curve::CurveParams, field::FieldElement, point::Point};
use num_bigint::{BigUint, RandBigInt};
use num_traits::{ToPrimitive, Zero};
use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let d: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();
    let targets: usize = args[2].parse().unwrap();
    let seed: u64 = args[3].parse().unwrap();
    let mfactor: f64 = args[4].parse().unwrap();
    let big = |v: &serde_json::Value| {
        BigUint::parse_bytes(v.to_string().trim_matches('"').as_bytes(), 10).unwrap()
    };
    let p = big(&d["field"]["modulus"]);
    let curve = CurveParams {
        name: "bsgs",
        p: p.clone(),
        a: big(&d["a"]),
        b: big(&d["b"]),
        gx: big(&d["generator"]["x"]),
        gy: big(&d["generator"]["y"]),
        n: big(&d["subgroup_order"]),
        h: 1,
    };
    let n = curve.n.clone();
    let a_fe = curve.a_fe();
    let g = Point::Affine {
        x: FieldElement::new(curve.gx.clone(), p.clone()),
        y: FieldElement::new(curve.gy.clone(), p.clone()),
    };
    let sm_cost = (1.5 * n.bits() as f64).ceil() as u64;
    let mut adds: u64 = 0;
    let t0 = Instant::now();
    let m = ((mfactor * (n.to_f64().unwrap() * targets as f64 / 4.0).sqrt()).ceil() as u64).max(1);
    let s_step = 2 * m + 1;

    // Baby steps: j*G for j = 1..M.
    let mut table: HashMap<BigUint, (u64, BigUint)> = HashMap::with_capacity(m as usize * 2);
    let mut cur = g.clone();
    for j in 1..=m {
        if let Point::Affine { x, y } = &cur {
            table.entry(x.value.clone()).or_insert((j, y.value.clone()));
        }
        cur = cur.add(&g, &a_fe);
        adds += 1;
    }
    let baby_inserts = m;
    // Giant step: -S*G.
    let neg_sg = g.scalar_mul(&BigUint::from(s_step), &a_fe).neg();
    adds += sm_cost;

    let mut plant = StdRng::seed_from_u64(seed ^ 0x9e3779b97f4a7c15);
    let mut ok = 0usize;
    let mut lookups: u64 = 0;
    let adds_baby = adds;
    for _ in 0..targets {
        let xk = plant.gen_biguint_below(&n); // planted (generation not counted)
        let q = g.scalar_mul(&xk, &a_fe);
        let mut cur = q.clone();
        let mut i: u64 = 0;
        let found = loop {
            lookups += 1;
            match &cur {
                Point::Affine { x, y } => {
                    if let Some((j, y2)) = table.get(&x.value) {
                        let same = *y2 == y.value;
                        let base = BigUint::from(i) * BigUint::from(s_step);
                        break if same {
                            (base + BigUint::from(*j)) % &n
                        } else {
                            (base + &n - BigUint::from(*j)) % &n
                        };
                    }
                }
                Point::Infinity => break (BigUint::from(i) * BigUint::from(s_step)) % &n,
            }
            cur = cur.add(&neg_sg, &a_fe);
            adds += 1;
            i += 1;
            if i > 4 * (n.to_u64().unwrap() / s_step + 2) {
                panic!("giant-step search exceeded the full cycle");
            }
        };
        adds += sm_cost; // verification, priced
        if g.scalar_mul(&found, &a_fe) == q {
            ok += 1;
        }
    }
    println!(
        "{{\"n_bits\":{},\"targets\":{},\"mfactor\":{},\"m\":{},\"ok\":{},\"adds_baby\":{},\"baby_inserts\":{},\"giant_lookups\":{},\"adds_total\":{},\"adds_per_target_amortized\":{:.1},\"ms\":{:.1}}}",
        n.bits(), targets, mfactor, m, ok, adds_baby, baby_inserts, lookups, adds, adds as f64 / targets as f64, t0.elapsed().as_secs_f64() * 1e3
    );
    let _ = Zero::is_zero(&BigUint::zero());
}
