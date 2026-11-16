//! Pilot harness: `lin21` in the native CDCL with clause deletion, a learning-free leaf and
//! multi-word field blocks (n = 131, ECC2K-130 curve).
//!
//! Build: rustc -O -C target-cpu=native n131.rs   (standard library only)
//! Solvers in this binary:
//!   - `fastcdcl`     research/sat_native_lin21_fast_20261116/fastcdcl.rs, UNCHANGED (arm `merged`);
//!   - `fastcdcl_mw`  the copy in this directory (its header lists every change): arms
//!                    `mw` (W = 1, switches off: control), `del` (keep 64), `leaf`, `delleaf`.
//! Field, curve, pair oracle, plain scan and model checks for n <= 63 are
//! `base_lin21_verbatim.rs` of the lin21-fast pilot, included unchanged. The generic-width field,
//! the fast scan leaf port and the point-form pair enumeration are in `mw.rs`.
//!
//! Subcommands: selfcheck | dt | dtmw | small | k130targets | k130

#[path = "../../examples/pdp_lin_cdcl.rs"]
mod pdp_lin_cdcl;
#[path = "../sat_native_lin21_fast_20261116/fastcdcl.rs"]
mod fastcdcl;
mod fastcdcl_mw;
mod mw;
use pdp_lin_cdcl::{ChainInstance, LinCdcl, Result as LinResult};

include!("../sat_native_lin21_fast_20261116/base_lin21_verbatim.rs");

/// The `lin21` encoding for the unmodified `fastcdcl` (same transcription as l21fast.rs).
fn build_merged(n: u32, l: u32, irr: u64, x_r: u64, b: u64) -> (fastcdcl::FastCdcl, Vec<u32>, Vec<u32>, Vec<u32>) {
    let nn = n as usize;
    let mut next = 1u32;
    let mut block = || {
        let v: Vec<u32> = (next..next + n).collect();
        next += n;
        v
    };
    let x1 = block();
    let u = block();
    let a2 = block();
    let bb = block();
    let k = block();
    let e = block();
    let w = block();
    let c = block();
    let t = block();
    let s = block();
    let p = block();
    let st = block();
    let h = block();
    let zz = block();
    let q = block();
    let mut sv = fastcdcl::FastCdcl::new((next - 1) as usize, n, irr);
    let sq = sv.sq_matrix();
    let mr = sv.mul_matrix(x_r);
    let xr2 = sv.gf_mul(x_r, x_r);
    for j in l as usize..nn {
        sv.add_clause(vec![-(x1[j] as i32)]);
        sv.add_clause(vec![-(s[j] as i32)]);
        sv.add_clause(vec![-(zz[j] as i32)]);
    }
    for j in (2 * l - 1) as usize..nn {
        sv.add_clause(vec![-(p[j] as i32)]);
    }
    sv.add_linear_map(&a2, &x1, &sq, xr2);
    sv.add_linear_map(&bb, &x1, &mr, 0);
    sv.add_linear_map(&c, &bb, &sq, b);
    let sq_plus_id: Vec<u64> = (0..nn).map(|j| sq[j] ^ (1u64 << j)).collect();
    sv.add_linear_map(&w, &u, &sq_plus_id, 0);
    for kk in 0..nn {
        let mut vars = vec![h[kk]];
        for j in 0..nn {
            if (sq[j] >> kk) & 1 == 1 {
                vars.push(p[j]);
                vars.push(st[j]);
            }
        }
        sv.add_xor(vars, (b >> kk) & 1 == 1);
    }
    for kk in 0..nn {
        let mut vars = vec![q[kk], p[kk]];
        for j in 0..nn {
            if (sq[j] >> kk) & 1 == 1 {
                vars.push(zz[j]);
            }
        }
        sv.add_xor(vars, false);
    }
    sv.add_gate(k.clone(), a2.clone(), bb.clone());
    sv.add_gate(bb.clone(), k.clone(), e.clone());
    sv.add_gate(e.clone(), w.clone(), c.clone());
    sv.add_gate(k.clone(), u.clone(), t.clone());
    sv.add_gate(t.clone(), s.clone(), st.clone());
    sv.add_gate(t.clone(), p.clone(), h.clone());
    sv.add_gate(s.clone(), zz.clone(), q.clone());
    sv.set_priority(&x1[..l as usize]);
    (sv, x1, s, zz)
}

fn wbit<const W: usize>(a: &[u64; W], i: usize) -> bool {
    (a[i / 64] >> (i % 64)) & 1 == 1
}

/// The same encoding, constraint for constraint and in the same order, for `fastcdcl_mw`.
fn build_mw<const W: usize>(n: u32, l: u32, irr: [u64; W], x_r: [u64; W], b: [u64; W]) -> (fastcdcl_mw::FastCdcl<W>, Vec<u32>, Vec<u32>, Vec<u32>) {
    let nn = n as usize;
    let mut next = 1u32;
    let mut block = || {
        let v: Vec<u32> = (next..next + n).collect();
        next += n;
        v
    };
    let x1 = block();
    let u = block();
    let a2 = block();
    let bb = block();
    let k = block();
    let e = block();
    let w = block();
    let c = block();
    let t = block();
    let s = block();
    let p = block();
    let st = block();
    let h = block();
    let zz = block();
    let q = block();
    let mut sv = fastcdcl_mw::FastCdcl::<W>::new((next - 1) as usize, n, irr);
    let sq = sv.sq_matrix();
    let mr = sv.mul_matrix(x_r);
    let xr2 = sv.gf_mul(x_r, x_r);
    for j in l as usize..nn {
        sv.add_clause(vec![-(x1[j] as i32)]);
        sv.add_clause(vec![-(s[j] as i32)]);
        sv.add_clause(vec![-(zz[j] as i32)]);
    }
    for j in (2 * l - 1) as usize..nn {
        sv.add_clause(vec![-(p[j] as i32)]);
    }
    sv.add_linear_map(&a2, &x1, &sq, xr2);
    sv.add_linear_map(&bb, &x1, &mr, [0u64; W]);
    sv.add_linear_map(&c, &bb, &sq, b);
    let sq_plus_id: Vec<[u64; W]> = (0..nn)
        .map(|j| {
            let mut v = sq[j];
            v[j / 64] ^= 1u64 << (j % 64);
            v
        })
        .collect();
    sv.add_linear_map(&w, &u, &sq_plus_id, [0u64; W]);
    for kk in 0..nn {
        let mut vars = vec![h[kk]];
        for j in 0..nn {
            if wbit(&sq[j], kk) {
                vars.push(p[j]);
                vars.push(st[j]);
            }
        }
        sv.add_xor(vars, wbit(&b, kk));
    }
    for kk in 0..nn {
        let mut vars = vec![q[kk], p[kk]];
        for j in 0..nn {
            if wbit(&sq[j], kk) {
                vars.push(zz[j]);
            }
        }
        sv.add_xor(vars, false);
    }
    sv.add_gate(k.clone(), a2.clone(), bb.clone());
    sv.add_gate(bb.clone(), k.clone(), e.clone());
    sv.add_gate(e.clone(), w.clone(), c.clone());
    sv.add_gate(k.clone(), u.clone(), t.clone());
    sv.add_gate(t.clone(), s.clone(), st.clone());
    sv.add_gate(t.clone(), p.clone(), h.clone());
    sv.add_gate(s.clone(), zz.clone(), q.clone());
    sv.set_priority(&x1[..l as usize]);
    (sv, x1, s, zz)
}

#[derive(Clone, Copy)]
struct Cfg {
    del: Option<usize>,
    leaf: bool,
}

fn cfg_of(arm: &str) -> Cfg {
    match arm {
        "mw" => Cfg { del: None, leaf: false },
        "del" => Cfg { del: Some(64), leaf: false },
        "del1" => Cfg { del: Some(1), leaf: false },
        "leaf" => Cfg { del: None, leaf: true },
        "delleaf" => Cfg { del: Some(64), leaf: true },
        _ => panic!("unknown mw arm {arm}"),
    }
}

struct Out<const W: usize> {
    status: &'static str,
    xs: Option<[[u64; W]; 3]>,
    conflicts: u64,
    wall_ms: f64,
    json: String,
}

fn fxw<const W: usize>(a: [u64; W], b: [u64; W]) -> [u64; W] {
    let mut o = a;
    for i in 0..W {
        o[i] ^= b[i];
    }
    o
}

fn run_mw<const W: usize>(n: u32, l: u32, irr: [u64; W], x_r: [u64; W], cfg: Cfg) -> Out<W> {
    let mut one = [0u64; W];
    one[0] = 1;
    let t0 = Instant::now();
    let (mut sv, x1, s, zz) = build_mw::<W>(n, l, irr, x_r, one);
    sv.delete_keep = cfg.del;
    sv.leaf_nolearn = cfg.leaf;
    let build_ms = t0.elapsed().as_secs_f64() * 1e3;
    let t1 = Instant::now();
    let r = sv.solve();
    let wall_ms = t1.elapsed().as_secs_f64() * 1e3;
    let (status, xs) = match r {
        fastcdcl_mw::Result::Sat => {
            let z = sv.model_block(&zz);
            let sm = sv.model_block(&s);
            ("sat", Some([sv.model_block(&x1), z, fxw(z, sm)]))
        }
        fastcdcl_mw::Result::Unsat => ("unsat", None),
        fastcdcl_mw::Result::Unknown => ("unknown", None),
    };
    let st = &sv.stats;
    let tot: u64 = st.prof_ns.iter().sum();
    let json = format!(
        "\"conflicts\":{},\"decisions\":{},\"propagations\":{},\"xor_propagations\":{},\"gate_fwd_div_mixed\":{:?},\"learnt_clauses\":{},\"learnt_lits\":{},\"deleted_clauses\":{},\"chrono_flips\":{},\"max_live_learnts\":{},\"row_inserts\":{},\"row_xors\":{},\"gauss_words\":{},\"mat_copy_words\":{},\"unit_overflows\":{},\"prof_ns\":{:?},\"clause_prop_share\":{:.5},\"n_vars\":{},\"build_ms\":{:.4},\"wall_ms\":{:.4}",
        st.conflicts, st.decisions, st.propagations, st.xor_propagations, [st.gate_forward, st.gate_divide, st.gate_mixed],
        st.learnt_clauses, st.learnt_lits, st.deleted_clauses, st.chrono_flips, st.max_live_learnts, st.row_inserts, st.row_xors,
        st.row_xor_words, st.mat_copy_words, st.unit_overflows, st.prof_ns, st.prof_ns[0] as f64 / tot.max(1) as f64, sv.n_vars, build_ms, wall_ms
    );
    Out { status, xs, conflicts: st.conflicts, wall_ms, json }
}

fn run_merged(n: u32, l: u32, irr: u64, x_r: u64) -> Out<1> {
    let t0 = Instant::now();
    let (mut sv, x1, s, zz) = build_merged(n, l, irr, x_r, 1);
    let build_ms = t0.elapsed().as_secs_f64() * 1e3;
    let t1 = Instant::now();
    let r = sv.solve();
    let wall_ms = t1.elapsed().as_secs_f64() * 1e3;
    let (status, xs) = match r {
        fastcdcl::Result::Sat => {
            let z = sv.model_block(&zz);
            let sm = sv.model_block(&s);
            ("sat", Some([[sv.model_block(&x1)], [z], [z ^ sm]]))
        }
        fastcdcl::Result::Unsat => ("unsat", None),
        fastcdcl::Result::Unknown => ("unknown", None),
    };
    let st = &sv.stats;
    let tot: u64 = st.prof_ns.iter().sum();
    let json = format!(
        "\"conflicts\":{},\"decisions\":{},\"propagations\":{},\"xor_propagations\":{},\"gate_fwd_div_mixed\":{:?},\"learnt_clauses\":{},\"learnt_lits\":{},\"deleted_clauses\":0,\"chrono_flips\":0,\"max_live_learnts\":{},\"row_inserts\":{},\"row_xors\":{},\"gauss_words\":{},\"mat_copy_words\":{},\"unit_overflows\":{},\"prof_ns\":{:?},\"clause_prop_share\":{:.5},\"n_vars\":{},\"build_ms\":{:.4},\"wall_ms\":{:.4}",
        st.conflicts, st.decisions, st.propagations, st.xor_propagations, [st.gate_forward, st.gate_divide, st.gate_mixed],
        st.learnt_clauses, st.learnt_lits, st.learnt_clauses, st.row_inserts, st.row_xors, st.row_xor_words, st.mat_copy_words,
        st.unit_overflows, st.prof_ns, st.prof_ns[0] as f64 / tot.max(1) as f64, sv.n_vars, build_ms, wall_ms
    );
    Out { status, xs, conflicts: st.conflicts, wall_ms, json }
}

fn xs1(xs: Option<[[u64; 1]; 3]>) -> Option<[u64; 3]> {
    xs.map(|x| [x[0][0], x[1][0], x[2][0]])
}

fn chk_json(cv: &Curve, l: u32, xs: Option<[u64; 3]>, x_r: u64) -> String {
    match xs {
        Some(xs) => {
            let (ib, lf, sm) = check_model(cv, l, xs, x_r);
            format!("\"in_base\":{ib},\"liftable\":{lf},\"sums_to_target\":{sm},\"chain_ok\":{}", oracle_has(cv, xs, x_r))
        }
        None => "\"in_base\":null,\"liftable\":null,\"sums_to_target\":null,\"chain_ok\":null".into(),
    }
}

fn arg(v: &[String], name: &str) -> Option<String> {
    v.iter().position(|a| a == name).map(|i| v[i + 1].clone())
}

fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

// ------------------------------------------------------------------ differential test, n <= 63

/// Fresh targets: uniform admissible abscissae and planted sums of three base points.
fn dt_cell(n: u32, l: u32, per: usize, seed: u64) -> (String, bool) {
    let irr = find_irr(n);
    let gf = Gf::new(n, irr);
    let cv = Curve { gf: &gf, a: 1, b: 1 };
    let pow = 1u64 << l;
    let mut rng = SplitMix(seed ^ ((n as u64) << 32) ^ ((l as u64) << 48));
    let base: Vec<Pt> = (1..pow).filter_map(|x| cv.lift_x(x)).collect();
    let arms = ["mw", "del", "del1", "leaf", "delleaf"];
    let (mut tested, mut dec, mut planted) = (0u64, 0u64, 0u64);
    let (mut verdict_mism, mut bad_models, mut nonlift, mut models) = (0u64, 0u64, 0u64, 0u64);
    let (mut ctrl_conf_diff, mut w2_conf_diff) = (0u64, 0u64);
    let mut conf_diff = [0u64; 5];
    let mut conf_sum = [0u64; 5];
    let mut conf_merged = 0u64;
    let mut flips = 0u64;
    let mut deleted = 0u64;
    while (tested as usize) < per {
        let want_planted = tested % 2 == 1;
        let x = if want_planted {
            let mut acc = Pt::Inf;
            for _ in 0..3 {
                let p = base[(rng.next() % base.len() as u64) as usize];
                acc = cv.add(acc, if rng.next() & 1 == 1 { cv.neg(p) } else { p });
            }
            match acc {
                Pt::Aff(x, _) if x >= pow => x,
                _ => continue,
            }
        } else {
            let x = rng.next() & ((1u64 << n) - 1);
            if !target_ok(&cv, l, x) {
                continue;
            }
            x
        };
        tested += 1;
        planted += want_planted as u64;
        let ok = target_ok(&cv, l, x);
        let alg = oracle_pairs(&cv, l, x).witness.is_some();
        dec += alg as u64;
        let m = run_merged(n, l, irr, x);
        conf_merged += m.conflicts;
        if (m.status == "sat") != alg {
            verdict_mism += 1;
        }
        for (ai, arm) in arms.iter().enumerate() {
            let r = run_mw::<1>(n, l, [irr], [x], cfg_of(arm));
            if r.status != m.status || (r.status == "sat") != alg {
                verdict_mism += 1;
            }
            if r.conflicts != m.conflicts {
                conf_diff[ai] += 1;
                if ai == 0 {
                    ctrl_conf_diff += 1;
                }
            }
            conf_sum[ai] += r.conflicts;
            if let Some(xs) = xs1(r.xs) {
                models += 1;
                let (ib, lf, sm) = check_model(&cv, l, xs, x);
                if !ib || !oracle_has(&cv, xs, x) {
                    bad_models += 1;
                }
                if ok && !(lf && sm) {
                    nonlift += 1;
                }
            }
            if *arm == "delleaf" {
                flips += r.json.split("\"chrono_flips\":").nth(1).unwrap().split(',').next().unwrap().parse::<u64>().unwrap();
                deleted += r.json.split("\"deleted_clauses\":").nth(1).unwrap().split(',').next().unwrap().parse::<u64>().unwrap();
            }
        }
        // two-word blocks on a one-word field: the same search is required
        let r2 = run_mw::<2>(n, l, [irr, 0], [x, 0], cfg_of("mw"));
        if r2.conflicts != m.conflicts || r2.status != m.status {
            w2_conf_diff += 1;
        }
    }
    let pass = verdict_mism == 0 && bad_models == 0 && nonlift == 0 && ctrl_conf_diff == 0 && w2_conf_diff == 0;
    (
        format!("{{\"test\":\"dt\",\"n\":{n},\"l\":{l},\"seed\":{seed},\"instances\":{tested},\"planted\":{planted},\"decomposable\":{dec},\"verdict_mismatches\":{verdict_mism},\"models_checked\":{models},\"bad_models\":{bad_models},\"nonliftable_models_rule_ok\":{nonlift},\"ctrl_conflict_diffs_vs_merged\":{ctrl_conf_diff},\"w2_ctrl_diffs_vs_merged\":{w2_conf_diff},\"arms\":{arms:?},\"conflict_count_diffs_vs_merged\":{conf_diff:?},\"sum_conflicts\":{conf_sum:?},\"sum_conflicts_merged\":{conf_merged},\"delleaf_chrono_flips\":{flips},\"delleaf_deleted\":{deleted},\"pass\":{pass}}}"),
        pass,
    )
}

// ------------------------------------------------------------------ generic width

fn low_terms(n: usize) -> Vec<usize> {
    match n {
        67 => vec![5, 2, 1, 0],
        71 => vec![6, 0],
        // ECC2K-130 field: z^131 + z^8 + z^3 + z^2 + 1 (ecc2k130/benchmarks/sparse-basis-compound/PROTOCOL.md, f_1)
        131 => vec![8, 3, 2, 0],
        _ => {
            assert!(n <= 63);
            let irr = find_irr(n as u32);
            (0..n).filter(|&t| (irr >> t) & 1 == 1).collect()
        }
    }
}

struct Xs(u64);
impl Xs {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

fn rand_fe<const W: usize>(rng: &mut Xs, n: usize) -> [u64; W] {
    let mut o = [0u64; W];
    for w in 0..W {
        let lo = 64 * w;
        if n > lo {
            let bits = (n - lo).min(64);
            o[w] = rng.next() & if bits == 64 { u64::MAX } else { (1u64 << bits) - 1 };
        }
    }
    o
}

/// (kind, x_R) targets: uniform rational abscissae outside V, and planted sums of three base points.
fn gen_targets<const W: usize>(cx: &mw::Ctx<W>, rng: &mut Xs, n_uniform: usize, n_planted: usize) -> Vec<(&'static str, [u64; W])> {
    let mut out = Vec::new();
    while out.len() < n_uniform {
        let x = rand_fe::<W>(rng, cx.n);
        if mw::fsmall(&x, cx.d).is_some() || cx.lift_x(x).is_none() {
            continue;
        }
        out.push(("uniform", x));
    }
    let np = cx.points.len() as u64;
    while out.len() < n_uniform + n_planted {
        let mut acc = mw::Ctx::<W>::inf();
        for _ in 0..3 {
            acc = cx.add(acc, cx.points[1 + (rng.next() % (np - 1)) as usize]);
        }
        if acc.inf || mw::fsmall(&acc.x, cx.d).is_some() {
            continue;
        }
        out.push(("planted", acc.x));
    }
    out
}

fn irr_words<const W: usize>(gf: &mw::Gf<W>) -> [u64; W] {
    gf.irr
}

/// Multi-word differential cell: solver arms against the point-form pair enumeration and the
/// fast leaf port; models must lift to points with a signed sum equal to the target.
fn dtmw_cell<const W: usize>(n: usize, d: usize, a: u64, n_uniform: usize, n_planted: usize, seed: u64) -> (String, bool) {
    let gf = mw::Gf::<W>::new(n, &low_terms(n));
    let irr_ok = gf.irreducible_prime_degree();
    let irr = irr_words(&gf);
    let cx = mw::Ctx::<W>::new(gf, d, a, 1);
    let mut rng = Xs(seed ^ ((n as u64) << 32) ^ ((d as u64) << 48) ^ ((W as u64) << 56));
    let targets = gen_targets(&cx, &mut rng, n_uniform, n_planted);
    let mut ws = mw::Scratch::<W>::new();
    let (mut mism, mut bad_models, mut models, mut bad_hits, mut sat) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut conf_ctrl, mut conf_dl) = (0u64, 0u64);
    for (_, x) in &targets {
        let r = cx.lift_x(*x).unwrap();
        assert!(cx.on_curve(&r));
        let (mut hp, mut hs) = (Vec::new(), Vec::new());
        cx.pairs(r, &mut ws, &mut hp);
        cx.scan(r, &mut ws, &mut hs);
        for t in hp.iter().chain(hs.iter()) {
            if !cx.triple_ok(*t, r) {
                bad_hits += 1;
            }
        }
        let truth = !hp.is_empty();
        sat += truth as u64;
        if hs.is_empty() == truth {
            mism += 1;
        }
        for arm in ["mw", "delleaf", "del1", "leaf"] {
            let o = run_mw::<W>(n as u32, d as u32, irr, *x, cfg_of(arm));
            if (o.status == "sat") != truth || o.status == "unknown" {
                mism += 1;
            }
            if arm == "mw" {
                conf_ctrl += o.conflicts;
            }
            if arm == "delleaf" {
                conf_dl += o.conflicts;
            }
            if let Some(xs) = o.xs {
                models += 1;
                let in_base = xs.iter().all(|v| mw::fsmall(v, d).is_some());
                let (lf, sm) = cx.lift_check(xs, *x);
                if !(in_base && lf && sm) {
                    bad_models += 1;
                }
            }
        }
    }
    let pass = irr_ok && mism == 0 && bad_models == 0 && bad_hits == 0;
    (
        format!("{{\"test\":\"dtmw\",\"W\":{W},\"n\":{n},\"d\":{d},\"a\":{a},\"seed\":{seed},\"irreducible\":{irr_ok},\"A\":{},\"targets\":{},\"decomposable\":{sat},\"verdict_mismatches\":{mism},\"models_checked\":{models},\"bad_models\":{bad_models},\"bad_hits\":{bad_hits},\"sum_conflicts_mw\":{conf_ctrl},\"sum_conflicts_delleaf\":{conf_dl},\"pass\":{pass}}}", cx.admissible(), targets.len()),
        pass,
    )
}

/// The fast leaf port on one n <= 63 target (n odd): (verdict, leaves, consistent, median ms, setup ms).
fn fastleaf_small(n: u32, l: u32, x_r: u64, reps: usize) -> Option<(bool, mw::ScanStat, f64, f64, usize)> {
    if n % 2 == 0 {
        return None;
    }
    let gf = mw::Gf::<1>::new(n as usize, &low_terms(n as usize));
    let cx = mw::Ctx::<1>::new(gf, l as usize, 1, 1);
    let r = cx.lift_x([x_r])?;
    let mut ws = mw::Scratch::<1>::new();
    let mut out = Vec::new();
    let st = cx.scan(r, &mut ws, &mut out);
    let n_hits = out.len();
    let mut times = Vec::new();
    for _ in 0..reps {
        out.clear();
        let t0 = Instant::now();
        let s2 = cx.scan(r, &mut ws, &mut out);
        times.push(t0.elapsed().as_secs_f64() * 1e3);
        assert!(s2.leaves == st.leaves && out.len() == n_hits);
    }
    Some((n_hits > 0, st, median(&mut times), cx.setup_ns as f64 / 1e6, n_hits))
}

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let cmd = v.get(1).map(|s| s.as_str()).unwrap_or("");
    match cmd {
        "selfcheck" => {
            let clm = mw::clmul_selfcheck();
            let gf = mw::Gf::<3>::new(131, &low_terms(131));
            let irr_ok = gf.irreducible_prime_degree();
            // generic field against the bit-serial reference at one word
            let mut rng = Xs(4242);
            let mut field_ok = true;
            for n in [17u32, 29, 41, 62] {
                let irr = find_irr(n);
                let g1 = mw::Gf::<1>::from_irr_u64(n as usize, irr);
                for _ in 0..2000 {
                    let (a, b) = (rng.next() & ((1u64 << n) - 1), rng.next() & ((1u64 << n) - 1));
                    field_ok &= g1.mul([a], [b])[0] == raw_mul(n, irr, a, b);
                    if a != 0 {
                        field_ok &= g1.mul(g1.inv([a]), [a])[0] == 1;
                    }
                }
            }
            for _ in 0..2000 {
                let a = rand_fe::<3>(&mut rng, 131);
                if !mw::fz(&a) {
                    field_ok &= gf.mul(gf.inv(a), a) == mw::fe::<3>(1);
                }
            }
            // the panel's factor-base sizes A at d = 4..10 (RESEARCH_ECC2K130_RR_SOLVER_PANEL.md section 11)
            let panel = [10usize, 17, 34, 74, 131, 252, 519];
            let mut got = Vec::new();
            for d in 4..=10 {
                let cx = mw::Ctx::<3>::new(mw::Gf::<3>::new(131, &low_terms(131)), d, 0, 1);
                got.push(cx.admissible());
            }
            let a_ok = got == panel;
            println!("{{\"test\":\"selfcheck\",\"clmul_backend\":\"{}\",\"clmul_matches_portable\":{clm},\"f131_irreducible\":{irr_ok},\"field_ok\":{field_ok},\"A_d4_to_d10\":{got:?},\"panel_A\":{panel:?},\"A_matches_panel\":{a_ok}}}", mw::clmul_backend());
            if !(clm && irr_ok && field_ok && a_ok) {
                std::process::exit(1);
            }
        }
        "dt" => {
            let seed: u64 = arg(&v, "--seed").expect("--seed").parse().unwrap();
            let per: usize = arg(&v, "--per").map(|s| s.parse().unwrap()).unwrap_or(400);
            let mut all = true;
            for (n, l) in [(10u32, 3u32), (12, 3), (13, 3), (14, 4), (15, 4), (17, 5), (20, 6)] {
                let (js, pass) = dt_cell(n, l, per, seed);
                println!("{js}");
                all &= pass;
            }
            if !all {
                std::process::exit(1);
            }
        }
        "dtmw" => {
            let seed: u64 = arg(&v, "--seed").expect("--seed").parse().unwrap();
            let k: usize = arg(&v, "--per").map(|s| s.parse().unwrap()).unwrap_or(6);
            let mut all = true;
            let mut emit = |r: (String, bool)| {
                println!("{}", r.0);
                all &= r.1;
            };
            emit(dtmw_cell::<1>(17, 5, 1, k, k, seed));
            emit(dtmw_cell::<1>(29, 5, 1, k, k, seed));
            emit(dtmw_cell::<2>(17, 5, 1, k, k, seed));
            emit(dtmw_cell::<2>(67, 3, 1, k, k, seed));
            emit(dtmw_cell::<2>(67, 4, 0, k, k, seed));
            emit(dtmw_cell::<2>(71, 4, 1, k, k, seed));
            emit(dtmw_cell::<3>(71, 4, 0, k, k, seed));
            emit(dtmw_cell::<3>(131, 3, 0, k, k, seed));
            emit(dtmw_cell::<3>(131, 4, 0, k, k, seed));
            emit(dtmw_cell::<3>(131, 5, 0, k, k, seed));
            emit(dtmw_cell::<3>(131, 6, 0, k / 2, k / 2, seed));
            if !all {
                std::process::exit(1);
            }
        }
        "small" => {
            let l: u32 = arg(&v, "--l").expect("--l").parse().unwrap();
            let n: u32 = arg(&v, "--n").map(|s| s.parse().unwrap()).unwrap_or(3 * l + 2);
            let x_r: u64 = arg(&v, "--xr").expect("--xr").parse().unwrap();
            let idx: usize = arg(&v, "--idx").map(|s| s.parse().unwrap()).unwrap_or(0);
            let reps: usize = arg(&v, "--reps").map(|s| s.parse().unwrap()).unwrap_or(21);
            let irr = find_irr(n);
            let gf = Gf::new(n, irr);
            let cv = Curve { gf: &gf, a: 1, b: 1 };
            let head = |name: &str, pos: usize| format!("\"series\":\"small\",\"arm\":\"{name}\",\"order\":{pos},\"idx\":{idx},\"n\":{n},\"l\":{l},\"irr\":{irr},\"x_r\":{x_r}");
            // plain scan of the lin21 harness
            let out = scan21(&cv, l, x_r, false);
            let mut times: Vec<f64> = Vec::new();
            for _ in 0..reps {
                let t0 = Instant::now();
                let o = scan21(&cv, l, x_r, false);
                times.push(t0.elapsed().as_secs_f64() * 1e3);
                assert!(o.witness == out.witness);
            }
            let status = if out.witness.is_some() { "sat" } else { "unsat" };
            println!("{{{},\"status\":\"{status}\",\"leaves\":{},\"solutions\":{},\"reps\":{reps},\"wall_ms\":{:.6}}}", head("scan", 0), out.leaves, out.solutions, median(&mut times));
            // fast leaf port (n odd only)
            if let Some((sat, st, ms, setup_ms, hits)) = fastleaf_small(n, l, x_r, reps) {
                println!(
                    "{{{},\"status\":\"{}\",\"leaves\":{},\"leaves_consistent\":{},\"hits\":{hits},\"reps\":{reps},\"wall_ms\":{ms:.6},\"ns_per_leaf\":{:.2},\"setup_ms\":{setup_ms:.4}}}",
                    head("fastleaf", 1), if sat { "sat" } else { "unsat" }, st.leaves, st.consistent, ms * 1e6 / st.leaves.max(1) as f64
                );
            }
            // solver arms, rotated by the target index; the second run of `merged` is the A/A control
            let list = ["merged", "mw", "del", "leaf", "delleaf", "merged"];
            let mut merged_seen = false;
            for k in 0..list.len() {
                let arm = list[(k + idx) % list.len()];
                let (name, o) = if arm == "merged" {
                    let o = run_merged(n, l, irr, x_r);
                    let name = if merged_seen { "merged-b" } else { "merged" };
                    merged_seen = true;
                    (name, o)
                } else {
                    (arm, run_mw::<1>(n, l, [irr], [x_r], cfg_of(arm)))
                };
                let xs = xs1(o.xs);
                println!("{{{},\"status\":\"{}\",\"model_xs\":{},{},{}}}", head(name, k + 2), o.status, xs_json(xs), chk_json(&cv, l, xs, x_r), o.json);
            }
        }
        "k130targets" => {
            let d: usize = arg(&v, "--d").expect("--d").parse().unwrap();
            let seed: u64 = arg(&v, "--seed").expect("--seed").parse().unwrap();
            let nu: usize = arg(&v, "--uniform").map(|s| s.parse().unwrap()).unwrap_or(8);
            let npl: usize = arg(&v, "--planted").map(|s| s.parse().unwrap()).unwrap_or(2);
            let cx = mw::Ctx::<3>::new(mw::Gf::<3>::new(131, &low_terms(131)), d, 0, 1);
            let mut rng = Xs(seed ^ ((d as u64) << 48));
            for (idx, (kind, x)) in gen_targets(&cx, &mut rng, nu, npl).iter().enumerate() {
                println!("{{\"idx\":{idx},\"kind\":\"{kind}\",\"n\":131,\"d\":{d},\"seed\":{seed},\"x_r\":\"{}\"}}", mw::fhex(x));
            }
        }
        "k130" => {
            let d: usize = arg(&v, "--d").expect("--d").parse().unwrap();
            let x_r: [u64; 3] = mw::fparse(&arg(&v, "--xr").expect("--xr"));
            let idx: usize = arg(&v, "--idx").map(|s| s.parse().unwrap()).unwrap_or(0);
            let kind = arg(&v, "--kind").unwrap_or_else(|| "uniform".into());
            let arm_solver = arg(&v, "--solver").unwrap_or_else(|| "delleaf".into());
            let with_pairs = arg(&v, "--pairs").map(|s| s == "1").unwrap_or(true);
            let reps: usize = arg(&v, "--reps").map(|s| s.parse().unwrap()).unwrap_or(if d <= 10 { 21 } else { 5 });
            let gf = mw::Gf::<3>::new(131, &low_terms(131));
            let irr = gf.irr;
            let cx = mw::Ctx::<3>::new(gf, d, 0, 1);
            let a = cx.admissible() as u64;
            let np = cx.points.len() as u64;
            let r = cx.lift_x(x_r).expect("target not rational");
            assert!(cx.on_curve(&r));
            let head = |name: &str, pos: usize| {
                format!("\"series\":\"k130\",\"arm\":\"{name}\",\"order\":{pos},\"idx\":{idx},\"kind\":\"{kind}\",\"n\":131,\"d\":{d},\"x_r\":\"{}\",\"A\":{a},\"base_points\":{np},\"pair_count_CA2\":{}", mw::fhex(&x_r), a * (a - 1) / 2)
            };
            let mut ws = mw::Scratch::<3>::new();
            let order: [&str; 4] = match idx % 3 {
                0 => ["fastleaf", "solver", "pairs", "solver-b"],
                1 => ["solver", "pairs", "fastleaf", "solver-b"],
                _ => ["pairs", "solver", "fastleaf", "solver-b"],
            };
            for (pos, arm) in order.iter().enumerate() {
                match *arm {
                    "fastleaf" => {
                        let mut out = Vec::new();
                        let st = cx.scan(r, &mut ws, &mut out);
                        let ok = out.iter().all(|t| cx.triple_ok(*t, r));
                        let hits = out.len();
                        let mut times = Vec::new();
                        for _ in 0..reps {
                            out.clear();
                            let t0 = Instant::now();
                            cx.scan(r, &mut ws, &mut out);
                            times.push(t0.elapsed().as_secs_f64() * 1e3);
                        }
                        let ms = median(&mut times);
                        println!(
                            "{{{},\"status\":\"{}\",\"leaves\":{},\"leaves_consistent\":{},\"splits\":{},\"hits\":{hits},\"hits_verified\":{ok},\"reps\":{reps},\"wall_ms\":{ms:.6},\"ns_per_leaf\":{:.2},\"setup_ms\":{:.4}}}",
                            head("fastleaf", pos), if hits > 0 { "sat" } else { "unsat" }, st.leaves, st.consistent, st.splits, ms * 1e6 / st.leaves.max(1) as f64, cx.setup_ns as f64 / 1e6
                        );
                    }
                    "pairs" => {
                        if !with_pairs {
                            continue;
                        }
                        let mut out = Vec::new();
                        let t0 = Instant::now();
                        let tested = cx.pairs(r, &mut ws, &mut out);
                        let ms = t0.elapsed().as_secs_f64() * 1e3;
                        let ok = out.iter().all(|t| cx.triple_ok(*t, r));
                        println!(
                            "{{{},\"status\":\"{}\",\"point_pairs_tested\":{tested},\"hits\":{},\"hits_verified\":{ok},\"reps\":1,\"wall_ms\":{ms:.6},\"ns_per_point_pair\":{:.2}}}",
                            head("pairs", pos), if out.is_empty() { "unsat" } else { "sat" }, out.len(), ms * 1e6 / tested as f64
                        );
                    }
                    _ => {
                        let o = run_mw::<3>(131, d as u32, irr, x_r, cfg_of(&arm_solver));
                        let chk = match o.xs {
                            Some(xs) => {
                                let in_base = xs.iter().all(|v| mw::fsmall(v, d).is_some());
                                let (lf, sm) = cx.lift_check(xs, x_r);
                                format!("\"model_xs\":[{},{},{}],\"in_base\":{in_base},\"liftable\":{lf},\"sums_to_target\":{sm}", xs[0][0], xs[1][0], xs[2][0])
                            }
                            None => "\"model_xs\":null,\"in_base\":null,\"liftable\":null,\"sums_to_target\":null".into(),
                        };
                        println!("{{{},\"variant\":\"{arm_solver}\",\"status\":\"{}\",{chk},{}}}", head(arm, pos), o.status, o.json);
                    }
                }
            }
        }
        _ => {
            eprintln!("usage: n131 selfcheck | dt --seed S [--per K] | dtmw --seed S [--per K] | small --l L --xr X --idx I | k130targets --d D --seed S | k130 --d D --xr HEX --idx I --kind K");
            std::process::exit(2);
        }
    }
}
