fn poly_power(a: &Poly, n: u8, fp: &Fp) -> Poly {
    (0..n).fold(Poly::one(), |v, _| v.mul(a, fp))
}
fn restrict(x: &Poly, y: &Poly, fp: &Fp) -> Poly {
    super::target::monomials(4)
        .iter()
        .zip(QF)
        .fold(Poly::zero(), |s, (&(a, b, _), v)| {
            s.add(
                &poly_power(x, a, fp)
                    .mul(&poly_power(y, b, fp), fp)
                    .scale(v, fp),
                fp,
            )
        })
}
fn rational_roots(g: &Poly, fp: &Fp) -> Vec<u64> {
    assert!(!g.is_zero());
    if g.deg() <= 0 {
        return vec![];
    }
    let x = Poly::x();
    let h = Poly::gcd(g, &x.powmod(P, g, fp).sub(&x, fp), fp);
    let roots =
        hec_native_ic::oracle::split(&h, fp, &mut Default::default()).expect("linear_part_split");
    roots.as_slice().iter().map(|a| a.0).collect()
}
fn fast_points() -> Vec<[u64; 3]> {
    let fp = Fp::new(P);
    let x = Poly::x();
    let mut out = Vec::new();
    for a in 0..P {
        let g = restrict(&Poly::from_coeffs(&[a]), &x, &fp);
        for b in rational_roots(&g, &fp) {
            out.push([a, b, 1])
        }
    }
    let mut inf = Poly::zero();
    for (&(a, _, z), v) in super::target::monomials(4).iter().zip(QF) {
        if z == 0 {
            inf.c[a as usize] = (inf.c[a as usize] + v) % P;
            inf.n = inf.n.max(a as usize + 1)
        }
    }
    for a in rational_roots(&inf, &fp) {
        out.push([a, 1, 0])
    }
    out
}
fn pair_intersections(p: [u64; 3], q: [u64; 3], fp: &Fp) -> Option<Vec<[u64; 3]>> {
    assert_eq!(p[2], 1);
    assert_eq!(q[2], 1);
    let t = Poly::x();
    let (x, y, aa, bb, infinity) = if p[0] != q[0] {
        let a = fp.sub(q[1], p[1]) * fp.inv(fp.sub(q[0], p[0])) % P;
        let b = fp.sub(p[1], a * p[0] % P);
        (
            t,
            Poly::from_coeffs(&[b, a]),
            p[0],
            q[0],
            if a == 0 { [1, 0, 0] } else { [fp.inv(a), 1, 0] },
        )
    } else {
        (Poly::from_coeffs(&[p[0]]), t, p[1], q[1], [0, 1, 0])
    };
    let f = restrict(&x, &y, fp);
    let den = Poly::from_coeffs(&[fp.neg(aa), 1]).mul(&Poly::from_coeffs(&[fp.neg(bb), 1]), fp);
    let (res, rem) = f.divrem(&den, fp);
    assert!(rem.is_zero());
    assert!(res.deg() <= 2);
    let roots = hec_native_ic::oracle::split(&res.monic(fp), fp, &mut Default::default())?;
    if roots.as_slice().iter().any(|a| a.1 != 1) {
        return None;
    }
    let mut hits = vec![p, q];
    for &(r, _) in roots.as_slice() {
        hits.push([x.eval(r, fp), y.eval(r, fp), 1])
    }
    if res.deg() == 1 {
        hits.push(infinity)
    } else if res.deg() != 2 {
        return None;
    }
    hits.sort_unstable();
    hits.dedup();
    (hits.len() == 4).then_some(hits)
}
fn small_kernel(rows: &[Vec<u64>], width: usize) -> Vec<u64> {
    let fp = Fp::new(ELL);
    let mut piv: Vec<Option<Vec<u64>>> = vec![None; width];
    for orig in rows {
        let mut r = orig.clone();
        for c in 0..width {
            if r[c] == 0 {
                continue;
            }
            if let Some(p) = &piv[c] {
                let a = r[c];
                for k in c..width {
                    r[k] = (r[k] + ELL - a * p[k] % ELL) % ELL
                }
            } else {
                let a = fp.inv(r[c]);
                for x in &mut r[c..] {
                    *x = *x * a % ELL
                }
                piv[c] = Some(r);
                break;
            }
        }
    }
    let rank = piv.iter().filter(|p| p.is_some()).count();
    println!(
        "SMALL_MATRIX rows={} columns={width} rank={rank} nullity={}",
        rows.len(),
        width - rank
    );
    assert_eq!(rank, width - 1);
    let mut w = vec![0; width];
    w[piv.iter().position(|p| p.is_none()).unwrap()] = 1;
    for c in (0..width).rev() {
        if let Some(p) = &piv[c] {
            w[c] = (ELL - mdot(&p[c + 1..], &w[c + 1..])) % ELL
        }
    }
    for r in rows {
        assert_eq!(mdot(r, &w), 0)
    }
    w
}
fn pair_relations(pts: &[[u64; 3]], reference: &[u64]) -> Vec<Option<u64>> {
    let fp = Fp::new(P);
    let m = 48usize;
    assert!(m < pts.len());
    assert!(pts[..m].iter().all(|p| p[2] == 1));
    let h = [0usize, 1, 2, 3];
    let hline = [1, 0, 0];
    assert!(h.iter().all(|i| pdot(hline, pts[*i]) == 0));
    let lookup: HashMap<_, _> = pts.iter().enumerate().map(|(i, p)| (*p, i)).collect();
    let mut seen = std::collections::HashSet::new();
    let mut full = Vec::new();
    let mut pairs = 0;
    let mut duplicate = 0;
    let mut rejected = 0;
    let mut lp_counts = [0usize; 3];
    for a in 0..m {
        for b in a + 1..m {
            pairs += 1;
            let mut l = [
                fp.sub(pts[a][1], pts[b][1]),
                fp.sub(pts[b][0], pts[a][0]),
                fp.sub(pts[a][0] * pts[b][1] % P, pts[a][1] * pts[b][0] % P),
            ];
            let inv = fp.inv(*l.iter().find(|a| **a != 0).unwrap());
            for x in &mut l {
                *x = *x * inv % P
            }
            if !seen.insert(l) {
                duplicate += 1;
                continue;
            }
            let Some(hit) = pair_intersections(pts[a], pts[b], &fp) else {
                rejected += 1;
                continue;
            };
            assert!(hit.iter().all(|p| pdot(l, *p) == 0));
            let ids: Vec<_> = hit
                .iter()
                .map(|p| *lookup.get(p).expect("point_enumeration_complete"))
                .collect();
            let nl = ids.iter().filter(|i| **i >= m).count();
            assert!(nl <= 2);
            lp_counts[nl] += 1;
            let mut row = vec![0; pts.len()];
            for &i in &ids {
                row[i] = (row[i] + 1) % ELL
            }
            for i in h {
                row[i] = (row[i] + ELL - 1) % ELL
            }
            assert_eq!(mdot(&row, reference), 0);
            full.push(row);
        }
    }
    let mont = super::lp_trace::Mont::new(ELL);
    let sparse: Vec<Vec<(u32, u64)>> = full
        .iter()
        .map(|r| {
            r.iter()
                .enumerate()
                .skip(1)
                .filter(|(_, v)| **v != 0)
                .map(|(i, v)| ((i - 1) as u32, mont.to(*v)))
                .collect()
        })
        .collect();
    let (mut ops, mut entries) = (0, 0);
    let (fb, trace) =
        super::lp_trace::eliminate_lp_trace(&mont, m - 1, sparse.clone(), &mut ops, &mut entries);
    let (mut rops, mut rentries) = (0, 0);
    let rm = super::lp_reference::Mont::new(ELL);
    let rfb = super::lp_reference::eliminate_lp(&rm, m - 1, sparse, &mut rops, &mut rentries);
    assert_eq!(fb, rfb);
    assert_eq!((ops, entries), (rops, rentries));
    let dense: Vec<_> = fb
        .iter()
        .map(|r| {
            let mut d = vec![0; m - 1];
            for &(c, v) in r {
                assert!((c as usize) < m - 1);
                d[c as usize] = mont.from(v)
            }
            d
        })
        .collect();
    let w = small_kernel(&dense, m - 1);
    let mut known = vec![None; pts.len()];
    known[0] = Some(0);
    for (i, v) in w.into_iter().enumerate() {
        known[i + 1] = Some(v)
    }
    for (c, r) in trace.iter().rev() {
        let mut value = 0;
        let mut ok = true;
        for &(cc, a) in r {
            if cc == *c {
                continue;
            }
            if let Some(v) = known[cc as usize + 1] {
                value = (value + mont.from(a) * v) % ELL
            } else {
                ok = false;
                break;
            }
        }
        if ok {
            known[*c as usize + 1] = Some((ELL - value) % ELL)
        }
    }
    let anchor = (1..m).find(|i| reference[*i] != 0).unwrap();
    let scale = known[anchor].unwrap() * Fp::new(ELL).inv(reference[anchor]) % ELL;
    assert_ne!(scale, 0);
    for (i, v) in known.iter().enumerate() {
        if let Some(v) = v {
            assert_eq!(*v, reference[i] * scale % ELL)
        }
    }
    let mut checked = 0;
    for r in &full {
        if r.iter().zip(&known).all(|(a, b)| *a == 0 || b.is_some()) {
            let sum = r
                .iter()
                .zip(&known)
                .fold(0, |s, (a, b)| (s + a * b.unwrap_or(0)) % ELL);
            assert_eq!(sum, 0);
            checked += 1
        }
    }
    println!("PAIR_COLLECTOR small_base={m} pairs={pairs} unique_lines={} duplicate_lines={duplicate} rejected={rejected} split_lines={} lp0={} lp1={} lp2={} eliminated={} known={} residual_checks={checked} reference_scale={scale} sparse_ops={ops} sparse_entries={entries} trace_matches_reference=true",seen.len(),full.len(),lp_counts[0],lp_counts[1],lp_counts[2],trace.len(),known.iter().filter(|v|v.is_some()).count());
    println!("KNOWN_LOGS values={known:?}");
    known
}
fn descend_known(
    j: &super::target::QuarticJacobian,
    x: super::target::QClass,
    pts: &[[u64; 3]],
    logs: &[Option<u64>],
    label: &str,
) -> u64 {
    let base = super::descent::effective(j, &j.zero(), pts).unwrap();
    let base_log = base.iter().map(|i| logs[*i].unwrap()).sum::<u64>() % ELL;
    let step = logs.iter().position(|v| v.is_some_and(|x| x != 0)).unwrap();
    let sl = logs[step].unwrap();
    let delta = super::target::difference(j, pts[step], pts[0], pts);
    let mut q = x;
    for a in 0..=64 {
        if let Some(roots) = super::descent::effective(j, &q, pts) {
            if roots.iter().all(|i| logs[*i].is_some()) {
                let v = (roots.iter().map(|i| logs[*i].unwrap()).sum::<u64>() + 2 * ELL
                    - base_log
                    - a * sl % ELL)
                    % ELL;
                assert_ne!(v, 0);
                println!("PAIR_DESCENT label={label} translates={a} roots={roots:?} log={v}");
                return v;
            }
        }
        println!("PAIR_DESCENT_EXCLUDED label={label} translates={a}");
        if a < 64 {
            q = j.add_raw(&q, &delta)
        }
    }
    panic!("known_descent_cap")
}
pub fn run_pair() {
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
    let pts = fast_points();
    assert_eq!(pts, super::target::rational_points(&QF, P));
    println!(
        "POINTS fast_count={} affine_polynomials=127 exact_reference_match=true",
        pts.len()
    );
    let tj = super::target::QuarticJacobian::with_points(P, QF, pts.clone()).unwrap();
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
    let (reference, _, _) = relations(&pts);
    let logs = pair_relations(&pts, &reference);
    let g = mapper.map(&d1, "D1");
    let q = mapper.map(&b, "B");
    let gl = descend_known(&tj, g, &pts, &logs, "D1");
    let ql = descend_known(&tj, q, &pts, &logs, "B");
    let k = ql * Fp::new(ELL).inv(gl) % ELL;
    assert_eq!(refc.mul(&d1, k), b);
    assert_eq!(super::target::scalar(&tj, g, k), q);
    println!("RECOVERED k={k} source_verified=true destination_verified=true");
    println!("PASS pair_large_prime_ic=true exact_reference_checks=true timing_comparison=false");
}
