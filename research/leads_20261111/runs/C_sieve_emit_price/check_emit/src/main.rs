//! Independent validator check for `research/jv_sieve_negrho_20261102/`
//! (plain JV sieve route vs a matched negation-map rho at p = 1511).
//! Written from the frozen PROTOCOL.md and the record schemas; shares no code
//! with the producer's `fit/` (own RNG, permutation tests, bootstrap, quantile).
//!
//! Usage: negrho_validator_check <repo_root> <replay_dir> <out_json>

use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const B: usize = 20000;
const GAE: f64 = 331.0;

fn load(path: &str) -> Vec<Value> {
    let t = fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    match serde_json::from_str::<Value>(&t).unwrap_or_else(|e| panic!("{path}: {e}")) {
        Value::Array(v) => v,
        v => vec![v],
    }
}
fn u(v: &Value, k: &str) -> u64 {
    v.get(k).and_then(|x| x.as_u64()).unwrap_or_else(|| panic!("missing u64 {k}"))
}
fn fl(v: &Value, k: &str) -> f64 {
    v.get(k).and_then(|x| x.as_f64()).unwrap_or_else(|| panic!("missing f64 {k}"))
}
fn bo(v: &Value, k: &str) -> bool {
    v.get(k).and_then(|x| x.as_bool()).unwrap_or_else(|| panic!("missing bool {k}"))
}
fn rel(a: f64, b: f64) -> f64 {
    ((a - b) / b).abs()
}
fn diff_keys(a: &Value, b: &Value, skip_wall: bool) -> Vec<String> {
    let (ma, mb) = (a.as_object().unwrap(), b.as_object().unwrap());
    let mut out = Vec::new();
    for (k, va) in ma {
        if skip_wall && k == "wall_ms" {
            continue;
        }
        if mb.get(k) != Some(va) {
            out.push(k.clone());
        }
    }
    for k in mb.keys() {
        if !(skip_wall && k == "wall_ms") && !ma.contains_key(k) {
            out.push(format!("{k}(only right)"));
        }
    }
    out
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        ((self.next() as u128 * n as u128) >> 64) as usize
    }
    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
}
fn quant(s: &[f64], q: f64) -> f64 {
    let h = (s.len() - 1) as f64 * q;
    let lo = h.floor() as usize;
    let hi = (lo + 1).min(s.len() - 1);
    s[lo] + (h - lo as f64) * (s[hi] - s[lo])
}
fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}
fn sd(x: &[f64]) -> f64 {
    let m = mean(x);
    (x.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (x.len() - 1) as f64).sqrt()
}
fn geo(x: &[f64]) -> f64 {
    (x.iter().map(|v| v.ln()).sum::<f64>() / x.len() as f64).exp()
}

#[derive(Clone)]
struct Rho {
    p: u64,
    seed: u64,
    sh: f64,
    st: f64,
    sph: f64,
    spt: f64,
    steps: f64,
    nat_b: f64, // six native counters / sqrt l
    nat_a: f64, // six + fp_adds / sqrt l
}
fn rho_of(r: &Value, bad: &mut Vec<String>, tag: &str) -> Rho {
    let o = &r["ops"];
    let sl = (u(r, "l") as f64).sqrt();
    let c = fl(r, "c_add_e");
    let six = ["canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp"].iter().map(|k| u(o, k)).sum::<u64>();
    let fpa = u(o, "fp_adds");
    let mt = 24 * u(o, "mul") + 18 * u(o, "sq") + 62 * u(o, "inv");
    let mh = 55 * u(o, "mul") + 55 * u(o, "sq") + 166 * u(o, "inv");
    let ok = bo(r, "correct") && !bo(r, "capped") && bo(r, "negation") && u(r, "batch") == 16 && u(r, "r") == 256
        && u(r, "dp_bits") == 16 && c == GAE
        && fl(r, "muls_tuned") == mt as f64 && fl(r, "muls_harness") == mh as f64
        && rel(fl(r, "muls_harness") / (c * sl), fl(r, "s_harness")) < 1e-12
        && rel(fl(r, "muls_tuned") / (c * sl), fl(r, "s_tuned")) < 1e-12
        && rel((fl(r, "muls_harness") + (fpa + six) as f64) / (c * sl), fl(r, "s_plus_harness")) < 1e-12
        && rel((fl(r, "muls_tuned") + (fpa + six) as f64) / (c * sl), fl(r, "s_plus_tuned")) < 1e-12
        && u(r, "steps") == u(o, "walk_adds");
    if !ok {
        bad.push(tag.to_string());
    }
    Rho {
        p: u(r, "p"),
        seed: u(r, "seed"),
        sh: fl(r, "s_harness"),
        st: fl(r, "s_tuned"),
        sph: fl(r, "s_plus_harness"),
        spt: fl(r, "s_plus_tuned"),
        steps: u(r, "steps") as f64 / sl,
        nat_b: six as f64 / sl,
        nat_a: (six + fpa) as f64 / sl,
    }
}
#[derive(Clone, Copy)]
struct Cov {
    s: f64,
    sp: f64,
    nat_a: f64,
    nat_b: f64,
    base_a: f64,
    unem: f64,
}
fn cov_of(r: &Value) -> Cov {
    let sl = (u(r, "l") as f64).sqrt();
    // LEAD-C-jv-sieve-emit-price: with EMIT_A_JSON=<file> ({"<seed>": A_emit}),
    // the layer-A proxy (unemitted multiplications) is replaced by the emitted
    // additions of the same scope; unset, the validator's computation unchanged.
    let unem = match std::env::var("EMIT_A_JSON") {
        Ok(f) => {
            let m: Value = serde_json::from_str(&fs::read_to_string(&f).unwrap()).unwrap();
            m[u(r, "seed").to_string()].as_f64().unwrap_or_else(|| panic!("EMIT_A_JSON: no seed {}", u(r, "seed")))
        }
        Err(_) => (u(r, "total_muls") - u(r, "sieve_muls") - u(r, "enum_muls")) as f64,
    };
    let base_a = (u(r, "sieve_adds") + u(r, "sieve_lookups") + u(r, "enum_adds")) as f64;
    Cov {
        s: u(r, "total_muls") as f64 / (GAE * sl),
        sp: u(r, "total_plus") as f64 / (GAE * sl),
        nat_a: (base_a + unem) / sl,
        nat_b: u(r, "sieve_lookups") as f64 / sl,
        base_a: base_a / sl,
        unem: unem / sl,
    }
}
/// Reference means [sh, st, sph, spt, steps, nat_b, nat_a] of a set of runs.
fn refm(rs: &[&Rho]) -> [f64; 7] {
    let n = rs.len() as f64;
    let mut m = [0.0; 7];
    for r in rs {
        for (i, v) in [r.sh, r.st, r.sph, r.spt, r.steps, r.nat_b, r.nat_a].iter().enumerate() {
            m[i] += v / n;
        }
    }
    m
}
const CELLS: [&str; 20] = [
    "S/rho U1", "S/rho U2", "S+/rho U1 (rho unpriced)", "S+/rho U2 (rho unpriced)", "S+/rhoS+ U1", "S+/rhoS+ U2",
    "S/rho U1 (ratio of means)", "S/rho U2 (ratio of means)",
    "band A U1 w=0", "band A U1 w=0.1", "band A U1 w=1", "band A U2 w=0", "band A U2 w=0.1", "band A U2 w=1",
    "band B U1 w=0", "band B U1 w=0.1", "band B U1 w=1", "band B U2 w=0", "band B U2 w=0.1", "band B U2 w=1",
];
fn cells(c: &[Cov], m: &[f64; 7]) -> Vec<f64> {
    let s: Vec<f64> = c.iter().map(|x| x.s).collect();
    let sp: Vec<f64> = c.iter().map(|x| x.sp).collect();
    let mut out = vec![
        geo(&s) / m[0], geo(&s) / m[1], geo(&sp) / m[0], geo(&sp) / m[1], geo(&sp) / m[2], geo(&sp) / m[3],
        mean(&s) / m[0], mean(&s) / m[1],
    ];
    for (layer, rn) in [(0usize, m[6]), (1, m[5])] {
        for rho_u in [m[0], m[1]] {
            for w in [0.0, 0.1, 1.0] {
                let cv: Vec<f64> = c.iter().map(|x| x.s + w * if layer == 0 { x.nat_a } else { x.nat_b }).collect();
                out.push(geo(&cv) / (rho_u + w * rn));
            }
        }
    }
    out
}
/// The absolute band values (ic, rho) of a cell set: geo of cover cost, reference cost.
fn band_abs(c: &[Cov], m: &[f64; 7], layer_a: bool, u2: bool) -> Vec<(f64, f64)> {
    [0.0, 0.1, 1.0]
        .iter()
        .map(|&w| {
            let cv: Vec<f64> = c.iter().map(|x| x.s + w * if layer_a { x.nat_a } else { x.nat_b }).collect();
            let rho = if u2 { m[1] } else { m[0] } + w * if layer_a { m[6] } else { m[5] };
            (geo(&cv), rho)
        })
        .collect()
}
fn ks(a: &[f64], b: &[f64]) -> f64 {
    let mut all: Vec<f64> = a.iter().chain(b).copied().collect();
    all.sort_by(|x, y| x.partial_cmp(y).unwrap());
    all.dedup();
    let cdf = |v: &[f64], t: f64| v.iter().filter(|x| **x <= t).count() as f64 / v.len() as f64;
    all.iter().map(|&t| (cdf(a, t) - cdf(b, t)).abs()).fold(0.0, f64::max)
}
fn kw(a: &[f64], b: &[f64]) -> f64 {
    let mut all: Vec<(f64, usize)> = a.iter().map(|x| (*x, 0)).chain(b.iter().map(|x| (*x, 1))).collect();
    all.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
    let n = all.len();
    let mut ranks = vec![0.0; n];
    let mut i = 0;
    let mut tie = 0.0;
    while i < n {
        let mut j = i;
        while j + 1 < n && all[j + 1].0 == all[i].0 {
            j += 1;
        }
        let r = (i + j) as f64 / 2.0 + 1.0;
        for k in i..=j {
            ranks[k] = r;
        }
        let t = (j - i + 1) as f64;
        tie += t * t * t - t;
        i = j + 1;
    }
    let (mut r0, mut r1) = (0.0, 0.0);
    for (k, (_, g)) in all.iter().enumerate() {
        if *g == 0 { r0 += ranks[k] } else { r1 += ranks[k] }
    }
    let nf = n as f64;
    let h = 12.0 / (nf * (nf + 1.0)) * (r0 * r0 / a.len() as f64 + r1 * r1 / b.len() as f64) - 3.0 * (nf + 1.0);
    h / (1.0 - tie / (nf * nf * nf - nf))
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let root = a[1].clone();
    let replay = a[2].clone();
    let outp = a[3].clone();
    let d = format!("{root}/research/jv_sieve_negrho_20261102/results");
    let old = format!("{root}/research/jv_cover_negrho_rescore_20261101/results");
    let fba = format!("{root}/research/jv_sieve_fbline_20261102/results");
    let mut fails: Vec<String> = Vec::new();
    let check = |ok: bool, what: String, fails: &mut Vec<String>| {
        println!("  [{}] {}", if ok { "ok" } else { "FAIL" }, what);
        if !ok {
            fails.push(what);
        }
    };
    let mut out = Map::new();

    println!("# Inputs and receipts");
    let cover_raw = fs::read(format!("{d}/inputs/ic_1511_sparse.json")).unwrap();
    let v2_raw = fs::read(format!("{root}/research/jv_sieve_strong_rho_v2_20261020/raw/ic_1511_sparse.json")).unwrap();
    check(cover_raw == v2_raw, "cover input results/inputs/ic_1511_sparse.json is byte-identical to V2 raw/ic_1511_sparse.json".into(), &mut fails);
    let cover = load(&format!("{d}/inputs/ic_1511_sparse.json"));
    let cseeds: Vec<u64> = cover.iter().map(|r| u(r, "seed")).collect();
    let cok = cover.iter().all(|r| bo(r, "solved") && bo(r, "correct") && u(r, "p") == 1511 && r["lin_mode"] == "sparse" && fl(r, "c_add_e") == GAE);
    check(cover.len() == 16 && cseeds == (17..=32).collect::<Vec<_>>() && cok, format!("cover: 16 V2 sparse rows, seeds 17-32, all solved, correct, sparse, p=1511, c_add_e 331: {cok}"), &mut fails);
    let mut bad = Vec::new();
    let mut new_runs: Vec<Rho> = Vec::new();
    let mut ident_bad = 0;
    for run in 0..2u64 {
        for s in 17..=24u64 {
            let path = format!("{d}/runs/rhostrong_p1511_s{s}_run{run}.json");
            let v = load(&path);
            assert_eq!(v.len(), 1);
            let r = &v[0];
            if u(r, "p") != 1511 || u(r, "seed") != s || u(r, "run") != run {
                bad.push(format!("name/content mismatch {path}"));
            }
            let cv = cover.iter().find(|c| u(c, "seed") == s).unwrap();
            if u(r, "l") != u(cv, "l") || r["c_add_e"] != cv["c_add_e"] {
                ident_bad += 1;
            }
            new_runs.push(rho_of(r, &mut bad, &path));
        }
    }
    check(ident_bad == 0, format!("identity: l and c_add_e of every new run equal V2's sieve row of the same seed ({ident_bad} bad of 16)"), &mut fails);
    let mut old_runs: Vec<Rho> = Vec::new();
    for p in [1301u64, 1451, 1601] {
        for run in 0..2 {
            for r in load(&format!("{old}/inputs/old_rhostrong/rhostrong_p{p}_run{run}.json")) {
                old_runs.push(rho_of(&r, &mut bad, &format!("old p{p} run{run}")));
            }
        }
    }
    let mut newdir: Vec<String> = fs::read_dir(format!("{old}/new")).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).filter(|n| n.ends_with(".json")).collect();
    newdir.sort();
    for n in &newdir {
        let v = load(&format!("{old}/new/{n}"));
        assert_eq!(v.len(), 1);
        old_runs.push(rho_of(&v[0], &mut bad, n));
    }
    check(bad.is_empty() && old_runs.len() == 126 && new_runs.len() == 16,
        format!("rho receipts: {} new + {} #3278 runs correct, uncapped, negation, batch 16, r 256, dp 16, c_add_e 331; muls_tuned = 24 mul + 18 sq + 62 inv and muls_harness = 55 mul + 55 sq + 166 inv exactly; s, s+ (+= fp_adds + six native) to 1e-12; steps = walk_adds (bad: {:?})", new_runs.len(), old_runs.len(), bad), &mut fails);
    let s3278: Vec<f64> = old_runs.iter().map(|r| r.sh).collect();
    let t3278: Vec<f64> = old_runs.iter().map(|r| r.st).collect();
    println!("  selftest: #3278 pooled s_harness {:.4} s_tuned {:.4} (n={})", mean(&s3278), mean(&t3278), old_runs.len());

    // ---- replay controls
    println!("\n# Replays (every field but wall_ms)");
    let rn = load(&format!("{d}/replay/new_p1451_s1_run0.json"));
    let ro = load(&format!("{d}/replay/old_p1451_s1_run0.json"));
    let rall = load(&format!("{d}/replay/old_p1451_allseeds_run0.json"));
    let src = load(&format!("{old}/inputs/old_rhostrong/rhostrong_p1451_run0.json"));
    let src1 = src.iter().find(|r| u(r, "seed") == 1).unwrap();
    check(rall == src, "producer replay: old_p1451_allseeds_run0.json equals #3278 inputs/old_rhostrong/rhostrong_p1451_run0.json (all fields)".into(), &mut fails);
    check(ro.len() == 1 && diff_keys(&ro[0], src1, false).is_empty(), "producer replay: old_p1451_s1_run0.json is the seed-1 row of that file (all fields incl. wall_ms)".into(), &mut fails);
    let dk = diff_keys(&rn[0], &ro[0], true);
    check(rn.len() == 1 && dk.is_empty(), format!("producer replay control 3: new_p1451_s1_run0 vs #3278 record, differing {dk:?}"), &mut fails);
    let mut replays = Vec::new();
    for (mine, theirs, tag) in [
        (format!("{replay}/rho_p1511_s23_run0.json"), format!("{d}/runs/rhostrong_p1511_s23_run0.json"), "study-B rho p=1511 seed 23 run 0"),
        (format!("{replay}/ref_p1451_s1_run6.json"), format!("{old}/new/rhostrong_p1451_s1_run6.json"), "#3278 reference rho p=1451 seed 1 run 6"),
    ] {
        if !Path::new(&mine).exists() {
            println!("  (replay file absent: {mine})");
            continue;
        }
        let m = load(&mine);
        let t = load(&theirs);
        let dk = diff_keys(&m[0], &t[0], true);
        check(m.len() == 1 && dk.is_empty(), format!("validator replay {tag} (main build d21002a3): differing {dk:?}; steps {} wall {:.0}s vs {:.0}s", u(&m[0], "steps"), fl(&m[0], "wall_ms") / 1e3, fl(&t[0], "wall_ms") / 1e3), &mut fails);
        replays.push(json!({"run": tag, "identical_except_wall_ms": dk.is_empty(), "steps": u(&m[0], "steps"),
            "muls_harness": fl(&m[0], "muls_harness"), "s_harness": fl(&m[0], "s_harness"), "replay_wall_ms": fl(&m[0], "wall_ms"), "original_wall_ms": fl(&t[0], "wall_ms")}));
    }
    for s in [21u64, 32] {
        let mine = format!("{replay}/P_1511_s{s}.json");
        if !Path::new(&mine).exists() {
            continue;
        }
        let m = load(&mine);
        let t = cover.iter().find(|r| u(r, "seed") == s).unwrap();
        let dk = diff_keys(&m[0], t, true);
        check(m.len() == 1 && dk.is_empty(), format!("validator replay cover (V2 sparse row) p=1511 seed {s} (main build): differing {dk:?}; total_muls {}", u(&m[0], "total_muls")), &mut fails);
        replays.push(json!({"run": format!("cover sieve P p=1511 seed {s} (V2 ic_1511_sparse row)"), "identical_except_wall_ms": dk.is_empty(), "total_muls": u(&m[0], "total_muls"), "total_plus": u(&m[0], "total_plus")}));
    }
    out.insert("replays".into(), Value::Array(replays));

    // ---- positive control and homogeneity
    println!("\n# Positive control and homogeneity (frozen tests)");
    let theory = (std::f64::consts::PI / 4.0).sqrt() / (1.0 - 1.0 / 256.0f64).sqrt();
    let ms: Vec<f64> = new_runs.iter().map(|r| r.steps).collect();
    let os: Vec<f64> = old_runs.iter().map(|r| r.steps).collect();
    let z = (mean(&ms) - theory) / (sd(&ms) / (ms.len() as f64).sqrt());
    let zo = (mean(&os) - theory) / (sd(&os) / (os.len() as f64).sqrt());
    check(z.abs() <= 3.0, format!("control 2: matched steps/sqrt(l) mean {:.4} sd {:.4} se {:.4}, theory {theory:.5}, z = {z:.3} (|z| <= 3); #3278 pooled mean {:.4} sd {:.4} z = {zo:.3}", mean(&ms), sd(&ms), sd(&ms) / 4.0, mean(&os), sd(&os)), &mut fails);
    let mut hom = Map::new();
    let mut homog = true;
    for (name, f) in [("steps/sqrt(l)", 0usize), ("s_harness", 1), ("s_tuned", 2)] {
        let get = |r: &Rho| match f { 0 => r.steps, 1 => r.sh, _ => r.st };
        let x: Vec<f64> = new_runs.iter().map(get).collect();
        let y: Vec<f64> = old_runs.iter().map(get).collect();
        let dm = (mean(&x) - mean(&y)).abs();
        let dks = ks(&x, &y);
        let hkw = kw(&x, &y);
        let mut all: Vec<f64> = x.iter().chain(&y).copied().collect();
        let mut rng = Rng(0x0005_EED0_0000_0000 + f as u64);
        let (mut c1, mut c2, mut c3) = (0usize, 0usize, 0usize);
        for _ in 0..B {
            rng.shuffle(&mut all);
            let (g1, g2) = all.split_at(x.len());
            if (mean(g1) - mean(g2)).abs() >= dm - 1e-15 { c1 += 1 }
            if ks(g1, g2) >= dks - 1e-12 { c2 += 1 }
            if kw(g1, g2) >= hkw - 1e-12 { c3 += 1 }
        }
        let pv = |c: usize| (c + 1) as f64 / (B + 1) as f64;
        println!("  {name}: means {:.4} vs {:.4}; |diff| {dm:.4} perm p {:.4}; KS D {dks:.4} perm p {:.4}; KW H {hkw:.4} perm p {:.4}", mean(&x), mean(&y), pv(c1), pv(c2), pv(c3));
        if f == 0 {
            homog = pv(c1) >= 0.05 && pv(c2) >= 0.05;
        }
        hom.insert(name.into(), json!({"mean_matched": mean(&x), "mean_3278": mean(&y), "absdiff": dm, "perm_p": pv(c1), "ks_d": dks, "ks_perm_p": pv(c2), "kw_h": hkw, "kw_perm_p": pv(c3)}));
    }
    println!("  frozen rule: homogeneous iff both steps/sqrt(l) permutation p >= 0.05 -> {homog}; primary reference = {}", if homog { "pooled 142" } else { "matched 16" });
    out.insert("homogeneity".into(), Value::Object(hom));
    out.insert("positive_control".into(), json!({"matched_mean": mean(&ms), "matched_sd": sd(&ms), "z": z, "theory": theory, "pooled3278_z": zo}));

    // ---- reference groups
    let mut pooled: Vec<Rho> = old_runs.clone();
    pooled.extend(new_runs.iter().cloned());
    let mk_groups = |rs: &[Rho]| -> Vec<Vec<usize>> {
        let mut g: BTreeMap<(u64, u64), Vec<usize>> = BTreeMap::new();
        for (i, r) in rs.iter().enumerate() {
            g.entry((r.p, r.seed)).or_default().push(i);
        }
        g.into_values().collect()
    };
    let gp = mk_groups(&pooled);
    let gm = mk_groups(&new_runs);
    let mp = refm(&pooled.iter().collect::<Vec<_>>());
    let mm = refm(&new_runs.iter().collect::<Vec<_>>());
    println!("\n# Reference means [s_harness, s_tuned, s_plus_harness, s_plus_tuned, steps/sqrt(l), native6/sqrt(l), (native6+fp_adds)/sqrt(l)]");
    println!("  matched (16 runs, {} groups): {:?}", gm.len(), mm.map(|x| (x * 1e5).round() / 1e5));
    println!("  pooled ({} runs, {} groups): {:?}", pooled.len(), gp.len(), mp.map(|x| (x * 1e5).round() / 1e5));
    out.insert("reference_means".into(), json!({"matched": mm, "pooled": mp, "order": ["s_harness","s_tuned","s_plus_harness","s_plus_tuned","steps_over_sqrt_l","native6_over_sqrt_l","native6_plus_fp_adds_over_sqrt_l"]}));

    // ---- headline cells with the joint bootstrap
    let covs: BTreeMap<u64, Cov> = cover.iter().map(|r| (u(r, "seed"), cov_of(r))).collect();
    let mut cellsout = Map::new();
    let mut seedc = 0xC0DE_2026_1103_0000u64;
    for (sname, seeds) in [("seeds 17-24", (17..=24u64).collect::<Vec<_>>()), ("all 16 seeds", (17..=32u64).collect())] {
        for (rname, rs, groups) in [("matched", &new_runs, &gm), ("pooled", &pooled, &gp)] {
            let c: Vec<Cov> = seeds.iter().map(|s| covs[s]).collect();
            let m = refm(&rs.iter().collect::<Vec<_>>());
            let point = cells(&c, &m);
            seedc = seedc.wrapping_add(0x9E37_79B9);
            let mut rng = Rng(seedc);
            let mut bs: Vec<Vec<f64>> = vec![Vec::with_capacity(B); CELLS.len()];
            for _ in 0..B {
                let cb: Vec<Cov> = (0..c.len()).map(|_| c[rng.below(c.len())]).collect();
                let mut rb: Vec<&Rho> = Vec::with_capacity(rs.len());
                for g in groups.iter() {
                    for _ in 0..g.len() {
                        rb.push(&rs[g[rng.below(g.len())]]);
                    }
                }
                let v = cells(&cb, &refm(&rb));
                for (k, x) in v.into_iter().enumerate() {
                    bs[k].push(x);
                }
            }
            println!("\n## cover {sname}, reference {rname}{}", if rname == "pooled" { " [PRIMARY]" } else { "" });
            let mut cm = Map::new();
            for (k, name) in CELLS.iter().enumerate() {
                bs[k].sort_by(|x, y| x.partial_cmp(y).unwrap());
                let (lo, hi) = (quant(&bs[k], 0.025), quant(&bs[k], 0.975));
                let verdict = if hi < 1.0 { "below" } else if lo > 1.0 { "above" } else { "not resolved" };
                println!("  {name:<28} {:.5} [{:.5}, {:.5}] {verdict} (x{:.1} [{:.1}, {:.1}] below rho)", point[k], lo, hi, 1.0 / point[k], 1.0 / hi, 1.0 / lo);
                cm.insert(name.to_string(), json!([point[k], lo, hi]));
            }
            cellsout.insert(format!("{sname} | {rname}"), Value::Object(cm));
            if sname == "seeds 17-24" && rname == "pooled" {
                let mut ba = Map::new();
                for (lab, la, u2) in [("A_U1", true, false), ("A_U2", true, true), ("B_U1", false, false), ("B_U2", false, true)] {
                    let v = band_abs(&c, &m, la, u2);
                    println!("  band {lab} absolute (ic GAE/sqrt l, rho GAE/sqrt l): w=0 {:.6}/{:.6}  w=0.1 {:.6}/{:.6}  w=1 {:.6}/{:.6}", v[0].0, v[0].1, v[1].0, v[1].1, v[2].0, v[2].1);
                    ba.insert(lab.into(), json!({"0": {"ic": v[0].0, "rho": v[0].1}, "0.1": {"ic": v[1].0, "rho": v[1].1}, "1": {"ic": v[2].0, "rho": v[2].1}}));
                }
                out.insert("band_absolute_primary".into(), Value::Object(ba));
                // robustness: multiplier k on the unemitted proxy that would bring band A at w = 1 to parity
                for (lab, rho_u) in [("U1", m[0]), ("U2", m[1])] {
                    let f = |k: f64| geo(&c.iter().map(|x| x.s + (x.base_a + k * x.unem)).collect::<Vec<_>>()) / (rho_u + m[6]);
                    let (mut lo, mut hi) = (0.0, 1.0);
                    while f(hi) < 1.0 { hi *= 2.0; }
                    for _ in 0..200 { let mid = 0.5 * (lo + hi); if f(mid) < 1.0 { lo = mid } else { hi = mid } }
                    println!("  band A {lab} w=1: ratio {:.4} at k=1; parity needs k = {:.1} unemitted F4/descent/LA additions per unemitted multiplication", f(1.0), lo);
                    out.insert(format!("unemitted_breakeven_k_{lab}"), json!(lo));
                }
            }
        }
    }
    out.insert("cells".into(), Value::Object(cellsout));

    // ---- section 5a: every unpriced counter of every arm, with its count (primary seeds / reference)
    println!("\n# Section 5a counter list (means per run; cover over seeds 17-24, rho over the pooled 142 runs)");
    {
        let rows: Vec<&Value> = cover.iter().filter(|r| (17..=24).contains(&u(r, "seed"))).collect();
        let mc = |k: &str| mean(&rows.iter().map(|r| u(r, k) as f64).collect::<Vec<_>>());
        let un = mean(&rows.iter().map(|r| (u(r, "total_muls") - u(r, "sieve_muls") - u(r, "enum_muls")) as f64).collect::<Vec<_>>());
        println!("  cover: sieve_adds {:.4e}, sieve_lookups {:.4e}, enum_adds {:.4e} (emitted); F4/descent/LA/setup additions: no counter (layer-A proxy = unemitted muls {:.4e}); byte counters: none", mc("sieve_adds"), mc("sieve_lookups"), mc("enum_adds"), un);
        let mut allr: Vec<Value> = Vec::new();
        for p in [1301u64, 1451, 1601] {
            for run in 0..2 {
                allr.extend(load(&format!("{old}/inputs/old_rhostrong/rhostrong_p{p}_run{run}.json")));
            }
        }
        for n in &newdir {
            allr.extend(load(&format!("{old}/new/{n}")));
        }
        for run in 0..2u64 {
            for s in 17..=24u64 {
                allr.extend(load(&format!("{d}/runs/rhostrong_p1511_s{s}_run{run}.json")));
            }
        }
        let mr = |k: &str| mean(&allr.iter().map(|r| u(&r["ops"], k) as f64).collect::<Vec<_>>());
        println!("  rho ({} runs): canon_cmp {:.4e}, hash {:.4e}, dp_check {:.4e}, scalar_ops {:.4e}, table_ops {:.4e}, cycle_cmp {:.4e}, fp_adds {:.4e}; byte counters: none", allr.len(), mr("canon_cmp"), mr("hash"), mr("dp_check"), mr("scalar_ops"), mr("table_ops"), mr("cycle_cmp"), mr("fp_adds"));
    }

    // ---- LEADS.md combined numbers: product of ratios vs direct division
    println!("\n# LEADS.md 'Combined' (p = 1511): product of two ratios vs direct division of the arm records by the pooled reference");
    let h = load(&format!("{fba}/H_1511.json"));
    let uu = load(&format!("{fba}/U_1511.json"));
    let pp = load(&format!("{fba}/P_1511.json"));
    let ratio16 = |arm: &[Value], plus: bool| geo(&arm.iter().zip(&pp).map(|(x, y)| {
        let k = if plus { "total_plus" } else { "total_muls" };
        u(x, k) as f64 / u(y, k) as f64
    }).collect::<Vec<_>>());
    let c8: Vec<Cov> = (17..=24u64).map(|s| covs[&s]).collect();
    let s8 = geo(&c8.iter().map(|x| x.s).collect::<Vec<_>>());
    let sp8 = geo(&c8.iter().map(|x| x.sp).collect::<Vec<_>>());
    let direct = |arm: &[Value], plus: bool, seeds: std::ops::RangeInclusive<u64>| geo(&arm.iter().filter(|r| seeds.contains(&u(r, "seed"))).map(|r| {
        let c = cov_of(r);
        if plus { c.sp } else { c.s }
    }).collect::<Vec<_>>());
    let (hr, ur) = (ratio16(&h, false), ratio16(&uu, true));
    let lines = [
        ("H S/rho U1", s8 / mp[0] * hr, direct(&h, false, 17..=24) / mp[0], direct(&h, false, 17..=32) / mp[0]),
        ("H S/rho U2", s8 / mp[1] * hr, direct(&h, false, 17..=24) / mp[1], direct(&h, false, 17..=32) / mp[1]),
        ("U S+/rho U1", sp8 / mp[0] * ur, direct(&uu, true, 17..=24) / mp[0], direct(&uu, true, 17..=32) / mp[0]),
        ("U S+/rho U2", sp8 / mp[1] * ur, direct(&uu, true, 17..=24) / mp[1], direct(&uu, true, 17..=32) / mp[1]),
    ];
    println!("  H S ratio at p=1511 (16 seeds) {hr:.4}; U S+ ratio at p=1511 (16 seeds) {ur:.4}");
    let mut comb = Map::new();
    for (n, prod, d8, d16) in lines {
        println!("  {n}: product {prod:.5} (x{:.1}); direct seeds 17-24 {d8:.5} (x{:.1}); direct 16 seeds {d16:.5}", 1.0 / prod, 1.0 / d8);
        comb.insert(n.into(), json!({"product": prod, "direct_17_24": d8, "direct_17_32": d16}));
    }
    out.insert("leads_combined".into(), Value::Object(comb));
    // ---- the producer's fit.json point estimates against the recomputation
    println!("\n# Producer fit.json point estimates vs validator recomputation");
    {
        let fj: Value = serde_json::from_str(&fs::read_to_string(format!("{root}/research/jv_sieve_negrho_20261102/fit.json")).unwrap()).unwrap();
        let mut worst: f64 = 0.0;
        let mut n = 0;
        let mut gate = Vec::new();
        for (set, cm) in out["cells"].as_object().unwrap() {
            let key = set.replace(" | ", "|");
            for (cell, v) in cm.as_object().unwrap() {
                let claimed = fj[format!("{key}|{cell}")]["point"].as_f64().unwrap_or_else(|| panic!("fit.json lacks {key}|{cell}"));
                let mine = v[0].as_f64().unwrap();
                worst = worst.max(rel(mine, claimed));
                n += 1;
                if key == "seeds 17-24|pooled" {
                    gate.push(json!({"name": format!("{key}|{cell}"), "claimed": claimed, "recomputed": mine, "rel_tol": 1e-9}));
                }
            }
        }
        let rm = &fj["reference_means"];
        for (i, nm) in ["s_harness", "s_tuned", "steps_per_sqrt_l", "native_per_sqrt_l", "fp_adds_per_sqrt_l"].iter().enumerate() {
            let mine = [mp[0], mp[1], mp[4], mp[5], mp[6] - mp[5]][i];
            let claimed = rm["pooled"][i].as_f64().unwrap();
            worst = worst.max(rel(mine, claimed));
            n += 1;
            gate.push(json!({"name": format!("pooled reference mean {nm} (142 runs)"), "claimed": claimed, "recomputed": mine, "rel_tol": 1e-9}));
        }
        let zc = fj["controls"]["z_matched"].as_f64().unwrap();
        worst = worst.max(rel(z, zc));
        n += 1;
        gate.push(json!({"name": "positive control z (matched 16 runs vs 0.88796)", "claimed": zc, "recomputed": z, "rel_tol": 1e-9}));
        let hs = &fj["homogeneity"]["steps/sqrt(l)"];
        let hm = &out["homogeneity"]["steps/sqrt(l)"];
        for (a, b) in [("absdiff", "absdiff"), ("ks", "ks_d"), ("kw", "kw_h")] {
            let (c, m) = (hs[a].as_f64().unwrap(), hm[b].as_f64().unwrap());
            worst = worst.max(rel(m, c));
            n += 1;
            gate.push(json!({"name": format!("homogeneity statistic {a} (steps/sqrt l)"), "claimed": c, "recomputed": m, "rel_tol": 1e-9}));
        }
        let ok = worst < 1e-9;
        println!("  [{}] {n} deterministic numbers (80 ratio cells, 5 reference means, z, 3 test statistics): max relative difference {worst:.3e}", if ok { "ok" } else { "FAIL" });
        if !ok {
            fails.push("fit.json point estimates differ".into());
        }
        out.insert("gate_headline".into(), Value::Array(gate));
    }
    out.insert("failures".into(), json!(fails));
    println!("\n# Failures: {}", if fails.is_empty() { "none".to_string() } else { fails.join(" | ") });
    fs::write(&outp, serde_json::to_string_pretty(&Value::Object(out)).unwrap()).unwrap();
}
