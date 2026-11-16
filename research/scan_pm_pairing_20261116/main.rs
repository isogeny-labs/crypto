//! `scan_pm`: ±P pairing in the linear 2 + 1 scan, against the repository's
//! `lin21-ht` arm (`research/scan_pm_pairing_20261116/PROTOCOL.md`).
//!
//! ```text
//! scan_pm run --cells a:n:d,… --seed S --out DIR [--no 64 --planted 8 --yes 8
//!             --reps 3 --shard i/m --exh-max-d 11 --set-max-d 16]
//! scan_pm summarize --dir DIR --out DIR
//! ```
//!
//! Arms, all in one process, interleaved per target:
//! `A1`, `A2` — `linear21.rs`, byte-identical to the repository's file,
//! `Variant::HalfTrace` (the baseline and its A/A control);
//! `BC` — the same t stage in this study's copy of the leaf;
//! `PM` — ±P pairing, one shared inversion per pair;
//! `LZ` — the library's `add_many_lazy`; `LP` — the library's existing
//! pairing `add_many_lazy_soa_paired`.

#[path = "linear21.rs"]
mod linear21;
#[path = "linear21_pm.rs"]
mod pm;

use crypto_lib::cryptanalysis::ic_boundary::koblitz_instance;
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_standard_subspace_factor_base, KoblitzCurve,
};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::hint::black_box;
use std::io::Write as _;
use std::time::Instant;

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn num<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    flag(args, name)
        .map(|v| v.parse().unwrap_or_else(|_| panic!("{name}: bad number {v}")))
        .unwrap_or(default)
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: u64) -> u64 {
        self.next() % m
    }
}

const ARMS: [&str; 6] = ["A1", "PM", "BC", "LP", "A2", "LZ"];
const STAGES: [&str; 5] = ["t", "build", "solve", "split", "member"];
const OPS: [&str; 5] = ["mul", "sqr", "inv", "tlook", "wop"];

fn header() -> Vec<String> {
    let mut h: Vec<String> = [
        "a", "n", "d", "shard", "tid", "class", "k", "pi", "pj", "pk", "verdict", "leaves",
        "consistent", "splits", "verify_adds", "pm_fallback", "lp_kernel", "lp_declined",
        "ns_A1", "ns_A2", "ns_BC", "ns_PM", "ns_LZ", "ns_LP", "ts_BC", "ts_PM", "ts_LZ", "ts_LP",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    for arm in ["bc", "pm"] {
        for s in STAGES {
            for o in OPS {
                h.push(format!("{arm}_{s}_{o}"));
            }
        }
    }
    for arm in ["lz", "lp"] {
        for o in OPS {
            h.push(format!("{arm}_t_{o}"));
        }
    }
    for g in [
        "g_first", "g_set_pm", "g_set_lz", "g_set_lp", "g_exh", "g_wit", "g_repo", "g_leaf", "nset",
    ] {
        h.push(g.to_string());
    }
    h
}

fn cmd_run(args: &[String]) {
    let cells: Vec<(u8, u32, u32)> = flag(args, "--cells")
        .expect("--cells a:n:d,…")
        .split(',')
        .map(|s| {
            let p: Vec<&str> = s.split(':').collect();
            (p[0].parse().unwrap(), p[1].parse().unwrap(), p[2].parse().unwrap())
        })
        .collect();
    let seed: u64 = num(args, "--seed", 0);
    assert!(seed != 0, "--seed is required");
    let want_no: usize = num(args, "--no", 64);
    let want_planted: usize = num(args, "--planted", 8);
    let want_yes: usize = num(args, "--yes", 8);
    let reps: usize = num(args, "--reps", 3);
    let exh_max_d: u32 = num(args, "--exh-max-d", 11);
    let set_max_d: u32 = num(args, "--set-max-d", 16);
    let (shard, shards): (u64, u64) = flag(args, "--shard")
        .map(|s| {
            let (i, m) = s.split_once('/').unwrap();
            (i.parse().unwrap(), m.parse().unwrap())
        })
        .unwrap_or((0, 1));
    let out = std::path::PathBuf::from(flag(args, "--out").expect("--out DIR"));
    std::fs::create_dir_all(&out).unwrap();
    let per = |w: usize| w / shards as usize;
    for &(a, n, d) in &cells {
        let t0 = Instant::now();
        let kc = KoblitzCurve::new(a, n).expect("Koblitz curve");
        let fb = build_standard_subspace_factor_base(&kc, d).expect("standard subspace base");
        let inst = koblitz_instance(a, n).expect("instance");
        assert!(
            inst.irreducible.degree == kc.curve.irreducible.degree
                && inst.irreducible.low_terms == kc.curve.irreducible.low_terms
        );
        let base: Vec<FastPoint> = fb.points.iter().map(|p| inst.fast.lift(p)).collect();
        drop(fb);
        let repo =
            linear21::Linear21::new(&inst.gf, &inst.fast, n, d as usize, inst.b, base.clone());
        let lin = pm::Lin::new(&inst.gf, &inst.fast, n, d as usize, inst.b, base);
        let setup_s = t0.elapsed().as_secs_f64();
        let pts = lin.points.len();
        let path = out.join(format!("rows-n{n}-d{d}-s{shard}of{shards}.tsv"));
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(
            f,
            "# a={a} n={n} d={d} seed={seed} shard={shard}/{shards} reps={reps} fb_points={pts} pairs={} singles={} folds={} setup_s={setup_s:.2} r={}",
            lin.pairs, lin.singles, inst.gf.folds(), inst.r
        )
        .unwrap();
        writeln!(f, "{}", header().join("\t")).unwrap();
        eprintln!(
            "n={n} d={d}: |F|={pts} pairs={} singles={} setup {setup_s:.1}s",
            lin.pairs, lin.singles
        );
        let lz_model = lin.model_t(false);
        let lp_model = lin.model_t(true);
        let cseed = seed ^ (n as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f) ^ (d as u64) << 48;
        let mut rng = Rng(cseed);
        let mut prng = Rng(cseed ^ 0x706c_616e_7465_6421 ^ shard);
        let (mut n_no, mut n_yes, mut n_pl) = (0usize, 0usize, 0usize);
        let mut cand = 0u64;
        let mut tid = 0u64;
        let mut all_a: Vec<[u32; 3]> = Vec::new();
        let mut all_b: Vec<[u32; 3]> = Vec::new();
        let mut sink = 0u64;
        while n_no < per(want_no) || n_pl < per(want_planted) {
            // Planted targets are interleaved with the seeded ones.
            let planted = n_pl < per(want_planted) && (n_no >= per(want_no) || tid % 8 == 7);
            let (r, k, tri) = if planted {
                let pick = |g: &mut Rng| loop {
                    let i = g.below(pts as u64) as usize;
                    if lin.points[i].x != 0 {
                        return i;
                    }
                };
                let (i, j, l) = (pick(&mut prng), pick(&mut prng), pick(&mut prng));
                let s = inst.fast.add(inst.fast.add(lin.points[i], lin.points[j]), lin.points[l]);
                if s.infinity {
                    continue;
                }
                (s, 0u64, [i as i64, j as i64, l as i64])
            } else {
                let k = 1 + rng.below(inst.r - 1);
                cand += 1;
                if (cand - 1) % shards != shard {
                    continue;
                }
                (inst.fast.mul_u64(inst.generator, k), k, [-1i64; 3])
            };
            assert!(inst.fast.is_on_curve(r) && !r.infinity);
            // Untimed pre-pass: the class of the target.
            let mut c0 = linear21::Counts::default();
            let w0 = repo
                .decide(r, linear21::Variant::HalfTrace, &mut c0, || false)
                .unwrap();
            assert!(!c0.truncated, "kernel cap exceeded");
            let class = if planted {
                assert!(w0.is_some(), "a planted target was not decomposed");
                n_pl += 1;
                "planted"
            } else if w0.is_some() {
                if n_yes >= per(want_yes) {
                    continue;
                }
                n_yes += 1;
                "yes"
            } else {
                if n_no >= per(want_no) {
                    continue;
                }
                n_no += 1;
                "no"
            };
            // Timed decision mode, arms rotated per target and repetition.
            let mut best = [u64::MAX; 6];
            let mut tbest = [u64::MAX; 4];
            let mut wit: [Option<[usize; 3]>; 6] = [None; 6];
            let mut repo_c = linear21::Counts::default();
            let mut bc_c = pm::Counts::default();
            let mut pm_c = pm::Counts::default();
            let mut lp_o = pm::Ops::default();
            for rep in 0..reps {
                for slot in 0..6 {
                    let arm = (slot + tid as usize + 2 * rep) % 6;
                    let mut o = pm::Ops::default();
                    let mut c = pm::Counts::default();
                    let t = Instant::now();
                    let w = match ARMS[arm] {
                        "A1" | "A2" => {
                            let mut rc = linear21::Counts::default();
                            let w = repo
                                .decide(black_box(r), linear21::Variant::HalfTrace, &mut rc, || false)
                                .unwrap();
                            repo_c = rc;
                            w
                        }
                        "BC" => {
                            let w = lin.decide_base::<false, false, false>(
                                black_box(r), &mut c, &mut o, &mut all_a, &mut sink,
                            );
                            bc_c = c;
                            w
                        }
                        "PM" => {
                            let w = lin.decide_pm::<false, false, false>(
                                black_box(r), &mut c, &mut o, &mut all_a, &mut sink,
                            );
                            pm_c = c;
                            w
                        }
                        "LZ" => lin.decide_lz::<false, false>(black_box(r), &mut c, &mut all_a, &mut sink),
                        _ => {
                            let w = lin.decide_lp::<false, false>(
                                black_box(r), &mut c, &mut o, &mut all_a, &mut sink,
                            );
                            lp_o = o;
                            w
                        }
                    };
                    let ns = t.elapsed().as_nanos() as u64;
                    best[arm] = best[arm].min(ns);
                    wit[arm] = black_box(w);
                }
                // t stage alone, after the rotation.
                for s in 0..4 {
                    let which = (s + tid as usize + rep) % 4;
                    let mut o = pm::Ops::default();
                    let mut c = pm::Counts::default();
                    let t = Instant::now();
                    match which {
                        0 => drop(lin.decide_base::<false, false, true>(black_box(r), &mut c, &mut o, &mut all_a, &mut sink)),
                        1 => drop(lin.decide_pm::<false, false, true>(black_box(r), &mut c, &mut o, &mut all_a, &mut sink)),
                        2 => drop(lin.decide_lz::<false, true>(black_box(r), &mut c, &mut all_a, &mut sink)),
                        _ => drop(lin.decide_lp::<false, true>(black_box(r), &mut c, &mut o, &mut all_a, &mut sink)),
                    }
                    tbest[which] = tbest[which].min(t.elapsed().as_nanos() as u64);
                }
            }
            black_box(sink);
            // Gates.
            let g_first = wit.iter().all(|w| *w == wit[0]) && wit[0] == w0;
            let g_wit = match wit[0] {
                Some([i, j, l]) => {
                    inst.fast.add(inst.fast.add(lin.points[i], lin.points[j]), lin.points[l]) == r
                }
                None => true,
            };
            let g_repo = repo_c.trials == bc_c.trials
                && repo_c.consistent == bc_c.consistent
                && repo_c.splits == bc_c.splits
                && repo_c.gauss_xors == bc_c.gauss_xors
                && repo_c.field_muls == bc_c.field_muls
                && repo_c.field_invs == bc_c.field_invs
                && repo_c.verify_adds == bc_c.verify_adds;
            let g_leaf = bc_c.trials == pm_c.trials
                && bc_c.consistent == pm_c.consistent
                && bc_c.splits == pm_c.splits
                && bc_c.gauss_xors == pm_c.gauss_xors
                && bc_c.verify_adds == pm_c.verify_adds;
            // Counted, all-witness passes (untimed).
            let (mut obc, mut opm) = (pm::Ops::default(), pm::Ops::default());
            let (mut cb, mut cp) = (pm::Counts::default(), pm::Counts::default());
            all_a.clear();
            lin.decide_base::<true, true, false>(r, &mut cb, &mut obc, &mut all_a, &mut sink);
            all_a.sort_unstable();
            all_a.dedup();
            all_b.clear();
            lin.decide_pm::<true, true, false>(r, &mut cp, &mut opm, &mut all_b, &mut sink);
            all_b.sort_unstable();
            all_b.dedup();
            assert!(!cb.truncated && !cp.truncated);
            let g_set_pm = all_a == all_b && (all_a.is_empty() == wit[0].is_none());
            let (mut g_set_lz, mut g_set_lp, mut g_exh) = (-1i32, -1i32, -1i32);
            if d <= set_max_d {
                let mut c = pm::Counts::default();
                let mut o = pm::Ops::default();
                all_b.clear();
                lin.decide_lz::<true, false>(r, &mut c, &mut all_b, &mut sink);
                all_b.sort_unstable();
                all_b.dedup();
                g_set_lz = i32::from(all_a == all_b);
                all_b.clear();
                lin.decide_lp::<true, false>(r, &mut c, &mut o, &mut all_b, &mut sink);
                all_b.sort_unstable();
                all_b.dedup();
                g_set_lp = i32::from(all_a == all_b);
            }
            if d <= exh_max_d {
                g_exh = i32::from(lin.exhaustive(r) == all_a);
            }
            let mut row = String::new();
            write!(
                row,
                "{a}\t{n}\t{d}\t{shard}\t{tid}\t{class}\t{k}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                tri[0], tri[1], tri[2], u8::from(wit[0].is_some()), cb.trials, cb.consistent,
                cb.splits, cb.verify_adds, opm.fallback_leaves, lp_o.lp_kernel_blocks,
                lp_o.lp_declined_blocks
            )
            .unwrap();
            // ns columns in header order A1 A2 BC PM LZ LP.
            for name in ["A1", "A2", "BC", "PM", "LZ", "LP"] {
                let idx = ARMS.iter().position(|x| *x == name).unwrap();
                write!(row, "\t{}", best[idx]).unwrap();
            }
            for t in tbest {
                write!(row, "\t{t}").unwrap();
            }
            for o in [&obc, &opm] {
                for s in 0..5 {
                    for k in 0..5 {
                        write!(row, "\t{}", o.s[s][k]).unwrap();
                    }
                }
            }
            for m in [&lz_model, &lp_model] {
                for v in m {
                    write!(row, "\t{v}").unwrap();
                }
            }
            write!(
                row,
                "\t{}\t{}\t{g_set_lz}\t{g_set_lp}\t{g_exh}\t{}\t{}\t{}\t{}",
                u8::from(g_first), u8::from(g_set_pm), u8::from(g_wit), u8::from(g_repo),
                u8::from(g_leaf), all_a.len()
            )
            .unwrap();
            writeln!(f, "{row}").unwrap();
            f.flush().unwrap();
            tid += 1;
        }
        eprintln!(
            "n={n} d={d}: {n_no} no, {n_yes} yes, {n_pl} planted in {:.1}s",
            t0.elapsed().as_secs_f64()
        );
    }
}

// ── summarize ─────────────────────────────────────────────────────────

struct Row {
    v: Vec<String>,
}

struct Table {
    col: HashMap<String, usize>,
    rows: Vec<Row>,
    meta: Vec<String>,
}

impl Table {
    fn f(&self, r: &Row, name: &str) -> f64 {
        r.v[self.col[name]].parse().unwrap()
    }
    fn s<'a>(&self, r: &'a Row, name: &str) -> &'a str {
        &r.v[self.col[name]]
    }
}

fn ratio_ci(x: &[f64], y: &[f64], rng: &mut Rng, boot: usize) -> (f64, f64, f64) {
    let point = x.iter().sum::<f64>() / y.iter().sum::<f64>();
    let n = x.len();
    let mut reps: Vec<f64> = (0..boot)
        .map(|_| {
            let (mut sx, mut sy) = (0.0, 0.0);
            for _ in 0..n {
                let i = rng.below(n as u64) as usize;
                sx += x[i];
                sy += y[i];
            }
            sx / sy
        })
        .collect();
    reps.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| reps[((boot - 1) as f64 * p).round() as usize];
    (point, q(0.025), q(0.975))
}

fn cmd_summarize(args: &[String]) {
    let dir = std::path::PathBuf::from(flag(args, "--dir").expect("--dir DIR"));
    let out = std::path::PathBuf::from(flag(args, "--out").expect("--out DIR"));
    let boot: usize = num(args, "--boot", 2000);
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            let n = p.file_name().unwrap().to_string_lossy().to_string();
            n.starts_with("rows-") && n.ends_with(".tsv")
        })
        .collect();
    files.sort();
    let mut cells: std::collections::BTreeMap<(u32, u32), Table> = Default::default();
    for p in &files {
        let txt = std::fs::read_to_string(p).unwrap();
        let mut lines = txt.lines();
        let meta = lines.next().unwrap().to_string();
        let head: Vec<&str> = lines.next().unwrap().split('\t').collect();
        for l in lines {
            let v: Vec<String> = l.split('\t').map(String::from).collect();
            assert_eq!(v.len(), head.len(), "short row in {}", p.display());
            let n: u32 = v[1].parse().unwrap();
            let d: u32 = v[2].parse().unwrap();
            let t = cells.entry((n, d)).or_insert_with(|| Table {
                col: head.iter().enumerate().map(|(i, h)| (h.to_string(), i)).collect(),
                rows: Vec::new(),
                meta: Vec::new(),
            });
            t.rows.push(Row { v });
            if !t.meta.contains(&meta) {
                t.meta.push(meta.clone());
            }
        }
    }
    let mut rng = Rng(0x7363_616e_5f70_6d21);
    let mut md = String::new();
    let mut js = String::from("{\n  \"schema\": \"scan_pm_pairing/v1\",\n  \"tag\": \"pcc:unverified\",\n  \"cells\": [\n");
    let mut gates = String::from("| n | d | \\|F\\| pairs+singles | targets (no / yes / planted) | first witness equal, 6 arms | set BC = PM | set BC = LZ | set BC = LP | set = exhaustive | witness adds up | BC counters = repository's | PM leaf counters = BC's | PM fallback leaves | LP kernel / declined blocks |\n|--:|--:|--|--|--|--|--|--|--|--|--|--|--:|--|\n");
    let mut stage = String::from("| n | d | leaves/target | consistent | A1 ns/leaf | field ops/leaf (M+S+I) | t stage share, field ops | t stage share, with elimination priced | t stage share, wall (ts_BC / A1) | Amdahl ceiling, field ops | Amdahl ceiling, priced | Amdahl ceiling, wall |\n|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n");
    let mut counted = String::from("| n | arm | mul | sqr | inv | table lookups | word ops | field ops (M+S+(n+8)I) | BC/arm, field ops | priced total (8M+8S+8(n+8)I+T+W) | BC/arm, priced | BC/arm, t stage field ops |\n|--:|--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n");
    let mut wall = String::from("| n | A/A: A1/A2 [95%] | A1/PM [95%] | A1/BC [95%] | BC/PM [95%] | A1/LZ [95%] | A1/LP [95%] | PM/LP [95%] |\n|--:|--|--|--|--|--|--|--|\n");
    let mut twall = String::from("| n | t stage ns/leaf: BC | PM | LZ | LP | BC/PM [95%] | BC/LZ [95%] | BC/LP [95%] |\n|--:|--:|--:|--:|--:|--|--|--|\n");
    // (n, field ratio, priced ratio, wall outside band) for PM and LP.
    let mut dec: Vec<(u32, [f64; 2], [f64; 2], [bool; 2], [f64; 2])> = Vec::new();
    let ncell = cells.len();
    for (ci, ((n, d), t)) in cells.iter().enumerate() {
        let (n, d) = (*n, *d);
        let no: Vec<&Row> = t.rows.iter().filter(|r| t.s(r, "class") == "no").collect();
        let cnt = |c: &str| t.rows.iter().filter(|r| t.s(r, "class") == c).count();
        let allg = |g: &str| {
            let (mut ok, mut na) = (0, 0);
            for r in &t.rows {
                match t.s(r, g) {
                    "1" => ok += 1,
                    "-1" => na += 1,
                    _ => {}
                }
            }
            if na == t.rows.len() {
                "not run".to_string()
            } else {
                format!("{ok}/{}", t.rows.len() - na)
            }
        };
        let sum_all = |name: &str| t.rows.iter().map(|r| t.f(r, name)).sum::<f64>();
        let fbmeta: String = t.meta[0]
            .split(' ')
            .filter(|w| w.starts_with("fb_points=") || w.starts_with("pairs=") || w.starts_with("singles="))
            .map(|w| w.split('=').nth(1).unwrap())
            .collect::<Vec<_>>()
            .join(" / ");
        writeln!(
            gates,
            "| {n} | {d} | {fbmeta} | {} / {} / {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} / {} |",
            no.len(), cnt("yes"), cnt("planted"), allg("g_first"), allg("g_set_pm"),
            allg("g_set_lz"), allg("g_set_lp"), allg("g_exh"), allg("g_wit"), allg("g_repo"),
            allg("g_leaf"), sum_all("pm_fallback"), sum_all("lp_kernel"), sum_all("lp_declined")
        )
        .unwrap();
        let m = no.len() as f64;
        let mean = |name: &str| no.iter().map(|r| t.f(r, name)).sum::<f64>() / m;
        let col = |name: &str| -> Vec<f64> { no.iter().map(|r| t.f(r, name)).collect() };
        let inv_price = (n + 8) as f64;
        // ops[arm][stage][op], means over the no class.
        let ops = |arm: &str| -> [[f64; 5]; 5] {
            let mut o = [[0.0; 5]; 5];
            for (s, sn) in STAGES.iter().enumerate() {
                for (k, on) in OPS.iter().enumerate() {
                    o[s][k] = mean(&format!("{arm}_{sn}_{on}"));
                }
            }
            o
        };
        let bc = ops("bc");
        let pmo = ops("pm");
        let with_t = |model: &str| -> [[f64; 5]; 5] {
            let mut o = bc;
            for (k, on) in OPS.iter().enumerate() {
                o[0][k] = mean(&format!("{model}_t_{on}"));
            }
            o
        };
        let lz = with_t("lz");
        let lp = with_t("lp");
        let field = |o: &[f64; 5]| o[0] + o[1] + inv_price * o[2];
        let priced = |o: &[f64; 5]| 8.0 * field(o) + o[3] + o[4];
        let tot = |o: &[[f64; 5]; 5]| -> [f64; 5] {
            let mut s = [0.0; 5];
            for st in o {
                for k in 0..5 {
                    s[k] += st[k];
                }
            }
            s
        };
        let leaves = mean("leaves");
        let a1 = col("ns_A1");
        let t_field = field(&bc[0]) / field(&tot(&bc));
        let t_priced = priced(&bc[0]) / priced(&tot(&bc));
        let t_wall = mean("ts_BC") / mean("ns_A1");
        writeln!(
            stage,
            "| {n} | {d} | {leaves:.0} | {:.1}% | {:.0} | {:.2} | {:.1}% | {:.1}% | {:.1}% | {:.3} | {:.3} | {:.3} |",
            100.0 * mean("consistent") / leaves, mean("ns_A1") / leaves,
            field(&tot(&bc)) / leaves, 100.0 * t_field, 100.0 * t_priced, 100.0 * t_wall,
            1.0 / (1.0 - t_field), 1.0 / (1.0 - t_priced), 1.0 / (1.0 - t_wall)
        )
        .unwrap();
        let mut jarms = String::new();
        let mut fr = [0.0; 2];
        let mut pr = [0.0; 2];
        for (name, o, model) in [
            ("BC (= lin21-ht)", &bc, false),
            ("PM", &pmo, false),
            ("LZ (model)", &lz, true),
            ("LP (model)", &lp, true),
        ] {
            let s = tot(o);
            let (rf, rp, rt) = (
                field(&tot(&bc)) / field(&s),
                priced(&tot(&bc)) / priced(&s),
                field(&bc[0]) / field(&o[0]),
            );
            if name == "PM" {
                fr[0] = rf;
                pr[0] = rp;
            }
            if name.starts_with("LP") {
                fr[1] = rf;
                pr[1] = rp;
            }
            writeln!(
                counted,
                "| {n} | {name} | {:.0} | {:.0} | {:.1} | {:.0} | {:.0} | {:.0} | {rf:.4} | {:.0} | {rp:.4} | {rt:.4} |",
                s[0], s[1], s[2], s[3], s[4], field(&s), priced(&s)
            )
            .unwrap();
            write!(
                jarms,
                "{}\"{}\": {{\"modelled_t_stage\": {model}, \"mul\": {:.3}, \"sqr\": {:.3}, \"inv\": {:.4}, \"tlook\": {:.3}, \"wop\": {:.3}, \"field_ops\": {:.3}, \"priced\": {:.3}, \"t_stage_field_ops\": {:.3}, \"bc_over_arm_field\": {rf:.5}, \"bc_over_arm_priced\": {rp:.5}, \"bc_over_arm_t_stage_field\": {rt:.5}}}",
                if jarms.is_empty() { "" } else { ", " },
                name.split(' ').next().unwrap(), s[0], s[1], s[2], s[3], s[4], field(&s),
                priced(&s), field(&o[0])
            )
            .unwrap();
        }
        let rci = |x: &[f64], y: &[f64], rng: &mut Rng| ratio_ci(x, y, rng, boot);
        let fm = |r: (f64, f64, f64)| format!("{:.3} [{:.3}, {:.3}]", r.0, r.1, r.2);
        let (a2, bcw, pmw, lzw, lpw) =
            (col("ns_A2"), col("ns_BC"), col("ns_PM"), col("ns_LZ"), col("ns_LP"));
        let aa = rci(&a1, &a2, &mut rng);
        let r_pm = rci(&a1, &pmw, &mut rng);
        let r_bc = rci(&a1, &bcw, &mut rng);
        let r_bp = rci(&bcw, &pmw, &mut rng);
        let r_lz = rci(&a1, &lzw, &mut rng);
        let r_lp = rci(&a1, &lpw, &mut rng);
        let r_pl = rci(&pmw, &lpw, &mut rng);
        writeln!(
            wall,
            "| {n} | {} | {} | {} | {} | {} | {} | {} |",
            fm(aa), fm(r_pm), fm(r_bc), fm(r_bp), fm(r_lz), fm(r_lp), fm(r_pl)
        )
        .unwrap();
        let (tb, tp, tz, tl) = (col("ts_BC"), col("ts_PM"), col("ts_LZ"), col("ts_LP"));
        let (q_pm, q_lz, q_lp) =
            (rci(&tb, &tp, &mut rng), rci(&tb, &tz, &mut rng), rci(&tb, &tl, &mut rng));
        writeln!(
            twall,
            "| {n} | {:.1} | {:.1} | {:.1} | {:.1} | {} | {} | {} |",
            mean("ts_BC") / leaves, mean("ts_PM") / leaves, mean("ts_LZ") / leaves,
            mean("ts_LP") / leaves, fm(q_pm), fm(q_lz), fm(q_lp)
        )
        .unwrap();
        // The A/A band, widened symmetrically about 1.
        let band_hi = aa.2.max(1.0 / aa.1);
        let band_lo = aa.1.min(1.0 / aa.2);
        dec.push((n, fr, pr, [r_pm.1 > band_hi, r_lp.1 > band_hi], [r_pm.0, r_lp.0]));
        let j3 = |r: (f64, f64, f64)| format!("[{:.5}, {:.5}, {:.5}]", r.0, r.1, r.2);
        writeln!(
            js,
            "    {{\"n\": {n}, \"d\": {d}, \"meta\": {:?}, \"targets_no\": {}, \"targets_yes\": {}, \"targets_planted\": {}, \"leaves_per_target\": {leaves:.2}, \"a1_ns_per_leaf\": {:.2}, \"t_stage_share\": {{\"field_ops\": {t_field:.5}, \"priced\": {t_priced:.5}, \"wall\": {t_wall:.5}}}, \"counted_per_no_target\": {{{jarms}}}, \"wall_ratio_point_lo_hi\": {{\"aa_a1_over_a2\": {}, \"aa_band_symmetric\": [{band_lo:.5}, {band_hi:.5}], \"a1_over_pm\": {}, \"a1_over_bc\": {}, \"bc_over_pm\": {}, \"a1_over_lz\": {}, \"a1_over_lp\": {}, \"pm_over_lp\": {}}}, \"t_stage_wall_ratio\": {{\"bc_over_pm\": {}, \"bc_over_lz\": {}, \"bc_over_lp\": {}}}, \"gates\": {{\"first\": \"{}\", \"set_pm\": \"{}\", \"set_lz\": \"{}\", \"set_lp\": \"{}\", \"exhaustive\": \"{}\", \"witness\": \"{}\", \"bc_equals_repo_counters\": \"{}\", \"pm_leaf_counters\": \"{}\"}}}}{}",
            t.meta, no.len(), cnt("yes"), cnt("planted"), mean("ns_A1") / leaves, j3(aa),
            j3(r_pm), j3(r_bc), j3(r_bp), j3(r_lz), j3(r_lp), j3(r_pl), j3(q_pm), j3(q_lz),
            j3(q_lp), allg("g_first"), allg("g_set_pm"), allg("g_set_lz"), allg("g_set_lp"),
            allg("g_exh"), allg("g_wit"), allg("g_repo"), allg("g_leaf"),
            if ci + 1 < ncell { "," } else { "" }
        )
        .unwrap();
    }
    // Decision under PROTOCOL.md §6, for PM (primary) and LP (secondary),
    // on field operations (primary metric) and on the priced total.
    let top: Vec<_> = dec.iter().rev().take(3).collect();
    let mut dmd = String::new();
    let mut djs = String::new();
    for (ai, arm) in ["PM", "LP"].iter().enumerate() {
        for (mi, metric) in ["field operations", "priced total"].iter().enumerate() {
            let cr: Vec<f64> = top.iter().map(|t| if mi == 0 { t.1[ai] } else { t.2[ai] }).collect();
            let outside: Vec<bool> = top.iter().map(|t| t.3[ai]).collect();
            let verdict = if top.len() < 3 {
                "not evaluated (fewer than three cells)"
            } else if cr.iter().all(|&c| c >= 1.05) && outside.iter().all(|&b| b) {
                "holds"
            } else if cr[0] < 1.02 || outside.iter().all(|&b| !b) {
                "fails"
            } else {
                "inconclusive"
            };
            let ns: Vec<String> = top.iter().map(|t| t.0.to_string()).collect();
            writeln!(
                dmd,
                "- **{arm}, counted metric = {metric}: {verdict}.** n = {} (largest first): counted BC/{arm} = {}; wall A1/{arm} = {}; outside the A/A band: {}. [pcc:unverified]",
                ns.join(", "),
                cr.iter().map(|c| format!("{c:.4}")).collect::<Vec<_>>().join(", "),
                top.iter().map(|t| format!("{:.3}", t.4[ai])).collect::<Vec<_>>().join(", "),
                outside.iter().map(|b| if *b { "yes" } else { "no" }).collect::<Vec<_>>().join(", ")
            )
            .unwrap();
            write!(
                djs,
                "{}{{\"arm\": \"{arm}\", \"metric\": \"{metric}\", \"verdict\": \"{verdict}\"}}",
                if djs.is_empty() { "" } else { ", " }
            )
            .unwrap();
        }
    }
    writeln!(js, "  ],\n  \"decisions\": [{djs}]\n}}").unwrap();
    writeln!(md, "# ±P pairing in the linear 2 + 1 scan: generated tables\n\nGenerated by `scan_pm summarize` from the `rows-*.tsv` of one run directory. Every number is `[pcc:unverified]`: a local pilot on a shared container. Counted and wall statistics are over the non-decomposable (\"no\") class; `BC` is this study's copy of the repository arm (its repository-level counters are checked equal to `lin21-ht`'s on every target), `A1`/`A2` the repository file itself.\n").unwrap();
    writeln!(md, "## 1. Gates\n\n{gates}").unwrap();
    writeln!(md, "## 2. Stage breakdown of the baseline (no class)\n\n{stage}").unwrap();
    writeln!(md, "## 3. Counted operations per target (no class, means)\n\nLZ and LP: leaf stages as BC, t stage from a model of the library kernels (not instrumented).\n\n{counted}").unwrap();
    writeln!(md, "## 4. Whole-leaf wall ratios (sum over the no class of the per-target minimum over repetitions; paired bootstrap)\n\n{wall}").unwrap();
    writeln!(md, "## 5. t stage alone, wall\n\n{twall}").unwrap();
    writeln!(md, "## 6. Decision under the pre-registered rule\n\n{dmd}").unwrap();
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("tables.md"), &md).unwrap();
    std::fs::write(out.join("results.json"), &js).unwrap();
    print!("{md}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("run") => cmd_run(&args),
        Some("summarize") => cmd_summarize(&args),
        _ => eprintln!("usage: scan_pm run|summarize …"),
    }
}
