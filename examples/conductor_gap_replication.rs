//! Frozen preparation, serial execution and analysis for the 2026-10-25 replication.
use crypto_lib::cryptanalysis::{
    curve_id,
    ecbench::{band, canonical::sha256_hex, record::Record, resources},
    ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimeInstance, PrimePoint},
    ic_framework::{
        cm_orbit::CmOrbitBase,
        stages::{FactorBaseBuilder, InstanceCtx, Params},
    },
    residual_walk::is_prime_u64,
};
use num_bigint::BigUint;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Instant, SystemTime, UNIX_EPOCH},
};
const ROOT: &str = "research/conductor_gap_replication_20261025";
const EXECUTION_GUARD_SECONDS: u64 = 7200;
type Key = (u64, u64, u64, u64, u64);
fn u(v: &Value, k: &str) -> u64 {
    v[k].as_u64()
        .unwrap_or_else(|| panic!("missing integer {k}"))
}
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    let p = p.as_ref();
    if let Some(d) = p.parent() {
        fs::create_dir_all(d).unwrap();
    }
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .unwrap();
    writeln!(f, "{}", serde_json::to_string_pretty(v).unwrap()).unwrap();
}
fn lines(p: impl AsRef<Path>) -> Vec<Value> {
    BufReader::new(File::open(p).unwrap())
        .lines()
        .map(|s| serde_json::from_str(&s.unwrap()).unwrap())
        .collect()
}
fn key(v: &Value) -> Key {
    (u(v, "p"), u(v, "a"), u(v, "b"), u(v, "gx"), u(v, "gy"))
}
fn run(exe: &str, args: &[String], log: &Path, cap: u64) {
    if let Some(d) = log.parent() {
        fs::create_dir_all(d).unwrap();
    }
    let f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(log)
        .unwrap();
    let out = Command::new("timeout")
        .arg("--signal=TERM")
        .arg(cap.to_string())
        .arg(exe)
        .args(args)
        .stdout(Stdio::from(f.try_clone().unwrap()))
        .stderr(Stdio::from(f))
        .status()
        .unwrap();
    assert!(
        out.success(),
        "{exe} failed: {out}; retained log {}",
        log.display()
    );
}
fn strings(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|s| s.to_string()).collect()
}
fn curve(v: &Value, order: u64, r: u64) -> Value {
    json!({"kind":"prime_explicit","p":v["p"],"a":v["a"],"b":v["b"],"gx":v["gx"],"gy":v["gy"],"group_order":order,"r":r})
}
fn arm(name: &str, method: &str, role: &str, params: Value) -> Value {
    json!({"name":name,"role":role,"method":{"id":method,"params":params}})
}
fn spec(
    curves: Vec<Value>,
    targets: u64,
    seed: u64,
    tseed: u64,
    arms: Vec<Value>,
    label: &str,
) -> Value {
    json!({"schema":"ecbench.spec/v1","label":label,"description":"Frozen conductor-gap replication; independent one-target solves, no shared setup; see PROTOCOL.md",
        "workloads":{"curves":curves,"targets_per_curve":targets,"target_seed":tseed,"target_kind":"planted_shared"},
        "arms":arms,"measurement":{"rounds":1,"warmup":0,"order":"shuffle","seed":seed,"isolation_required":"L0","timeout_seconds":120}})
}
fn push_spec(out: &Path, names: &mut Vec<Value>, name: &str, doc: Value, panel: &str) {
    let p = out.join("specs").join(format!("{name}.json"));
    save(&p, &doc);
    names.push(json!({"name":name,"path":p,"panel":panel}));
}
fn prepare_a(out: &Path, specs: &mut Vec<Value>, metadata: &mut Vec<Value>) {
    let mut classes = Vec::new();
    let mut seen_class = BTreeSet::new();
    for d in [11, 19, 43, 67] {
        for bits in [32, 34] {
            let class_started = Instant::now();
            let ci = classes.len();
            let cdir = out.join(format!("a-class{ci}"));
            fs::create_dir_all(&cdir).unwrap();
            let mut roots = BTreeSet::new();
            let mut verts = BTreeSet::new();
            let mut all = Vec::new();
            let mut first: Option<Value> = None;
            let mut accepted = Vec::new();
            for ks in 701..=720 {
                if accepted.len() == 3 {
                    break;
                }
                let pair_path = cdir.join(format!("pair-kernel{ks}.json"));
                run(
                    "target/release/volcano-gap",
                    &strings(&[
                        "construct",
                        "--bits",
                        &bits.to_string(),
                        "--depth",
                        "4",
                        "--d0",
                        &d.to_string(),
                        "--max-cofactor",
                        "256",
                        "--seed",
                        "2026102501",
                        "--kernel-seed",
                        &ks.to_string(),
                        "--out",
                        pair_path.to_str().unwrap(),
                    ]),
                    &cdir.join(format!("construct-kernel{ks}.log")),
                    900u64
                        .checked_sub(class_started.elapsed().as_secs())
                        .expect("class preparation deadline"),
                );
                let pair = load(&pair_path);
                if !roots.insert(u(&pair["s2_root"], "j")) {
                    continue;
                }
                let cl = &pair["class"];
                let p = u(cl, "p");
                let n = u(cl, "n");
                let order = u(cl, "order");
                let ell = u(cl, "gap_prime");
                assert!((ell as u128).pow(4) > p as u128, "gap is below p^(1/4)");
                if let Some(f) = &first {
                    assert_eq!(f["s1_root"], pair["s1_root"]);
                    assert_eq!(f["class"]["p"], cl["p"]);
                    assert_eq!(f["class"]["t"], cl["t"]);
                } else {
                    assert!(seen_class
                        .insert((cl["p"].clone().to_string(), cl["t"].clone().to_string())));
                    first = Some(pair.clone());
                }
                let vi = accepted.len() + 1;
                let sides = if vi == 1 {
                    vec![("s1", 0), ("s2", vi)]
                } else {
                    vec![("s2", vi)]
                };
                for (side, volcano) in sides {
                    let rroot = &pair[format!("{side}_root")];
                    let export = cdir.join(format!("volcano{volcano}"));
                    let g = rroot["generator"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(Value::to_string)
                        .collect::<Vec<_>>()
                        .join(",");
                    run(
                        "target/release/volcano-rho",
                        &strings(&[
                            "--curve",
                            &format!("{p},{},{}", rroot["a"], rroot["b"]),
                            "--order",
                            &order.to_string(),
                            "--generator",
                            &g,
                            "--ell",
                            "2",
                            "--seed",
                            "2026102501",
                            "--per-level",
                            "100000",
                            "--models",
                            "2",
                            "--targets",
                            "2",
                            "--ecbench",
                            export.to_str().unwrap(),
                            "--out",
                            export.to_str().unwrap(),
                        ]),
                        &cdir.join(format!("export-volcano{volcano}.log")),
                        900u64
                            .checked_sub(class_started.elapsed().as_secs())
                            .expect("class preparation deadline"),
                    );
                    let rows = lines(export.join("curves.jsonl"));
                    let mut counts = [0; 5];
                    for row in &rows {
                        let level = u(row, "level") as usize;
                        assert!(level < 5);
                        if u(row, "model") == 0 {
                            counts[level] += 1;
                            assert!(verts.insert(u(row, "j")), "overlapping volcano j");
                        }
                        let cs = curve(row, order, n);
                        metadata.push(json!({"panel":"A","class":ci,"d":d,"bits":bits,"volcano":volcano,"level":level,"model":row["model"],"vertex":row["vertex"],"curve":cs,"j":row["j"]}));
                        all.push((row.clone(), cs));
                    }
                    assert_eq!(counts, [1, 3, 6, 12, 24]);
                    if volcano > 0 {
                        let cert = cdir.join(format!("certificate{volcano}.json"));
                        let list = format!(
                            "{},{}",
                            cdir.join("volcano0/curves.jsonl").display(),
                            export.join("curves.jsonl").display()
                        );
                        run(
                            "target/release/volcano-gap",
                            &strings(&[
                                "certify",
                                "--pair",
                                pair_path.to_str().unwrap(),
                                "--seed",
                                "2026102505",
                                "--curves",
                                &list,
                                "--out",
                                cert.to_str().unwrap(),
                            ]),
                            &cdir.join(format!("certify{volcano}.log")),
                            900u64
                                .checked_sub(class_started.elapsed().as_secs())
                                .expect("class preparation deadline"),
                        );
                        assert_eq!(load(cert)["all_ok"], true);
                    }
                }
                accepted.push(json!({"kernel_seed":ks,"pair":pair_path}));
            }
            assert_eq!(accepted.len(), 3);
            for (level, t) in [192, 64, 32, 16, 8].iter().enumerate() {
                let cs = all
                    .iter()
                    .filter(|(row, _)| u(row, "model") == 0 && u(row, "level") == level as u64)
                    .map(|(_, c)| c.clone())
                    .collect();
                push_spec(
                    out,
                    specs,
                    &format!("a-c{ci}-level{level}"),
                    spec(
                        cs,
                        *t,
                        2026102502,
                        2026102501,
                        vec![arm("rho-neg", "rho.negation", "reference", json!({}))],
                        "A: balanced targets per level",
                    ),
                    "A-primary",
                );
            }
            let mut selected = BTreeSet::new();
            for m in metadata
                .iter()
                .filter(|m| m["panel"] == "A" && u(m, "class") == ci as u64 && u(m, "model") == 0)
            {
                if u(m, "level") == 0 {
                    selected.insert((u(m, "volcano"), u(m, "vertex")));
                }
            }
            for v in 0..4 {
                let min = metadata
                    .iter()
                    .filter(|m| {
                        m["panel"] == "A"
                            && u(m, "class") == ci as u64
                            && u(m, "volcano") == v
                            && u(m, "level") == 4
                            && u(m, "model") == 0
                    })
                    .map(|m| u(m, "vertex"))
                    .min()
                    .unwrap();
                selected.insert((v, min));
            }
            let ctrl: Vec<Value> = metadata
                .iter()
                .filter(|m| {
                    m["panel"] == "A"
                        && u(m, "class") == ci as u64
                        && selected.contains(&(u(m, "volcano"), u(m, "vertex")))
                })
                .map(|m| m["curve"].clone())
                .collect();
            assert_eq!(ctrl.len(), 16);
            push_spec(
                out,
                specs,
                &format!("a-c{ci}-controls"),
                spec(
                    ctrl,
                    32,
                    2026102503,
                    2026102501,
                    vec![
                        arm("rho-neg-aa", "rho.negation", "control", json!({})),
                        arm("rho-plain", "rho.plain", "baseline", json!({})),
                    ],
                    "A: real rho positive control and coordinate/A-A controls",
                ),
                "A-controls",
            );
            push_spec(
                out,
                specs,
                &format!("a-c{ci}-bsgs"),
                spec(
                    all.iter()
                        .filter(|(row, _)| u(row, "model") == 0)
                        .map(|(_, c)| c.clone())
                        .collect(),
                    2,
                    2026102502,
                    2026102501,
                    vec![arm("bsgs-neg", "bsgs.negation", "baseline", json!({}))],
                    "A: exact transported-target BSGS control",
                ),
                "A-bsgs",
            );
            classes.push(json!({"class":ci,"d":d,"bits":bits,"pair":first,"accepted":accepted}));
        }
    }
    save(out.join("a-classes.json"), &json!(classes));
}
fn mm(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}
fn pw(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut v = 1;
    while e > 0 {
        if e & 1 != 0 {
            v = mm(v, a, m);
        }
        a = mm(a, a, m);
        e >>= 1;
    }
    v
}
fn kernel_degree(t: u64, n: u64) -> u64 {
    let x = mm(t % n, n.div_ceil(2), n);
    let mut ord = n - 1;
    let mut q = n - 1;
    let mut f = 2;
    while f * f <= q {
        if q.is_multiple_of(f) {
            while q.is_multiple_of(f) {
                q /= f;
            }
            while ord.is_multiple_of(f) && pw(x, ord / f, n) == 1 {
                ord /= f;
            }
        }
        f += 1;
    }
    if q > 1 {
        while ord.is_multiple_of(q) && pw(x, ord / q, n) == 1 {
            ord /= q;
        }
    }
    if ord.is_multiple_of(2) {
        ord / 2
    } else {
        ord
    }
}
fn j_value(c: &PrimeCurve) -> u64 {
    let p = c.p;
    let a3 = mm(mm(c.a, c.a, p), c.a, p);
    let den = (mm(4, a3, p) + mm(27, mm(c.b, c.b, p), p)) % p;
    mm(mm(1728 % p, mm(4, a3, p), p), pw(den, p - 2, p), p)
}
fn witness(c: &PrimeCurve, r: u64) -> Option<PrimePoint> {
    if (mm(4, mm(mm(c.a, c.a, c.p), c.a, c.p), c.p) + mm(27, mm(c.b, c.b, c.p), c.p))
        .is_multiple_of(c.p)
    {
        return None;
    }
    for x in 1..=8 {
        if let Some(y) = c.sqrt(c.rhs(x)) {
            if y > 0 {
                let pt = PrimePoint::affine(x, y.min(c.p - y));
                let mut ops = GroupOps::default();
                return c.mul(&mut ops, pt, r).infinity.then_some(pt);
            }
        }
    }
    None
}
fn explicit(c: &PrimeCurve, r: u64, g: PrimePoint) -> Value {
    json!({"kind":"prime_explicit","p":c.p,"a":c.a,"b":c.b,"group_order":r,"r":r,"gx":g.x,"gy":g.y})
}
fn prepare_b(out: &Path, specs: &mut Vec<Value>, metadata: &mut Vec<Value>) {
    let mut rng = StdRng::seed_from_u64(2026102510);
    let mut classes = Vec::new();
    for d in [19u64, 43] {
        let mut found = 0;
        let mut class_started = Instant::now();
        'prime: for ell in 8191u64..65536 {
            if !is_prime_u64(ell) {
                continue;
            }
            for t in (1u64..=511).step_by(2) {
                assert!(
                    class_started.elapsed().as_secs() < 900,
                    "class preparation deadline"
                );
                let p = (t * t + d * ell * ell) / 4;
                let r = p + 1 - t;
                // Pre-data amendment 2026-10-25: exclude anomalous curves and
                // select the canonical degree-q CM kernel on the quadratic twist.
                let q = (d + 1) / 4;
                if t == 1
                    || !(t + 2 * q - ell % (2 * q) + 2).is_multiple_of(2 * q)
                    || !is_prime_u64(p)
                    || !is_prime_u64(r)
                    || (r as u128).pow(2) <= 16 * p as u128
                {
                    continue;
                }
                let kd = kernel_degree(t, ell);
                if 8 * kd < ell - 1 {
                    continue;
                }
                let j = (p - if d == 19 { 884736 % p } else { 884736000 % p }) % p;
                let k = (1728 + p - j) % p;
                let base = PrimeCurve {
                    p,
                    a: mm(mm(3, j, p), k, p),
                    b: mm(mm(2, j, p), mm(k, k, p), p),
                };
                let mut nr = 2;
                while pw(nr, (p - 1) / 2, p) != p - 1 {
                    nr += 1;
                }
                let twist = PrimeCurve {
                    p,
                    a: mm(base.a, mm(nr, nr, p), p),
                    b: mm(base.b, mm(mm(nr, nr, p), nr, p), p),
                };
                let crater = if let Some(g) = witness(&base, r) {
                    (base, g)
                } else {
                    let g = witness(&twist, r).expect("crater order");
                    (twist, g)
                };
                assert_eq!(j_value(&crater.0), j);
                let ci = classes.len();
                let mut models = vec![(crater.0, crater.1, 0u64)];
                let mut js = BTreeSet::from([j]);
                let mut tries = Vec::new();
                for floor in 1..=3 {
                    let mut accepted = None;
                    for attempt in 1..=64 * d * ell + 65536 {
                        if attempt % 256 == 0 {
                            assert!(
                                class_started.elapsed().as_secs() < 900,
                                "class preparation deadline"
                            );
                        }
                        let c = PrimeCurve {
                            p,
                            a: rng.gen_range(0..p),
                            b: rng.gen_range(1..p),
                        };
                        if j_value(&c) == j || js.contains(&j_value(&c)) {
                            continue;
                        }
                        if let Some(g) = witness(&c, r) {
                            accepted = Some((c, g));
                            tries.push(attempt);
                            break;
                        }
                    }
                    let (c, g) = accepted.expect("bounded floor search");
                    assert!(js.insert(j_value(&c)));
                    models.push((c, g, floor));
                }
                let mut cs = Vec::new();
                let mut craters = Vec::new();
                for (c, g, volcano) in models {
                    for model in 0..2 {
                        let z = if model == 0 { 1 } else { 7 };
                        let z2 = mm(z, z, p);
                        let z3 = mm(z2, z, p);
                        let cm = PrimeCurve {
                            p,
                            a: mm(c.a, mm(z2, z2, p), p),
                            b: mm(c.b, mm(z3, z3, p), p),
                        };
                        let gm = PrimePoint::affine(mm(g.x, z2, p), mm(g.y, z3, p));
                        assert!(cm.is_on_curve(gm));
                        assert!(cm.mul(&mut GroupOps::default(), gm, r).infinity);
                        let spec = explicit(&cm, r, gm);
                        metadata.push(json!({"panel":"B","class":ci,"d":d,"volcano":volcano,"level":if volcano==0{0}else{1},"model":model,"vertex":volcano,"curve":spec,"j":j_value(&cm)}));
                        cs.push(spec.clone());
                        if volcano == 0 {
                            craters.push(spec);
                        }
                    }
                }
                let params = |mode: &str| json!({"factor_base":format!("cm-scalar-orbit:columns=4,length=32,abs_d={d},mode={mode}"),"oracle":"mitm:negation_folded=1","linalg":"incremental-gauss","targets":"walk","max_trials":"1000000"});
                push_spec(
                    out,
                    specs,
                    &format!("b-c{ci}"),
                    spec(
                        cs,
                        16,
                        2026102512,
                        2026102511,
                        vec![
                            arm("rho-neg", "rho.negation", "reference", json!({})),
                            arm("ic-scalar", "ic.pipeline", "baseline", params("scalar")),
                            arm(
                                "ic-recurrence",
                                "ic.pipeline",
                                "candidate",
                                params("recurrence"),
                            ),
                        ],
                        "B: harder-kernel-field CM collector",
                    ),
                    "B",
                );
                // Separate spec preserves exactly the same workload/algorithm seeds.
                push_spec(
                    out,
                    specs,
                    &format!("b-c{ci}-velu"),
                    spec(
                        craters,
                        16,
                        2026102512,
                        2026102511,
                        vec![arm("ic-velu", "ic.pipeline", "candidate", params("velu"))],
                        "B: crater CM-map control",
                    ),
                    "B-velu",
                );
                classes.push(json!({"class":ci,"d":d,"p":p,"t":t,"r":r,"gap_prime":ell,"kernel_abscissa_degree":kd,"floor_search_candidates":tries}));
                found += 1;
                class_started = Instant::now();
                if found == 2 {
                    break 'prime;
                }
                break;
            }
        }
        assert_eq!(found, 2);
    }
    save(out.join("b-classes.json"), &json!(classes));
}
fn prepare(out: &Path, panel: &str) {
    assert!(!out.exists(), "fresh preparation directory required");
    fs::create_dir_all(out).unwrap();
    let mut specs = Vec::new();
    let mut metadata = Vec::new();
    if panel == "A" {
        prepare_a(out, &mut specs, &mut metadata);
    } else {
        assert_eq!(panel, "B");
        prepare_b(out, &mut specs, &mut metadata);
    }
    save(out.join("curves.json"), &json!(metadata));
    save(
        out.join("manifest.json"),
        &json!({"schema":"conductor-gap-replication/v1","panel":panel,"specs":specs}),
    );
}
fn identities(out: &Path) {
    let metadata = load(out.join("curves.json"));
    let source = out.join("curves.json").display().to_string();
    let mut entries = Vec::new();
    for m in metadata.as_array().unwrap() {
        let c = &m["curve"];
        let (p, a, b, order, r, gx, gy) = (
            u(c, "p"),
            u(c, "a"),
            u(c, "b"),
            u(c, "group_order"),
            u(c, "r"),
            u(c, "gx"),
            u(c, "gy"),
        );
        let id = curve_id::prime(
            &BigUint::from(p),
            &BigUint::from(a),
            &BigUint::from(b),
            &BigUint::from(order),
        )
        .unwrap();
        let field = json!({"characteristic":p,"degree":1,"representation":"prime","element_encoding":"hex integer modulo the characteristic"});
        let representation = json!({"model":"short Weierstrass","coefficients":[0,0,0,a,b],"subgroup_order":r.to_string(),"cofactor":order/r,"generator":[format!("0x{gx:x}"),format!("0x{gy:x}")],"target_group":"prime-order subgroup"});
        let preimage = json!({"field":field,"curve":representation});
        let digest = sha256_hex(
            crypto_lib::cryptanalysis::ecbench::canonical::canonical(&preimage)
                .unwrap()
                .as_bytes(),
        );
        let fhash = sha256_hex(
            crypto_lib::cryptanalysis::ecbench::canonical::canonical(&field)
                .unwrap()
                .as_bytes(),
        );
        entries.push(json!({"slug":id.slug,"icv1":id.icv1,"model_json":id.model_json,"family":"prime",
            "params":{"p":p.to_string(),"a":a.to_string(),"b":b.to_string()},"trace":p as i64+1-order as i64,"order":order.to_string(),"j":j_value(&PrimeCurve{p,a,b}).to_string(),"end":"unk",
            "aliases":[],"standard_names":[],"sources":[source],
            "representations":[{"ec1":format!("EC1P{}Cfph{}",64-p.leading_zeros(),&digest[..12]),"curve_uid":format!("urn:ec-record:1:sha256:{digest}"),"field_sha256":fhash,"field":field,"curve":representation,"sources":[source]}],
            "normalized_aliases":[curve_id::normalise(&id.slug),curve_id::normalise(&id.icv1)]}));
    }
    save(out.join("identities.json"), &json!(entries));
}
fn check_cm_bases(out: &Path) {
    let metadata = load(out.join("curves.json"));
    let mut checked = Vec::new();
    for m in metadata.as_array().unwrap() {
        let c = &m["curve"];
        let inst = PrimeInstance {
            name: "frozen CM orbit control".into(),
            curve: PrimeCurve {
                p: u(c, "p"),
                a: u(c, "a"),
                b: u(c, "b"),
            },
            group_order: u(c, "r"),
            r: u(c, "r"),
            cofactor: 1,
            generator: (u(c, "gx"), u(c, "gy")),
        };
        let ctx = InstanceCtx {
            group: &inst.curve,
            generator: inst.generator_point(),
            target: PrimePoint::INFINITY,
            r: inst.r,
            cofactor: 1,
            group_order: inst.r,
            name: inst.name.clone(),
            field_degree: None,
        };
        let mut p = Params::default();
        p.set("columns", "4")
            .set("length", "32")
            .set("abs_d", u(m, "d").to_string())
            .set("mode", "scalar");
        let scalar = CmOrbitBase { instance: &inst }
            .build(&ctx, &p, &mut GroupOps::default())
            .unwrap();
        p.set("mode", "recurrence");
        let recurrence = CmOrbitBase { instance: &inst }
            .build(&ctx, &p, &mut GroupOps::default())
            .unwrap();
        assert_eq!(scalar.points, recurrence.points);
        assert_eq!(scalar.col_of, recurrence.col_of);
        assert_eq!(scalar.coef_of, recurrence.coef_of);
        if u(m, "volcano") == 0 {
            p.set("mode", "velu");
            let map = CmOrbitBase { instance: &inst }
                .build(&ctx, &p, &mut GroupOps::default())
                .unwrap();
            assert_eq!(scalar.points, map.points);
            assert_eq!(scalar.col_of, map.col_of);
            assert_eq!(scalar.coef_of, map.coef_of);
        }
        checked.push(json!({"curve":c,"d":m["d"],"volcano":m["volcano"],"model":m["model"],"abscissae":128,"points_columns_coefficients_identical":true,"velu_checked":u(m,"volcano")==0}));
    }
    save(out.join("base-equality.json"), &json!({"checked":checked}));
}
fn evidence_bytes(path: &Path) -> u64 {
    fs::read_dir(path)
        .unwrap()
        .map(|entry| {
            let e = entry.unwrap();
            let ty = e.file_type().unwrap();
            if ty.is_dir() {
                evidence_bytes(&e.path())
            } else if ty.is_file() {
                e.metadata().unwrap().len()
            } else {
                0
            }
        })
        .sum()
}
fn execute(out: &Path, audit: bool, selection: &str) {
    let manifest = load(out.join("manifest.json"));
    let clock = || {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    };
    let receipt = Path::new(ROOT).join("execution-start.json");
    if !receipt.exists() {
        save(
            &receipt,
            &json!({"unix_seconds":clock(),"guard_seconds":EXECUTION_GUARD_SECONDS}),
        );
    }
    let first = u(&load(&receipt), "unix_seconds");
    for s in manifest["specs"].as_array().unwrap() {
        let control = s["panel"] == "A-controls";
        if (selection == "controls" && !control) || (selection == "primary" && control) {
            continue;
        }
        assert!(
            clock() - first < EXECUTION_GUARD_SECONDS,
            "120-minute admission guard"
        );
        assert!(
            evidence_bytes(Path::new(ROOT)) < 8 * 1024 * 1024 * 1024,
            "8 GiB evidence guard"
        );
        let name = s["name"].as_str().unwrap();
        let session = Path::new(ROOT).join("sessions").join(name);
        let (args, log) = if audit {
            (
                strings(&[
                    "verify",
                    "--dir",
                    session.to_str().unwrap(),
                    "--replay-all",
                    "--exit-code",
                    "--out",
                    &format!("{}.audit.json", session.display()),
                ]),
                PathBuf::from(format!("{}.audit.log", session.display())),
            )
        } else {
            (
                strings(&[
                    "run",
                    "--wait",
                    "--spec",
                    s["path"].as_str().unwrap(),
                    "--out",
                    session.to_str().unwrap(),
                    "--cpus",
                    "none",
                    "--allow-busy",
                    "--quiet",
                ]),
                PathBuf::from(format!("{}.run.log", session.display())),
            )
        };
        run(
            "target/release/ecbench",
            &args,
            &log,
            EXECUTION_GUARD_SECONDS - (clock() - first),
        );
        if !audit {
            for line in BufReader::new(File::open(session.join("records.jsonl")).unwrap()).lines() {
                let r: Value = serde_json::from_str(&line.unwrap()).unwrap();
                assert!(
                    r["outcome"]["status"] != "wrong_answer"
                        && r["outcome"]["status"] != "crashed"
                        && r["outcome"]["status"] != "error"
                        && !(r["outcome"]["recovered"].is_string()
                            && r["outcome"]["matches_target"] == false),
                    "wrong answer or infrastructure failure; stop before admitting another session"
                );
            }
        }
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(Path::new(ROOT).join("execution.jsonl"))
            .unwrap();
        writeln!(
            f,
            "{}",
            json!({"unix_seconds":stamp,"name":name,"audit":audit,"status":"command_completed"})
        )
        .unwrap();
        println!("{name}: {}", if audit { "replayed" } else { "recorded" });
    }
}
fn gate_a(out: &Path) {
    let manifest = load(out.join("manifest.json"));
    let mut costs: BTreeMap<(u64, String), Vec<f64>> = BTreeMap::new();
    let mut count = 0;
    for s in manifest["specs"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| s["panel"] == "A-controls")
    {
        let name = s["name"].as_str().unwrap();
        let ci = name
            .strip_prefix("a-c")
            .unwrap()
            .split('-')
            .next()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let path = Path::new(ROOT)
            .join("sessions")
            .join(name)
            .join("records.jsonl");
        for r in lines(path) {
            assert_eq!(
                r["outcome"]["status"], "verified",
                "control must recover correctly"
            );
            let arm = r["arm"].as_str().unwrap().to_string();
            let value = r["cost"]["s"].as_f64().expect("counted S");
            costs.entry((ci, arm)).or_default().push(value);
            count += 1;
        }
    }
    assert_eq!(count, 8192);
    let logs = (0..8)
        .map(|ci| {
            let mean = |arm: &str| {
                let xs = &costs[&(ci, arm.into())];
                xs.iter().sum::<f64>() / xs.len() as f64
            };
            (mean("rho-plain") / mean("rho-neg-aa")).ln()
        })
        .collect::<Vec<_>>();
    let result = interval(&logs, 2026102515);
    let passed = result["ci95"][0].as_f64().unwrap() > 1.;
    save(
        Path::new(ROOT).join("positive-control-gate.json"),
        &json!({"records":count,"rho_plain_over_negation":result,"passed":passed}),
    );
    assert!(passed, "positive control failed; primary must not start");
}
fn percentile(xs: &[f64], q: f64) -> f64 {
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);
    v[((v.len() - 1) as f64 * q).round() as usize]
}
fn interval(logs: &[f64], seed: u64) -> Value {
    assert!(!logs.is_empty());
    let mut rng = StdRng::seed_from_u64(seed);
    let bs = (0..10000)
        .map(|_| {
            (0..logs.len())
                .map(|_| logs[rng.gen_range(0..logs.len())])
                .sum::<f64>()
                / logs.len() as f64
        })
        .map(f64::exp)
        .collect::<Vec<_>>();
    json!({"ratio":(logs.iter().sum::<f64>()/logs.len()as f64).exp(),
        "ci90":[percentile(&bs,0.05),percentile(&bs,0.95)],"ci95":[percentile(&bs,0.025),percentile(&bs,0.975)],
        "ci98":[percentile(&bs,0.01),percentile(&bs,0.99)],"ci99":[percentile(&bs,0.005),percentile(&bs,0.995)],
        "class_ratios":logs.iter().map(|x|x.exp()).collect::<Vec<_>>()})
}
fn analyze(out: &Path) {
    let manifest = load(out.join("manifest.json"));
    let meta = load(out.join("curves.json"));
    let map: BTreeMap<Key, &Value> = meta
        .as_array()
        .unwrap()
        .iter()
        .map(|m| (key(&m["curve"]), m))
        .collect();
    let mut cells: BTreeMap<(u64, u64, u64, String), Vec<f64>> = BTreeMap::new();
    let mut controls: BTreeMap<(u64, String, u64), Vec<f64>> = BTreeMap::new();
    let mut main_workloads: BTreeMap<(u64, String), f64> = BTreeMap::new();
    let mut aa_workloads: BTreeMap<(u64, String), f64> = BTreeMap::new();
    let mut status = BTreeMap::<String, u64>::new();
    let mut bsgs: BTreeMap<(u64, u64), BTreeSet<u64>> = BTreeMap::new();
    let mut counts = 0;
    let result_dir = Path::new(ROOT).join("analysis");
    fs::create_dir_all(&result_dir).unwrap();
    let panel = manifest["panel"].as_str().unwrap();
    let mut csv = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(result_dir.join(format!("records-{panel}.csv")))
        .unwrap();
    writeln!(csv,"class,volcano,level,model,arm,p,a,b,r,gx,gy,qx,qy,recovered,status,s,total_gae,target_index").unwrap();
    let mut source_hashes = Vec::new();
    for s in manifest["specs"].as_array().unwrap() {
        let name = s["name"].as_str().unwrap();
        let session = Path::new(ROOT).join("sessions").join(name);
        let raw = session.join("records.jsonl");
        source_hashes.push(json!({"path":raw,"sha256":sha256_hex(&fs::read(&raw).unwrap())}));
        for line in BufReader::new(File::open(raw).unwrap()).lines() {
            let rec: Value = serde_json::from_str(&line.unwrap()).unwrap();
            if rec["warmup"] == true {
                continue;
            }
            let m = map[&key(&rec["workload"]["curve_spec"])];
            let arm = rec["arm"].as_str().unwrap();
            let st = rec["outcome"]["status"].as_str().unwrap();
            *status.entry(st.into()).or_default() += 1;
            counts += 1;
            let cs = &m["curve"];
            let hx = |v: &Value| {
                u64::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
            };
            let recovered = rec["outcome"]["recovered"].as_str().unwrap_or("");
            writeln!(
                csv,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                m["class"],
                m["volcano"],
                m["level"],
                m["model"],
                arm,
                cs["p"],
                cs["a"],
                cs["b"],
                cs["r"],
                cs["gx"],
                cs["gy"],
                hx(&rec["workload"]["target"][0]),
                hx(&rec["workload"]["target"][1]),
                recovered,
                st,
                rec["cost"]["s"],
                rec["cost"]["total_gae"],
                rec["workload"]["target_index"]
            )
            .unwrap();
            if st != "verified" {
                continue;
            }
            let sv = rec["cost"]["s"].as_f64().unwrap();
            let wid = rec["workload"]["workload_id"].as_str().unwrap().to_string();
            if panel == "A" && s["panel"] == "A-primary" {
                main_workloads.insert((u(m, "class"), wid.clone()), sv);
                cells
                    .entry((u(m, "class"), u(m, "level"), u(m, "volcano"), arm.into()))
                    .or_default()
                    .push(sv);
            }
            if panel == "A" && s["panel"] == "A-controls" {
                controls
                    .entry((u(m, "class"), arm.into(), u(m, "model")))
                    .or_default()
                    .push(sv);
                if arm == "rho-neg-aa" && u(m, "model") == 0 {
                    aa_workloads.insert((u(m, "class"), wid), sv);
                }
            }
            if panel == "A" && arm == "bsgs-neg" {
                bsgs.entry((u(m, "class"), u(&rec["workload"], "target_index")))
                    .or_default()
                    .insert(sv.to_bits());
            }
            if panel == "B" {
                cells
                    .entry((u(m, "class"), u(m, "volcano"), u(m, "model"), arm.into()))
                    .or_default()
                    .push(sv);
            }
        }
    }
    let mut result = json!({"panel":panel,"records":counts,"statuses":status,"source_files":source_hashes,"complete":status.len()==1&&status.contains_key("verified")});
    if panel == "A" {
        assert_eq!(counts, 41856, "unexpected planned record count");
        assert!(
            status.len() == 1 && status.contains_key("verified"),
            "failed primary/control attempt prevents inference"
        );
        assert!(bsgs.values().all(|s| s.len() == 1), "BSGS control failed");
        let mean = |xs: &Vec<f64>| xs.iter().sum::<f64>() / xs.len() as f64;
        let mut positive = Vec::new();
        let mut coordinate = Vec::new();
        let mut aa = Vec::new();
        let mut aa_pairs = 0;
        for ci in 0..8 {
            let n0 = mean(&controls[&(ci, "rho-neg-aa".into(), 0)]);
            let n1 = mean(&controls[&(ci, "rho-neg-aa".into(), 1)]);
            let p0 = mean(&controls[&(ci, "rho-plain".into(), 0)]);
            let p1 = mean(&controls[&(ci, "rho-plain".into(), 1)]);
            positive.push(((p0 + p1) / (n0 + n1)).ln());
            coordinate.push((n1 / n0).ln());
            let pairs: Vec<_> = main_workloads
                .iter()
                .filter(|((c, _), _)| *c == ci)
                .filter_map(|(k, v)| aa_workloads.get(k).map(|a| (*v, *a)))
                .collect();
            assert_eq!(pairs.len(), 160);
            aa_pairs += pairs.len();
            aa.push(
                (pairs.iter().map(|(_, a)| a).sum::<f64>()
                    / pairs.iter().map(|(a, _)| a).sum::<f64>())
                .ln(),
            );
        }
        let pos = interval(&positive, 2026102515);
        let aa = interval(&aa, 2026102516);
        let controls_pass = pos["ci95"][0].as_f64().unwrap() > 1.0
            && aa["ci95"][0].as_f64().unwrap() <= 1.0
            && aa["ci95"][1].as_f64().unwrap() >= 1.0;
        result["controls"] = json!({"rho_plain_over_negation":pos,"aa":aa,"matched_aa_pairs":aa_pairs,"coordinate_model1_over_model0":interval(&coordinate,2026102517),"passed":controls_pass});
        let mut level_results = Vec::new();
        let mut overall = vec![0.; 8];
        for level in 0..5 {
            let mut logs = Vec::new();
            let mut detailed = Vec::new();
            for ci in 0..8 {
                let mean = |vi| {
                    let vs = &cells[&(ci, level, vi, "rho-neg".into())];
                    assert_eq!(vs.len(), 192);
                    vs.iter().sum::<f64>() / vs.len() as f64
                };
                let a = mean(0);
                let floors = [mean(1), mean(2), mean(3)];
                let ratio = floors.iter().sum::<f64>() / 3. / a;
                logs.push(ratio.ln());
                overall[ci as usize] += ratio.ln() / 5.;
                detailed.push(json!({"class":ci,"s1_mean_s":a,"s2_means_s":floors,"individual_ratios":floors.iter().map(|x|x/a).collect::<Vec<_>>()}));
            }
            let mut row = interval(&logs, 2026102504 + level);
            row["level"] = json!(level);
            row["class_means"] = json!(detailed);
            row["equivalent_5pct_nominal"] = json!(
                controls_pass
                    && row["ci90"][0].as_f64().unwrap() >= 0.95
                    && row["ci90"][1].as_f64().unwrap() <= 1.05
            );
            row["equivalent_5pct_simultaneous"] = json!(
                controls_pass
                    && row["ci98"][0].as_f64().unwrap() >= 0.95
                    && row["ci98"][1].as_f64().unwrap() <= 1.05
            );
            level_results.push(row);
        }
        result["levels"] = json!(level_results);
        result["equal_level_secondary"] = interval(&overall, 2026102509);
        result["bsgs_exactness"] = json!(true);
    } else {
        assert_eq!(counts, 1664, "unexpected B record count");
        let rows=cells.iter().map(|((cl,v,m,arm),ss)|json!({"class":cl,"volcano":v,"model":m,"arm":arm,"verified":ss.len(),"mean_s":ss.iter().sum::<f64>()/ss.len()as f64})).collect::<Vec<_>>();
        result["cells"] = json!(rows);
        result["interpretation"]=json!("Counted lower-bound and resource diagnostics; inspect all outcomes and common pricing bands before ranking.");
    }
    save(result_dir.join(format!("panel-{panel}.json")), &result);
    println!(
        "{}",
        serde_json::to_string_pretty(&result["statuses"]).unwrap()
    );
}
fn diagnostics(out: &Path) {
    let manifest = load(out.join("manifest.json"));
    let mut sessions = Vec::new();
    for s in manifest["specs"].as_array().unwrap() {
        let name = s["name"].as_str().unwrap();
        let session = Path::new(ROOT).join("sessions").join(name);
        let records: Vec<Record> =
            BufReader::new(File::open(session.join("records.jsonl")).unwrap())
                .lines()
                .map(|l| serde_json::from_str(&l.unwrap()).unwrap())
                .collect();
        let priced = band::band(&records, &[0., 0.1, 1.]).unwrap();
        let resource = resources::session(&session).unwrap();
        let mut phase_native = BTreeMap::<String, u64>::new();
        let mut top_counters = BTreeMap::<String, u64>::new();
        let mut unpriced = BTreeMap::<String, Option<u64>>::new();
        let mut wall = 0u64;
        let mut process_wall = 0u64;
        for r in &records {
            wall += r.time.solve_wall_ns.unwrap_or(0);
            process_wall += r.time.process_wall_ns;
            for (k, v) in &r.counters {
                *top_counters.entry(format!("{}:{k}", r.arm)).or_default() += v;
            }
            for p in &r.phases {
                for (k, v) in &p.native {
                    *phase_native
                        .entry(format!("{}:{}:{k}", r.arm, p.name))
                        .or_default() += v;
                }
            }
            for (k, v) in band::unpriced_counts(r) {
                let total = unpriced.entry(format!("{}:{k}", r.arm)).or_insert(Some(0));
                *total = total.and_then(|x| v.map(|y| x + y));
            }
        }
        sessions.push(
            json!({"name":name,"session":session,"records":records.len(),
            "band":priced,"resources":resource,"phase_native_totals":phase_native,
            "top_counter_totals":top_counters,"unpriced_counter_totals":unpriced,
            "descriptive_l0_solve_wall_ns":wall,"descriptive_l0_process_wall_ns":process_wall}),
        );
    }
    let panel = manifest["panel"].as_str().unwrap_or("calibration");
    save(
        Path::new(ROOT)
            .join("analysis")
            .join(format!("diagnostics-{panel}.json")),
        &json!({"panel":panel,"sessions":sessions}),
    );
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    assert!(args.len() >= 2, "prepare-a|prepare-b|run|audit|analyze OUT");
    let out = Path::new(&args[1]);
    match args[0].as_str() {
        "prepare-a" => prepare(out, "A"),
        "prepare-b" => prepare(out, "B"),
        "identities" => identities(out),
        "check-bases" => check_cm_bases(out),
        "run" => execute(out, false, "all"),
        "audit" => execute(out, true, "all"),
        "run-controls" => execute(out, false, "controls"),
        "audit-controls" => execute(out, true, "controls"),
        "run-primary" => execute(out, false, "primary"),
        "audit-primary" => execute(out, true, "primary"),
        "gate-a" => gate_a(out),
        "analyze" => analyze(out),
        "diagnostics" => diagnostics(out),
        _ => panic!("unknown mode"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kernel_degree_accounts_for_sign() {
        assert_eq!(kernel_degree(2, 7), 1);
        assert_eq!(kernel_degree(4, 7), 3);
    }
    #[test]
    fn bootstrap_preserves_a_fixed_class_ratio() {
        let x = interval(&[1.02_f64.ln(); 8], 42);
        for k in ["ci90", "ci95", "ci98", "ci99"] {
            assert!((x[k][0].as_f64().unwrap() - 1.02).abs() < 1e-12);
            assert!((x[k][1].as_f64().unwrap() - 1.02).abs() < 1e-12);
        }
    }
}
