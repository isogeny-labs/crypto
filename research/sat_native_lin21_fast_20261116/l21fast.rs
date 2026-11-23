//! Pilot: wall-time gap between the `lin21` SAT arm and the plain native 2+1 scan.
//!
//! Standalone: `rustc -O l21fast.rs` (standard library only). Three solvers in one binary:
//!   - `pdp_lin_cdcl`  : examples/pdp_lin_cdcl.rs, UNCHANGED (arm `lin21`, the baseline);
//!   - `lincdcl_prof`  : the same file plus timers and counters only (prof.sed);
//!   - `fastcdcl`      : the modified copy (arm `lin21-fast`; its header lists every change).
//! Field, curve, pair oracle, plain scan, the original `Lin21Instance` and the model checks are
//! `base_lin21_verbatim.rs` = lines 16..862 of research/sat_native_lin21_20261116/lin21.rs,
//! extracted by run.sh and compared byte for byte at every build.
//!
//! Subcommands: `selftest`, `profile`, `run` (all arms on one target, one JSON line per arm).

#[path = "../../examples/pdp_lin_cdcl.rs"]
mod pdp_lin_cdcl;
mod fastcdcl;
mod lincdcl_prof;
use pdp_lin_cdcl::{ChainInstance, LinCdcl, Result as LinResult};

include!("base_lin21_verbatim.rs");

/// The `lin21` encoding (transcription of `Lin21Instance::build` with prio_u0 = false) for any
/// solver type with the `LinCdcl` builder API. Returns (solver, x1, s, zz).
macro_rules! lin21_build {
    ($S:ty, $n:expr, $l:expr, $irr:expr, $x_r:expr, $b:expr) => {{
        let (n, l, irr, x_r, b): (u32, u32, u64, u64, u64) = ($n, $l, $irr, $x_r, $b);
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
        let mut sv = <$S>::new((next - 1) as usize, n, irr);
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
    }};
}

struct ArmOut {
    status: &'static str,
    xs: Option<[u64; 3]>,
    conflicts: u64,
    json: String,
}

fn run_fast(n: u32, l: u32, irr: u64, x_r: u64, b: u64) -> ArmOut {
    let t0 = Instant::now();
    let (mut sv, x1, s, zz) = lin21_build!(fastcdcl::FastCdcl, n, l, irr, x_r, b);
    // tuning knob (step 2, non-measured seeds only): FAST_PIVOT=<static><dynamic>, h or l each
    if let Ok(pv) = std::env::var("FAST_PIVOT") {
        sv.pivot_high_static = pv.as_bytes()[0] == b'h';
        sv.pivot_high = pv.as_bytes()[1] == b'h';
    }
    let build_ms = t0.elapsed().as_secs_f64() * 1e3;
    let t1 = Instant::now();
    let r = sv.solve();
    let wall_ms = t1.elapsed().as_secs_f64() * 1e3;
    let (status, xs) = match r {
        fastcdcl::Result::Sat => {
            let z = sv.model_block(&zz);
            let sm = sv.model_block(&s);
            ("sat", Some([sv.model_block(&x1), z, z ^ sm]))
        }
        fastcdcl::Result::Unsat => ("unsat", None),
        fastcdcl::Result::Unknown => ("unknown", None),
    };
    let st = &sv.stats;
    let rw = sv.row_words();
    let json = format!(
        "\"conflicts\":{},\"decisions\":{},\"propagations\":{},\"xor_propagations\":{},\"gate_linearisations\":{},\"gate_fwd_div_mixed\":{:?},\"learnt_clauses\":{},\"learnt_lits\":{},\"learnt_priority_width\":{:?},\"row_inserts\":{},\"row_xors\":{},\"row_words\":{},\"gauss_words\":{},\"col_xors\":{},\"col_xor_words\":{},\"mat_copies\":{},\"mat_copy_words\":{},\"unit_overflows\":{},\"prof_names\":{:?},\"prof_ns\":{:?},\"n_vars\":{},\"build_ms\":{:.4},\"wall_ms\":{:.4}",
        st.conflicts, st.decisions, st.propagations, st.xor_propagations, st.gate_linearisations, [st.gate_forward, st.gate_divide, st.gate_mixed],
        st.learnt_clauses, st.learnt_lits, st.learnt_priority_width, st.row_inserts, st.row_xors, rw,
        st.row_xor_words, st.col_xors, st.col_xor_words, st.mat_copies, st.mat_copy_words, st.unit_overflows,
        fastcdcl::PROF_NAMES, st.prof_ns, sv.n_vars, build_ms, wall_ms
    );
    ArmOut { status, xs, conflicts: st.conflicts, json }
}

/// The profiling copy of the original solver (same search; timers and counters added).
fn run_prof(n: u32, l: u32, irr: u64, x_r: u64, b: u64) -> ArmOut {
    let (mut sv, x1, s, zz) = lin21_build!(lincdcl_prof::LinCdcl, n, l, irr, x_r, b);
    let t1 = Instant::now();
    let r = sv.solve();
    let wall_ms = t1.elapsed().as_secs_f64() * 1e3;
    let (status, xs) = match r {
        lincdcl_prof::Result::Sat => {
            let z = sv.model_block(&zz);
            let sm = sv.model_block(&s);
            ("sat", Some([sv.model_block(&x1), z, z ^ sm]))
        }
        lincdcl_prof::Result::Unsat => ("unsat", None),
        lincdcl_prof::Result::Unknown => ("unknown", None),
    };
    let st = &sv.stats;
    let json = format!(
        "\"conflicts\":{},\"decisions\":{},\"propagations\":{},\"xor_propagations\":{},\"gje_calls\":{},\"gate_linearisations\":{},\"learnt_clauses\":{},\"learnt_lits\":{},\"row_xors\":{},\"gauss_words\":{},\"explain_calls\":{},\"explain_lits\":{},\"prof_names\":[\"clause_prop\",\"gate_inject\",\"gje_build\",\"gje_eliminate\",\"gje_scan_explain\",\"analyze\",\"backtrack\",\"pick_branch\",\"learn_add\"],\"prof_ns\":{:?},\"n_vars\":{},\"wall_ms\":{:.4}",
        st.conflicts, st.decisions, st.propagations, st.xor_propagations, st.gje_calls,
        st.gate_linearisations, st.learnt_clauses, st.learnt_lits, st.row_xors, st.row_xor_words,
        st.explain_calls, st.explain_lits, st.prof_ns, sv.n_vars, wall_ms
    );
    ArmOut { status, xs, conflicts: st.conflicts, json }
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

/// Exhaustive / sampled agreement: pair oracle = scan = original lin21 = lin21-fast.
fn fast_selftest_cell(n: u32, l: u32, sample: Option<usize>, seed: u64) -> (String, bool) {
    let irr = find_irr(n);
    let gf = Gf::new(n, irr);
    let cv = Curve { gf: &gf, a: 1, b: 1 };
    let pow = 1u64 << l;
    let mut cand: Vec<u64> = (pow..(1u64 << n)).filter(|&x| cv.lift_x(x).is_some()).collect();
    let total = cand.len();
    if let Some(k) = sample {
        let mut rng = SplitMix(seed);
        let mut keep = Vec::new();
        for _ in 0..k.min(cand.len()) {
            let i = (rng.next() % cand.len() as u64) as usize;
            keep.push(cand.swap_remove(i));
        }
        cand = keep;
    }
    let (mut tested, mut dec, mut mism, mut bad_models, mut models, mut nonlift_ok, mut excl) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut conf_orig, mut conf_fast) = (0u64, 0u64);
    for &x in &cand {
        tested += 1;
        let ok = target_ok(&cv, l, x);
        excl += !ok as u64;
        let alg = oracle_pairs(&cv, l, x).witness.is_some();
        dec += alg as u64;
        let sc = scan21(&cv, l, x, false).witness.is_some();
        let orig = sat_arm("lin21", n, l, irr, x, 1, u64::MAX);
        let fast = run_fast(n, l, irr, x, 1);
        let prof = run_prof(n, l, irr, x, 1);
        if sc != alg || (orig.status == "sat") != alg || (fast.status == "sat") != alg || fast.status == "unknown" || prof.status != orig.status {
            mism += 1;
        }
        if let Some(xs) = fast.xs {
            models += 1;
            let (ib, lf, sm) = check_model(&cv, l, xs, x);
            if !ib || !oracle_has(&cv, xs, x) {
                bad_models += 1;
            }
            if ok && !(lf && sm) {
                nonlift_ok += 1;
            }
        }
        conf_fast += fast.conflicts;
        conf_orig += prof.conflicts;
    }
    let pass = mism == 0 && bad_models == 0 && nonlift_ok == 0;
    (
        format!("{{\"n\":{n},\"l\":{l},\"irr\":{irr},\"rational_targets_outside_V\":{total},\"targets_tested\":{tested},\"rule_excluded\":{excl},\"alg_decomposable\":{dec},\"verdict_mismatches\":{mism},\"fast_models\":{models},\"fast_bad_models\":{bad_models},\"fast_nonliftable_models_rule_ok\":{nonlift_ok},\"sum_conflicts_lin21\":{conf_orig},\"sum_conflicts_lin21_fast\":{conf_fast},\"pass\":{pass}}}"),
        pass,
    )
}

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let cmd = v.get(1).map(|s| s.as_str()).unwrap_or("");
    match cmd {
        "selftest" => {
            let mut all = true;
            for (n, l, sample) in [(11u32, 3u32, None), (8, 3, None), (7, 3, None), (14, 4, Some(400usize)), (17, 5, Some(120)), (20, 6, Some(40))] {
                let (js, pass) = fast_selftest_cell(n, l, sample, 78);
                println!("{js}");
                all &= pass;
            }
            if !all {
                std::process::exit(1);
            }
        }
        "profile" | "run" => {
            let l: u32 = arg(&v, "--l").expect("--l").parse().unwrap();
            let n: u32 = arg(&v, "--n").map(|s| s.parse().unwrap()).unwrap_or(3 * l + 2);
            let x_r: u64 = arg(&v, "--xr").expect("--xr").parse().unwrap();
            let reps: usize = arg(&v, "--reps").map(|s| s.parse().unwrap()).unwrap_or(21);
            let with_prof = cmd == "profile" || arg(&v, "--prof").map(|s| s == "1").unwrap_or(false);
            let arms: Vec<String> = arg(&v, "--arms")
                .unwrap_or_else(|| if cmd == "profile" { "prof,fast".into() } else { "scan,fast,lin21,fast".into() })
                .split(',')
                .map(|s| s.to_string())
                .collect();
            let irr = find_irr(n);
            let gf = Gf::new(n, irr);
            let cv = Curve { gf: &gf, a: 1, b: 1 };
            let mut fast_rep = 0;
            let mut arms = arms;
            if with_prof && cmd == "run" {
                arms.push("prof".into());
            }
            for (pos, arm) in arms.iter().enumerate() {
                let head = |name: &str| format!("\"arm\":\"{name}\",\"order\":{pos},\"n\":{n},\"l\":{l},\"irr\":{irr},\"x_r\":{x_r}");
                match arm.as_str() {
                    "scan" => {
                        gf.reset_counts();
                        let out = scan21(&cv, l, x_r, false);
                        let (muls, invs, ass) = (gf.muls.get(), gf.invs.get(), gf.as_solves.get());
                        let mut times: Vec<f64> = Vec::new();
                        for _ in 0..reps {
                            let t0 = Instant::now();
                            let o = scan21(&cv, l, x_r, false);
                            times.push(t0.elapsed().as_secs_f64() * 1e3);
                            assert!(o.witness == out.witness);
                        }
                        times.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        let status = if out.witness.is_some() { "sat" } else { "unsat" };
                        println!(
                            "{{{},\"status\":\"{status}\",\"model_xs\":{},{},\"x1_visited\":{},\"x1_with_roots\":{},\"leaves\":{},\"leaves_consistent\":{},\"solutions\":{},\"field_muls\":{muls},\"field_invs\":{invs},\"as_solves\":{ass},\"xtimes\":{},\"row_xors\":{},\"gauss_words\":{},\"reps\":{reps},\"wall_ms\":{:.6},\"wall_ms_min\":{:.6}}}",
                            head("scan"), xs_json(out.witness), chk_json(&cv, l, out.witness, x_r), out.x1_visited, out.x1_with_roots,
                            out.leaves, out.leaves_consistent, out.solutions, out.xtimes, out.row_xors, 2 * out.row_xors,
                            times[times.len() / 2], times[0]
                        );
                    }
                    "lin21" => {
                        let r = sat_arm("lin21", n, l, irr, x_r, 1, u64::MAX);
                        println!("{{{},\"status\":\"{}\",\"model_xs\":{},{},{}}}", head("lin21"), r.status, xs_json(r.xs), chk_json(&cv, l, r.xs, x_r), r.json);
                    }
                    "fast" => {
                        let r = run_fast(n, l, irr, x_r, 1);
                        let name = if fast_rep == 0 { "lin21-fast" } else { "lin21-fast-b" };
                        fast_rep += 1;
                        println!("{{{},\"status\":\"{}\",\"model_xs\":{},{},{}}}", head(name), r.status, xs_json(r.xs), chk_json(&cv, l, r.xs, x_r), r.json);
                    }
                    "prof" => {
                        let r = run_prof(n, l, irr, x_r, 1);
                        println!("{{{},\"status\":\"{}\",\"model_xs\":{},{},{}}}", head("lin21-prof"), r.status, xs_json(r.xs), chk_json(&cv, l, r.xs, x_r), r.json);
                    }
                    _ => panic!("unknown arm {arm}"),
                }
            }
        }
        _ => {
            eprintln!("usage: l21fast selftest | profile --l L --xr X | run --l L --xr X [--arms scan,fast,lin21,fast --prof 1 --n N --reps K]");
            std::process::exit(2);
        }
    }
}

fn arg(v: &[String], name: &str) -> Option<String> {
    v.iter().position(|a| a == name).map(|i| v[i + 1].clone())
}
