use super::*;
use std::collections::HashSet;
fn odd_modulus(p: u64, n: usize) -> u64 {
    if n == 1 {
        return 0;
    }
    'candidate: for tail in 1..p.pow(n as u32) {
        if tail % p == 0 {
            continue;
        }
        let mut coeff = vec![0; n + 1];
        let mut t = tail;
        for x in &mut coeff[..n] {
            *x = t % p;
            t /= p;
        }
        coeff[n] = 1;
        for d in 1..=n / 2 {
            for tail in 0..p.pow(d as u32) {
                let mut div = vec![0; d + 1];
                let mut t = tail;
                for x in &mut div[..d] {
                    *x = t % p;
                    t /= p;
                }
                div[d] = 1;
                let mut rr = coeff.clone();
                for i in (d..=n).rev() {
                    let c = rr[i];
                    for j in 0..=d {
                        rr[i - d + j] = (rr[i - d + j] + p - c * div[j] % p) % p;
                    }
                }
                if rr[..d].iter().all(|x| *x == 0) {
                    continue 'candidate;
                }
            }
        }
        return tail;
    }
    panic!("no irreducible modulus")
}
fn points(e: &Elliptic) -> Vec<Point> {
    let mut out = vec![Point::O];
    for x in 0..e.f.size() {
        out.extend(e.lift(x));
    }
    out
}
fn instance(e: &Elliptic, pts: &[Point], family: &str, parameter: u64) -> Option<StudyInstance> {
    let order = pts.len() as u64;
    let &(r, exponent) = factor(order).last()?;
    if r < 5 || exponent != 1 || (order / r).is_multiple_of(r) {
        return None;
    }
    let h = order / r;
    let generator = pts.iter().map(|&p| e.mul(p, h)).find(|p| !p.inf)?;
    let i = StudyInstance {
        curve: Elliptic {
            f: e.f.fresh(),
            c: e.c,
            binary: e.binary,
        },
        order,
        r,
        h,
        generator,
        family: family.into(),
        parameter,
        evidence: json!({"kind":"exhaustive_point_count","order":order}),
    };
    i.validate().ok()?;
    Some(i)
}
fn sumset(e: &Elliptic, base: &[Point], n: usize) -> HashSet<Point> {
    let mut sums = HashSet::from([Point::O]);
    for _ in 0..n {
        let mut next = HashSet::new();
        for &a in &sums {
            for &b in base {
                next.insert(e.add(a, b));
            }
        }
        sums = next;
    }
    sums
}
fn random_base(e: &Elliptic, base: &[Point], pts: &[Point], rng: &mut Rng) -> Vec<Point> {
    let mut out: Vec<_> = base.iter().copied().filter(|&p| e.neg(p) == p).collect();
    let needed = (base.len() - out.len()) / 2;
    let choices: Vec<_> = pts
        .iter()
        .copied()
        .filter(|&p| !p.inf && p < e.neg(p))
        .collect();
    let mut seen = HashSet::new();
    while seen.len() < needed {
        let p = choices[(rng.next() % choices.len() as u64) as usize];
        if seen.insert(p) {
            out.push(p);
            out.push(e.neg(p));
        }
    }
    out
}
fn trace(f: &Field, mut a: u64) -> u64 {
    let mut t = 0;
    for _ in 0..f.n {
        t ^= a;
        a = f.sq(a);
    }
    assert!(t <= 1);
    t
}
fn row(
    e: &Elliptic,
    base: &[Point],
    pts: &[Point],
    n: usize,
    seed: u64,
    meta: Value,
    inst: Option<&StudyInstance>,
) -> Value {
    let sums = sumset(e, base, n);
    let mut rng = Rng(seed);
    let controls: Vec<_> = (0..32)
        .map(|_| sumset(e, &random_base(e, base, pts, &mut rng), n).len())
        .collect();
    let mut out = json!({"meta":meta,"field":{"p":e.f.p,"n":e.f.n,"modulus":e.f.modulus},"coefficients":e.c,"order":pts.len(),"base_signed_points":base.len(),"summands":n,"reachable":sums.len(),"random_reachable":controls});
    if let Some(i) = inst {
        let mut g = Point::O;
        let mut reachable = 0;
        for _ in 0..i.r {
            if sums.contains(&g) {
                reachable += 1;
            }
            g = e.add(g, i.generator);
        }
        assert!(g.inf);
        out["subgroup_order"] = json!(i.r);
        out["subgroup_reachable"] = json!(reachable);
    }
    if e.binary {
        let doubled: HashSet<_> = pts.iter().map(|&p| e.add(p, p)).collect();
        let kernel: HashSet<_> = pts
            .iter()
            .copied()
            .filter(|p| p.inf || trace(&e.f, p.x ^ e.c[2]) == 0)
            .collect();
        assert_eq!(doubled, kernel);
        let chi = trace(&e.f, e.c[2]);
        assert!(base.iter().all(|p| trace(&e.f, p.x ^ e.c[2]) == chi));
        assert!(sums.iter().all(|p| kernel.contains(p)));
        out["trace_kernel_size"] = json!(kernel.len());
        out["factor_base_coset"] = json!(chi);
        out["doubled_equals_trace_kernel"] = json!(true);
        out["global_poisson_reference"] = json!(1.0 - (-0.5f64).exp());
    }
    out
}
fn decomp_spec(i: &StudyInstance, label: &str, cap: u64, random: bool) -> Value {
    let oracle = if i.curve.binary {
        if random {
            "subtract"
        } else {
            "linearized"
        }
    } else {
        "semaev"
    };
    let mut arms = vec![
        arm(
            "rho_plain",
            "rho.three_leads_plain_v1",
            json!({"trial_cap":cap.to_string()}),
        ),
        arm(
            "rho_neg",
            "rho.three_leads_negation_v1",
            json!({"trial_cap":cap.to_string()}),
        ),
        arm(
            "structured_ic",
            "ic.three_leads_decomposition_v1",
            json!({"trial_cap":cap.to_string(),"oracle":oracle,"random_base":"0","base_seed":"20261018"}),
        ),
    ];
    if random {
        arms.push(arm("random_ic","ic.three_leads_decomposition_v1",json!({"trial_cap":cap.to_string(),"oracle":"semaev","random_base":"1","base_seed":"20261018"})));
    }
    spec(label, vec![curve(i)], 8, arms)
}
pub fn generate(dir: &str) {
    let dir = Path::new(dir);
    fs::create_dir(dir).unwrap();
    let mut rows = BufWriter::new(fs::File::create(dir.join("coverage.jsonl")).unwrap());
    let mut fixtures = vec![];
    let mut rejects = vec![];
    for m in 3..=9 {
        let q = 1 << m;
        let mut f = Field::new(2, 2 * m, binary_modulus(2 * m));
        f.diagnostic_tables();
        let xs = subfield_elements(&f, q);
        let mut found = 0;
        let mut candidate = 0;
        while found < 3 {
            let a = (3 + 17 * candidate) % f.size();
            let b = (2 + candidate) % f.size();
            candidate += 1;
            if b == 0 || f.pow(b, q) == b {
                continue;
            }
            let e = Elliptic {
                f: f.clone(),
                c: [b, 0, a, 1],
                binary: true,
            };
            let pts = points(&e);
            let Some(i) = instance(&e, &pts, "subfield", m as u64) else {
                rejects.push(json!({"family":"binary","m":m,"a":a,"b":b,"order":pts.len(),"reason":"no usable odd-prime subgroup"}));
                continue;
            };
            let base: Vec<_> = xs.iter().flat_map(|&x| e.lift(x)).collect();
            let result = row(
                &e,
                &base,
                &pts,
                2,
                20261018 + q + found,
                json!({"family":"binary","q":q,"index":found}),
                Some(&i),
            );
            writeln!(rows, "{result}").unwrap();
            rows.flush().unwrap();
            write(
                dir.join(format!("binary-{q}-{found}.json")),
                &decomp_spec(
                    &i,
                    &format!("three-leads-binary-{q}-{found}"),
                    100 * q * q + 10000,
                    false,
                ),
            );
            fixtures.push(i);
            found += 1;
            eprintln!("binary q={q} curve={found}");
        }
        // Descended-b control, independent of the eligible primary cohort.
        let e = Elliptic {
            f: f.clone(),
            c: [1, 0, 3, 1],
            binary: true,
        };
        let pts = points(&e);
        let base: Vec<_> = xs.iter().flat_map(|&x| e.lift(x)).collect();
        writeln!(
            rows,
            "{}",
            row(
                &e,
                &base,
                &pts,
                2,
                20261018 + q,
                json!({"family":"binary_descended_control","q":q}),
                None
            )
        )
        .unwrap();
        rows.flush().unwrap();
    }
    for p in [5u64, 7] {
        for n in 2..=5 {
            let mut f = Field::new(p, n, odd_modulus(p, n));
            f.diagnostic_tables();
            for index in 0..3 {
                let mut a = p + 2 + index;
                let mut b = p + 3 + index;
                let e = loop {
                    let e = Elliptic {
                        f: f.clone(),
                        c: [b, a, 0, 1],
                        binary: false,
                    };
                    if f.add(f.mul(4, f.mul(f.sq(a), a)), f.mul(27 % p, f.sq(b))) != 0 {
                        break e;
                    }
                    a += 1;
                    b += 1;
                };
                let pts = points(&e);
                let base: Vec<_> = (0..p).flat_map(|x| e.lift(x)).collect();
                let inst = if n == 2 {
                    instance(&e, &pts, "subfield", 1)
                } else {
                    None
                };
                writeln!(
                    rows,
                    "{}",
                    row(
                        &e,
                        &base,
                        &pts,
                        n,
                        20261018 + p + n as u64 + index,
                        json!({"family":"odd","p":p,"n":n,"index":index}),
                        inst.as_ref()
                    )
                )
                .unwrap();
                rows.flush().unwrap();
                if let Some(i) = inst {
                    write(
                        dir.join(format!("odd-{p}-{index}.json")),
                        &decomp_spec(&i, &format!("three-leads-odd-{p}-{index}"), 100000, false),
                    );
                    fixtures.push(i);
                }
                eprintln!("odd p={p} n={n} curve={index}");
            }
        }
    }
    for family in ["pkm_kummer", "pkm_dickson"] {
        for depth in 2..=4 {
            for lo in [4096u64, 8192, 16384] {
                let mut count = 0;
                for p in lo..2 * lo {
                    if !is_prime(p) || (p - 1) % (1 << depth) != 0 {
                        continue;
                    }
                    let mut f = Field::new(p, 1, 0);
                    f.diagnostic_tables();
                    let e = Elliptic {
                        f: f.clone(),
                        c: [3, 2, 0, 1],
                        binary: false,
                    };
                    let pts = points(&e);
                    let Some(i) = instance(&e, &pts, family, depth) else {
                        rejects.push(json!({"family":family,"depth":depth,"lo":lo,"p":p,"reason":"no usable odd-prime subgroup"}));
                        continue;
                    };
                    let xs = tower_elements(&f, depth, family == "pkm_dickson");
                    let d = 1 << depth;
                    if family == "pkm_kummer" {
                        assert!(xs.iter().all(|&x| f.pow(x, d) == 1));
                    } else {
                        let dickson = |x| {
                            let (mut a, mut b) = (2, x);
                            for _ in 2..=d {
                                let c = f.sub(f.mul(x, b), a);
                                a = b;
                                b = c;
                            }
                            b
                        };
                        let c = dickson(xs[0]);
                        assert_eq!((0..p).filter(|&x| dickson(x) == c).collect::<Vec<_>>(), xs);
                    }
                    let base: Vec<_> = xs.iter().flat_map(|&x| e.lift(x)).collect();
                    writeln!(
                        rows,
                        "{}",
                        row(
                            &e,
                            &base,
                            &pts,
                            2,
                            20261018 + p,
                            json!({"family":family,"depth":depth,"lo":lo,"p":p,"index":count}),
                            Some(&i)
                        )
                    )
                    .unwrap();
                    rows.flush().unwrap();
                    write(
                        dir.join(format!("{family}-{depth}-{lo}-{p}.json")),
                        &decomp_spec(
                            &i,
                            &format!("three-leads-{family}-{depth}-{lo}-{p}"),
                            100 * p + 10000,
                            true,
                        ),
                    );
                    fixtures.push(i);
                    count += 1;
                    if count == 23 {
                        break;
                    }
                }
                assert_eq!(count, 23);
                eprintln!("PKM {family} depth={depth} lo={lo} done");
            }
        }
    }
    write(
        dir.join("fixtures.json"),
        &serde_json::to_value(fixtures).unwrap(),
    );
    write(dir.join("rejected.json"), &json!(rejects));
}

/// Complete finite-domain comparison with direct rational point pair sums.
/// Tables accelerate this correctness census only; no cost is taken from it.
pub fn verify_oracles(input: &str, output: &str) {
    let instances: Vec<StudyInstance> =
        serde_json::from_str(&fs::read_to_string(input).unwrap()).unwrap();
    let mut writer = BufWriter::new(fs::File::create(output).unwrap());
    for mut i in instances {
        i.curve.f.diagnostic_tables();
        let e = &i.curve;
        let pts = points(e);
        for random in [false, true] {
            if random && i.family == "subfield" {
                continue;
            }
            let base = build_factor_base(&i, random, 20261018);
            let expected = sumset(e, &base.raw, 2);
            let mut roots = 0;
            let mut found = 0;
            let oracle = if e.binary { "linearized" } else { "semaev" };
            for &p in &pts {
                let actual = decompose(e, &base, p, oracle, i.parameter, &mut roots);
                assert_eq!(
                    actual.is_some(),
                    expected.contains(&p),
                    "family={} p={} n={} a={} b={} random={} target={:?}",
                    i.family,
                    e.f.p,
                    e.f.n,
                    e.c[1],
                    e.c[0],
                    random,
                    p
                );
                if let Some(pair) = actual {
                    assert_eq!(e.add(pair[0], pair[1]), p);
                    found += 1;
                }
            }
            let zero_trace = if e.binary {
                let xs = subfield_elements(&e.f, 1 << i.parameter);
                assert!(xs.iter().all(|&x| trace(&e.f, x) == 0));
                Some(xs.len())
            } else {
                None
            };
            writeln!(writer,"{}",json!({"family":i.family,"p":e.f.p,"n":e.f.n,"a":if e.binary{e.c[2]}else{e.c[1]},"b":e.c[0],"parameter":i.parameter,"random_base":random,"tested_targets":pts.len(),"reachable":found,"root_calls":roots,"disagreements":0,"subfield_elements_with_zero_absolute_trace":zero_trace,"diagnostic_tables_only":true})).unwrap();
            writer.flush().unwrap();
        }
        eprintln!("oracle census {} p={} n={}", i.family, e.f.p, e.f.n);
    }
}

pub fn supplementary_controls(output: &str) {
    let mut writer = BufWriter::new(fs::File::create(output).unwrap());
    for m in 3..=9 {
        let q = 1 << m;
        let mut f = Field::new(2, 2 * m, binary_modulus(2 * m));
        f.diagnostic_tables();
        let a = (0..f.size()).find(|&a| trace(&f, a) == 1).unwrap();
        let generic_b = (1..f.size()).find(|&b| f.pow(b, q) != b).unwrap();
        for (b, family) in [
            (generic_b, "binary_coset1_control"),
            (1, "binary_descended_coset1_control"),
        ] {
            let e = Elliptic {
                f: f.clone(),
                c: [b, 0, a, 1],
                binary: true,
            };
            let pts = points(&e);
            let xs = subfield_elements(&f, q);
            let base: Vec<_> = xs.iter().flat_map(|&x| e.lift(x)).collect();
            let result = row(
                &e,
                &base,
                &pts,
                2,
                2026101922 + q,
                json!({"family":family,"q":q,"supplementary":true}),
                None,
            );
            writeln!(writer, "{result}").unwrap();
            writer.flush().unwrap();
        }
    }
    for p in [5, 7] {
        for n in 2..=5 {
            let mut f = Field::new(p, n, odd_modulus(p, n));
            f.diagnostic_tables();
            let e = Elliptic {
                f: f.clone(),
                c: [1, 1, 0, 1],
                binary: false,
            };
            let pts = points(&e);
            let base: Vec<_> = (0..p).flat_map(|x| e.lift(x)).collect();
            let result = row(
                &e,
                &base,
                &pts,
                n,
                2026101922 + p + n as u64,
                json!({"family":"odd_descended_control","p":p,"n":n,"supplementary":true}),
                None,
            );
            writeln!(writer, "{result}").unwrap();
            writer.flush().unwrap();
        }
    }
}

/// Unconditional uniform x-set null, separate from point-count-matched controls.
pub fn random_x_controls(input: &str, output: &str) {
    let rows: Vec<Value> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let mut writer = BufWriter::new(fs::File::create(output).unwrap());
    for (index, row) in rows.iter().enumerate() {
        let field = &row["field"];
        let mut f = Field::new(
            field["p"].as_u64().unwrap(),
            field["n"].as_u64().unwrap() as usize,
            field["modulus"].as_u64().unwrap(),
        );
        f.diagnostic_tables();
        let c: Vec<_> = row["coefficients"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect();
        let e = Elliptic {
            f: f.clone(),
            c: c.try_into().unwrap(),
            binary: f.p == 2,
        };
        let family = row["meta"]["family"].as_str().unwrap();
        let size = if family.starts_with("binary") {
            row["meta"]["q"].as_u64().unwrap()
        } else if family.starts_with("odd") {
            f.p
        } else {
            1 << row["meta"]["depth"].as_u64().unwrap()
        };
        let mut rng = Rng(2026101926 + index as u64);
        let mut counts = vec![];
        let mut base_sizes = vec![];
        let upper = u64::MAX - u64::MAX % f.size();
        for _ in 0..32 {
            let mut xs = HashSet::new();
            while xs.len() < size as usize {
                let draw = rng.next();
                if draw < upper {
                    xs.insert(draw % f.size());
                }
            }
            let mut xs: Vec<_> = xs.into_iter().collect();
            xs.sort_unstable();
            let base: Vec<_> = xs.iter().flat_map(|&x| e.lift(x)).collect();
            base_sizes.push(base.len());
            counts.push(sumset(&e, &base, row["summands"].as_u64().unwrap() as usize).len());
        }
        writeln!(writer,"{}",json!({"meta":row["meta"],"field":row["field"],"coefficients":row["coefficients"],"order":row["order"],"reachable":row["reachable"],"summands":row["summands"],"random_reachable":counts,"random_signed_base_sizes":base_sizes,"x_set_size":size,"null":"unconditional uniform x sets; actual lift count not conditioned","seed":2026101926+index as u64})).unwrap();
        writer.flush().unwrap();
    }
}
