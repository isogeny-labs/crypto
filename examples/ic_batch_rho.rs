//! Counted multi-target ("anchored") Pollard rho on a prime-order curve group, for comparison with
//! `ic_multi_target` in the same unit (exact group additions; a scalar multiplication is priced at
//! 1.5 * bits(n) additions).  Memory-unbounded reference: every visited point of the known-log walks
//! is stored (no distinguished-point overhead, which FAVOURS rho), lookups are by x only (negation
//! map: P and -P share a key).  Phase 1 builds a database of points a*G visited by restarted
//! r-adding walks; phase 2 walks from Q_t + (random known multiple of G) until the x-coordinate is in
//! the database; a hit gives x_t = (+-a' - a) mod n.  Every recovered log is verified (priced).
//!
//! usage: ic_batch_rho <instance.json> <targets> <seed> <kappa>   (database size = kappa * sqrt(n T))
use crypto_lib::ecc::{curve::CurveParams, field::FieldElement, point::Point};
use num_bigint::{BigUint, RandBigInt};
use num_traits::{ToPrimitive, Zero};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;
use std::time::Instant;

const ARMS: usize = 32;
const RESTART: usize = 32;
const BANK: usize = 64;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let d: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();
    let targets: usize = args[2].parse().unwrap();
    let seed: u64 = args[3].parse().unwrap();
    let kappa: f64 = args[4].parse().unwrap();
    let big = |v: &serde_json::Value| {
        BigUint::parse_bytes(v.to_string().trim_matches('"').as_bytes(), 10).unwrap()
    };
    let p = big(&d["field"]["modulus"]);
    let curve = CurveParams {
        name: "batch-rho",
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
    let mut rng = StdRng::seed_from_u64(seed);
    let sm_cost = (1.5 * n.bits() as f64).ceil() as u64;
    let mut adds: u64 = 0;
    let t0 = Instant::now();

    // Bank and arms (G-multiples only).
    let mut bank: Vec<(Point, BigUint)> = Vec::new();
    for _ in 0..BANK {
        let k = rng.gen_biguint_below(&n);
        bank.push((g.scalar_mul(&k, &a_fe), k));
        adds += sm_cost;
    }
    let arms: Vec<(Point, BigUint)> = (0..ARMS)
        .map(|_| {
            let k = rng.gen_range(0..BANK);
            bank[k].clone()
        })
        .collect();
    let xkey = |pt: &Point| -> Option<BigUint> {
        match pt {
            Point::Affine { x, .. } => Some(x.value.clone()),
            Point::Infinity => None,
        }
    };
    let ykey = |pt: &Point| -> BigUint {
        match pt {
            Point::Affine { y, .. } => y.value.clone(),
            Point::Infinity => BigUint::zero(),
        }
    };

    // Phase 1: database of known points (x -> (a, y)).
    let db_target = (kappa * ((n.to_f64().unwrap_or(1.0)) * targets as f64).sqrt()).ceil() as usize;
    let mut db: HashMap<BigUint, (BigUint, BigUint)> = HashMap::with_capacity(db_target * 2);
    let mut cur = Point::Infinity;
    let mut ca = BigUint::zero();
    let mut steps = RESTART;
    while db.len() < db_target {
        if steps >= RESTART || matches!(cur, Point::Infinity) {
            let (k1, k2, k3) = (
                rng.gen_range(0..BANK),
                rng.gen_range(0..BANK),
                rng.gen_range(0..BANK),
            );
            let s = bank[k1].0.add(&bank[k2].0, &a_fe).add(&bank[k3].0, &a_fe);
            adds += 2;
            cur = s;
            ca = (&bank[k1].1 + &bank[k2].1 + &bank[k3].1) % &n;
            steps = 0;
        }
        let kidx = match &cur {
            Point::Affine { x, .. } => {
                (x.value.iter_u64_digits().next().unwrap_or(0) as usize) % ARMS
            }
            Point::Infinity => 0,
        };
        cur = cur.add(&arms[kidx].0, &a_fe);
        adds += 1;
        ca = (&ca + &arms[kidx].1) % &n;
        steps += 1;
        if let Some(x) = xkey(&cur) {
            db.entry(x).or_insert((ca.clone(), ykey(&cur)));
        }
    }
    let adds_db = adds;

    // Phase 2: targets.
    let mut plant = StdRng::seed_from_u64(seed ^ 0x9e3779b97f4a7c15);
    let mut ok = 0usize;
    let mut steps_total = 0u64;
    for _ in 0..targets {
        let xk = plant.gen_biguint_below(&n);
        let q = g.scalar_mul(&xk, &a_fe); // planted (not counted)
        let mut cur = Point::Infinity;
        let mut ca = BigUint::zero();
        let mut st = RESTART;
        let found = loop {
            if st >= RESTART || matches!(cur, Point::Infinity) {
                let (k1, k2) = (rng.gen_range(0..BANK), rng.gen_range(0..BANK));
                let s = bank[k1].0.add(&bank[k2].0, &a_fe).add(&q, &a_fe);
                adds += 2;
                cur = s;
                ca = (&bank[k1].1 + &bank[k2].1) % &n;
                st = 0;
            }
            let kidx = match &cur {
                Point::Affine { x, .. } => {
                    (x.value.iter_u64_digits().next().unwrap_or(0) as usize) % ARMS
                }
                Point::Infinity => 0,
            };
            cur = cur.add(&arms[kidx].0, &a_fe);
            adds += 1;
            steps_total += 1;
            ca = (&ca + &arms[kidx].1) % &n;
            st += 1;
            if let Some(x) = xkey(&cur) {
                if let Some((a2, y2)) = db.get(&x) {
                    let same = *y2 == ykey(&cur);
                    // R = ca G + Q.  Same sign: ca G + Q = a2 G -> x = a2 - ca.  Opposite: = -a2 G -> x = -a2 - ca.
                    let v = if same {
                        (a2 + &n - &ca) % &n
                    } else {
                        (&n + &n - a2 - &ca) % &n
                    };
                    break v;
                }
            }
            if steps_total > 2_000_000_000 {
                panic!("target search exceeded budget");
            }
        };
        adds += sm_cost; // verification, priced
        if g.scalar_mul(&found, &a_fe) == q {
            ok += 1;
        }
    }
    println!(
        "{{\"n_bits\":{},\"targets\":{},\"kappa\":{},\"db\":{},\"ok\":{},\"adds_db\":{},\"adds_total\":{},\"adds_per_target_amortized\":{:.1},\"target_steps\":{},\"ms\":{:.1}}}",
        n.bits(), targets, kappa, db.len(), ok, adds_db, adds, adds as f64 / targets as f64, steps_total, t0.elapsed().as_secs_f64() * 1e3
    );
}
