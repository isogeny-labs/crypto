//! Multi-target prime-field index calculus, counted in group additions (experimental).
//!
//! The factor-base logarithms y_i = log_G F_i are found once from relations on G alone
//! (a G = +-F_i +-F_j, found by a restarted r-adding walk and a pair-sum table).  Every
//! target Q_t then needs one relation a G + b Q_t = +-F_i +-F_j with y_i, y_j known, and
//! x_t = b^-1 (+-y_i +-y_j - a) mod n.  Group additions are counted exactly in this file;
//! a scalar multiplication is priced at 1.5 * bits(n) additions.  Linear algebra, hashing
//! and field work are NOT priced (reported separately as wall time), so the counts are
//! lower bounds.
//!
//! usage: ic_multi_target <instance.json> <fb> <targets> <seed> [g_relation_factor=2.0]
use crypto_lib::cryptanalysis::ec_index_calculus::{
    build_factor_base, gaussian_eliminate_mod_n_particular,
};
use crypto_lib::ecc::{curve::CurveParams, field::FieldElement, point::Point};
use crypto_lib::utils::mod_inverse;
use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;
use std::time::Instant;

type Key = (BigUint, BigUint);
const ARMS: usize = 32;
const RESTART: usize = 32;
const BANK: usize = 64;
const MAXB: u64 = 8;

struct Counter {
    adds: u64,
    sm_cost: u64,
}
impl Counter {
    fn add(&mut self, p: &Point, q: &Point, a: &FieldElement) -> Point {
        self.adds += 1;
        p.add(q, a)
    }
    fn smul(&mut self, p: &Point, k: &BigUint, a: &FieldElement) -> Point {
        self.adds += self.sm_cost;
        p.scalar_mul(k, a)
    }
}

fn key(p: &Point) -> Option<Key> {
    match p {
        Point::Affine { x, y } => Some((x.value.clone(), y.value.clone())),
        Point::Infinity => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let d: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();
    let m: usize = args[2].parse().unwrap();
    let targets: usize = args[3].parse().unwrap();
    let seed: u64 = args[4].parse().unwrap();
    let gfac: f64 = args.get(5).map(|s| s.parse().unwrap()).unwrap_or(2.0);
    let big = |v: &serde_json::Value| {
        BigUint::parse_bytes(v.to_string().trim_matches('"').as_bytes(), 10).unwrap()
    };
    let p = big(&d["field"]["modulus"]);
    let curve = CurveParams {
        name: "multi",
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
    let mut c = Counter {
        adds: 0,
        sm_cost: (1.5 * n.bits() as f64).ceil() as u64,
    };
    let t0 = Instant::now();

    // Factor base and pair-sum table (once).
    let fb = build_factor_base(&curve, m);
    let m = fb.len();
    let mut table: HashMap<Key, (usize, i64, usize, i64)> = HashMap::with_capacity(m * m * 2);
    // Negation is free on a curve: -(F_i + F_j) = (-F_i) + (-F_j) and -(F_i - F_j) = (-F_i) + F_j, so two
    // additions per pair give all four sign combinations (unless MT_FULL_TABLE is set, the old 4-addition build).
    let full = std::env::var("MT_FULL_TABLE").is_ok();
    for i in 0..m {
        for j in i..m {
            if full {
                for s_i in [1i64, -1] {
                    for s_j in [1i64, -1] {
                        let pi = if s_i > 0 {
                            fb[i].point.clone()
                        } else {
                            fb[i].point.neg()
                        };
                        let pj = if s_j > 0 {
                            fb[j].point.clone()
                        } else {
                            fb[j].point.neg()
                        };
                        if let Some(k) = key(&c.add(&pi, &pj, &a_fe)) {
                            table.entry(k).or_insert((i, s_i, j, s_j));
                        }
                    }
                }
            } else {
                let sum = c.add(&fb[i].point, &fb[j].point, &a_fe);
                if let Some(k) = key(&sum) {
                    table.entry(k).or_insert((i, 1, j, 1));
                }
                if let Some(k) = key(&sum.neg()) {
                    table.entry(k).or_insert((i, -1, j, -1));
                }
                if i != j {
                    let dif = c.add(&fb[i].point, &fb[j].point.neg(), &a_fe);
                    if let Some(k) = key(&dif) {
                        table.entry(k).or_insert((i, 1, j, -1));
                    }
                    if let Some(k) = key(&dif.neg()) {
                        table.entry(k).or_insert((i, -1, j, 1));
                    }
                }
            }
        }
    }
    let adds_table = c.adds;

    // Bank of random multiples of G (shared by every walk).
    let mut bank: Vec<(Point, BigUint)> = Vec::with_capacity(BANK);
    for _ in 0..BANK {
        let k = rng.gen_biguint_below(&n);
        let pt = c.smul(&g, &k, &a_fe);
        bank.push((pt, k));
    }
    let adds_bank = c.adds - adds_table;
    eprintln!(
        "phase A done: table entries {} adds {}",
        table.len(),
        c.adds
    );

    // Generic walk: arms are bank points shifted by beta*Q (Q optional).
    struct Walk {
        r: Point,
        a: BigUint,
        b: BigUint,
        steps: usize,
        arms: Vec<(Point, BigUint, BigUint)>,
    }
    // Build a walk over `qm` = [Q, 2Q, ..., MAXB Q] (empty for the G-only phase).
    let new_walk_arms = |c: &mut Counter, rng: &mut StdRng, qm: &[Point]| -> Walk {
        let mut arms = Vec::with_capacity(ARMS);
        for _ in 0..ARMS {
            let k = rng.gen_range(0..BANK);
            if qm.is_empty() {
                arms.push((bank[k].0.clone(), bank[k].1.clone(), BigUint::zero()));
            } else {
                let beta = rng.gen_range(1..=MAXB) as usize;
                let pt = c.add(&bank[k].0, &qm[beta - 1], &a_fe);
                arms.push((pt, bank[k].1.clone(), BigUint::from(beta as u64)));
            }
        }
        Walk {
            r: Point::Infinity,
            a: BigUint::zero(),
            b: BigUint::zero(),
            steps: 0,
            arms,
        }
    };
    let a_ref = &a_fe;
    let bank_ref = &bank;
    let do_restart = |w: &mut Walk, c: &mut Counter, rng: &mut StdRng, qm: &[Point]| {
        // Start = sum of three random bank points (64^3 distinct starts), plus beta*Q for a target walk.
        let (k1, k2, k3) = (
            rng.gen_range(0..BANK),
            rng.gen_range(0..BANK),
            rng.gen_range(0..BANK),
        );
        let s12 = c.add(&bank_ref[k1].0, &bank_ref[k2].0, a_ref);
        let mut pt = c.add(&s12, &bank_ref[k3].0, a_ref);
        w.a = (&bank_ref[k1].1 + &bank_ref[k2].1 + &bank_ref[k3].1) % &n;
        if qm.is_empty() {
            w.b = BigUint::zero();
        } else {
            let beta = if qm.len() == 1 {
                1
            } else {
                rng.gen_range(1..=MAXB) as usize
            };
            pt = c.add(&pt, &qm[beta - 1], a_ref);
            w.b = BigUint::from(beta as u64);
        }
        w.r = pt;
        w.steps = 0;
    };
    let new_walk2 = |c: &mut Counter, rng: &mut StdRng, qm: &[Point]| -> Walk {
        let mut w = new_walk_arms(c, rng, qm);
        do_restart(&mut w, c, rng, qm);
        w
    };
    // One step: returns Some(key) after advancing.
    let step = |w: &mut Walk, c: &mut Counter, rng: &mut StdRng, qm: &[Point]| -> Option<Key> {
        if w.steps >= RESTART || matches!(w.r, Point::Infinity) {
            do_restart(w, c, rng, qm);
        }
        let kidx = match &w.r {
            Point::Affine { x, .. } => {
                (x.value.iter_u64_digits().next().unwrap_or(0) as usize) % ARMS
            }
            Point::Infinity => 0,
        };
        let (pt, ak, bk) = w.arms[kidx].clone();
        w.r = c.add(&w.r, &pt, a_ref);
        w.a = (&w.a + &ak) % &n;
        w.b = (&w.b + &bk) % &n;
        w.steps += 1;
        key(&w.r)
    };

    // Phase B: relations on G alone, until columns are determined.
    let want = (gfac * m as f64).ceil() as usize;
    let mut rows: Vec<Vec<BigUint>> = Vec::new();
    let mut rhs: Vec<BigUint> = Vec::new();
    let mut edges: Vec<(usize, i64, usize, i64, BigUint)> = Vec::new();
    let mut walk = new_walk2(&mut c, &mut rng, &[]);
    let mut trials_g = 0u64;
    while rows.len() < want {
        trials_g += 1;
        if let Some(k) = step(&mut walk, &mut c, &mut rng, &[]) {
            if let Some(&(i, s_i, j, s_j)) = table.get(&k) {
                let mut row = vec![BigUint::zero(); m];
                let sm = |s: i64| {
                    if s > 0 {
                        BigUint::one()
                    } else {
                        &n - BigUint::one()
                    }
                };
                row[i] = (&row[i] + sm(s_i)) % &n;
                row[j] = (&row[j] + sm(s_j)) % &n;
                rows.push(row);
                rhs.push(walk.a.clone() % &n);
                edges.push((i, s_i, j, s_j, walk.a.clone() % &n));
            }
        }
    }
    let adds_grel = c.adds - adds_table - adds_bank;
    eprintln!(
        "phase B done: rows {} trials_g {} adds {}",
        rows.len(),
        trials_g,
        c.adds
    );
    let dense_ops_est = 2 * (m as u64) * (m as u64) * (rows.len() as u64);
    let use_dense = std::env::var("MT_DENSE").is_ok();
    let t_la = Instant::now();
    let (values, det, la_ops): (Vec<BigUint>, Vec<bool>, u64) = if use_dense {
        let sol = gaussian_eliminate_mod_n_particular(&mut rows, &mut rhs, &n)
            .expect("G system inconsistent");
        (sol.values.clone(), sol.determined.clone(), dense_ops_est)
    } else {
        sparse_solve(m, &edges, &n)
    };
    let la_ms = t_la.elapsed().as_secs_f64() * 1e3;
    let sol_values = values;
    let ndet = det.iter().filter(|&&b| b).count();
    eprintln!("determined columns {} / {}", ndet, m);
    let setup_ms = t0.elapsed().as_secs_f64() * 1e3;

    // Phase C: targets.
    let adds_before_targets = c.adds;
    let mut ok = 0usize;
    let mut trials_t = 0u64;
    let mut skipped = 0u64;
    let mut plant_rng = StdRng::seed_from_u64(seed ^ 0x9e3779b97f4a7c15);
    let t1 = Instant::now();
    for _ in 0..targets {
        let xk = plant_rng.gen_biguint_below(&n); // planted (generation not counted)
        let q = g.scalar_mul(&xk, &a_fe);
        // v3 (default): pure-G arms, b = 1 (the start carries Q_t); MT_V2=1 restores per-target arms bQ_t.
        let v2 = std::env::var("MT_V2").is_ok();
        let mut qm: Vec<Point> = vec![q.clone()];
        let mut w;
        if v2 {
            for _ in 1..MAXB {
                let nxt = c.add(qm.last().unwrap(), &q, &a_fe);
                qm.push(nxt);
            }
            w = new_walk2(&mut c, &mut rng, &qm);
        } else {
            w = new_walk2(&mut c, &mut rng, &[]);
            do_restart(&mut w, &mut c, &mut rng, &qm);
        }
        if std::env::var("MT_DEBUG").is_ok() {
            eprintln!("target walk start, trials so far {}", trials_t);
        }
        let found = loop {
            trials_t += 1;
            if let Some(k) = step(&mut w, &mut c, &mut rng, &qm) {
                if let Some(&(i, s_i, j, s_j)) = table.get(&k) {
                    if det[i] && det[j] {
                        break (i, s_i, j, s_j);
                    }
                    skipped += 1;
                }
            }
            if trials_t > 200_000_000 {
                panic!("target search exceeded budget");
            }
        };
        let (i, s_i, j, s_j) = found;
        let term = |s: i64, y: &BigUint| if s > 0 { y.clone() } else { (&n - y) % &n };
        let rhs_v = (term(s_i, &sol_values[i]) + term(s_j, &sol_values[j]) + &n - (&w.a % &n)) % &n;
        let binv = mod_inverse(&w.b, &n).expect("b not invertible");
        let x = (rhs_v * binv) % &n;
        // verification of the recovered log (priced as one scalar multiplication)
        if c.smul(&g, &x, &a_fe) == q {
            ok += 1;
        }
    }
    let adds_targets = c.adds - adds_before_targets;
    println!(
        "{{\"n_bits\":{:.1},\"m\":{},\"targets\":{},\"ok\":{},\"determined\":{},\"adds_table\":{},\"adds_bank\":{},\"adds_grel\":{},\"adds_targets\":{},\"adds_total\":{},\"adds_per_target\":{:.1},\"adds_per_target_amortized\":{:.1},\"trials_g\":{},\"trials_targets\":{},\"skipped_undetermined\":{},\"la_ops\":{},\"dense_la_ops_est\":{},\"la_ms\":{:.2},\"setup_ms\":{:.1},\"targets_ms\":{:.1}}}",
        (n.bits() as f64), m, targets, ok, ndet, adds_table, adds_bank, adds_grel, adds_targets, c.adds,
        adds_targets as f64 / targets as f64, c.adds as f64 / targets as f64, trials_g, trials_t, skipped, la_ops, dense_ops_est, la_ms, setup_ms,
        t1.elapsed().as_secs_f64() * 1e3
    );
}

/// Near-linear solve of s_i y_i + s_j y_j = a (mod n) for a signed graph: BFS labels each node as
/// alpha + beta * t (beta = +-1) from a component root t; one non-tree edge with a non-zero
/// coefficient fixes t.  Returns (values, determined, mod-n operation count).
fn sparse_solve(
    m: usize,
    edges: &[(usize, i64, usize, i64, BigUint)],
    n: &BigUint,
) -> (Vec<BigUint>, Vec<bool>, u64) {
    let neg = |x: &BigUint| (n - (x % n)) % n;
    let sgn = |s: i64, x: &BigUint| if s > 0 { x % n } else { neg(x) };
    let mut ops = 0u64;
    let mut vals = vec![BigUint::zero(); m];
    let mut det = vec![false; m];
    // self-loop rows (i == j): (s_i + s_j) y_i = a
    let mut adj: Vec<Vec<(usize, i64, i64, BigUint)>> = vec![Vec::new(); m];
    for (i, s_i, j, s_j, a) in edges {
        if i == j {
            let c = s_i + s_j;
            if c != 0 {
                let cinv = mod_inverse(&BigUint::from(c.unsigned_abs()), n).expect("2 invertible");
                let mut y = (a * cinv) % n;
                if c < 0 {
                    y = neg(&y);
                }
                vals[*i] = y;
                det[*i] = true;
                ops += 3;
            }
        } else {
            adj[*i].push((*j, *s_i, *s_j, a.clone()));
            adj[*j].push((*i, *s_j, *s_i, a.clone()));
        }
    }
    let mut alpha: Vec<Option<(BigUint, bool)>> = vec![None; m]; // (alpha, beta_is_plus)
    for root in 0..m {
        if alpha[root].is_some() || det[root] {
            continue;
        }
        // Collect the component with labels; also detect t.
        let mut comp = vec![root];
        alpha[root] = Some((BigUint::zero(), true));
        let mut t: Option<BigUint> = None;
        let mut idx = 0;
        while idx < comp.len() {
            let u = comp[idx];
            idx += 1;
            let (au, bu_plus) = alpha[u].clone().unwrap();
            for (v, s_u, s_v, a) in adj[u].clone() {
                // y_v = s_v (a - s_u y_u): alpha_v = s_v (a - s_u alpha_u), beta_v = -s_v s_u beta_u
                let av = sgn(s_v, &((a.clone() + n - sgn(s_u, &au)) % n));
                let bv_plus = if s_u * s_v > 0 { !bu_plus } else { bu_plus };
                ops += 4;
                match &alpha[v] {
                    None => {
                        alpha[v] = Some((av, bv_plus));
                        comp.push(v);
                    }
                    Some((av0, bv0_plus)) => {
                        // consistency / fix t: (beta_v - beta_v') t = alpha_v' - alpha_v
                        if *bv0_plus != bv_plus && t.is_none() {
                            let diff = (av0 + n - &av) % n; // alpha_v' - alpha_v
                            let two_inv =
                                mod_inverse(&BigUint::from(2u32), n).expect("2 invertible");
                            // beta difference = +-2, so t = diff / (beta_v' - beta_v)
                            let mut tv = (diff * two_inv) % n;
                            // (beta_v - beta_v') t = alpha_v' - alpha_v: the coefficient is -2 when beta_v is minus
                            if !bv_plus {
                                tv = neg(&tv);
                            }
                            t = Some(tv);
                            ops += 4;
                        }
                    }
                }
            }
        }
        if let Some(tv) = t {
            for u in comp {
                let (au, b_plus) = alpha[u].clone().unwrap();
                vals[u] = if b_plus {
                    (au + &tv) % n
                } else {
                    (au + n - &tv) % n
                };
                det[u] = true;
                ops += 2;
            }
        }
    }
    (vals, det, ops)
}
