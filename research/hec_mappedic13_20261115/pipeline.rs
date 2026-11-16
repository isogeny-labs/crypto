const ELL: u64 = 1028893;
fn mdot(a: &[u64], b: &[u64]) -> u64 {
    a.iter().zip(b).fold(0, |s, (&x, &y)| (s + x * y) % ELL)
}
fn relations(pts: &[[u64; 3]]) -> (Vec<u64>, usize, usize) {
    let lines = (0..P)
        .flat_map(|a| (0..P).map(move |b| [1, a, b]))
        .chain((0..P).map(|b| [0, 1, b]))
        .chain(std::iter::once([0, 0, 1]));
    let mut rows = Vec::new();
    let mut href = None;
    let mut supports = Vec::new();
    for l in lines {
        let hit: Vec<_> = pts
            .iter()
            .enumerate()
            .filter(|(_, p)| pdot(l, **p) == 0)
            .map(|(i, _)| i)
            .collect();
        if hit.len() != 4 {
            continue;
        }
        let h = href.get_or_insert_with(|| hit.clone());
        let mut r = vec![0; pts.len()];
        for &i in &hit {
            r[i] = (r[i] + 1) % ELL
        }
        for &i in h.iter() {
            r[i] = (r[i] + ELL - 1) % ELL
        }
        rows.push(r);
        supports.push(hit);
    }
    let width = pts.len() - 1;
    let mut pivots: Vec<Option<Vec<u64>>> = vec![None; width];
    let f = Fp::new(ELL);
    for original in &rows {
        let mut r = original[1..].to_vec();
        for col in 0..width {
            if r[col] == 0 {
                continue;
            }
            if let Some(p) = &pivots[col] {
                let a = r[col];
                for k in col..width {
                    r[k] = (r[k] + ELL - a * p[k] % ELL) % ELL
                }
            } else {
                let a = f.inv(r[col]);
                for v in &mut r[col..] {
                    *v = *v * a % ELL
                }
                pivots[col] = Some(r);
                break;
            }
        }
    }
    let rank = pivots.iter().filter(|r| r.is_some()).count();
    println!("RELATIONS points={} lines_examined={} split_lines={} anchored_columns={width} rank={rank} nullity={} reference={href:?}",pts.len(),P*P+P+1,rows.len(),width-rank);
    assert_eq!(rank, width - 1, "factor_base_relation_rank");
    let free = pivots.iter().position(|r| r.is_none()).unwrap();
    let mut w = vec![0; width];
    w[free] = 1;
    for col in (0..width).rev() {
        if let Some(r) = &pivots[col] {
            w[col] = (ELL - mdot(&r[col + 1..], &w[col + 1..])) % ELL
        }
    }
    let mut logs = vec![0];
    logs.extend(w);
    for r in &rows {
        assert_eq!(mdot(r, &logs), 0)
    }
    let mut broken = rows[0].clone();
    let c = logs.iter().position(|a| *a != 0).unwrap();
    broken[c] = (broken[c] + 1) % ELL;
    assert_ne!(mdot(&broken, &logs), 0);
    println!("LOG_KERNEL coefficients={logs:?} all_relations=true corrupt_row_rejected=true");
    // Principal-divisor controls on the first eight split lines, using actual classes.
    let tj = super::target::QuarticJacobian::new(P, QF).unwrap();
    let mut classes = HashMap::new();
    let mut sum = |ids: &[usize]| {
        let mut s = tj.zero();
        for &i in ids {
            let q = *classes
                .entry(i)
                .or_insert_with(|| super::target::difference(&tj, pts[i], pts[0], pts));
            s = tj.add_raw(&s, &q)
        }
        s
    };
    let reference = sum(href.as_ref().unwrap());
    for line in supports.iter().take(8) {
        assert_eq!(sum(line), reference)
    }
    println!("LINE_CONTROLS principal_divisor_checks=8 pass=true");
    (logs, rank, rows.len())
}
fn descend(
    j: &super::target::QuarticJacobian,
    x: super::target::QClass,
    pts: &[[u64; 3]],
    logs: &[u64],
    label: &str,
) -> u64 {
    let base = super::descent::effective(j, &j.zero(), pts).expect("rational_base_divisor");
    let base_log = base.iter().map(|i| logs[*i]).sum::<u64>() % ELL;
    let step = logs.iter().position(|a| *a != 0).unwrap();
    let delta = super::target::difference(j, pts[step], pts[0], pts);
    let mut q = x;
    for a in 0..=64u64 {
        if let Some(roots) = super::descent::effective(j, &q, pts) {
            let value = (roots.iter().map(|i| logs[*i]).sum::<u64>() + 2 * ELL
                - base_log
                - a * logs[step] % ELL)
                % ELL;
            assert_ne!(value, 0, "nonzero_prime_input_log");
            println!("DESCENT label={label} translates={a} roots={roots:?} base={base:?} step={step} step_log={} class_log={value}",logs[step]);
            return value;
        }
        println!("DESCENT_EXCLUDED label={label} translates={a} reason=not_three_distinct_rational_points");
        if a < 64 {
            q = j.add_raw(&q, &delta)
        }
    }
    panic!("descent_cap64")
}
pub fn run_pipeline() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("CAP aggregate600seconds");
        std::process::exit(124)
    });
    let args: Vec<_> = std::env::args().collect();
    let (c, refc, d1, ks0) = load_inputs(&args);
    let j = JC::new(c.field);
    jet_controls(&j);
    let ks = &ks0[1];
    let hs: Vec<_> = ks.iter().map(|p| construct(&c, p).unwrap()).collect();
    let neg: Vec<_> = ks
        .iter()
        .map(|p| ks.iter().position(|q| *q == c.neg(p)).unwrap())
        .collect();
    let atext = fs::read_to_string(&args[3]).unwrap();
    let alpha: Vec<E> = atext
        .lines()
        .filter(|l| l.starts_with("ALPHA 1 "))
        .map(|l| {
            let n: Vec<u64> = l
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect();
            std::array::from_fn(|i| n[i + 2])
        })
        .collect();
    assert_eq!(alpha.len(), 27);
    let chars = chars3(&c.field);
    let t = refc.mul(&refc.place(0, 44), 1028893);
    assert!(!refc.is_zero(&t));
    assert!(refc.is_zero(&refc.mul(&t, 2)));
    assert_eq!(t.v, Poly::zero());
    let td = Div {
        u: Pol::embed(&t.u),
        v: Pol::embed(&t.v),
    };
    let w0 = c.f.exact(&td.u, &c.field);
    let wi = inverse(&w0, &td.u, &c.field).unwrap();
    let u0 = JP::embed(&td.u, &j);
    let w = JP::embed(&w0, &j);
    let vin = JP(vec![j.var(0), j.var(1), j.var(2)]);
    let v2 = vin.mul(&vin, &j);
    let u2 = v2.neg(&j).mul(&JP::embed(&wi, &j), &j).rem(&u0, &j);
    let w2 = v2.neg(&j).sub(&u2.mul(&w, &j), &j).exact(&u0, &j);
    let u4 = u2
        .mul(&w2, &j)
        .neg(&j)
        .mul(&JP::embed(&wi, &j), &j)
        .rem(&u0, &j);
    let u = u0.add(&u2, &j).add(&u4, &j);
    let pts = super::target::rational_points(&QF, P);
    let tj = super::target::QuarticJacobian::new(P, QF).unwrap();
    let mapper = Mapper {
        c: &c,
        refc: &refc,
        td: &td,
        u: &u,
        v: &vin,
        j: &j,
        ks,
        hs: &hs,
        neg: &neg,
        alpha: &alpha,
        chars: &chars,
        pts: &pts,
        tj: &tj,
    };
    let places = hec_native_ic::instance::factor_base(&refc);
    let (bx, by) = places[1];
    println!("FIXTURE_V2 B_place=({bx},{by})");
    let b = refc.mul(&refc.place(bx, by), 2);
    assert!(!refc.is_zero(&b));
    assert_ne!(b, d1);
    println!(
        "SOURCE_INPUT generator_u={:?} generator_v={:?} target_u={:?} target_v={:?}",
        d1.u, d1.v, b.u, b.v
    );
    let g = mapper.map(&d1, "D1");
    let q = mapper.map(&b, "B");
    let (logs, rank, count) = relations(&pts);
    let gl = descend(&tj, g, &pts, &logs, "D1");
    let ql = descend(&tj, q, &pts, &logs, "B");
    let k = ql * Fp::new(ELL).inv(gl) % ELL;
    assert_eq!(
        super::target::scalar(&tj, g, k),
        q,
        "destination_scalar_check"
    );
    assert_eq!(refc.mul(&d1, k), b, "source_scalar_check");
    let wrong = (k + 1) % ELL;
    assert_ne!(refc.mul(&d1, wrong), b);
    assert_ne!(super::target::scalar(&tj, g, wrong), q);
    println!("RECOVERED k={k} source_verified=true destination_verified=true wrong_scalar_rejected=true relation_rank={rank} split_lines={count}");
    let mut params = hec_native_ic::ic::IcParams::default();
    params.max_trials = 200000;
    let control = hec_native_ic::ic::solve_dlp(&refc, &d1, &b, ELL, &params);
    assert_eq!(control.k, Some(k));
    println!(
        "HYPERELLIPTIC_CONTROL seed={} trials={} relations={} k={:?} same_source_target=true",
        params.seed, control.trials, control.relations, control.k
    );
    println!(
        "PASS source_to_quartic_index_calculus=true source_verified=true timing_comparison=false"
    );
}
