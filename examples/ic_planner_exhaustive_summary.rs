//! `research/ic_planner_exhaustive_20261026/analysis/summary.json`: every
//! PROTOCOL.md prediction, evaluated mechanically from the committed ecbench
//! outputs (`band --json`, `compare --save`), the null JSONs and, for R4, the
//! residual study's band JSONs. Reads nothing else.
//!
//! Native port of the study's former `scripts/summary.py`; it writes the same
//! bytes. The null bootstrap draws from CPython's `random.Random(20261107)`
//! (MT19937 seeded by `init_by_array`, `choice` by rejection on
//! `getrandbits(n.bit_length())`), reproduced here exactly, and float sums are
//! CPython's compensated builtin `sum`.
//!
//! ```text
//! cargo run --release --example ic_planner_exhaustive_summary [STUDY_DIR]
//! ```

use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

use crypto_lib::cryptanalysis::ecc2k130_pyjson::{
    dumps_floats, float_repr, parse_json, py_float, py_repr, py_sum_floats, Json, Obj, Style,
};

const R: [&str; 4] = ["R-p16", "R-p18", "R-p36", "R-p37"];
const B: [&str; 4] = ["B-hb19", "B-hb23", "B-hb25", "B-hb27"];
const K: [&str; 6] = ["K-k47", "K-k57", "K-k41", "K-k53", "K-k59", "K-k61"];

fn die(msg: String) -> ! {
    eprintln!("{msg}");
    exit(1)
}

fn load(path: &Path) -> Json {
    let text = fs::read_to_string(path).unwrap_or_else(|e| die(format!("{}: {e}", path.display())));
    parse_json(&text).unwrap_or_else(|e| die(format!("{}: {e}", path.display())))
}

fn at<'a>(v: &'a Json, key: &str) -> &'a Json {
    v.as_obj()
        .and_then(|o| o.get(key))
        .unwrap_or_else(|| die(format!("KeyError: {key:?}")))
}

fn list(v: &Json) -> &[Json] {
    match v {
        Json::List(l) => l,
        _ => die(format!("expected a list, found {}", py_repr(v))),
    }
}

fn num(v: &Json) -> f64 {
    py_float(v).unwrap_or_else(|e| die(e))
}

fn obj(pairs: Vec<(&str, Json)>) -> Json {
    let mut o = Obj::new();
    for (k, v) in pairs {
        o.set(k, v);
    }
    Json::Obj(o)
}

fn int(n: usize) -> Json {
    Json::Int(n as i128)
}

fn fsum(terms: &[f64]) -> f64 {
    match py_sum_floats(terms) {
        Json::Float(x) => x,
        Json::Int(i) => i as f64,
        _ => unreachable!(),
    }
}

/// Names in `dir` matching `prefix*suffix`, sorted (`sorted(glob(...))`).
fn glob(dir: &Path, prefix: &str, suffix: &str) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = match fs::read_dir(dir) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
                    n.len() >= prefix.len() + suffix.len()
                        && n.starts_with(prefix)
                        && n.ends_with(suffix)
                })
            })
            .collect(),
        Err(_) => Vec::new(),
    };
    out.sort_by(|a, b| a.as_os_str().cmp(b.as_os_str()));
    out
}

// ── CPython's random.Random ────────────────────────────────────────

struct Mt {
    s: [u32; 624],
    i: usize,
}

impl Mt {
    /// `random.Random(seed)` for a non-negative int seed below 2^32.
    fn new(seed: u32) -> Self {
        let mut mt = Mt {
            s: [0; 624],
            i: 624,
        };
        mt.init_genrand(19650218);
        let key = [seed];
        let (mut i, mut j) = (1usize, 0usize);
        for _ in 0..624.max(key.len()) {
            let prev = mt.s[i - 1] ^ (mt.s[i - 1] >> 30);
            mt.s[i] = (mt.s[i] ^ prev.wrapping_mul(1664525))
                .wrapping_add(key[j])
                .wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= 624 {
                mt.s[0] = mt.s[623];
                i = 1;
            }
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..623 {
            let prev = mt.s[i - 1] ^ (mt.s[i - 1] >> 30);
            mt.s[i] = (mt.s[i] ^ prev.wrapping_mul(1566083941)).wrapping_sub(i as u32);
            i += 1;
            if i >= 624 {
                mt.s[0] = mt.s[623];
                i = 1;
            }
        }
        mt.s[0] = 0x8000_0000;
        mt
    }

    fn init_genrand(&mut self, seed: u32) {
        self.s[0] = seed;
        for i in 1..624 {
            let prev = self.s[i - 1] ^ (self.s[i - 1] >> 30);
            self.s[i] = 1812433253u32.wrapping_mul(prev).wrapping_add(i as u32);
        }
        self.i = 624;
    }

    fn next_u32(&mut self) -> u32 {
        if self.i >= 624 {
            for k in 0..624 {
                let y = (self.s[k] & 0x8000_0000) | (self.s[(k + 1) % 624] & 0x7fff_ffff);
                let mut v = self.s[(k + 397) % 624] ^ (y >> 1);
                if y & 1 != 0 {
                    v ^= 0x9908_b0df;
                }
                self.s[k] = v;
            }
            self.i = 0;
        }
        let mut y = self.s[self.i];
        self.i += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^ (y >> 18)
    }

    /// `_randbelow_with_getrandbits(n)` for 0 < n < 2^32.
    fn below(&mut self, n: usize) -> usize {
        let k = usize::BITS - n.leading_zeros();
        loop {
            let r = (self.next_u32() >> (32 - k)) as usize;
            if r < n {
                return r;
            }
        }
    }
}

// ── The study ──────────────────────────────────────────────────────

struct Study {
    root: PathBuf,
}

impl Study {
    fn band(&self, s: &str) -> Json {
        load(&self.root.join("analysis").join(format!("band-{s}.json")))
    }

    /// The single saved `compare` of `a -> b` at weight `w`.
    fn cmp(&self, s: &str, a: &str, b: &str, w: &str) -> Json {
        let wf: f64 = w.parse().expect("weight");
        let dir = self.root.join("sessions").join(s).join("comparisons");
        let mut hits = Vec::new();
        for f in glob(&dir, &format!("{a}__{b}"), ".json") {
            let d = load(&f);
            let pw = d.as_obj().and_then(|o| o.get("pricing_weight"));
            let pw = match pw {
                Some(v) if v.truthy() => num(v),
                _ => 0.0,
            };
            if pw == wf {
                hits.push(d);
            }
        }
        if hits.len() != 1 {
            die(format!(
                "{s} {a}->{b} w={w}: {} saved comparisons",
                hits.len()
            ));
        }
        let o = at(&hits[0], "ops");
        obj(vec![
            ("ratio", at(o, "ratio_b_over_a").clone()),
            ("ci95", at(o, "ci95").clone()),
            ("pairs", at(o, "pairs").clone()),
        ])
    }
}

fn ci(c: &Json, i: usize) -> f64 {
    num(&list(at(c, "ci95"))[i])
}

fn ratio(c: &Json) -> f64 {
    num(at(c, "ratio"))
}

fn below(c: &Json) -> bool {
    ci(c, 1) < 1.0
}

fn above(c: &Json) -> bool {
    ci(c, 0) > 1.0
}

fn slope(points: &[(f64, f64)]) -> f64 {
    let xs: Vec<f64> = points.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = points.iter().map(|p| p.1).collect();
    let mx = fsum(&xs) / xs.len() as f64;
    let my = fsum(&ys) / ys.len() as f64;
    let num: Vec<f64> = points.iter().map(|(x, y)| (x - mx) * (y - my)).collect();
    let den: Vec<f64> = xs.iter().map(|x| (x - mx) * (x - mx)).collect();
    fsum(&num) / fsum(&den)
}

fn ints(v: &Json) -> Vec<i128> {
    list(v)
        .iter()
        .map(|x| {
            x.as_int()
                .unwrap_or_else(|| die(format!("not an int: {}", py_repr(x))))
        })
        .collect()
}

/// int / int as Python's true division (exact here: the sums are small).
fn mean(xs: &[i128]) -> f64 {
    let s: i128 = xs.iter().sum();
    assert!(s.unsigned_abs() < 1 << 53);
    s as f64 / xs.len() as f64
}

fn null_case(path: &Path) -> Json {
    let d = load(path);
    let c = ints(at(at(&d, "curve"), "group_ops"));
    let f = ints(at(at(&d, "curve_free"), "group_ops"));
    let mut rng = Mt::new(20261107);
    let mut boot = Vec::with_capacity(4000);
    for _ in 0..4000 {
        let sc: i128 = (0..c.len()).map(|_| c[rng.below(c.len())]).sum();
        let sf: i128 = (0..f.len()).map(|_| f[rng.below(f.len())]).sum();
        let mc = mean(&[sc]) / c.len() as f64;
        let mf = mean(&[sf]) / f.len() as f64;
        boot.push(mf / mc);
    }
    boot.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let ratio = mean(&f) / mean(&c);
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    obj(vec![
        ("case", Json::str(stem)),
        ("r", at(&d, "r").clone()),
        ("base", at(&d, "base").clone()),
        ("runs", at(&d, "runs").clone()),
        (
            "verified",
            Json::List(vec![
                at(at(&d, "curve"), "verified").clone(),
                at(at(&d, "curve_free"), "verified").clone(),
            ]),
        ),
        ("ratio", Json::Float(ratio)),
        (
            "ci95",
            Json::List(vec![Json::Float(boot[99]), Json::Float(boot[3899])]),
        ),
        (
            "excludes_1",
            Json::Bool(!(boot[99] <= 1.0 && 1.0 <= boot[3899])),
        ),
    ])
}

fn per<F: Fn(&str) -> Json>(keys: &[&str], f: F) -> Json {
    obj(keys.iter().map(|k| (*k, f(k))).collect())
}

fn count<I: IntoIterator<Item = bool>>(it: I) -> usize {
    it.into_iter().filter(|b| *b).count()
}

fn main() {
    let root = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            Path::new(env!("CARGO_MANIFEST_DIR")).join("research/ic_planner_exhaustive_20261026")
        });
    let earlier_r = root
        .parent()
        .unwrap()
        .join("ic_prime_residual_lp_20261026")
        .join("analysis");
    let st = Study { root: root.clone() };

    let mut sessions = Obj::new();
    for s in R.iter().chain(&B).chain(&K) {
        let bd = st.band(s);
        let rows = list(at(&bd, "rows"));
        let weights = list(at(&bd, "weights"));
        let mut verified = Obj::new();
        let mut mean_s = Obj::new();
        for r in rows {
            let arm = at(r, "arm").as_str().unwrap_or_else(|| die("arm".into()));
            verified.set(
                arm,
                Json::List(vec![at(r, "verified").clone(), at(r, "runs").clone()]),
            );
            let mut m = Obj::new();
            for (w, v) in weights.iter().zip(list(at(r, "mean_s"))) {
                m.set(&py_repr(w), v.clone());
            }
            mean_s.set(arm, Json::Obj(m));
        }
        sessions.set(
            s,
            obj(vec![
                ("slug", at(&rows[0], "curve").clone()),
                ("log2_r", at(&rows[0], "log2_r").clone()),
                ("verified", Json::Obj(verified)),
                ("mean_s", Json::Obj(mean_s)),
            ]),
        );
    }
    let sess = Json::Obj(sessions.clone());
    let log2_r = |s: &str| num(at(at(&sess, s), "log2_r"));
    let mean_w0 = |s: &str, a: &str| num(at(at(at(at(&sess, s), "mean_s"), a), &float_repr(0.0)));

    let mut p = Obj::new();

    // R1: res4/plan < 1 at w = 0, interval below 1, at every new size.
    let r1 = per(&R, |s| st.cmp(s, "plan", "res4", "0"));
    let held = R.iter().all(|s| below(at(&r1, s)));
    p.set("R1", obj(vec![("rows", r1), ("held", Json::Bool(held))]));
    // R2: S_1(res4)/S_1(plan) < 1.33 at p36 and p37.
    let r2s = ["R-p36", "R-p37"];
    let r2 = per(&r2s, |s| st.cmp(s, "plan", "res4", "1"));
    let held = r2s.iter().all(|s| ratio(at(&r2, s)) < 1.33);
    p.set("R2", obj(vec![("rows", r2), ("held", Json::Bool(held))]));
    // R3 (risky): at p37 the w = 1 point estimate of res4/plan is below 1.
    let r3 = st.cmp("R-p37", "plan", "res4", "1");
    let r3h = ratio(&r3) < 1.0;
    let differ = (ratio(&st.cmp("R-p37", "plan", "res4", "0")) < 1.0) != r3h;
    p.set(
        "R3",
        obj(vec![
            ("row", r3),
            ("held", Json::Bool(r3h)),
            ("w0_and_w1_orders_differ", Json::Bool(differ)),
        ]),
    );
    // R4: slope of ln GAE on ln r over all ten sizes, at w = 0.
    let arms4 = ["res4", "plan"];
    let mut pts: Vec<Vec<(f64, f64)>> = vec![Vec::new(), Vec::new()];
    let files: Vec<PathBuf> = glob(&earlier_r, "band-p", ".json")
        .into_iter()
        .chain(
            R.iter()
                .map(|s| root.join("analysis").join(format!("band-{s}.json"))),
        )
        .collect();
    for f in files {
        let bd = load(&f);
        for r in list(at(&bd, "rows")) {
            let arm = at(r, "arm").as_str().unwrap_or_default();
            if let Some(i) = arms4.iter().position(|a| *a == arm) {
                let lr = num(at(r, "log2_r")) * std::f64::consts::LN_2;
                pts[i].push((lr, num(&list(at(r, "mean_s"))[0]).ln() + lr / 2.0));
            }
        }
    }
    let sl: Vec<f64> = pts.iter().map(|p| slope(p)).collect();
    p.set(
        "R4",
        obj(vec![
            (
                "points",
                obj(vec![
                    ("res4", int(pts[0].len())),
                    ("plan", int(pts[1].len())),
                ]),
            ),
            (
                "slope",
                obj(vec![
                    ("res4", Json::Float(sl[0])),
                    ("plan", Json::Float(sl[1])),
                ]),
            ),
            (
                "held",
                Json::Bool((0.45..=0.55).contains(&sl[0]) && (0.60..=0.70).contains(&sl[1])),
            ),
        ]),
    );
    // R5: the cheapest residual arm at w = 0 is res2 or res4 at every size.
    let res = ["res2", "res4", "res8", "res16", "res32"];
    let r5 = per(&R, |s| {
        let mut best = res[0];
        for a in &res[1..] {
            if mean_w0(s, a) < mean_w0(s, best) {
                best = a;
            }
        }
        Json::str(best)
    });
    let held = R
        .iter()
        .all(|s| matches!(at(&r5, s).as_str(), Some("res2" | "res4")));
    p.set(
        "R5",
        obj(vec![("cheapest", r5), ("held", Json::Bool(held))]),
    );

    // N1: at most 2 of 16 null intervals exclude 1.
    let nulls: Vec<Json> = glob(&root.join("analysis"), "null-", ".json")
        .iter()
        .map(|f| null_case(f))
        .collect();
    let excl = count(nulls.iter().map(|c| at(c, "excludes_1").truthy()));
    let n = nulls.len();
    p.set(
        "N1",
        obj(vec![
            ("cases", Json::List(nulls)),
            ("n", int(n)),
            ("excluding_1", int(excl)),
            ("held", Json::Bool(n == 16 && excl <= 2)),
        ]),
    );

    // W1: S_1(m141)/S_1(plan) < 1 with interval below 1 on >= 3 of 4; at w = 0, ratio >= 1 on >= 3 of 4.
    let w1 = per(&R, |s| {
        obj(vec![
            ("0", st.cmp(s, "plan", "m141", "0")),
            ("1", st.cmp(s, "plan", "m141", "1")),
        ])
    });
    let w1b = count(R.iter().map(|s| below(at(at(&w1, s), "1"))));
    let w0a = count(R.iter().map(|s| ratio(at(at(&w1, s), "0")) >= 1.0));
    p.set(
        "W1",
        obj(vec![
            ("rows", w1),
            ("w1_below", int(w1b)),
            ("w0_at_least_1", int(w0a)),
            ("held", Json::Bool(w1b >= 3 && w0a >= 3)),
        ]),
    );
    // W2: at w = 0 and 1, dm1/plan and dp1/plan each > 1 on >= 3 of 4.
    let w2 = per(&B, |s| {
        per(&["dm1", "dp1"], |b| {
            per(&["0", "1"], |w| st.cmp(s, "plan", b, w))
        })
    });
    let mut cnt = Obj::new();
    let mut all3 = true;
    for b in ["dm1", "dp1"] {
        for w in ["0", "1"] {
            let c = count(B.iter().map(|s| ratio(at(at(at(&w2, s), b), w)) > 1.0));
            all3 &= c >= 3;
            cnt.set(&format!("{b}@{w}"), int(c));
        }
    }
    p.set(
        "W2",
        obj(vec![
            ("rows", w2),
            ("counts_above_1", Json::Obj(cnt)),
            ("held", Json::Bool(all3)),
        ]),
    );
    // W3: S_1(pipeline71)/S_1(pipeline) < 1 with interval below 1 on >= 4 of 6; at w = 0 > 1 on >= 4 of 6.
    let w3 = per(&K, |s| {
        per(&["0", "0.3", "1"], |w| {
            st.cmp(s, "pipeline", "pipeline71", w)
        })
    });
    let w1b = count(K.iter().map(|s| below(at(at(&w3, s), "1"))));
    let w0a = count(K.iter().map(|s| ratio(at(at(&w3, s), "0")) > 1.0));
    p.set(
        "W3",
        obj(vec![
            ("rows", w3),
            ("w1_below", int(w1b)),
            ("w0_above_1", int(w0a)),
            ("held", Json::Bool(w1b >= 4 && w0a >= 4)),
        ]),
    );

    // K1: descent/pipeline > 1 with interval above 1 at all six sizes (counted S).
    let k1 = per(&K, |s| st.cmp(s, "pipeline", "descent", "0"));
    let held = K.iter().all(|s| above(at(&k1, s)));
    p.set("K1", obj(vec![("rows", k1), ("held", Json::Bool(held))]));
    // K2: descent/rho and pipeline/rho each grow with log2 r.
    let mut order: Vec<&str> = K.to_vec();
    order.sort_by(|a, b| log2_r(a).partial_cmp(&log2_r(b)).unwrap_or(Ordering::Equal));
    let mut k2rows = Obj::new();
    let mut k2held = true;
    let mut pairs = Obj::new();
    for a in ["pipeline", "descent"] {
        let rs: Vec<(String, Json, f64, Json)> = order
            .iter()
            .map(|s| {
                let c = st.cmp(s, "rho", a, "0");
                (
                    s.to_string(),
                    at(at(&sess, s), "log2_r").clone(),
                    ratio(&c),
                    at(&c, "ratio").clone(),
                )
            })
            .collect();
        let inc = count(rs.windows(2).map(|w| w[0].2 < w[1].2));
        k2held &= inc == rs.len() - 1;
        pairs.set(a, int(inc));
        k2rows.set(
            a,
            Json::List(
                rs.into_iter()
                    .map(|(s, l, _, r)| Json::List(vec![Json::Str(s), l, r]))
                    .collect(),
            ),
        );
    }
    p.set(
        "K2",
        obj(vec![
            ("rows", Json::Obj(k2rows)),
            ("held", Json::Bool(k2held)),
            ("spearman_like_increasing_pairs", Json::Obj(pairs)),
        ]),
    );

    // Per-curve tables for the README and figures (all from compare --save).
    let l2 = |s: &str| at(at(&sess, s), "log2_r").clone();
    let w4 = ["0", "0.1", "0.3", "1"];
    let w01 = ["0", "1"];
    let tr = per(&R, |s| {
        obj(vec![
            ("log2_r", l2(s)),
            (
                "over_plan",
                per(&["res2", "res4", "res8", "m141"], |b| {
                    per(&w4, |w| st.cmp(s, "plan", b, w))
                }),
            ),
            (
                "over_rho",
                per(&["plan", "res2", "res4"], |b| {
                    per(&w01, |w| st.cmp(s, "rho", b, w))
                }),
            ),
        ])
    });
    let tb = per(&B, |s| {
        obj(vec![
            ("log2_r", l2(s)),
            (
                "over_plan",
                per(&["dm1", "dp1"], |b| per(&w4, |w| st.cmp(s, "plan", b, w))),
            ),
            ("plan_over_rho", per(&w01, |w| st.cmp(s, "rho", "plan", w))),
        ])
    });
    let tk = per(&K, |s| {
        obj(vec![
            ("log2_r", l2(s)),
            (
                "p71_over_pipeline",
                per(&w4, |w| st.cmp(s, "pipeline", "pipeline71", w)),
            ),
            (
                "descent_over_pipeline",
                st.cmp(s, "pipeline", "descent", "0"),
            ),
            (
                "pipeline_over_rho",
                per(&w01, |w| st.cmp(s, "rho", "pipeline", w)),
            ),
            ("descent_over_rho", st.cmp(s, "rho", "descent", "0")),
        ])
    });
    // The ten-size residual ladder: the earlier study's res4/plan (w = 0, 0.1, 1) and this study's.
    let early = load(&earlier_r.join("summary.json"));
    let mut ladder: Vec<Json> = list(at(&early, "rows"))
        .iter()
        .map(|r| {
            obj(vec![
                ("source", Json::str("ic_prime_residual_lp_20261026")),
                ("tag", at(r, "tag").clone()),
                ("log2_r", at(r, "log2_r").clone()),
                ("res4_over_plan", at(r, "res4_over_plan").clone()),
            ])
        })
        .collect();
    for s in R {
        let res4 = at(at(at(&tr, s), "over_plan"), "res4");
        ladder.push(obj(vec![
            ("source", Json::str("ic_planner_exhaustive_20261026")),
            ("tag", Json::str(s)),
            ("log2_r", l2(s)),
            (
                "res4_over_plan",
                per(&["0", "0.1", "1"], |w| at(res4, w).clone()),
            ),
        ]));
    }
    let tables = obj(vec![
        ("R", tr),
        ("B", tb),
        ("K", tk),
        ("res4_ladder", Json::List(ladder)),
    ]);

    let all_verified = sessions.iter().all(|(_, s)| {
        at(s, "verified").as_obj().unwrap().iter().all(|(_, v)| {
            let v = list(v);
            v[0] == v[1]
        })
    });
    p.set("all_verified", Json::Bool(all_verified));

    let out = obj(vec![
        ("schema", Json::str("ic_planner_exhaustive.summary/v1")),
        ("sessions", sess.clone()),
        ("predictions", Json::Obj(p.clone())),
        ("tables", tables),
    ]);
    let path = root.join("analysis").join("summary.json");
    fs::write(&path, dumps_floats(&out, Style::Indent(1), false) + "\n")
        .unwrap_or_else(|e| die(format!("{}: {e}", path.display())));
    for (k, v) in p.iter() {
        let held = match v {
            Json::Bool(b) => *b,
            _ => at(v, "held").truthy(),
        };
        println!("{k} {}", if held { "True" } else { "False" });
    }
}
