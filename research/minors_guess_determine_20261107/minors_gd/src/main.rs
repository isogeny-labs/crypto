//! minors_gd: charged comparison of the Mahalanobis minors / guess-and-determine
//! ECDLP line against matched negation rho. Standalone; counted unit = F_p
//! multiplications (inversions priced at 1.5*bits(p) multiplications, adds
//! uncharged for every arm). See ../PROTOCOL.md.
mod curve; mod field; mod gd; mod kernel; mod rho; mod rng; mod scan;
use curve::*; use field::*; use rng::*; use scan::*;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Params { pub nprime: Option<usize>, pub d: Option<usize>, pub dev_check: bool }

#[derive(Default, Debug, Clone)]
pub struct RunRec {
    solver: String, bits: u32, idx: u64, seed: u64, p: u64, n: u64,
    success: bool, correct: bool, cap: f64,
    muls: u64, invs: u64, adds: u64, charged: f64,
    attempts: u64, kernels_built: u64, bad_candidates: u64, nprime: usize, l: usize, d: usize,
    minors_tested: f64, naive_minor_evals: f64, hash_inserts: u64,
    minor_size: usize, extra: String, wall_ms: f64,
}

impl RunRec {
    fn json(&self) -> String {
        format!("{{\"schema\":\"minors_gd.run/v1\",\"solver\":\"{}\",\"bits\":{},\"idx\":{},\"seed\":\"{:016x}\",\"p\":{},\"n\":{},\"success\":{},\"correct\":{},\"cap\":{:e},\"muls\":{},\"invs\":{},\"adds\":{},\"charged\":{:.1},\"attempts\":{},\"kernels_built\":{},\"bad_candidates\":{},\"nprime\":{},\"l\":{},\"d\":{},\"minors_tested\":{:e},\"naive_minor_evals\":{:e},\"hash_inserts\":{},\"minor_size\":{},\"extra\":{{{}}},\"wall_ms_informational\":{:.1}}}",
            self.solver, self.bits, self.idx, self.seed, self.p, self.n, self.success, self.correct, self.cap, self.muls, self.invs, self.adds, self.charged,
            self.attempts, self.kernels_built, self.bad_candidates, self.nprime, self.l, self.d, self.minors_tested, self.naive_minor_evals, self.hash_inserts, self.minor_size, self.extra, self.wall_ms)
    }
}

fn verify(c: &Curve, inst: &Instance, k: u64) -> bool {
    // charged verification [k]G == Q (same for every arm)
    c.mul_scalar(k, Some(inst.g)) == Some(inst.q)
}

pub fn run(solver: &str, inst: &Instance, cap: f64, prm: &Params) -> RunRec {
    let t0 = Instant::now();
    let c = Curve::new(inst.p, inst.a, inst.b);
    let f = &c.f;
    let invw = inv_weight(inst.p);
    let cost = || f.muls.get() as f64 + invw * f.invs.get() as f64;
    let mut rec = RunRec { solver: solver.to_string(), bits: inst.bits, idx: inst.idx, seed: inst.seed, p: inst.p, n: inst.n, cap, ..Default::default() };
    let mut rng = Rng::new(mix(inst.seed, hash_str(solver)));
    if solver == "rho" {
        let o = rho::solve(&c, inst.g, inst.q, inst.n, inst.bits, &mut rng, cap);
        rec.attempts = 1;
        rec.extra = format!("\"steps\":{},\"dps\":{},\"restarts\":{},\"escapes\":{},\"special_adds\":{},\"useless_collisions\":{},\"cap_hit\":{}", o.steps, o.dps, o.restarts, o.escapes, o.special_adds, o.useless_collisions, o.cap_hit);
        if let Some(k) = o.k { rec.success = verify(&c, inst, k); rec.correct = k == inst.k; }
    } else {
        let is_gd = solver.starts_with("gd");
        let mut np = prm.nprime.unwrap_or(inst.bits as usize);
        if is_gd && np % 2 == 1 { np += 1; }
        let l = 3 * np;
        let d = if is_gd { prm.d.unwrap_or_else(|| gd::choose_d(l, inst.n, 0.5)) } else { 0 };
        rec.nprime = np; rec.l = l; rec.d = d;
        let over = || cost() > cap;
        let mut st = ScanStats::default();
        let mut gs = gd::GdStats::default();
        let dev_c = if prm.dev_check { Some(Curve::new(inst.p, inst.a, inst.b)) } else { None };
        loop {
            if over() { break; }
            rec.attempts += 1;
            let kern = match kernel::build(&c, inst.g, inst.q, inst.n, np, &mut rng) { Some(k) => k, None => continue };
            rec.kernels_built += 1;
            let res = match solver {
                "mgd.all2" => all2(f, &kern.c, kern.l, &mut st, &over),
                "mgd.gesc" => gesc(f, &kern.c, kern.l, &mut st, &over),
                "mgd.apm2" => apm2(f, &kern.c, kern.l, &mut st, &over),
                s if s.starts_with("gd") => gd::attempt(f, &kern, d, &mut rng, &mut st, &mut gs, &over),
                _ => panic!("unknown solver {}", solver),
            };
            match res {
                ScanRes::OverBudget => break,
                ScanRes::None => {}
                ScanRes::Found(fd) => {
                    if let Some(dc) = &dev_c {
                        if !kernel::support_sums_to_o(dc, inst.g, inst.q, &kern, &fd.alpha, &fd.beta) { eprintln!("DEV CHECK FAILED: support does not sum to O ({} bits {} {})", inst.bits, solver, inst.idx); }
                    }
                    match kern.recover(&fd.alpha, &fd.beta) {
                        Some(m) if verify(&c, inst, m) => { rec.success = true; rec.correct = m == inst.k; rec.minor_size = fd.alpha.len(); break; }
                        _ => { rec.bad_candidates += 1; }
                    }
                }
            }
        }
        rec.minors_tested = st.minors_tested; rec.naive_minor_evals = st.naive_minor_evals; rec.hash_inserts = st.hash_inserts;
        if is_gd { rec.extra = format!("\"b_tried\":{},\"leaves\":{},\"kernels_T\":{}", gs.b_tried, gs.leaves, gs.kernels); }
    }
    let cn = f.counters();
    rec.muls = cn.muls; rec.invs = cn.invs; rec.adds = cn.adds; rec.charged = charged(&cn, inst.p);
    rec.wall_ms = t0.elapsed().as_secs_f64() * 1e3;
    rec
}

fn arg(args: &[String], name: &str) -> Option<String> { args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned()) }

fn parse_f(s: &str) -> f64 { s.parse::<f64>().expect("number") }

fn selftest() {
    let mut fails = 0;
    // 1 field
    let mut r = Rng::new(1);
    for &p in &[1_000_003u64, 2_199_023_255_551 - 2 * 0, 1_099_511_627_689 | 3] {
        if !is_prime(p) { continue; }
        let f = Fp::new(p);
        for _ in 0..100000 { let (a, b) = (r.below(p), r.below(p)); let want = ((a as u128 * b as u128) % p as u128) as u64; if f.mul_nc(a, b) != want { fails += 1; } }
        for _ in 0..1000 { let a = 1 + r.below(p - 1); if f.mul_nc(a, f.inv_nc(a)) != 1 { fails += 1; } }
    }
    println!("[field] done, fails so far {}", fails);
    // 2 instances
    for bits in 8..=22u32 {
        let i = make_instance(bits, 0);
        let c = Curve::new(i.p, i.a, i.b);
        let ok = c.mul_scalar(i.n, Some(i.g)).is_none() && c.mul_scalar(i.k, Some(i.g)) == Some(i.q) && is_prime(i.n) && 64 - i.n.leading_zeros() == bits && c.on_curve(Some(i.g)) && c.on_curve(Some(i.q));
        if !ok { fails += 1; println!("[instance] FAIL bits {}", bits); }
    }
    println!("[instance] bits 8..22 checked, fails so far {}", fails);
    // 3 solvers
    let prm = Params { nprime: None, d: None, dev_check: true };
    for (solver, bits_list) in [("rho", vec![8u32, 12, 16, 20]), ("mgd.all2", vec![8, 10, 12]), ("mgd.gesc", vec![8, 10, 12]), ("mgd.apm2", vec![8, 10, 12]), ("gd", vec![8, 10, 12])] {
        for bits in bits_list {
            let mut ok = 0; let mut tot = 0; let mut wrong = 0;
            for idx in 0..6 {
                let inst = make_instance(bits, 1000 + idx);
                let rec = run(solver, &inst, 5e9, &prm);
                tot += 1; if rec.success { ok += 1; if !rec.correct { wrong += 1; } }
            }
            println!("[solve] {:9} bits {:2}: {}/{} solved, {} with k != planted", solver, bits, ok, tot, wrong);
            if ok != tot || wrong != 0 { fails += 1; }
        }
    }
    // negative controls: off-by-one candidate rejected by the verifier
    let inst = make_instance(14, 7);
    let c = Curve::new(inst.p, inst.a, inst.b);
    if verify(&c, &inst, inst.k % inst.n + 1) || !verify(&c, &inst, inst.k) { fails += 1; println!("[neg-control] verifier FAIL"); } else { println!("[neg-control] verifier accepts k, rejects k+1"); }
    println!("SELFTEST {}", if fails == 0 { "PASS" } else { "FAIL" });
    if fails != 0 { std::process::exit(1); }
}

/// Control: zero 1x1/2x2 minor frequency of the curve-derived C against an i.i.d. uniform matrix, per tested minor, vs 1/p.
fn controls(args: &[String]) {
    let bits: u32 = arg(args, "--bits").map(|s| s.parse().unwrap()).unwrap_or(14);
    let reps: u64 = arg(args, "--reps").map(|s| s.parse().unwrap()).unwrap_or(20);
    let np: usize = arg(args, "--nprime").map(|s| s.parse().unwrap()).unwrap_or(bits as usize);
    println!("# development control (not a result): zero-minor rate, curve-derived C vs uniform random matrix, bits {} nprime {} reps {}", bits, np, reps);
    let (mut zc1, mut zc2, mut zr1, mut zr2) = (0u64, 0u64, 0u64, 0u64);
    let mut skipped = 0u64; let mut done = 0u64; let mut exp2 = 0.0; let mut exp1 = 0.0;
    for rep in 0..reps {
        let inst = make_instance(bits, 5000 + rep);
        let c = Curve::new(inst.p, inst.a, inst.b);
        let mut rng = Rng::new(mix(inst.seed, 77));
        let k = match kernel::build(&c, inst.g, inst.q, inst.n, np, &mut rng) { Some(k) => k, None => continue };
        let l = k.l;
        let (a1, a2) = count_zero_minors(&c.f, &k.c, l);
        let skip_c = a1 > 0;
        zc1 += a1; if !skip_c { zc2 += a2; }
        let mut rm = vec![0u64; l * l];
        for v in rm.iter_mut() { *v = 1 + rng.below(inst.p - 1); }
        let (b1, b2) = count_zero_minors(&c.f, &rm, l);
        zr1 += b1; if !skip_c { zr2 += b2; }
        done += 1;
        exp1 += (l * l) as f64 / inst.p as f64;
        if !skip_c { exp2 += ((l * (l - 1) / 2) as f64).powi(2) / (inst.p as f64 - 1.0); } else { skipped += 1; }
    }
    println!("matrices {} ({} with a zero entry excluded from the 2x2 comparison): zero entries curve {} (random matrix has none by construction; uniform expectation {:.2}); zero 2x2 minors curve {} random {} (uniform expectation {:.1})", done, skipped, zc1, exp1, zc2, zr2, exp2);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("selftest") => selftest(),
        Some("controls") => controls(&args),
        Some("gen") => {
            let bits: u32 = arg(&args, "--bits").unwrap().parse().unwrap();
            let idx: u64 = arg(&args, "--idx").unwrap().parse().unwrap();
            let i = make_instance(bits, idx);
            println!("{{\"bits\":{},\"idx\":{},\"seed\":\"{:016x}\",\"p\":{},\"a\":{},\"b\":{},\"n\":{},\"g\":[{},{}],\"q\":[{},{}],\"k\":{}}}", i.bits, i.idx, i.seed, i.p, i.a, i.b, i.n, i.g.0, i.g.1, i.q.0, i.q.1, i.k);
        }
        Some("run") => {
            let solver = arg(&args, "--solver").unwrap();
            let bits: u32 = arg(&args, "--bits").unwrap().parse().unwrap();
            let idx: u64 = arg(&args, "--idx").unwrap().parse().unwrap();
            let cap = arg(&args, "--cap").map(|s| parse_f(&s)).unwrap_or(1e12);
            let prm = Params { nprime: arg(&args, "--nprime").map(|s| s.parse().unwrap()), d: arg(&args, "--d").map(|s| s.parse().unwrap()), dev_check: args.iter().any(|a| a == "--dev-check") };
            let inst = make_instance(bits, idx);
            println!("{}", run(&solver, &inst, cap, &prm).json());
        }
        Some("panel") => {
            // --solvers a,b --bits 20,24 --idx-from 0 --idx-to 100 --caps 20:1e9,24:1e9 [--nprime-div2 for gd_half] --out file
            let solvers: Vec<String> = arg(&args, "--solvers").unwrap().split(',').map(|s| s.to_string()).collect();
            let bits: Vec<u32> = arg(&args, "--bits").unwrap().split(',').map(|s| s.parse().unwrap()).collect();
            let (i0, i1): (u64, u64) = (arg(&args, "--idx-from").unwrap().parse().unwrap(), arg(&args, "--idx-to").unwrap().parse().unwrap());
            let caps: std::collections::HashMap<u32, f64> = arg(&args, "--caps").unwrap().split(',').map(|kv| { let mut it = kv.split(':'); (it.next().unwrap().parse().unwrap(), parse_f(it.next().unwrap())) }).collect();
            let threads: usize = arg(&args, "--threads").map(|s| s.parse().unwrap()).unwrap_or(1);
            let out = arg(&args, "--out").unwrap();
            let mut jobs = vec![];
            for &b in &bits { for idx in i0..i1 { for s in &solvers { jobs.push((s.clone(), b, idx)); } } }
            let jobs = std::sync::Arc::new(std::sync::Mutex::new(jobs.into_iter()));
            let file = std::sync::Arc::new(std::sync::Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(&out).unwrap()));
            std::thread::scope(|sc| {
                for _ in 0..threads {
                    let (jobs, file, caps) = (jobs.clone(), file.clone(), &caps);
                    sc.spawn(move || loop {
                        let job = { jobs.lock().unwrap().next() };
                        let (s, b, idx) = match job { Some(j) => j, None => break };
                        let inst = make_instance(b, idx);
                        let (sname, nprime) = if s == "gd_half" { ("gd", Some(((b as usize + 3) / 4) * 2)) } else { (s.as_str(), None) };
                        let prm = Params { nprime, d: None, dev_check: false };
                        let mut rec = run(sname, &inst, caps[&b], &prm);
                        rec.solver = s.clone();
                        use std::io::Write;
                        writeln!(file.lock().unwrap(), "{}", rec.json()).unwrap();
                    });
                }
            });
        }
        _ => eprintln!("usage: minors_gd selftest | controls | gen | run | panel"),
    }
}
